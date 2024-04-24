pub mod material;
pub mod mesh;

use std::ops::Index;
use gltf::{Accessor, buffer, Material, Mesh, texture};
use crate::util;
use std::sync::{Arc, Mutex};
use gltf::accessor::Dimensions;
use gltf::buffer::View;
use gltf::json::Value;
use gltf::texture::{MagFilter, MinFilter, WrappingMode};
use nalgebra::{Matrix2, Matrix3, Matrix4, SMatrix, SVector, Vector2, Vector3, Vector4};
use super::{GltfVecNum, mag_filter_to_gl_value, min_filter_to_gl_value, wrapping_mode_to_gl_value};

pub struct LoadedGltfBuffer<'a> {
    gltf: Arc<Mutex<LoadedGltf<'a>>>,
    index: usize,
    uri: String,
    data: Vec<u8>
}

pub struct LoadedGltfBufferView<'a> {
    gltf: Arc<Mutex<LoadedGltf<'a>>>,
    index: usize,
    buffer_index: usize,
    data_offset: usize,
    data_length: usize,
    data_stride: Option<usize>,
    target: Option<buffer::Target>
}

pub struct  LoadedGltfAccessor<'a> {
    gltf: Arc<Mutex<LoadedGltf<'a>>>,
    index: usize,
    buffer_view_index: usize,
    comp_size: usize,
    comp_count: usize,
    max_values: Option<Vec<Value>>,
    min_values: Option<Vec<Value>>,
    dimensions: Dimensions
}

pub struct LoadedGltfImage<'a> {
    gltf: Arc<Mutex<LoadedGltf<'a>>>,
    index: usize,
    uri: String,
    data: Vec<u8>
}

pub struct LoadedGltfSampler<'a> {
    gltf: Arc<Mutex<LoadedGltf<'a>>>,
    /// None if the sampler is the default one within the glTF.
    index: Option<usize>,
    mag_filter: Option<MagFilter>,
    min_filter: Option<MinFilter>,
    name: Option<String>,
    wrap_s: WrappingMode,
    wrap_t: WrappingMode
}

pub struct LoadedGltfTexture<'a> {
    gltf: Arc<Mutex<LoadedGltf<'a>>>,
    index: usize,
    source_index: usize,
    sampler_index: usize
}

pub struct LoadedGltfMaterial<'a> {
    gltf: Arc<Mutex<LoadedGltf<'a>>>,
    /// None if the material is the default one within the glTF.
    index: Option<usize>,
    pbr_metallic_roughness: material::PbrMetallicRoughnessInfo,
    normal_texture: Option<material::NormalTextureInfo>,
    occlusion_texture: Option<material::OcclusionTextureInfo>,
    emissive_texture: Option<material::EmissiveTextureInfo>,
    emissive_factor: material::EmissiveFactorInfo
}

pub struct LoadedGltfMesh<'a> {
    gltf: Arc<Mutex<LoadedGltf<'a>>>,
    index: usize,
    primitives: Vec<mesh::PrimitiveInfo>,
    /// Empty if not defined in glTF, or defined but values are not given.
    weights: Vec<f32>
}

pub struct LoadedGltf<'a> {
    buffers: Vec<LoadedGltfBuffer<'a>>,
    buffer_views: Vec<LoadedGltfBufferView<'a>>,
    accessors: Vec<LoadedGltfAccessor<'a>>,
    images: Vec<LoadedGltfImage<'a>>,
    samplers: Vec<LoadedGltfSampler<'a>>,
    textures: Vec<LoadedGltfTexture<'a>>,
    materials: Vec<LoadedGltfMaterial<'a>>,
    meshes: Vec<LoadedGltfMesh<'a>>
}

pub struct LoadedGltfWrapper<'a> {
    gltf: Arc<Mutex<LoadedGltf<'a>>>
}

impl<'a> LoadedGltfBuffer<'a> {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        index: usize,
        uri: String,
        data: Vec<u8>
    ) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            uri,
            data
        }
    }

    pub fn gltf(&self) -> Arc<Mutex<LoadedGltf<'a>>> {
        Arc::clone(&self.gltf)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn uri(&self) -> &String {
        &self.uri
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

pub enum LoadedGltfAccessorDatum {
    Scalar(GltfVecNum),
    Vec2(SVector<GltfVecNum, 2>),
    Vec3(SVector<GltfVecNum, 3>),
    Vec4(SVector<GltfVecNum, 4>),
    Mat2(SMatrix<GltfVecNum, 2, 2>),
    Mat3(SMatrix<GltfVecNum, 3, 3>),
    Mat4(SMatrix<GltfVecNum, 4, 4>)
}

impl<'a> LoadedGltfBufferView<'a> {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        index: usize,
        buffer_index: usize,
        data_offset: usize,
        data_length: usize,
        data_stride: Option<usize>,
        target: Option<buffer::Target>
    ) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            buffer_index,
            data_offset,
            data_length,
            data_stride,
            target
        }
    }

    pub fn new_from_view(gltf: &Arc<Mutex<LoadedGltf<'a>>>, view: &View) -> Self {
        Self::new(gltf, view.index(), view.buffer().index(), view.offset(),
                  view.length(), view.stride(), view.target())
    }

    pub fn gltf(&self) -> Arc<Mutex<LoadedGltf<'a>>> {
        Arc::clone(&self.gltf)
    }

    pub fn buffer_index(&self) -> usize {
        self.buffer_index
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn data_offset(&self) -> usize {
        self.data_offset
    }

    pub fn data_length(&self) -> usize {
        self.data_length
    }

    pub fn data_stride(&self) -> Option<usize> {
        self.data_stride
    }

    pub fn target(&self) -> Option<buffer::Target> {
        self.target
    }

    pub fn load_data(&self) -> Vec<u8> {
        self.load_data_strided(0)
    }

    pub fn load_data_strided(&self, stride_count: u32) -> Vec<u8> {
        let gltf = self.gltf.lock().unwrap();
        let buffer = gltf.buffers().get(self.buffer_index).unwrap();
        let mut result = util::new_buffer_vec(self.data_length, 0u8);
        let result_ptr = result.as_mut_ptr();
        unsafe {
            let data_ptr = buffer.data_ptr().add(self.data_offset)
                .add((stride_count as usize) * self.data_stride.unwrap_or(0));
            let mut result_ptrm = result_ptr;
            let mut data_ptrm = data_ptr;
            for _ in 0..self.data_length {
                *result_ptrm = *data_ptrm;
                result_ptrm = result_ptrm.add(1);
                data_ptrm = data_ptrm.add(1);
            }
        }
        result
    }
}

impl<'a> LoadedGltfAccessor<'a> {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        index: usize,
        buffer_view_index: usize,
        comp_size: usize,
        comp_count: usize,
        max_values: Option<Vec<Value>>,
        min_values: Option<Vec<Value>>,
        dimensions: Dimensions
    ) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            buffer_view_index,
            comp_size,
            comp_count,
            max_values,
            min_values,
            dimensions
        }
    }

    pub fn new_from_accessor(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        accessor: &Accessor
    ) -> Option<Self> {
        if let Some(max_val) = accessor.max() {
            if let Value::Array(max_val_arr) = max_val {
                if let Some(min_val) = accessor.min() {
                    if let Value::Array(min_val_arr) = min_val {
                        Some(Self::new(
                            gltf, accessor.index(), accessor.view().unwrap().index(),accessor.size(),
                            accessor.count(), Some(max_val_arr), Some(min_val_arr), accessor.dimensions()))
                    } else {
                        Some(Self::new(
                            gltf, accessor.index(), accessor.view().unwrap().index(),
                            accessor.size(), accessor.count(), Some(max_val_arr), None, accessor.dimensions()))
                    }
                } else {
                    Some(Self::new(
                        gltf, accessor.index(), accessor.view().unwrap().index(),
                        accessor.size(), accessor.count(), Some(max_val_arr), None, accessor.dimensions()))
                }
            } else if let Some(min_val) = accessor.min() {
                if let Value::Array(min_val_arr) = min_val {
                    Some(Self::new(
                        gltf, accessor.index(), accessor.view().unwrap().index(),
                        accessor.size(), accessor.count(), None, Some(min_val_arr), accessor.dimensions()))
                } else {
                    Some(Self::new(
                        gltf, accessor.index(), accessor.view().unwrap().index(),
                        accessor.size(), accessor.count(), None, None, accessor.dimensions()))
                }
            } else {
                Some(Self::new(
                    gltf, accessor.index(), accessor.view().unwrap().index(),
                    accessor.size(), accessor.count(), None, None, accessor.dimensions()))
            }
        } else if let Some(min_val) = accessor.min() {
            if let Value::Array(min_val_arr) = min_val {
                Some(Self::new(
                    gltf, accessor.index(), accessor.view().unwrap().index(),
                    accessor.size(), accessor.count(), None, Some(min_val_arr), accessor.dimensions()))
            } else {
                Some(Self::new(
                    gltf, accessor.index(), accessor.view().unwrap().index(),
                    accessor.size(), accessor.count(), None, None, accessor.dimensions()))
            }
        } else {
            Some(Self::new(
                gltf, accessor.index(), accessor.view().unwrap().index(),
                accessor.size(), accessor.count(), None, None, accessor.dimensions()))
        }
    }

    pub fn gltf(&self) -> Arc<Mutex<LoadedGltf<'a>>> {
        Arc::clone(&self.gltf)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn buffer_view_index(&self) -> usize {
        self.buffer_view_index
    }

    pub fn comp_size(&self) -> usize {
        self.comp_size
    }

    pub fn comp_count(&self) -> usize {
        self.comp_count
    }

    pub fn max_values(&self) -> Option<Vec<Value>> {
        self.max_values.clone()
    }

    pub fn min_values(&self) -> Option<Vec<Value>> {
        self.min_values.clone()
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn load_data(&self) -> Vec<LoadedGltfAccessorDatum> {
        let gltf = self.gltf.lock().unwrap();
        let buffer_view = gltf.buffer_views.get(self.buffer_view_index).unwrap();
        let mut result = Vec::new();

        for i in 0..self.comp_count {
            let datum_content = buffer_view.load_data_strided(i as u32);
            let datum = match self.dimensions {
                Dimensions::Scalar => {
                    let a0 = *datum_content.index(0);
                    let a1 = *datum_content.index(1);
                    let a2 = *datum_content.index(2);
                    let a3 = *datum_content.index(3);

                    let a = util::bits::u8_to_f32((a0, a1, a2, a3));

                    LoadedGltfAccessorDatum::scalar_of(a)
                }
                Dimensions::Vec2 => {
                    let x0 = *datum_content.index(0);
                    let x1 = *datum_content.index(1);
                    let x2 = *datum_content.index(2);
                    let x3 = *datum_content.index(3);
                    let y0 = *datum_content.index(4);
                    let y1 = *datum_content.index(5);
                    let y2 = *datum_content.index(6);
                    let y3 = *datum_content.index(7);

                    let x = util::bits::u8_to_f32((x0, x1, x2, x3));
                    let y = util::bits::u8_to_f32((y0, y1, y2, y3));

                    LoadedGltfAccessorDatum::vec2_of(x, y)
                }
                Dimensions::Vec3 => {
                    let x0 = *datum_content.index(0);
                    let x1 = *datum_content.index(1);
                    let x2 = *datum_content.index(2);
                    let x3 = *datum_content.index(3);
                    let y0 = *datum_content.index(4);
                    let y1 = *datum_content.index(5);
                    let y2 = *datum_content.index(6);
                    let y3 = *datum_content.index(7);
                    let z0 = *datum_content.index(8);
                    let z1 = *datum_content.index(9);
                    let z2 = *datum_content.index(10);
                    let z3 = *datum_content.index(11);

                    let x = util::bits::u8_to_f32((x0, x1, x2, x3));
                    let y = util::bits::u8_to_f32((y0, y1, y2, y3));
                    let z = util::bits::u8_to_f32((z0, z1, z2, z3));

                    LoadedGltfAccessorDatum::vec3_of(x, y, z)
                }
                Dimensions::Vec4 => {
                    let x0 = *datum_content.index(0);
                    let x1 = *datum_content.index(1);
                    let x2 = *datum_content.index(2);
                    let x3 = *datum_content.index(3);
                    let y0 = *datum_content.index(4);
                    let y1 = *datum_content.index(5);
                    let y2 = *datum_content.index(6);
                    let y3 = *datum_content.index(7);
                    let z0 = *datum_content.index(8);
                    let z1 = *datum_content.index(9);
                    let z2 = *datum_content.index(10);
                    let z3 = *datum_content.index(11);
                    let w0 = *datum_content.index(12);
                    let w1 = *datum_content.index(13);
                    let w2 = *datum_content.index(14);
                    let w3 = *datum_content.index(15);

                    let x = util::bits::u8_to_f32((x0, x1, x2, x3));
                    let y = util::bits::u8_to_f32((y0, y1, y2, y3));
                    let z = util::bits::u8_to_f32((z0, z1, z2, z3));
                    let w = util::bits::u8_to_f32((w0, w1, w2, w3));

                    LoadedGltfAccessorDatum::vec4_of(x, y, z, w)
                }
                Dimensions::Mat2 => {
                    let a00_0 = *datum_content.index(0);
                    let a00_1 = *datum_content.index(1);
                    let a00_2 = *datum_content.index(2);
                    let a00_3 = *datum_content.index(3);

                    let a01_0 = *datum_content.index(4);
                    let a01_1 = *datum_content.index(5);
                    let a01_2 = *datum_content.index(6);
                    let a01_3 = *datum_content.index(7);

                    let a10_0 = *datum_content.index(8);
                    let a10_1 = *datum_content.index(9);
                    let a10_2 = *datum_content.index(10);
                    let a10_3 = *datum_content.index(11);

                    let a11_0 = *datum_content.index(12);
                    let a11_1 = *datum_content.index(13);
                    let a11_2 = *datum_content.index(14);
                    let a11_3 = *datum_content.index(15);

                    let a00 = util::bits::u8_to_f32((a00_0, a00_1, a00_2, a00_3));
                    let a01 = util::bits::u8_to_f32((a01_0, a01_1, a01_2, a01_3));
                    let a10 = util::bits::u8_to_f32((a10_0, a10_1, a10_2, a10_3));
                    let a11 = util::bits::u8_to_f32((a11_0, a11_1, a11_2, a11_3));

                    LoadedGltfAccessorDatum::mat2_of(
                        a00, a01,
                        a10, a11
                    )
                }
                Dimensions::Mat3 => {
                    let a00_0 = *datum_content.index(0);
                    let a00_1 = *datum_content.index(1);
                    let a00_2 = *datum_content.index(2);
                    let a00_3 = *datum_content.index(3);

                    let a01_0 = *datum_content.index(4);
                    let a01_1 = *datum_content.index(5);
                    let a01_2 = *datum_content.index(6);
                    let a01_3 = *datum_content.index(7);

                    let a02_0 = *datum_content.index(8);
                    let a02_1 = *datum_content.index(9);
                    let a02_2 = *datum_content.index(10);
                    let a02_3 = *datum_content.index(11);

                    let a10_0 = *datum_content.index(12);
                    let a10_1 = *datum_content.index(13);
                    let a10_2 = *datum_content.index(14);
                    let a10_3 = *datum_content.index(15);

                    let a11_0 = *datum_content.index(16);
                    let a11_1 = *datum_content.index(17);
                    let a11_2 = *datum_content.index(18);
                    let a11_3 = *datum_content.index(19);

                    let a12_0 = *datum_content.index(20);
                    let a12_1 = *datum_content.index(21);
                    let a12_2 = *datum_content.index(22);
                    let a12_3 = *datum_content.index(23);

                    let a20_0 = *datum_content.index(24);
                    let a20_1 = *datum_content.index(25);
                    let a20_2 = *datum_content.index(26);
                    let a20_3 = *datum_content.index(27);

                    let a21_0 = *datum_content.index(28);
                    let a21_1 = *datum_content.index(29);
                    let a21_2 = *datum_content.index(30);
                    let a21_3 = *datum_content.index(31);

                    let a22_0 = *datum_content.index(32);
                    let a22_1 = *datum_content.index(33);
                    let a22_2 = *datum_content.index(34);
                    let a22_3 = *datum_content.index(35);

                    let a00 = util::bits::u8_to_f32((a00_0, a00_1, a00_2, a00_3));
                    let a01 = util::bits::u8_to_f32((a01_0, a01_1, a01_2, a01_3));
                    let a02 = util::bits::u8_to_f32((a02_0, a02_1, a02_2, a02_3));
                    let a10 = util::bits::u8_to_f32((a10_0, a10_1, a10_2, a10_3));
                    let a11 = util::bits::u8_to_f32((a11_0, a11_1, a11_2, a11_3));
                    let a12 = util::bits::u8_to_f32((a12_0, a12_1, a12_2, a12_3));
                    let a20 = util::bits::u8_to_f32((a20_0, a20_1, a20_2, a20_3));
                    let a21 = util::bits::u8_to_f32((a21_0, a21_1, a21_2, a21_3));
                    let a22 = util::bits::u8_to_f32((a22_0, a22_1, a22_2, a22_3));

                    LoadedGltfAccessorDatum::mat3_of(
                        a00, a01, a02,
                        a10, a11, a12,
                        a20, a21, a22
                    )
                }
                Dimensions::Mat4 => {
                    let a00_0 = *datum_content.index(0);
                    let a00_1 = *datum_content.index(1);
                    let a00_2 = *datum_content.index(2);
                    let a00_3 = *datum_content.index(3);

                    let a01_0 = *datum_content.index(4);
                    let a01_1 = *datum_content.index(5);
                    let a01_2 = *datum_content.index(6);
                    let a01_3 = *datum_content.index(7);

                    let a02_0 = *datum_content.index(8);
                    let a02_1 = *datum_content.index(9);
                    let a02_2 = *datum_content.index(10);
                    let a02_3 = *datum_content.index(11);

                    let a03_0 = *datum_content.index(12);
                    let a03_1 = *datum_content.index(13);
                    let a03_2 = *datum_content.index(14);
                    let a03_3 = *datum_content.index(15);

                    let a10_0 = *datum_content.index(16);
                    let a10_1 = *datum_content.index(17);
                    let a10_2 = *datum_content.index(18);
                    let a10_3 = *datum_content.index(19);

                    let a11_0 = *datum_content.index(20);
                    let a11_1 = *datum_content.index(21);
                    let a11_2 = *datum_content.index(22);
                    let a11_3 = *datum_content.index(23);

                    let a12_0 = *datum_content.index(24);
                    let a12_1 = *datum_content.index(25);
                    let a12_2 = *datum_content.index(26);
                    let a12_3 = *datum_content.index(27);

                    let a13_0 = *datum_content.index(28);
                    let a13_1 = *datum_content.index(29);
                    let a13_2 = *datum_content.index(30);
                    let a13_3 = *datum_content.index(31);

                    let a20_0 = *datum_content.index(32);
                    let a20_1 = *datum_content.index(33);
                    let a20_2 = *datum_content.index(34);
                    let a20_3 = *datum_content.index(35);

                    let a21_0 = *datum_content.index(36);
                    let a21_1 = *datum_content.index(37);
                    let a21_2 = *datum_content.index(38);
                    let a21_3 = *datum_content.index(39);

                    let a22_0 = *datum_content.index(40);
                    let a22_1 = *datum_content.index(41);
                    let a22_2 = *datum_content.index(42);
                    let a22_3 = *datum_content.index(43);

                    let a23_0 = *datum_content.index(44);
                    let a23_1 = *datum_content.index(45);
                    let a23_2 = *datum_content.index(46);
                    let a23_3 = *datum_content.index(47);

                    let a30_0 = *datum_content.index(48);
                    let a30_1 = *datum_content.index(49);
                    let a30_2 = *datum_content.index(50);
                    let a30_3 = *datum_content.index(51);

                    let a31_0 = *datum_content.index(52);
                    let a31_1 = *datum_content.index(53);
                    let a31_2 = *datum_content.index(54);
                    let a31_3 = *datum_content.index(55);

                    let a32_0 = *datum_content.index(56);
                    let a32_1 = *datum_content.index(57);
                    let a32_2 = *datum_content.index(58);
                    let a32_3 = *datum_content.index(59);

                    let a33_0 = *datum_content.index(60);
                    let a33_1 = *datum_content.index(61);
                    let a33_2 = *datum_content.index(62);
                    let a33_3 = *datum_content.index(63);

                    let a00 = util::bits::u8_to_f32((a00_0, a00_1, a00_2, a00_3));
                    let a01 = util::bits::u8_to_f32((a01_0, a01_1, a01_2, a01_3));
                    let a02 = util::bits::u8_to_f32((a02_0, a02_1, a02_2, a02_3));
                    let a03 = util::bits::u8_to_f32((a03_0, a03_1, a03_2, a03_3));
                    let a10 = util::bits::u8_to_f32((a10_0, a10_1, a10_2, a10_3));
                    let a11 = util::bits::u8_to_f32((a11_0, a11_1, a11_2, a11_3));
                    let a12 = util::bits::u8_to_f32((a12_0, a12_1, a12_2, a12_3));
                    let a13 = util::bits::u8_to_f32((a13_0, a13_1, a13_2, a13_3));
                    let a20 = util::bits::u8_to_f32((a20_0, a20_1, a20_2, a20_3));
                    let a21 = util::bits::u8_to_f32((a21_0, a21_1, a21_2, a21_3));
                    let a22 = util::bits::u8_to_f32((a22_0, a22_1, a22_2, a22_3));
                    let a23 = util::bits::u8_to_f32((a23_0, a23_1, a23_2, a23_3));
                    let a30 = util::bits::u8_to_f32((a30_0, a30_1, a30_2, a30_3));
                    let a31 = util::bits::u8_to_f32((a31_0, a31_1, a31_2, a31_3));
                    let a32 = util::bits::u8_to_f32((a32_0, a32_1, a32_2, a32_3));
                    let a33 = util::bits::u8_to_f32((a33_0, a33_1, a33_2, a33_3));

                    LoadedGltfAccessorDatum::mat4_of(
                        a00, a01, a02, a03,
                        a10, a11, a12, a13,
                        a20, a21, a22, a23,
                        a30, a31, a32, a33
                    )
                }
            };
            result.push(datum);
        }

        result
    }
}

impl LoadedGltfAccessorDatum {
    pub fn scalar_of(num: GltfVecNum) -> Self {
        Self::Scalar(num)
    }

    pub fn vec2_of(
        x: GltfVecNum,
        y: GltfVecNum
    ) -> Self {
        let vec = Vector2::new(x, y);
        Self::Vec2(vec)
    }

    pub fn vec3_of(
        x: GltfVecNum,
        y: GltfVecNum,
        z: GltfVecNum
    ) -> Self {
        let vec = Vector3::new(x, y, z);
        Self::Vec3(vec)
    }

    pub fn vec4_of(
        x: GltfVecNum,
        y: GltfVecNum,
        z: GltfVecNum,
        w: GltfVecNum
    ) -> Self {
        let vec = Vector4::new(x, y, z, w);
        Self::Vec4(vec)
    }

    pub fn mat2_of(
        a00: GltfVecNum, a01: GltfVecNum,
        a10: GltfVecNum, a11: GltfVecNum,
    ) -> Self {
        let mat = Matrix2::new(
            a00, a01,
            a10, a11);
        Self::Mat2(mat)
    }

    pub fn mat3_of(
        a00: GltfVecNum, a01: GltfVecNum, a02: GltfVecNum,
        a10: GltfVecNum, a11: GltfVecNum, a12: GltfVecNum,
        a20: GltfVecNum, a21: GltfVecNum, a22: GltfVecNum
    ) -> Self {
        let mat = Matrix3::new(
            a00, a01, a02,
            a10, a11, a12,
            a20, a21, a22);
        Self::Mat3(mat)
    }

    pub fn mat4_of(
        a00: GltfVecNum, a01: GltfVecNum, a02: GltfVecNum, a03: GltfVecNum,
        a10: GltfVecNum, a11: GltfVecNum, a12: GltfVecNum, a13: GltfVecNum,
        a20: GltfVecNum, a21: GltfVecNum, a22: GltfVecNum, a23: GltfVecNum,
        a30: GltfVecNum, a31: GltfVecNum, a32: GltfVecNum, a33: GltfVecNum
    ) -> Self {
        let mat = Matrix4::new(
            a00, a01, a02, a03,
            a10, a11, a12, a13,
            a20, a21, a22, a23,
            a30, a31, a32, a33);
        Self::Mat4(mat)
    }

    pub fn dimension(&self) -> usize {
        match *self {
            Self::Scalar(_) => 1,
            Self::Vec2(_) => 2,
            Self::Vec3(_) => 3,
            Self::Vec4(_) => 4,
            Self::Mat2(_) => 2,
            Self::Mat3(_) => 3,
            Self::Mat4(_) => 4
        }
    }
}

impl<'a> LoadedGltfImage<'a> {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        index: usize,
        uri: String,
        data: Vec<u8>
    ) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            uri,
            data
        }
    }

    pub fn gltf(&self) -> Arc<Mutex<LoadedGltf<'a>>> {
        Arc::clone(&self.gltf)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn uri(&self) -> &String {
        &self.uri
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

impl<'a> LoadedGltfSampler<'a> {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        index: Option<usize>,
        mag_filter: Option<MagFilter>,
        min_filter: Option<MinFilter>,
        name: Option<String>,
        wrap_s: WrappingMode,
        wrap_t: WrappingMode
    ) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            mag_filter,
            min_filter,
            name,
            wrap_s,
            wrap_t
        }
    }

    pub fn new_from_sampler(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        sampler: &texture::Sampler
    ) -> Self {
        let name = match sampler.name() {
            Some(str) => Some(String::from(str)),
            None => None
        };
        Self::new(
            gltf,
            sampler.index(),
            sampler.mag_filter(),
            sampler.min_filter(),
            name,
            sampler.wrap_s(),
            sampler.wrap_t())
    }

    pub fn gltf(&self) -> Arc<Mutex<LoadedGltf<'a>>> {
        Arc::clone(&self.gltf)
    }

    /// Returns None if the sampler is the default one within the glTF.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn mag_filter(&self) -> Option<MagFilter> {
        self.mag_filter
    }

    pub fn mag_filter_gl(&self) -> Option<u32> {
        Some(mag_filter_to_gl_value(self.mag_filter?))
    }

    pub fn min_filter(&self) -> Option<MinFilter> {
        self.min_filter
    }

    pub fn min_filter_gl(&self) -> Option<u32> {
        Some(min_filter_to_gl_value(self.min_filter?))
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn wrap_s(&self) -> WrappingMode {
        self.wrap_s
    }

    pub fn wrap_s_gl(&self) -> u32 {
        wrapping_mode_to_gl_value(self.wrap_s)
    }

    pub fn wrap_t(&self) -> WrappingMode {
        self.wrap_t
    }

    pub fn wrap_t_gl(&self) -> u32 {
        wrapping_mode_to_gl_value(self.wrap_t)
    }
}

impl<'a> LoadedGltfTexture<'a> {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        index: usize,
        source_index: usize,
        sampler_index: usize
    ) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            source_index,
            sampler_index
        }
    }

    pub fn new_from_texture(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        texture: &texture::Texture
    ) -> Self {
        Self::new(
            gltf,
            texture.index(),
            texture.source().index(),
            // We unwrap here because we are sure the sampler has its index
            // as it has been specified by this texture.
            texture.sampler().index().unwrap())
    }

    pub fn gltf(&self) -> Arc<Mutex<LoadedGltf<'a>>> {
        Arc::clone(&self.gltf)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn source_index(&self) -> usize {
        self.source_index
    }

    pub fn sampler_index(&self) -> usize {
        self.sampler_index
    }
}

impl<'a> LoadedGltfMaterial<'a> {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        index: Option<usize>,
        pbr_metallic_roughness: material::PbrMetallicRoughnessInfo,
        normal_texture: Option<material::NormalTextureInfo>,
        occlusion_texture: Option<material::OcclusionTextureInfo>,
        emissive_texture: Option<material::EmissiveTextureInfo>,
        emissive_factor: material::EmissiveFactorInfo
    ) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            pbr_metallic_roughness,
            normal_texture,
            occlusion_texture,
            emissive_texture,
            emissive_factor
        }
    }

    pub fn new_from_material(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        material: &Material
    ) -> Self {
        let pbrmr = material.pbr_metallic_roughness();
        let base_color_texture;
        let metallic_roughness_texture;
        match pbrmr.base_color_texture() {
            Some(texture) => {
                let info = material::PbrMetallicRoughnessTextureInfo::new(
                    texture.texture().index(), texture.tex_coord() as usize);
                base_color_texture = Some(info);
            }
            None => base_color_texture = None
        }
        match pbrmr.metallic_roughness_texture() {
            Some(texture) => {
                let info = material::PbrMetallicRoughnessTextureInfo::new(
                    texture.texture().index(), texture.tex_coord() as usize);
                metallic_roughness_texture = Some(info);
            }
            None => metallic_roughness_texture = None
        }
        let arr = pbrmr.base_color_factor();
        let base_color_factor = Vector4::new(arr[0], arr[1], arr[2], arr[3]);
        let metallic_factor = pbrmr.metallic_factor();
        let roughness_factor = pbrmr.roughness_factor();
        let pbr_metallic_roughness = material::PbrMetallicRoughnessInfo::new(
            base_color_texture,
            base_color_factor,
            metallic_roughness_texture,
            metallic_factor,
            roughness_factor);

        let normal_texture;
        match material.normal_texture() {
            Some(texture) => {
                let info = material::NormalTextureInfo::new(
                    texture.scale(), texture.texture().index(), texture.tex_coord() as usize);
                normal_texture = Some(info);
            }
            None => normal_texture = None
        }

        let occlusion_texture;
        match material.occlusion_texture() {
            Some(texture) => {
                let info = material::OcclusionTextureInfo::new(texture.strength(), texture.texture().index(), texture.tex_coord() as usize);
                occlusion_texture = Some(info);
            }
            None => occlusion_texture = None
        }

        let emissive_texture;
        match material.emissive_texture() {
            Some(texture) => {
                let info = material::EmissiveTextureInfo::new(texture.texture().index(), texture.tex_coord() as usize);
                emissive_texture = Some(info);
            }
            None => emissive_texture = None
        }

        let arr = material.emissive_factor();
        let emissive_factor = Vector3::new(arr[0], arr[1], arr[2]);

        Self::new(
            gltf,
            material.index(),
            pbr_metallic_roughness,
            normal_texture,
            occlusion_texture,
            emissive_texture,
            emissive_factor)
    }


    pub fn gltf(&self) -> &Arc<Mutex<LoadedGltf<'a>>> {
        &self.gltf
    }

    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn pbr_metallic_roughness(&self) -> &material::PbrMetallicRoughnessInfo {
        &self.pbr_metallic_roughness
    }

    pub fn normal_texture(&self) -> &Option<material::NormalTextureInfo> {
        &self.normal_texture
    }

    pub fn occlusion_texture(&self) -> &Option<material::OcclusionTextureInfo> {
        &self.occlusion_texture
    }

    pub fn emissive_texture(&self) -> &Option<material::EmissiveTextureInfo> {
        &self.emissive_texture
    }

    pub fn emissive_factor(&self) -> material::EmissiveFactorInfo {
        self.emissive_factor
    }
}

impl<'a> LoadedGltfMesh<'a> {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        index: usize,
        primitives: Vec<mesh::PrimitiveInfo>,
        weights: Vec<f32>
    ) -> Self {
        Self {
            gltf: Arc::clone(gltf),
            index,
            primitives,
            weights
        }
    }

    pub fn new_empty(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        index: usize
    ) -> Self {
        Self::new(gltf, index, Vec::new(), Vec::new())
    }

    pub fn new_from_mesh(
        gltf: &Arc<Mutex<LoadedGltf<'a>>>,
        mesh: &Mesh
    ) -> Self {
        let index = mesh.index();
        let mut primitives = Vec::new();
        mesh.primitives().for_each(|it| {
            let primitive_info =
                mesh::PrimitiveInfo::new_from_primitive(&it);
            primitives.push(primitive_info);
        });
        let mut weights_info = Vec::new();
        if let Some(weights) = mesh.weights() {
            weights.iter().for_each(|it| {
                weights_info.push(it.clone());
            });
        };
        Self::new(gltf, index, primitives, weights_info)
    }

    pub fn gltf(&self) -> &Arc<Mutex<LoadedGltf<'a>>> {
        &self.gltf
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn primitives(&self) -> &Vec<mesh::PrimitiveInfo> {
        &self.primitives
    }

    pub fn weights(&self) -> &Vec<f32> {
        &self.weights
    }
}

impl<'a> LoadedGltf<'a> {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            buffer_views: Vec::new(),
            accessors: Vec::new(),
            images: Vec::new(),
            samplers: Vec::new(),
            textures: Vec::new(),
            materials: Vec::new(),
            meshes: Vec::new()
        }
    }

    pub fn buffers(&self) -> &Vec<LoadedGltfBuffer<'a>> {
        &self.buffers
    }

    pub fn buffers_mut(&mut self) -> &mut Vec<LoadedGltfBuffer<'a>> {
        &mut self.buffers
    }

    pub fn buffer_views(&self) -> &Vec<LoadedGltfBufferView<'a>> {
        &self.buffer_views
    }

    pub fn buffer_views_mut(&mut self) -> &mut Vec<LoadedGltfBufferView<'a>> {
        &mut self.buffer_views
    }

    pub fn accessors(&self) -> &Vec<LoadedGltfAccessor<'a>> {
        &self.accessors
    }

    pub fn accessors_mut(&mut self) -> &mut Vec<LoadedGltfAccessor<'a>> {
        &mut self.accessors
    }

    pub fn images(&self) -> &Vec<LoadedGltfImage<'a>> {
        &self.images
    }

    pub fn images_mut(&mut self) -> &mut Vec<LoadedGltfImage<'a>> {
        &mut self.images
    }

    pub fn samplers(&self) -> &Vec<LoadedGltfSampler<'a>> {
        &self.samplers
    }

    pub fn samplers_mut(&mut self) -> &mut Vec<LoadedGltfSampler<'a>> {
        &mut self.samplers
    }

    pub fn textures(&self) -> &Vec<LoadedGltfTexture<'a>> {
        &self.textures
    }

    pub fn textures_mut(&mut self) -> &mut Vec<LoadedGltfTexture<'a>> {
        &mut self.textures
    }

    pub fn materials(&self) -> &Vec<LoadedGltfMaterial<'a>> {
        &self.materials
    }

    pub fn materials_mut(&mut self) -> &mut Vec<LoadedGltfMaterial<'a>> {
        &mut self.materials
    }

    pub fn meshes(&self) -> &Vec<LoadedGltfMesh<'a>> {
        &self.meshes
    }

    pub fn meshes_mut(&mut self) -> &mut Vec<LoadedGltfMesh<'a>> {
        &mut self.meshes
    }
}

impl<'a> LoadedGltfWrapper<'a> {
    pub fn new(gltf: LoadedGltf<'a>) -> Self {
        Self {
            gltf: Arc::new(Mutex::new(gltf))
        }
    }

    pub fn get(&self) -> &Arc<Mutex<LoadedGltf<'a>>> {
        &self.gltf
    }
}