extern crate jni;
extern crate gltf;

use std::sync::MutexGuard;
use jni::JNIEnv;
use jni::objects::{JByteArray, JObject};
use jni::sys::{jboolean, jbyte, jint};

pub mod util;

// ----- top.srcres.mods.modelassetlib.ModelAssetLib

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_NativeLibrary_initNative0<'local>(
    _: JNIEnv<'local>,
    _: JObject<'local>
) -> jboolean {
    println!("Native library initialized.");
    util::bool_to_jboolean(true)
}

// ----- top.srcres.mods.modelassetlib.client.model.AssetedEntityModel

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_client_model_AssetedEntityModel_nativeInit<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    gltf_data: JByteArray<'local>
) {
    let gltf_data_len = env.get_array_length(&gltf_data).unwrap();
    let mut gltf_data_vec: Vec<jbyte> = Vec::with_capacity(gltf_data_len as usize);
    for _ in 0..gltf_data_len {
        gltf_data_vec.push(0);
    }
    env.get_byte_array_region(&gltf_data, 0, gltf_data_vec.as_mut_slice()).unwrap();
    let gltf_data_u8: Vec<_> = gltf_data_vec.iter().map(|it| *it as u8).collect();
    let gltf_obj = gltf::Gltf::from_slice(gltf_data_u8.as_slice());
    match gltf_obj {
        Ok(gltf_obj) => {
            unsafe {
                env.set_rust_field(&this, "rust_gltfObj", gltf_obj).unwrap_or_else(|err| {
                    util::throw_runtime_exception(&mut env, &format!("Failed to set rust_gltfObj: {}", err)).unwrap();
                });
            }
        }
        Err(err) => {
            util::throw_runtime_exception(&mut env, &format!("Failed to create the glTF object: {}", err)).unwrap();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_client_model_AssetedEntityModel_nativeDestroy<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>
) {
    unsafe {
        let gltf_obj: gltf::Gltf = env.take_rust_field(&this, "rust_gltfObj").unwrap();
        drop(gltf_obj);
    }
}

#[no_mangle]
pub extern "system" fn Java_top_srcres_mods_modelassetlib_client_model_AssetedEntityModel_getGltfMeshCount<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>
) -> jint {
    let gltf_obj: MutexGuard<gltf::Gltf>;
    unsafe {
        gltf_obj = env.get_rust_field(&this, "rust_gltfObj").unwrap();
    }
    gltf_obj.meshes().len() as jint
}
