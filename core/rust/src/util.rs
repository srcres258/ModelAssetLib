extern crate jni;

use jni::JNIEnv;
use jni::sys::jboolean;
use jni::errors::Error;

pub fn bool_to_jboolean(val: bool) -> jboolean {
    if val { 1 } else { 0 }
}

pub fn jboolean_to_bool(val: jboolean) -> bool {
    if val == 0 { false } else { true }
}

pub fn throw_runtime_exception(env: &mut JNIEnv, message: &String) -> Result<(), Error> {
    env.throw_new("java/lang/RuntimeException", message)
}
