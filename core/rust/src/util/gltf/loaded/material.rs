extern crate nalgebra;

use crate::util::gltf::GltfVecNum;
use nalgebra::{SVector, Vector4};

pub struct PbrMetallicRoughnessTextureInfo {
    index: usize,
    tex_coord: usize
}

pub struct PbrMetallicRoughnessInfo {
    base_color_texture: Option<PbrMetallicRoughnessTextureInfo>,
    base_color_factor: SVector<GltfVecNum, 4>,
    metallic_roughness_texture: Option<PbrMetallicRoughnessTextureInfo>,
    metallic_factor: GltfVecNum,
    roughness_factor: GltfVecNum
}

pub struct NormalTextureInfo {
    scale: GltfVecNum,
    index: usize,
    tex_coord: usize
}

pub struct OcclusionTextureInfo {
    strength: GltfVecNum,
    index: usize,
    tex_coord: usize
}

pub type EmissiveTextureInfo = PbrMetallicRoughnessTextureInfo;

pub type EmissiveFactorInfo = SVector<GltfVecNum, 3>;

impl PbrMetallicRoughnessTextureInfo {
    pub fn new(
        index: usize,
        tex_coord: usize
    ) -> Self {
        Self {
            index,
            tex_coord
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn tex_coord(&self) -> usize {
        self.tex_coord
    }
}

impl PbrMetallicRoughnessInfo {
    pub fn new(
        base_color_texture: Option<PbrMetallicRoughnessTextureInfo>,
        base_color_factor: SVector<GltfVecNum, 4>,
        metallic_roughness_texture: Option<PbrMetallicRoughnessTextureInfo>,
        metallic_factor: GltfVecNum,
        roughness_factor: GltfVecNum
    ) -> Self {
        Self {
            base_color_texture,
            base_color_factor,
            metallic_roughness_texture,
            metallic_factor,
            roughness_factor
        }
    }

    pub fn new_from_raw(
        base_color_texture_index: usize,
        base_color_texture_tex_coord: usize,
        base_color_factor_arr: [GltfVecNum; 4],
        metallic_roughness_texture_index: usize,
        metallic_roughness_texture_tex_coord: usize,
        metallic_factor: GltfVecNum,
        roughness_factor: GltfVecNum
    ) -> Self {
        let base_color_texture = PbrMetallicRoughnessTextureInfo::new(
            base_color_texture_index, base_color_texture_tex_coord);
        let metallic_roughness_texture = PbrMetallicRoughnessTextureInfo::new(
            metallic_roughness_texture_index, metallic_roughness_texture_tex_coord);
        let base_color_factor = Vector4::new(
            base_color_factor_arr[0], base_color_factor_arr[1],
            base_color_factor_arr[2], base_color_factor_arr[3]);
        Self::new(
            Some(base_color_texture),
            base_color_factor,
            Some(metallic_roughness_texture),
            metallic_factor,
            roughness_factor)
    }

    pub fn base_color_texture(&self) -> &Option<PbrMetallicRoughnessTextureInfo> {
        &self.base_color_texture
    }


    pub fn base_color_factor(&self) -> SVector<GltfVecNum, 4> {
        self.base_color_factor
    }

    pub fn metallic_roughness_texture(&self) -> &Option<PbrMetallicRoughnessTextureInfo> {
        &self.metallic_roughness_texture
    }

    pub fn metallic_factor(&self) -> GltfVecNum {
        self.metallic_factor
    }

    pub fn roughness_factor(&self) -> GltfVecNum {
        self.roughness_factor
    }
}

impl NormalTextureInfo {
    pub fn new(
        scale: GltfVecNum,
        index: usize,
        tex_coord: usize
    ) -> Self {
        Self {
            scale,
            index,
            tex_coord
        }
    }


    pub fn scale(&self) -> GltfVecNum {
        self.scale
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn tex_coord(&self) -> usize {
        self.tex_coord
    }
}

impl OcclusionTextureInfo {
    pub fn new(
        strength: GltfVecNum,
        index: usize,
        tex_coord: usize
    ) -> Self {
        Self {
            strength,
            index,
            tex_coord
        }
    }


    pub fn strength(&self) -> GltfVecNum {
        self.strength
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn tex_coord(&self) -> usize {
        self.tex_coord
    }
}
