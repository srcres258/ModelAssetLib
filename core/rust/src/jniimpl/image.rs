extern crate jni;
extern crate image;
extern crate lazy_static;

use std::io::Cursor;
use std::sync::Mutex;
use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JObject};
use anyhow::Result;
use image::RgbaImage;
use jni::sys::{jbyte, jint, jstring};
use crate::util;
use crate::util::error_message::ErrorMessage;

lazy_static::lazy_static! {
    pub static ref ERROR_MESSAGE: Mutex<ErrorMessage> = Mutex::new(ErrorMessage::new());
}

pub fn record_error(msg: &str) {
    let err_msg = ERROR_MESSAGE.lock().unwrap();
    err_msg.mark_occurred();
    err_msg.set(msg);
}

pub fn handle_native_init<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>,
    raw_data: &JByteArray
) -> Result<()> {
    let raw_data_len = env.get_array_length(raw_data)?;
    let mut data: Vec<jbyte> = util::new_buffer_vec(raw_data_len as usize, 0);
    env.get_byte_array_region(raw_data, 0, data.as_mut_slice())?;
    let data_u8: Vec<u8> = data.iter().map(|x| *x as u8).collect();
    let image = image::io::Reader::new(Cursor::new(data_u8.as_slice()))
        .with_guessed_format()?.decode()?;
    let rgba8_image: RgbaImage = image.to_rgba8();

    unsafe {
        env.set_rust_field(this, "rust_imageObj", rgba8_image)?;
    }

    Ok(())
}

pub fn handle_native_init_with_format<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>,
    raw_data: &JByteArray,
    format_id: jint
) -> Result<()> {
    let raw_data_len = env.get_array_length(raw_data)?;
    let mut data: Vec<jbyte> = util::new_buffer_vec(raw_data_len as usize, 0);
    env.get_byte_array_region(raw_data, 0, data.as_mut_slice())?;
    
    let format = util::image::image_format_from_id(format_id)?;
    let data_u8: Vec<u8> = data.iter().map(|x| *x as u8).collect();
    let mut image_reader = image::io::Reader::new(Cursor::new(data_u8.as_slice()));
    image_reader.set_format(format);
    let image = image_reader.decode()?;
    let rgba8_image: RgbaImage = image.to_rgba8();

    unsafe {
        env.set_rust_field(this, "rust_imageObj", rgba8_image)?;
    }

    Ok(())
}

pub fn handle_native_destroy<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) -> Result<()> {
    let image: RgbaImage;
    unsafe {
        image = env.take_rust_field(this, "rust_imageObj")?;
    }
    drop(image);
    Ok(())
}

pub fn handle_is_error_occurred<'a>(
    _: &mut JNIEnv<'a>,
    _: &JClass<'a>
) -> bool {
    ERROR_MESSAGE.lock().unwrap().is_occurred()
}

pub fn handle_get_error_message<'a>(
    env: &mut JNIEnv<'a>,
    _: &JClass<'a>
) -> jstring {
    let msg = ERROR_MESSAGE.lock().unwrap().get();
    let msg_jstr = env.new_string(msg).unwrap();
    msg_jstr.as_raw()
}

pub fn handle_clear_error<'a>(
    _: &mut JNIEnv<'a>,
    _: &JClass<'a>
) {
    let msg = ERROR_MESSAGE.lock().unwrap();
    msg.set("");
    msg.clear_mark();
}

pub fn handle_get_width<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) -> Result<jint> {
    let image: RgbaImage;
    unsafe {
        image = env.take_rust_field(this, "rust_imageObj")?;
    }

    let result = image.width();

    unsafe {
        env.set_rust_field(this, "rust_imageObj", image)?;
    }

    Ok(result as jint)
}

pub fn handle_get_height<'a>(
    env: &mut JNIEnv<'a>,
    this: &JObject<'a>
) -> Result<jint> {
    let image: RgbaImage;
    unsafe {
        image = env.take_rust_field(this, "rust_imageObj")?;
    }

    let result = image.height();

    unsafe {
        env.set_rust_field(this, "rust_imageObj", image)?;
    }

    Ok(result as jint)
}
