use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetPackManifest {
    pub id: String,
    #[serde(default)]
    pub meshes: Vec<MeshAsset>,
    #[serde(default)]
    pub materials: Vec<String>,
    #[serde(default)]
    pub descriptors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshAsset {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub path: Option<String>,
}

impl AssetPackManifest {
    pub fn procedural_house_demo() -> Self {
        Self {
            id: "xr-house-demo".to_string(),
            meshes: vec![
                MeshAsset { id: "house".to_string(), kind: "procedural-house".to_string(), path: None },
                MeshAsset { id: "blue-cube".to_string(), kind: "cube".to_string(), path: None },
                MeshAsset { id: "red-ball".to_string(), kind: "sphere".to_string(), path: None },
                MeshAsset { id: "mug".to_string(), kind: "cylinder".to_string(), path: None },
            ],
            materials: vec!["toon-house".to_string(), "toon-roof".to_string(), "toon-blue-prop".to_string()],
            descriptors: vec!["host.adaptive.json".to_string(), "interaction.grab.json".to_string(), "materials.toon.json".to_string()],
        }
    }

    pub fn contains_mesh(&self, id: &str) -> bool {
        self.meshes.iter().any(|mesh| mesh.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural_pack_contains_house_and_cube() {
        let pack = AssetPackManifest::procedural_house_demo();
        assert!(pack.contains_mesh("house"));
        assert!(pack.contains_mesh("blue-cube"));
    }
}
