use crate::model;
use crate::service;

use axum::http::StatusCode;
use axum::Json;
use axum::extract::{ Query, Multipart };
use axum::extract;

pub async fn list_sheets() {
    
}

pub async fn add_sheet(
    mut multipart: Multipart,
) -> StatusCode 
{
    let mut new_sheet: Option<model::NewSheetRequest> = None;
    let mut columns: Vec<model::NewSheetMetaDataRequest> = Vec::new();
    let mut model_file: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some(name) = field.name() {
            match name {
                "sheet" => {
                    if let Ok(data) = field.bytes().await {
                        new_sheet = serde_json::from_slice(&data).unwrap();
                    }
                }
                "columns" => {
                    if let Ok(data) = field.bytes().await {
                        let a = serde_json::from_slice(&data);
                        columns = a.unwrap();
                    }
                }
                "model" => {
                    if model_file.is_some() {
                        return StatusCode::BAD_REQUEST;
                    }
    
                    if let Ok(data) = field.bytes().await {
                        model_file = Some(data.to_vec());
                    }
                }
                _ => {}
            }
        }
    }

    if columns.len() == 0 { return StatusCode::BAD_REQUEST; }

    if let Some(new_sheet) = new_sheet {
        service::add_sheet_to_db(new_sheet, columns, 1, model_file).await;
        // service::add_sheet_to_db_(new_sheet, columns, 1, model_file).await;
        
        // match service::add_sheet_to_db_(new_sheet, columns, 1, model_file).await {
        //     Ok(_) => return StatusCode::OK,
        //     Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        // }
    }

    StatusCode::BAD_REQUEST
}