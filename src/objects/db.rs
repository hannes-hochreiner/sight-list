use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct List {
    pub id: Uuid,
    pub version: Uuid,
    pub title: String,
}

#[derive(FromRow)]
pub struct Sight {
    pub id: Uuid,
    pub version: Uuid,
    pub title: String,
    pub lat: f64,
    pub long: f64,
    pub description: Option<String>,
    pub list_id: Uuid,
}
