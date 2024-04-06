extern crate jni;

use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JObject, JString};
use jni::sys::{jboolean, jbyteArray, jint, jstring};

pub mod jniimpl;
pub mod util;

// ----- top.srcres.mods.modelassetlib.ModelAssetLib

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_NativeLibrary_initNative0<'local>(
    _: JNIEnv<'local>,
    _: JObject<'local>
) -> jboolean {
    println!("Native library initialized.");
    util::jni::bool_to_jboolean(true)
}

// ----- top.srcres.mods.modelassetlib.gltf.Gltf

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_gltf_Gltf_nativeInit<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>
) {
    jniimpl::gltf::handle_native_init(&mut env, &this);
}

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_gltf_Gltf_nativeDestroy<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>
) {
    jniimpl::gltf::handle_native_destroy(&mut env, &this);
}

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_gltf_Gltf_getImageDataByURI<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    uri: JString
) -> jbyteArray {
    jniimpl::gltf::handle_get_image_data_by_uri(&mut env, &this, &uri)
}

// ----- top.srcres.mods.modelassetlib.image.Image

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_image_Image_nativeInit<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    raw_data: JByteArray
) -> jboolean {
    match jniimpl::image::handle_native_init(&mut env, &this, &raw_data) {
        Ok(_) => {
            util::jni::bool_to_jboolean(true)
        }
        Err(err) => {
            jniimpl::image::ERROR_MESSAGE.lock().unwrap().set(&format!(
                "jniimpl::image::handle_native_init failed: {}", err));
            util::jni::bool_to_jboolean(false)
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_image_Image_nativeInitWithFormat<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    raw_data: JByteArray,
    format_id: jint
) -> jboolean {
    match jniimpl::image::handle_native_init_with_format(&mut env, &this, &raw_data, format_id) {
        Ok(_) => {
            util::jni::bool_to_jboolean(true)
        }
        Err(err) => {
            jniimpl::image::ERROR_MESSAGE.lock().unwrap().set(&format!(
                "jniimpl::image::handle_native_init_with_format failed: {}", err));
            util::jni::bool_to_jboolean(false)
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_image_Image_nativeDestroy<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>
) {
    jniimpl::image::handle_native_destroy(&mut env, &this).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_image_ImageKt_nativeGetErrorMessage<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>
) -> jstring {
    jniimpl::image::handle_get_error_message(&mut env, &class)
}
