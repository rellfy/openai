use serde::Deserialize;

#[derive(Deserialize)]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub owned_by: String,
}

// TODO: move list_models function here for organization
