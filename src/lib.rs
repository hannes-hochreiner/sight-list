use uuid::Uuid;

pub mod objects;
pub mod sight_list_error;

pub fn get_js_id(id: &Uuid) -> String {
    id.to_string().replace("-", "_")
}
