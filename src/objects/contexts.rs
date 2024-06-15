use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ErrorContext {
    pub message: String,
}

#[derive(Serialize)]
pub struct ListListContext {
    pub lists: Vec<ListContext>,
}

#[derive(Serialize)]
pub struct MapMarkerContext {
    pub js_id: String,
    pub lat: f64,
    pub long: f64,
    pub content: String,
}

#[derive(Serialize)]
pub enum MapView {
    LatLongZoom {
        lat: f64,
        long: f64,
        zoom: i32,
    },
    Box {
        lat_min: f64,
        lat_max: f64,
        long_min: f64,
        long_max: f64,
    },
}

#[derive(Serialize)]
pub struct MapContext {
    pub view: MapView,
    pub markers: Vec<MapMarkerContext>,
}

#[derive(Serialize)]
pub struct ListPageContext {
    pub id: Uuid,
    pub title: String,
    pub map: MapContext,
    pub sights: Vec<SightContext>,
}

#[derive(Serialize)]
pub struct SightListContext {
    pub sights: Vec<SightContext>,
}

#[derive(Serialize)]
pub struct ListKmlContext {
    pub title: String,
    pub sights: Vec<SightContext>,
}

#[derive(Serialize)]
pub struct SightContext {
    pub id: Uuid,
    pub js_id: String,
    pub title: String,
    pub lat: f64,
    pub long: f64,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct ListContext {
    pub id: Uuid,
    pub title: String,
}

#[derive(Serialize)]
pub struct ListEditContext {
    pub id: Uuid,
    pub version: Uuid,
    pub fields: Option<ListEditFieldContext>,
}

#[derive(Serialize)]
pub struct ListEditFieldContext {
    pub title: String,
}

#[derive(Serialize)]
pub struct SightEditContext {
    pub list_id: Uuid,
    pub id: Uuid,
    pub js_id: String,
    pub version: Uuid,
    pub fields: Option<SightEditFieldContext>,
}

#[derive(Serialize)]
pub struct SightEditFieldContext {
    pub title: String,
    pub lat: f64,
    pub long: f64,
    pub description: Option<String>,
}
