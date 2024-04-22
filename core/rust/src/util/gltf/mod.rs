extern crate gltf;
extern crate nalgebra;

mod loaded;

use gltf::texture::{MagFilter, MinFilter, WrappingMode};
pub use loaded::LoadedGltfBuffer;
pub use loaded::LoadedGltfBufferView;
pub use loaded::LoadedGltfAccessor;
pub use loaded::LoadedGltfImage;
pub use loaded::LoadedGltfSampler;
pub use loaded::LoadedGltfTexture;
pub use loaded::LoadedGltfMaterial;
pub use loaded::LoadedGltf;
pub use loaded::LoadedGltfWrapper;
use crate::constants;

pub type GltfVecNum = f32;

pub fn mag_filter_to_gl_value(filter: MagFilter) -> u32 {
    match filter {
        MagFilter::Nearest => constants::opengl::GL_NEAREST,
        MagFilter::Linear => constants::opengl::GL_LINEAR
    }
}

pub fn mag_filter_from_gl_value(value: u32) -> Option<MagFilter> {
    match value {
        constants::opengl::GL_NEAREST => Some(MagFilter::Nearest),
        constants::opengl::GL_LINEAR => Some(MagFilter::Linear),
        _ => None
    }
}

pub fn min_filter_to_gl_value(filter: MinFilter) -> u32 {
    match filter {
        MinFilter::Nearest => constants::opengl::GL_NEAREST,
        MinFilter::Linear => constants::opengl::GL_LINEAR,
        MinFilter::NearestMipmapNearest => constants::opengl::GL_NEAREST_MIPMAP_NEAREST,
        MinFilter::LinearMipmapNearest => constants::opengl::GL_LINEAR_MIPMAP_NEAREST,
        MinFilter::NearestMipmapLinear => constants::opengl::GL_NEAREST_MIPMAP_LINEAR,
        MinFilter::LinearMipmapLinear => constants::opengl::GL_LINEAR_MIPMAP_LINEAR
    }
}

pub fn min_filter_from_gl_value(value: u32) -> Option<MinFilter> {
    match value {
        constants::opengl::GL_NEAREST => Some(MinFilter::Nearest),
        constants::opengl::GL_LINEAR => Some(MinFilter::Linear),
        constants::opengl::GL_NEAREST_MIPMAP_NEAREST => Some(MinFilter::NearestMipmapNearest),
        constants::opengl::GL_LINEAR_MIPMAP_NEAREST => Some(MinFilter::LinearMipmapNearest),
        constants::opengl::GL_NEAREST_MIPMAP_LINEAR => Some(MinFilter::NearestMipmapLinear),
        constants::opengl::GL_LINEAR_MIPMAP_LINEAR => Some(MinFilter::LinearMipmapLinear),
        _ => None
    }
}

pub fn wrapping_mode_to_gl_value(mode: WrappingMode) -> u32 {
    match mode {
        WrappingMode::ClampToEdge => constants::opengl::GL_CLAMP_TO_EDGE,
        WrappingMode::MirroredRepeat => constants::opengl::GL_MIRRORED_REPEAT,
        WrappingMode::Repeat => constants::opengl::GL_REPEAT
    }
}

pub fn wrapping_mode_from_gl_value(value: u32) -> Option<WrappingMode> {
    match value {
        constants::opengl::GL_CLAMP_TO_EDGE => Some(WrappingMode::ClampToEdge),
        constants::opengl::GL_MIRRORED_REPEAT => Some(WrappingMode::MirroredRepeat),
        constants::opengl::GL_REPEAT => Some(WrappingMode::Repeat),
        _ => None
    }
}
