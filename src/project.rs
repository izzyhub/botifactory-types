use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectJson {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectBody {
    pub project: ProjectJson,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProject {
    pub name: String,
}

impl CreateProject {
    pub fn new(name: String) -> Self {
        CreateProject { name }
    }
}

impl ProjectJson {
    pub fn new(id: i64, name: String, created_at: i64, updated_at: i64) -> Self {
        ProjectJson {
            id,
            name,
            created_at,
            updated_at,
        }
    }
}
