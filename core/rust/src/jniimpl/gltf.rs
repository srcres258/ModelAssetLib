extern crate jni;
extern crate gltf;
extern crate anyhow;

use jni::JNIEnv;
use jni::objects::{JByteArray, JObject, JString, JValue, JValueOwned};
use anyhow::Result;
use gltf::{buffer, image};
use jni::sys::{jbyte, jbyteArray, jsize};
use crate::util;
use std::sync::{Arc, Mutex};
use gltf::buffer::View;

pub struct LoadedGltfBuffer {
    gltf: Arc<Mutex<LoadedGltf>>,
    index: usize,
    uri: String,
    data: Vec<u8>
}

pub struct LoadedGltfBufferView {
    gltf: Arc<Mutex<LoadedGltf>>,
    index: usize,
    buffer_index: usize,
    data_offset: usize,
    data_length: usize,
    data_stride: Option<usize>,
    target: Option<buffer::Target>
}

pub struct LoadedGltfImage {
    gltf: Arc<Mutex<LoadedGltf>>,
    index: usize,
    uri: String,
    data: Vec<u8>
}

pub struct LoadedGltf {
    buffers: Vec<LoadedGltfBuffer>,
    buffer_views: Vec<LoadedGltfBufferView>,
    images: Vec<LoadedGltfImage>
}

pub struct LoadedGltfWrapper {
    gltf: Arc<Mutex<LoadedGltf>>
}

impl LoadedGltfBuffer {
    fn new(gltf: &Arc<Mutex<LoadedGltf>>, index: usize, uri: String, data: Vec<u8>) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            uri,
            data
        }
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_uri(&self) -> &String {
        &self.uri
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn get_data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

impl LoadedGltfBufferView {
    fn new(
        gltf: &Arc<Mutex<LoadedGltf>>,
        index: usize,
        buffer_index: usize,
        data_offset: usize,
        data_length: usize,
        data_stride: Option<usize>,
        target: Option<buffer::Target>
    ) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            buffer_index,
            data_offset,
            data_length,
            data_stride,
            target
        }
    }

    fn new_from_view(gltf: &Arc<Mutex<LoadedGltf>>, view: &View) -> Self {
        Self::new(gltf, view.index(), view.buffer().index(), view.offset(),
                  view.length(), view.stride(), view.target())
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_data_offset(&self) -> usize {
        self.data_offset
    }

    fn get_data_length(&self) -> usize {
        self.data_length
    }

    fn get_data_stride(&self) -> Option<usize> {
        self.data_stride
    }

    fn get_target(&self) -> Option<buffer::Target> {
        self.target
    }

    fn load_data(&self) -> Vec<u8> {
        self.load_data_strided(0)
    }

    fn load_data_strided(&self, stride_count: u32) -> Vec<u8> {
        let gltf = self.gltf.lock().unwrap();
        let buffer = gltf.get_buffers().get(self.buffer_index).unwrap();
        let mut result = util::new_buffer_vec(self.data_length, 0u8);
        let result_ptr = result.as_mut_ptr();
        unsafe {
            let data_ptr = buffer.get_data_ptr().add(self.data_offset)
                .add((stride_count as usize) * self.data_stride.unwrap_or(0));
            let mut result_ptrm = result_ptr;
            let mut data_ptrm = data_ptr;
            for _ in 0..self.data_length {
                *result_ptrm = *data_ptrm;
                result_ptrm = result_ptrm.add(1);
                data_ptrm = data_ptrm.add(1);
            }
        }
        result
    }
}

impl LoadedGltfImage {
    fn new(gltf: &Arc<Mutex<LoadedGltf>>, index: usize, uri: String, data: Vec<u8>) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            uri,
            data
        }
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_uri(&self) -> &String {
        &self.uri
    }
}

impl LoadedGltf {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            buffer_views: Vec::new(),
            images: Vec::new()
        }
    }

    pub fn get_buffers(&self) -> &Vec<LoadedGltfBuffer> {
        &self.buffers
    }

    pub fn get_buffers_mut(&mut self) -> &mut Vec<LoadedGltfBuffer> {
        &mut self.buffers
    }

    pub fn get_buffer_views(&self) -> &Vec<LoadedGltfBufferView> {
        &self.buffer_views
    }

    pub fn get_buffer_views_mut(&mut self) -> &mut Vec<LoadedGltfBufferView> {
        &mut self.buffer_views
    }

    pub fn get_images(&self) -> &Vec<LoadedGltfImage> {
        &self.images
    }

    pub fn get_images_mut(&mut self) -> &mut Vec<LoadedGltfImage> {
        &mut self.images
    }
}

impl LoadedGltfWrapper {
    fn new(gltf: LoadedGltf) -> Self {
        Self {
            gltf: Arc::new(Mutex::new(gltf))
        }
    }

    fn get(&self) -> &Arc<Mutex<LoadedGltf>> {
        &self.gltf
    }
}

pub fn get_native_callback<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) -> Result<JObject<'a>> {
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

pub fn get_initial_gltf_data<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) -> Result<JByteArray<'a>> {
    let callback = get_native_callback(env, this)?;
    Ok(JByteArray::from(env.call_method(callback, "getInitialGltfData", "()[B", &[])?.l()?))
}

fn init_gltf<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) {
    let gltf_data = get_initial_gltf_data(env, this);
    match gltf_data {
        Ok(gltf_data) => {
            let gltf_data_len = env.get_array_length(&gltf_data).unwrap();
            let mut gltf_data_vec: Vec<jbyte> = util::new_buffer_vec(gltf_data_len as usize, 0);
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

pub fn load_gltf<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) {
    let gltf_obj: gltf::Gltf;
    unsafe {
        gltf_obj = env.take_rust_field(this, "rust_gltfObj").unwrap();
    }

    let loaded_gltf = LoadedGltf::new();
    let loaded_gltf_wrapper = LoadedGltfWrapper::new(loaded_gltf);

    // Load buffers.
    gltf_obj.buffers().for_each(|it| {
        let mut loaded_gltf = loaded_gltf_wrapper.get().lock().unwrap();
        if let buffer::Source::Uri(uri) = it.source() {
            let uri_jstr = env.new_string(uri).unwrap();
            let data_arr = invoke_native_callback(
                env, this, "loadBufferFromURI", "(Ljava/lang/String;)[B",
                &[JValue::Object(&uri_jstr)]);
            match data_arr {
                Ok(data_arr) => {
                    let data_arr = JByteArray::from(data_arr.l().unwrap());
                    let data_arr_len = env.get_array_length(&data_arr).unwrap();
                    let mut data = util::new_buffer_vec(data_arr_len as usize, 0);
                    env.get_byte_array_region(data_arr, 0, data.as_mut_slice()).unwrap();
                    let buf = LoadedGltfBuffer::new(
                        loaded_gltf_wrapper.get(), it.index(), String::from(uri),
                        data.iter().map(|x| *x as u8).collect());
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

    // Load buffer views.
    gltf_obj.views().for_each(|it| {
        let mut loaded_gltf = loaded_gltf_wrapper.get().lock().unwrap();
        let loaded_buffer_view = LoadedGltfBufferView::new_from_view(
            loaded_gltf_wrapper.get(), &it);
        loaded_gltf.get_buffer_views_mut().push(loaded_buffer_view);
    });

    // Load images.
    gltf_obj.images().for_each(|it| {
        let mut loaded_gltf = loaded_gltf_wrapper.get().lock().unwrap();
        if let image::Source::Uri { uri, mime_type } = it.source() {
            let mime_type = mime_type.unwrap_or("");
            let uri_jstr = env.new_string(uri).unwrap();
            let mime_type_jstr = env.new_string(mime_type).unwrap();
            let data_arr = invoke_native_callback(
                env, this, "loadImageFromURI", "(Ljava/lang/String;Ljava/lang/String;)[B",
                &[JValue::Object(&uri_jstr), JValue::Object(&mime_type_jstr)]);
            match data_arr {
                Ok(data_arr) => {
                    let data_arr = JByteArray::from(data_arr.l().unwrap());
                    let data_arr_len = env.get_array_length(&data_arr).unwrap();
                    let mut data = util::new_buffer_vec(data_arr_len as usize, 0);
                    env.get_byte_array_region(data_arr, 0, data.as_mut_slice()).unwrap();
                    let img = LoadedGltfImage::new(
                        loaded_gltf_wrapper.get(), it.index(), String::from(uri),
                        data.iter().map(|x| *x as u8).collect());
                    loaded_gltf.get_images_mut().push(img);
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
        env.set_rust_field(this, "rust_loadedGltfObj", loaded_gltf_wrapper).unwrap_or_else(|err| {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(env, &format!("Failed to set rust object rust_loadedGltfObj: {}", err)).unwrap()
        });
        env.set_rust_field(this, "rust_gltfObj", gltf_obj).unwrap_or_else(|err| {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(env, &format!("Failed to set rust object rust_gltfObj: {}", err)).unwrap()
        });
    }
}

pub fn announce_gltf_image_uris<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) {
    let gltf_obj: gltf::Gltf;
    unsafe {
        gltf_obj = env.take_rust_field(this, "rust_gltfObj").unwrap();
    }

    gltf_obj.images().for_each(|it| {
        if let image::Source::Uri { uri, mime_type: _ } = it.source() {
            let uri_jstr = env.new_string(uri).unwrap();
            invoke_native_callback(env, this, "receiveImageURI", "(Ljava/lang/String;)V",
                                   &[JValue::Object(&uri_jstr)]).unwrap();
        }
    });

    unsafe {
        env.set_rust_field(this, "rust_gltfObj", gltf_obj).unwrap_or_else(|err| {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(env, &format!("Failed to set rust object rust_gltfObj: {}", err)).unwrap()
        });
    }
}

pub fn handle_native_init<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) {
    init_gltf(env, this);
    load_gltf(env, this);
    announce_gltf_image_uris(env, this);
}

pub fn handle_native_destroy<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) {
    unsafe {
        let gltf_obj: gltf::Gltf = env.take_rust_field(&this, "rust_gltfObj").unwrap();
        drop(gltf_obj);
    }
}

pub fn handle_get_image_data_by_uri<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>,
    uri_jstr: &JString
) -> jbyteArray {
    let loaded_gltf_obj: LoadedGltfWrapper;
    unsafe {
        loaded_gltf_obj = env.take_rust_field(this, "rust_loadedGltfObj").unwrap();
    }

    let uri = String::from(env.get_string(uri_jstr).unwrap());
    let mut target_img: Option<&LoadedGltfImage> = None;
    let loaded_gltf = loaded_gltf_obj.get().lock().unwrap();
    for image in loaded_gltf.get_images() {
        if image.uri == uri {
            target_img = Some(image);
            break;
        }
    }

    let mut result: Vec<u8> = Vec::new();
    if let Some(image) = target_img {
        result = Vec::with_capacity(image.data.len());
        image.data.iter().for_each(|x| result.push(*x));
    }
    
    drop(loaded_gltf);

    unsafe {
        env.set_rust_field(this, "rust_loadedGltfObj", loaded_gltf_obj).unwrap_or_else(|err| {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(env, &format!("Failed to set rust object rust_loadedGltfObj: {}", err)).unwrap()
        });
    }

    let jresult = env.new_byte_array(result.len() as jsize).unwrap();
    let result_jbyte: Vec<jbyte> = result.iter().map(|x| *x as jbyte).collect();
    env.set_byte_array_region(&jresult, 0, result_jbyte.as_slice()).unwrap();
    jresult.as_raw()
}
