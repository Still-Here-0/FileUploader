use file_uploader::api;

use axum::{
    routing::{ get, post, put, delete, patch },
    Router,
};

pub fn app() -> Router {

    let app = root_scream()
        .merge(sheet_routes());

    app
}

fn root_scream() -> Router {
    Router::new()
        .route("/", get(api::root::api_scream))
}

fn sheet_routes() -> Router {
    let path = "/sheet";

    Router::new()
        .route(&format!("{path}/add"), post(api::sheet::add_sheet))
}