use crate::common::{
    PolyMesh,
    MeshDef,
    TransPolyMeshPtr
};


pub fn get_flat_geometry(root_mesh: PolyMesh) -> Vec<MeshDef> {

    // Create a list of all found geometry
    let mut all_geo = Vec::new();

    // Begin recursive search for geometry
    get_flat_geometry_recursive(&root_mesh, None, &mut all_geo);

    return all_geo;

}

fn get_flat_geometry_recursive(root_mesh: &PolyMesh, parent_transform: Option<&TransPolyMeshPtr>, all_geo: &mut Vec<MeshDef>){


    // Search all children
    for child in &root_mesh.children {

        // Get the child with an absolute transform from its root
        let abs_child = child.new_from_transform_optional(parent_transform);

        // Get the child's mesh
        let child_mesh = abs_child.mesh.as_ref();

        // Check if the child contains geometry
        if child_mesh.contains_geometry() {

            // Transform the child's geometry to an absolute position
            let abs_geometry = child_mesh.geometry.as_ref().unwrap().transformed_by(&abs_child);

            // Add the geometry to the list
            all_geo.push(abs_geometry);

        }

        // Search for geometry
        get_flat_geometry_recursive(child.mesh.as_ref(), Some(&abs_child), all_geo);
       
    }

}