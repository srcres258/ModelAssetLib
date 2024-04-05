extern crate jni;

use jni::JNIEnv;
use jni::sys::jboolean;
use anyhow::{Context, Result};

pub fn bool_to_jboolean(val: bool) -> jboolean {
    if val { 1 } else { 0 }
}

pub fn jboolean_to_bool(val: jboolean) -> bool {
    if val == 0 { false } else { true }
}

pub fn throw_runtime_exception(env: &mut JNIEnv, message: &String) -> Result<()> {
    env.throw_new("java/lang/RuntimeException", message).with_context(
        || format!("Failed to throw java/lang/RuntimeException witb message: {}", message))
}