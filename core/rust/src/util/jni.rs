extern crate jni;

use std::sync::MutexGuard;
use jni::JNIEnv;
use jni::sys::jboolean;
use anyhow::{Context, Result};
use jni::objects::JObject;

pub fn bool_to_jboolean(val: bool) -> jboolean {
    if val { 1 } else { 0 }
}

pub fn jboolean_to_bool(val: jboolean) -> bool {
    if val == 0 { false } else { true }
}

pub fn throw_runtime_exception(env: &mut JNIEnv, message: &String) -> Result<()> {
    env.throw_new("java/lang/RuntimeException", message).with_context(
        || format!("Failed to throw java/lang/RuntimeException with message: {}", message))
}

pub fn clear_exception_if_occurred(env: &mut JNIEnv) {
    if let Ok(_) = env.exception_occurred() {
        env.exception_clear().unwrap();
    }
}

pub fn set_rust_field<'a, T>(env: &'a mut JNIEnv, obj: &'a JObject, field: &'a str, val: T)
    where T: Send + 'static {
    unsafe {
        env.set_rust_field(obj, field, val).expect(&format!("Failed to get rust field {}", field));
    }
}

pub fn get_rust_field<'a, T>(env: &'a mut JNIEnv, obj: &'a JObject, field: &'a str) -> MutexGuard<'a, T>
    where T: Send + 'static {
    unsafe {
        env.get_rust_field(obj, field).expect(&format!("Failed to set rust field {}", field))
    }
}
