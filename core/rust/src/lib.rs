extern crate jni;

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jboolean, jbyteArray};

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
