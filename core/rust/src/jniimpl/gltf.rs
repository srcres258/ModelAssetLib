extern crate jni;
extern crate gltf;
extern crate anyhow;

use jni::JNIEnv;
use jni::objects::{JByteArray, JObject, JValue, JValueOwned};
use anyhow::Result;
use gltf::buffer;
use jni::sys::jbyte;
use crate::util;

pub struct LoadedGltfBuffer {
    uri: String,
    data: Vec<u8>
}

pub struct LoadedGltf {
    buffers: Vec<LoadedGltfBuffer>
}

impl LoadedGltfBuffer {
    fn new(uri: String, data: Vec<u8>) -> LoadedGltfBuffer {
        LoadedGltfBuffer {
            uri,
            data
        }
    }
}

impl LoadedGltf {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new()
        }
    }

    pub fn get_buffers(&self) -> &Vec<LoadedGltfBuffer> {
        &self.buffers
    }

    pub fn get_buffers_mut(&mut self) -> &mut Vec<LoadedGltfBuffer> {
        &mut self.buffers
    }
}

pub fn get_native_callback<'a>(env: &mut JNIEnv<'a>, this: &JObject<'a>) -> Result<JObject<'a>> {
    let callback = env.get_field(this, "nativeCallback", "Ltop/srcres/mods/modelassetlib/gltf/Gltf$NativeCallback;");
    match callback {
        Ok(callback) => {
            Ok(callback.l().unwrap())
        }
        Err(err) => {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(env, &format!("Failed to obtain nativeCallback: {}", err)).unwrap();
            Err(err.into())
        }
    }
}

pub fn invoke_native_callback<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>,
    name: &'a str,
    sig: &'a str,
    args: &[JValue]
) -> Result<JValueOwned<'a>> {
    let callback = get_native_callback(env, this)?;
    let result = env.call_method(callback, name, sig, args);
    match result {
        Ok(result) => {
            Ok(result)
        }
        Err(err) => {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(env, &format!(
                "Failed to invoke {} {}: {}", name, sig, err)).unwrap();
            Err(err.into())
        }
    }
}

pub fn invoke_native_callback_no_args<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>,
    name: &'a str,
    sig: &'a str
) -> Result<JValueOwned<'a>> {
    invoke_native_callback(env, this, name, sig, &[])
}

pub fn get_initial_gltf_data<'a>(env: &mut JNIEnv<'a>, this: &JObject<'a>) -> Result<JByteArray<'a>> {
    let callback = get_native_callback(env, this)?;
    let gltf_data = env.call_method(callback, "getInitialGltfData", "()[B", &[]);
    match gltf_data {
        Ok(gltf_data) => {
            Ok(JByteArray::from(gltf_data.l().unwrap()))
        }
        Err(err) => {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(env, &format!("Failed to invoke getInitialGltfData: {}", err)).unwrap();
            Err(err.into())
        }
    }
}

fn init_gltf<'a>(env: &mut JNIEnv<'a>, this: &JObject<'a>) {
    let gltf_data = get_initial_gltf_data(env, this);
    match gltf_data {
        Ok(gltf_data) => {
            let gltf_data_len = env.get_array_length(&gltf_data).unwrap();
            let mut gltf_data_vec: Vec<jbyte> = Vec::with_capacity(gltf_data_len as usize);
            for _ in 0..gltf_data_len {
                gltf_data_vec.push(0);
            }
            env.get_byte_array_region(&gltf_data, 0, gltf_data_vec.as_mut_slice()).unwrap();
            let gltf_data_u8: Vec<_> = gltf_data_vec.iter().map(|it| *it as u8).collect();
            let gltf_obj = gltf::Gltf::from_slice(gltf_data_u8.as_slice());
            match gltf_obj {
                Ok(gltf_obj) => {
                    unsafe {
                        env.set_rust_field(this, "rust_gltfObj", gltf_obj).unwrap()
                    }
                }
                Err(err) => {
                    util::jni::clear_exception_if_occurred(env);
                    util::jni::throw_runtime_exception(env, &format!("Failed to create the glTF object: {}", err)).unwrap();
                }
            }
        }
        Err(_) => {
            ()
        }
    }
}

pub fn load_gltf<'a>(env: &mut JNIEnv<'a>, this: &JObject<'a>) {
    let gltf_obj: gltf::Gltf;
    unsafe {
        gltf_obj = env.take_rust_field(this, "rust_gltfObj").unwrap();
    }

    let mut loaded_gltf = LoadedGltf::new();
    gltf_obj.buffers().for_each(|it| {
        if let buffer::Source::Uri(uri) = it.source() {
            let uri_jstr = env.new_string(uri).unwrap();
            let data_arr = invoke_native_callback(
                env, this, "loadBufferFromURI", "(Ljava/lang/String;)[B",
                &[JValue::Object(&uri_jstr)]);
            match data_arr {
                Ok(data_arr) => {
                    let data_arr = JByteArray::from(data_arr.l().unwrap());
                    let data_arr_len = env.get_array_length(&data_arr).unwrap();
                    let mut data = Vec::with_capacity(data_arr_len as usize);
                    for _ in 0..data_arr_len {
                        data.push(0);
                    }
                    env.get_byte_array_region(data_arr, 0, data.as_mut_slice()).unwrap();
                    let buf = LoadedGltfBuffer::new(String::from(uri), data.iter().map(|x| *x as u8).collect());
                    loaded_gltf.get_buffers_mut().push(buf);
                }
                Err(err) => {
                    util::jni::clear_exception_if_occurred(env);
                    util::jni::throw_runtime_exception(env, &format!("Failed to load glTF buffer: {}", err)).unwrap();
                    return;
                }
            }
        }
    });

    unsafe {
        env.set_rust_field(this, "rust_loadedGltfObj", loaded_gltf).unwrap_or_else(|err| {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(env, &format!("Failed to set rust object rust_loadedGltfObj: {}", err)).unwrap()
        });
        env.set_rust_field(this, "rust_gltfObj", gltf_obj).unwrap_or_else(|err| {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(env, &format!("Failed to set rust object rust_gltfObj: {}", err)).unwrap()
        });
    }
}

pub fn handle_native_init<'a>(env: &mut JNIEnv<'a>, this: &JObject<'a>) {
    init_gltf(env, this);
    load_gltf(env, this);
}

pub fn handle_native_destroy<'a>(env: &mut JNIEnv<'a>, this: &JObject<'a>) {
    unsafe {
        let gltf_obj: gltf::Gltf = env.take_rust_field(&this, "rust_gltfObj").unwrap();
        drop(gltf_obj);
    }
}
