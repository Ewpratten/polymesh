use serde::{Deserialize, Serialize};
use super::{
    transform::PolyVector,
    serialization::data::{
        polymeta::{
            PolyMeta,
            PolyChildReference,
            LATEST_POLY_META_VERSION
        },
        mesh::MeshDef
    }
};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum MeshType {
    Group,
    Geometry,
    GeoGroup
}

/// TransPolyMeshPtr is a small wrapper around a reference to a PolyMesh, describing a transformation on the mesh
#[derive(Debug, Clone, PartialEq)]
pub struct TransPolyMeshPtr {

    /// File path to the referenced mesh
    pub path: String,

    /// Mesh reference
    pub mesh: Box<PolyMesh>,

    /// Optional translation
    pub translation: Option<PolyVector>

}

impl TransPolyMeshPtr {

    pub fn get_translation(&self) -> PolyVector {
        match self.translation {
            Some(x) => x,
            None => PolyVector::zero()
        }
    }

    pub fn new_from_transform_optional(&self, other: Option<&TransPolyMeshPtr>) -> Self {
        match other {
            Some(val) => self.new_from_transform(val),
            None => self.clone()
        }
    }

    pub fn new_from_transform(&self, other: &TransPolyMeshPtr) -> Self {
        
        // Get both translations
        let this_translation = self.get_translation();
        let other_translation = other.get_translation();

        // Create a new TransPolyMeshPtr
        Self {
            path: self.path.to_string(),
            mesh: self.mesh.clone(),
            translation: Some(this_translation + other_translation)
        }
    }

}

/// A PolyMesh is any mesh, weather it contains geometry, other meshes, or a mix of both
#[derive(Debug, Clone, PartialEq)]
pub struct PolyMesh {

    /// The type of this mesh
    pub mesh_type: MeshType,

    /// Possible geometry for this mesh
    pub geometry: Option<MeshDef>,

    /// Arbitrary metadata
    pub metadata: HashMap<String, String>,

    /// All children of this mesh
    pub children: Vec<TransPolyMeshPtr>

}

impl PolyMesh {

    /// Create a new PolyMesh
    pub fn new(mesh_type: MeshType, geometry: Option<MeshDef>) -> Self {
        PolyMesh {
            mesh_type,
            geometry,
            metadata: HashMap::new(),
            children: Vec::new()
        }
    }

    /// Add arbitrary data to the mesh
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Add a child to the mesh
    pub fn add_child(&mut self, child: TransPolyMeshPtr) {
        self.children.push(child);
    }

    // Try to get arbitrary metadata
    pub fn try_get_meta_field(&self, key: &str) -> Result<&String, ()> {
        if self.metadata.contains_key(key) {
            Ok(&self.metadata[key])
        } else {
            Err(())
        }
    }

    /// Try to fetch the mesh name from metadata
    pub fn get_name(&self) -> String {
        return match self.try_get_meta_field("name") {
            Ok(name) => name.to_string(),
            Err(_) => "Unnamed".to_string()
        };
    }

    /// Set the mesh name
    pub fn set_name(&mut self, name: String) {
        self.metadata.insert("name".to_string(), name);
    }

    /// Get if this mesh is requesting the BETA "Runtime Culling" feature
    pub fn uses_runtime_culling(&self) -> bool {
        return match self.try_get_meta_field("_beta_runtime_culling") {
            Ok(result) => result == "on",
            Err(_) => false
        };
    }

    pub fn enable_runtime_culling(&mut self) {
        self.add_metadata("_beta_runtime_culling".to_string(), "on".to_string());
    }

    /// Converts this mesh into a PolyMeta object that describes it
    pub fn to_poly_meta(&self) -> PolyMeta {

        // Collect children
        let mut children = Vec::new();
        for child in &self.children {
            children.push(PolyChildReference {
                path: (*child.path).to_string(),
                translation: child.translation
            })
        }

        // Build output
        return PolyMeta {
            version: LATEST_POLY_META_VERSION,
            mesh_type: self.mesh_type,
            metadata: self.metadata.clone(),
            children: children
        };
    }

    /// Check if the mesh contains geometry
    pub fn contains_geometry(&self) -> bool {
        return self.mesh_type == MeshType::Geometry || self.mesh_type == MeshType::GeoGroup || self.geometry.is_some();
    }

}