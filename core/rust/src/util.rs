extern crate jni;

use jni::sys::jboolean;

pub fn bool_to_jboolean(val: bool) -> jboolean {
    if val { 1 } else { 0 }
}

pub fn jboolean_to_bool(val: jboolean) -> bool {
    if val == 0 { false } else { true }
}
