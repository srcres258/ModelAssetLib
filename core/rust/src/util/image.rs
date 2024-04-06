extern crate image;
extern crate anyhow;
extern crate thiserror;

use image::ImageFormat;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageUtilError {
    #[error("The image format id {0} is wrong.")]
    WrongImageFormatId(i32)
}

pub fn image_format_from_id(id: i32) -> Result<ImageFormat> {
    match id {
        0 => Ok(ImageFormat::Png),
        1 => Ok(ImageFormat::Jpeg),
        2 => Ok(ImageFormat::Gif),
        3 => Ok(ImageFormat::WebP),
        4 => Ok(ImageFormat::Pnm),
        5 => Ok(ImageFormat::Tiff),
        6 => Ok(ImageFormat::Tga),
        7 => Ok(ImageFormat::Dds),
        8 => Ok(ImageFormat::Bmp),
        9 => Ok(ImageFormat::Ico),
        10 => Ok(ImageFormat::Hdr),
        11 => Ok(ImageFormat::OpenExr),
        12 => Ok(ImageFormat::Farbfeld),
        13 => Ok(ImageFormat::Avif),
        14 => Ok(ImageFormat::Qoi),
        _ => Err(ImageUtilError::WrongImageFormatId(id).into())
    }
}
