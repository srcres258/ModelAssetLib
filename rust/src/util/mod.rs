use jni::sys::jboolean;

pub fn bool_to_jboolean(val: bool) -> jboolean {
    if val { 1 } else { 0 }
}
