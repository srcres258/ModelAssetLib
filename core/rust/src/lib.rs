extern crate jni;

use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jboolean;

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
