extern crate jni;

use jni::JNIEnv;
use jni::sys::{jboolean, jbyteArray};
use anyhow::{Context, Result};

pub fn bool_to_jboolean(val: bool) -> jboolean {
    if val { 1 } else { 0 }
}

pub fn jboolean_to_bool(val: jboolean) -> bool {
    if val == 0 { false } else { true }
}

pub fn throw_runtime_exception(env: &mut JNIEnv, message: &String) -> Result<()> {
    env.throw_new("top/srcres/mods/modelassetlib/NativeRuntimeException", message).with_context(
        || format!("Failed to throw native runtime exception with message: {}", message))
}

pub fn clear_exception_if_occurred(env: &mut JNIEnv) {
    if let Ok(_) = env.exception_occurred() {
        env.exception_clear().unwrap();
    }
}

pub fn new_empty_byte_array(env: &mut JNIEnv) -> jbyteArray {
    env.new_byte_array(0).unwrap().as_raw()
}
