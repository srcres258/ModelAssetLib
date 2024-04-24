extern crate jni;
extern crate gltf;
extern crate anyhow;

use jni::JNIEnv;
use jni::objects::{JByteArray, JObject, JString, JValue, JValueOwned};
use anyhow::Result;
use gltf::{buffer, image};
use jni::sys::{jbyte, jbyteArray, jsize};
use crate::util;
use crate::util::gltf::{LoadedGltfAccessor, LoadedGltf, LoadedGltfBuffer, LoadedGltfWrapper, LoadedGltfImage, LoadedGltfBufferView, LoadedGltfSampler, LoadedGltfTexture, LoadedGltfMaterial, LoadedGltfMesh};

pub fn get_native_callback<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) -> Result<JObject<'a>> {
    let callback = env.get_field(
        this, "nativeCallback",
        "Ltop/srcres/mods/modelassetlib/gltf/Gltf$NativeCallback;");
    match callback {
        Ok(callback) => {
            Ok(callback.l().unwrap())
        }
        Err(err) => {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(
                env, &format!("Failed to obtain nativeCallback: {}", err)).unwrap();
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

    /*
    The glTF data hierarchy:
    (details: Page 1 at https://www.khronos.org/files/gltf20-reference-guide.pdf)

    scene
    |
    *-> node <-------------------------*<----------------\
        |                              |                 |
        *-> camera                     |                 |
        |                              |                 |
        *-> mesh                       |                 |
            |                          |  #############  |
            *-> material               |  #  Skins    #<-/
            |   |                  /------#############
            |   *-> texture        |   |
            |       |              |   |  #############
            |       *-> sampler <--/   \--# Animation #
            |       |                     #############
            |       *-> image                   |
            |                                   |
            *-> accessor <----------------------/
                |
                *-> buffer view
                    |
                    *-> buffer
     */

    /*
    glTF data's loading order of the following code:
    0.  buffers
    1.  buffer views (from 0)
    2.  accessors (from 1)
    3.  images
    4.  samplers
    5.  textures (from 3 and 4)
    6.  materials (from 5)
    7.  meshes (from 2 and 6)
    8.  cameras
    9.  nodes (from 7 and 8)
    10. animations (from 2 and 9)
    11. skins (from 2 and 9)
    12. scenes (from 9)
    13. glTF load finishing-up works (from 12)
     */

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
                    loaded_gltf.buffers_mut().push(buf);
                }
                Err(err) => {
                    util::jni::clear_exception_if_occurred(env);
                    util::jni::throw_runtime_exception(
                        env, &format!("Failed to load glTF buffer: {}", err)).unwrap();
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
        loaded_gltf.buffer_views_mut().push(loaded_buffer_view);
    });

    // Load accessors.
    gltf_obj.accessors().for_each(|it| {
        let mut loaded_gltf = loaded_gltf_wrapper.get().lock().unwrap();
        let loaded_accessor = LoadedGltfAccessor::new_from_accessor(
            loaded_gltf_wrapper.get(), &it).unwrap();
        loaded_gltf.accessors_mut().push(loaded_accessor);
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
                    loaded_gltf.images_mut().push(img);
                }
                Err(err) => {
                    util::jni::clear_exception_if_occurred(env);
                    util::jni::throw_runtime_exception(
                        env, &format!("Failed to load glTF buffer: {}", err)).unwrap();
                    return;
                }
            }
        }
    });

    // Load samplers.
    gltf_obj.samplers().for_each(|it| {
        let mut loaded_gltf = loaded_gltf_wrapper.get().lock().unwrap();
        let loaded_sampler = LoadedGltfSampler::new_from_sampler(
            loaded_gltf_wrapper.get(), &it);
        loaded_gltf.samplers_mut().push(loaded_sampler);
    });

    // Load textures.
    gltf_obj.textures().for_each(|it| {
        let mut loaded_gltf = loaded_gltf_wrapper.get().lock().unwrap();
        let loaded_texture = LoadedGltfTexture::new_from_texture(
            loaded_gltf_wrapper.get(), &it);
        loaded_gltf.textures_mut().push(loaded_texture);
    });

    // Load materials.
    gltf_obj.materials().for_each(|it| {
        let mut loaded_gltf = loaded_gltf_wrapper.get().lock().unwrap();
        let loaded_material = LoadedGltfMaterial::new_from_material(
            loaded_gltf_wrapper.get(), &it);
        loaded_gltf.materials_mut().push(loaded_material);
    });

    // Load meshes.
    gltf_obj.meshes().for_each(|it| {
        let mut loaded_gltf = loaded_gltf_wrapper.get().lock().unwrap();
        let loaded_mesh = LoadedGltfMesh::new_from_mesh(
            loaded_gltf_wrapper.get(), &it);
        loaded_gltf.meshes_mut().push(loaded_mesh);
    });

    // Load cameras.
    // TODO

    // Load nodes.
    // TODO

    // Load animations.
    // TODO

    // Load skins.
    // TODO

    // Do glTF load finishing-up works.
    // TODO

    unsafe {
        env.set_rust_field(this, "rust_loadedGltfObj", loaded_gltf_wrapper).unwrap_or_else(|err| {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(
                env, &format!("Failed to set rust object rust_loadedGltfObj: {}", err)).unwrap()
        });
        env.set_rust_field(this, "rust_gltfObj", gltf_obj).unwrap_or_else(|err| {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(
                env, &format!("Failed to set rust object rust_gltfObj: {}", err)).unwrap()
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
            util::jni::throw_runtime_exception(
                env, &format!("Failed to set rust object rust_gltfObj: {}", err)).unwrap()
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
    for image in loaded_gltf.images() {
        if *image.uri() == uri {
            target_img = Some(image);
            break;
        }
    }

    let mut result: Vec<u8> = Vec::new();
    if let Some(image) = target_img {
        result = Vec::with_capacity(image.data().len());
        image.data().iter().for_each(|x| result.push(*x));
    }
    
    drop(loaded_gltf);

    unsafe {
        env.set_rust_field(this, "rust_loadedGltfObj", loaded_gltf_obj).unwrap_or_else(|err| {
            util::jni::clear_exception_if_occurred(env);
            util::jni::throw_runtime_exception(
                env, &format!("Failed to set rust object rust_loadedGltfObj: {}", err)).unwrap()
        });
    }

    let jresult = env.new_byte_array(result.len() as jsize).unwrap();
    let result_jbyte: Vec<jbyte> = result.iter().map(|x| *x as jbyte).collect();
    env.set_byte_array_region(&jresult, 0, result_jbyte.as_slice()).unwrap();
    jresult.as_raw()
}
