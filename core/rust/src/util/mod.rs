pub mod jni;
pub mod error_message;
pub mod image;
pub mod bits;
pub mod gltf;

pub fn new_buffer_vec<T>(
    capacity: usize,
    initial_val: T
) -> Vec<T> where T: Copy {
    let mut vec = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        vec.push(initial_val);
    }
    vec
}
