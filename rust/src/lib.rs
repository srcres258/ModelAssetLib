use ::jni::objects::JObject;
use ::jni::sys::{jboolean, JNIEnv};

pub mod jni;

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_NativeLibrary_initNative0<'local>(
    mut env: JNIEnv,
    this: JObject<'local>
) -> jboolean {
    println!("Native library initialized.");
    crate::jni::bool_to_jboolean(true)
}
