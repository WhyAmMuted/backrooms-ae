use std::{io::Error, ptr::null_mut, sync::Arc};

use crate::steamaudio_module::{
    IPLMaterial, IPLScene, IPLStaticMesh, IPLStaticMeshSettings, IPLTriangle, IPLVector3,
    SteamAudioErrors, Vector3,
    basic::{material::Material, scene::Scene},
    catch_error, iplStaticMeshAdd, iplStaticMeshCreate, iplStaticMeshRelease, iplStaticMeshRemove,
};

#[repr(C)]
pub struct Triangle {
    pub first_vertex: i32,
    pub second_vertex: i32,
    pub third_vertex: i32,
}

impl From<Triangle> for IPLTriangle {
    fn from(value: Triangle) -> Self {
        Self {
            indices: [value.first_vertex, value.second_vertex, value.third_vertex],
        }
    }
}

pub struct StaticMesh {
    scene: Option<Arc<Scene>>,
    static_mesh: Option<IPLStaticMesh>,

    count_vertices: i32,
    count_triangles: i32,
    count_materials: i32,

    vertices: Vec<Vector3>,
    triangles: Vec<Triangle>,
    material_indices: Vec<i32>,

    materials: Vec<Material>,
}

impl StaticMesh {
    pub fn new(
        verticles: Vec<Vector3>,
        triangles: Vec<Triangle>,
        material_indices: Vec<i32>,
        materials: Vec<Material>,

        scene: Option<Arc<Scene>>,
    ) -> Self {
        Self {
            scene: scene,
            static_mesh: None,
            count_vertices: verticles.len() as i32,
            count_triangles: triangles.len() as i32,
            count_materials: materials.len() as i32,
            vertices: verticles,
            triangles: triangles,
            material_indices: material_indices,
            materials: materials,
        }
    }

    pub fn create(&mut self) -> Result<(), SteamAudioErrors> {
        let scene = match &self.scene {
            Some(s) => *s.as_raw(),
            None => return Err(SteamAudioErrors::Unknown(0)),
        };
        let mut mesh: IPLStaticMesh = null_mut();

        let mut settings = IPLStaticMeshSettings {
            numMaterials: self.count_materials,
            numTriangles: self.count_triangles,
            numVertices: self.count_vertices,

            vertices: self.vertices.as_mut_ptr() as *mut Vector3,
            triangles: self.triangles.as_mut_ptr() as *mut IPLTriangle,

            materials: self.materials.as_mut_ptr() as *mut IPLMaterial,
            materialIndices: self.material_indices.as_mut_ptr() as *mut i32,
        };

        let status = unsafe { iplStaticMeshCreate(scene, &mut settings, &mut mesh) };

        match catch_error(SteamAudioErrors::convert(status), ()) {
            Err(e) => Err(e),
            Ok(data) => {
                self.static_mesh = Some(mesh);
                unsafe {
                    iplStaticMeshAdd(mesh, scene);
                }

                Ok(())
            }
        }
    }

    pub fn scene_bind(&mut self, scene: Arc<Scene>) {
        self.scene = Some(scene);
    }
}

impl Drop for StaticMesh {
    fn drop(&mut self) {
        if let (Some(mut mesh), Some(scene)) = (self.static_mesh, &self.scene) {
            unsafe {
                iplStaticMeshRemove(mesh, *scene.as_raw());
                iplStaticMeshRelease(&mut mesh);
            }
            self.static_mesh = None;
        }
    }
}
