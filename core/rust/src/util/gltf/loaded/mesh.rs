extern crate gltf;

pub struct PrimitiveInfo {
    index: usize,
    mode: gltf::mesh::Mode,
    /// None if not defined in glTF.
    indices: Option<usize>,
    attributes: Vec<AttributeInfo>,
    morph_targets: Vec<MorphTargetInfo>,
    /// None if not defined in glTF.
    material: Option<usize>
}

pub struct AttributeInfo {
    type_semantic: gltf::mesh::Semantic,
    accessor_index: usize
}

pub struct MorphTargetInfo {
    /// None if not defined in glTF.
    positions: Option<usize>,
    /// None if not defined in glTF.
    normals: Option<usize>,
    /// None if not defined in glTF.
    tangents: Option<usize>
}

impl PrimitiveInfo {
    pub fn new(
        index: usize,
        mode: gltf::mesh::Mode,
        indices: Option<usize>,
        attributes: Vec<AttributeInfo>,
        morph_targets: Vec<MorphTargetInfo>,
        material: Option<usize>
    ) -> Self {
        Self {
            index,
            mode,
            indices,
            attributes,
            morph_targets,
            material
        }
    }

    pub fn new_from_primitive(
        primitive: &gltf::mesh::Primitive
    ) -> Self {
        let index = primitive.index();
        let mode = primitive.mode();
        let indices;
        match primitive.indices() {
            Some(accessor) => {
                indices = Some(accessor.index());
            }
            None => indices = None
        }
        let mut attributes = Vec::new();
        for attr in primitive.attributes() {
            let sem = attr.0;
            let index = attr.1.index();
            attributes.push(AttributeInfo::new(sem, index));
        }
        let mut morph_targets = Vec::new();
        for target in primitive.morph_targets() {
            let mut target_info = MorphTargetInfo::new_empty();
            if let Some(positions) = target.positions() {
                target_info.set_positions(Some(positions.index()))
            }
            if let Some(normals) = target.normals() {
                target_info.set_normals(Some(normals.index()))
            }
            if let Some(tangents) = target.tangents() {
                target_info.set_tangents(Some(tangents.index()))
            }
            morph_targets.push(target_info)
        }
        let material = primitive.material().index();

        Self::new(index, mode, indices, attributes, morph_targets, material)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn mode(&self) -> gltf::mesh::Mode {
        self.mode
    }

    pub fn indices(&self) -> Option<usize> {
        self.indices
    }

    pub fn attributes(&self) -> &Vec<AttributeInfo> {
        &self.attributes
    }

    pub fn morph_targets(&self) -> &Vec<MorphTargetInfo> {
        &self.morph_targets
    }

    pub fn material(&self) -> Option<usize> {
        self.material
    }
}

impl AttributeInfo {
    pub fn new(
        type_semantic: gltf::mesh::Semantic,
        accessor_index: usize
    ) -> Self {
        Self {
            type_semantic,
            accessor_index
        }
    }

    pub fn type_semantic(&self) -> &gltf::mesh::Semantic {
        &self.type_semantic
    }

    pub fn accessor_index(&self) -> usize {
        self.accessor_index
    }
}

impl MorphTargetInfo {
    pub fn new(
        positions: Option<usize>,
        normals: Option<usize>,
        tangents: Option<usize>
    ) -> Self {
        Self {
            positions,
            normals,
            tangents
        }
    }

    pub fn new_empty() -> Self {
        Self::new(None, None, None)
    }


    pub fn positions(&self) -> Option<usize> {
        self.positions
    }

    pub fn set_positions(&mut self, positions: Option<usize>) {
        self.positions = positions
    }

    pub fn normals(&self) -> Option<usize> {
        self.normals
    }

    pub fn set_normals(&mut self, normals: Option<usize>) {
        self.normals = normals
    }

    pub fn tangents(&self) -> Option<usize> {
        self.tangents
    }

    pub fn set_tangents(&mut self, tangents: Option<usize>) {
        self.tangents = tangents
    }
}
