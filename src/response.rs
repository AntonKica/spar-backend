use serde::Serialize;

#[derive(Serialize)]
pub struct EnumResponse {
    pub code: i32,
    pub name: String,
}

