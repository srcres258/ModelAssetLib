extern crate gltf;
extern crate nalgebra;

use std::ops::Index;
use gltf::{Accessor, buffer, texture};
use crate::{constants, util};
use std::sync::{Arc, Mutex};
use gltf::accessor::Dimensions;
use gltf::buffer::View;
use gltf::json::Value;
use gltf::texture::{MagFilter, MinFilter, WrappingMode};
use nalgebra::{Matrix2, Matrix3, Matrix4, SMatrix, SVector, Vector2, Vector3, Vector4};

pub struct LoadedGltfBuffer {
    gltf: Arc<Mutex<LoadedGltf>>,
    index: usize,
    uri: String,
    data: Vec<u8>
}

pub struct LoadedGltfBufferView {
    gltf: Arc<Mutex<LoadedGltf>>,
    index: usize,
    buffer_index: usize,
    data_offset: usize,
    data_length: usize,
    data_stride: Option<usize>,
    target: Option<buffer::Target>
}

pub struct  LoadedGltfAccessor {
    gltf: Arc<Mutex<LoadedGltf>>,
    index: usize,
    buffer_view_index: usize,
    comp_size: usize,
    comp_count: usize,
    max_values: Option<Vec<Value>>,
    min_values: Option<Vec<Value>>,
    dimensions: Dimensions
}

pub struct LoadedGltfImage {
    gltf: Arc<Mutex<LoadedGltf>>,
    index: usize,
    uri: String,
    data: Vec<u8>
}

pub struct LoadedGltfSampler {
    gltf: Arc<Mutex<LoadedGltf>>,
    /// None if the sampler is the default one within the glTF.
    index: Option<usize>,
    mag_filter: Option<MagFilter>,
    min_filter: Option<MinFilter>,
    name: Option<String>,
    wrap_s: WrappingMode,
    wrap_t: WrappingMode
}

pub struct LoadedGltf {
    buffers: Vec<LoadedGltfBuffer>,
    buffer_views: Vec<LoadedGltfBufferView>,
    accessors: Vec<LoadedGltfAccessor>,
    images: Vec<LoadedGltfImage>,
    samplers: Vec<LoadedGltfSampler>
}

pub struct LoadedGltfWrapper {
    gltf: Arc<Mutex<LoadedGltf>>
}

impl LoadedGltfBuffer {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf>>,
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

    pub fn get_gltf(&self) -> Arc<Mutex<LoadedGltf>> {
        Arc::clone(&self.gltf)
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_uri(&self) -> &String {
        &self.uri
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

type GltfPosNum = f32;

pub enum LoadedGltfAccessorDatum {
    Scalar(GltfPosNum),
    Vec2(SVector<GltfPosNum, 2>),
    Vec3(SVector<GltfPosNum, 3>),
    Vec4(SVector<GltfPosNum, 4>),
    Mat2(SMatrix<GltfPosNum, 2, 2>),
    Mat3(SMatrix<GltfPosNum, 3, 3>),
    Mat4(SMatrix<GltfPosNum, 4, 4>)
}

impl LoadedGltfBufferView {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf>>,
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

    pub fn new_from_view(gltf: &Arc<Mutex<LoadedGltf>>, view: &View) -> Self {
        Self::new(gltf, view.index(), view.buffer().index(), view.offset(),
                  view.length(), view.stride(), view.target())
    }

    pub fn get_gltf(&self) -> Arc<Mutex<LoadedGltf>> {
        Arc::clone(&self.gltf)
    }

    pub fn get_buffer_index(&self) -> usize {
        self.buffer_index
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_data_offset(&self) -> usize {
        self.data_offset
    }

    pub fn get_data_length(&self) -> usize {
        self.data_length
    }

    pub fn get_data_stride(&self) -> Option<usize> {
        self.data_stride
    }

    pub fn get_target(&self) -> Option<buffer::Target> {
        self.target
    }

    pub fn load_data(&self) -> Vec<u8> {
        self.load_data_strided(0)
    }

    pub fn load_data_strided(&self, stride_count: u32) -> Vec<u8> {
        let gltf = self.gltf.lock().unwrap();
        let buffer = gltf.get_buffers().get(self.buffer_index).unwrap();
        let mut result = util::new_buffer_vec(self.data_length, 0u8);
        let result_ptr = result.as_mut_ptr();
        unsafe {
            let data_ptr = buffer.get_data_ptr().add(self.data_offset)
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

impl LoadedGltfAccessor {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf>>,
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
        gltf: &Arc<Mutex<LoadedGltf>>,
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

    pub fn get_gltf(&self) -> Arc<Mutex<LoadedGltf>> {
        Arc::clone(&self.gltf)
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_buffer_view_index(&self) -> usize {
        self.buffer_view_index
    }

    pub fn get_comp_size(&self) -> usize {
        self.comp_size
    }

    pub fn get_comp_count(&self) -> usize {
        self.comp_count
    }

    pub fn get_max_values(&self) -> Option<Vec<Value>> {
        self.max_values.clone()
    }

    pub fn get_min_values(&self) -> Option<Vec<Value>> {
        self.min_values.clone()
    }

    pub fn get_dimensions(&self) -> Dimensions {
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
    pub fn scalar_of(num: GltfPosNum) -> Self {
        Self::Scalar(num)
    }

    pub fn vec2_of(
        x: GltfPosNum,
        y: GltfPosNum
    ) -> Self {
        let vec = Vector2::new(x, y);
        Self::Vec2(vec)
    }

    pub fn vec3_of(
        x: GltfPosNum,
        y: GltfPosNum,
        z: GltfPosNum
    ) -> Self {
        let vec = Vector3::new(x, y, z);
        Self::Vec3(vec)
    }

    pub fn vec4_of(
        x: GltfPosNum,
        y: GltfPosNum,
        z: GltfPosNum,
        w: GltfPosNum
    ) -> Self {
        let vec = Vector4::new(x, y, z, w);
        Self::Vec4(vec)
    }

    pub fn mat2_of(
        a00: GltfPosNum, a01: GltfPosNum,
        a10: GltfPosNum, a11: GltfPosNum,
    ) -> Self {
        let mat = Matrix2::new(
            a00, a01,
            a10, a11);
        Self::Mat2(mat)
    }

    pub fn mat3_of(
        a00: GltfPosNum, a01: GltfPosNum, a02: GltfPosNum,
        a10: GltfPosNum, a11: GltfPosNum, a12: GltfPosNum,
        a20: GltfPosNum, a21: GltfPosNum, a22: GltfPosNum
    ) -> Self {
        let mat = Matrix3::new(
            a00, a01, a02,
            a10, a11, a12,
            a20, a21, a22);
        Self::Mat3(mat)
    }

    pub fn mat4_of(
        a00: GltfPosNum, a01: GltfPosNum, a02: GltfPosNum, a03: GltfPosNum,
        a10: GltfPosNum, a11: GltfPosNum, a12: GltfPosNum, a13: GltfPosNum,
        a20: GltfPosNum, a21: GltfPosNum, a22: GltfPosNum, a23: GltfPosNum,
        a30: GltfPosNum, a31: GltfPosNum, a32: GltfPosNum, a33: GltfPosNum
    ) -> Self {
        let mat = Matrix4::new(
            a00, a01, a02, a03,
            a10, a11, a12, a13,
            a20, a21, a22, a23,
            a30, a31, a32, a33);
        Self::Mat4(mat)
    }

    pub fn get_dimension(&self) -> usize {
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

impl LoadedGltfImage {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf>>,
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

    pub fn get_gltf(&self) -> Arc<Mutex<LoadedGltf>> {
        Arc::clone(&self.gltf)
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_uri(&self) -> &String {
        &self.uri
    }
    
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
    
    pub fn get_data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

impl LoadedGltfSampler {
    pub fn new(
        gltf: &Arc<Mutex<LoadedGltf>>,
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
        gltf: &Arc<Mutex<LoadedGltf>>,
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

    pub fn get_gltf(&self) -> Arc<Mutex<LoadedGltf>> {
        Arc::clone(&self.gltf)
    }

    /// Returns None if the sampler is the default one within the glTF.
    pub fn get_index(&self) -> Option<usize> {
        self.index
    }

    pub fn get_mag_filter(&self) -> Option<MagFilter> {
        self.mag_filter
    }

    pub fn get_mag_filter_gl(&self) -> Option<u32> {
        Some(mag_filter_to_gl_value(self.mag_filter?))
    }

    pub fn get_min_filter(&self) -> Option<MinFilter> {
        self.min_filter
    }

    pub fn get_min_filter_gl(&self) -> Option<u32> {
        Some(min_filter_to_gl_value(self.min_filter?))
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn get_wrap_s(&self) -> WrappingMode {
        self.wrap_s
    }

    pub fn get_wrap_s_gl(&self) -> u32 {
        wrapping_mode_to_gl_value(self.wrap_s)
    }

    pub fn get_wrap_t(&self) -> WrappingMode {
        self.wrap_t
    }

    pub fn get_wrap_t_gl(&self) -> u32 {
        wrapping_mode_to_gl_value(self.wrap_t)
    }
}

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

impl LoadedGltf {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            buffer_views: Vec::new(),
            accessors: Vec::new(),
            images: Vec::new(),
            samplers: Vec::new()
        }
    }

    pub fn get_buffers(&self) -> &Vec<LoadedGltfBuffer> {
        &self.buffers
    }

    pub fn get_buffers_mut(&mut self) -> &mut Vec<LoadedGltfBuffer> {
        &mut self.buffers
    }

    pub fn get_buffer_views(&self) -> &Vec<LoadedGltfBufferView> {
        &self.buffer_views
    }

    pub fn get_buffer_views_mut(&mut self) -> &mut Vec<LoadedGltfBufferView> {
        &mut self.buffer_views
    }

    pub fn get_accessors(&self) -> &Vec<LoadedGltfAccessor> {
        &self.accessors
    }

    pub fn get_accessors_mut(&mut self) -> &mut Vec<LoadedGltfAccessor> {
        &mut self.accessors
    }

    pub fn get_images(&self) -> &Vec<LoadedGltfImage> {
        &self.images
    }

    pub fn get_images_mut(&mut self) -> &mut Vec<LoadedGltfImage> {
        &mut self.images
    }

    pub fn get_samplers(&self) -> &Vec<LoadedGltfSampler> {
        &self.samplers
    }

    pub fn get_samplers_mut(&mut self) -> &mut Vec<LoadedGltfSampler> {
        &mut self.samplers
    }
}

impl LoadedGltfWrapper {
    pub fn new(gltf: LoadedGltf) -> Self {
        Self {
            gltf: Arc::new(Mutex::new(gltf))
        }
    }

    pub fn get(&self) -> &Arc<Mutex<LoadedGltf>> {
        &self.gltf
    }
}