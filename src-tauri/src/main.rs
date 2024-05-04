// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod postman_json_importer;

use once_cell::sync::OnceCell;

use std::{sync::Mutex, collections::HashMap};
use std::error::Error;
use std::ops::Deref;
use once_cell::sync::Lazy;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::generate_handler;

static GLOBAL_HTTP_CLIENT: Lazy<Mutex<reqwest::Client>> = Lazy::new(|| {
    let client = reqwest::Client::new();
    Mutex::new(client)
});

static GLOBAL_SQLITE_CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("../target/my_db.db3").unwrap();

    // Check if the request table exists
    let table_exists: Result<i64, rusqlite::Error> = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='request'",
        [],
        |row| row.get(0),
    );

    if table_exists.unwrap() > 0 {
        return Mutex::new(conn);
    }

    conn.execute(
        "CREATE TABLE request (

            uri TEXT NOT NULL,
            http_type TEXT NOT NULL,
            body TEXT NOT NULL,
            body_type TEXT NOT NULL
        )",
        (),
    ).unwrap();
    Mutex::new(conn)
});

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![greet, generate_guid, generate_http_request, save_request, get_all_saved_requests, open_docs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
async fn open_docs(window_id: String, handle: tauri::AppHandle){
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        window_id,
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap())
    ).build().unwrap();
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn generate_guid(version: &str) -> String {
    let result = match version {
        "v4" => generate_v4_guid(),
        _ => generate_v4_guid()
    };

    result
}

fn generate_v4_guid() -> String {
    let guid = uuid::Uuid::new_v4();
    guid.to_string()
}

#[derive(Debug, Deserialize, Serialize)]
struct HttpRequest {
    uri: String,
    http_type: String,
    body: String,
    body_type: String
}


#[tauri::command]
fn save_request(request: HttpRequest) -> Result<String,()> {
    println!("{:?}", request);
    let conn = GLOBAL_SQLITE_CONNECTION.lock().unwrap();

    let result = conn.execute(
        "INSERT INTO request (uri, http_type, body, body_type) VALUES (?1, ?2, ?3, ?4)",
        (request.uri, request.http_type, request.body, request.body_type),
    );

    match result {
        Ok(_) => Ok("Request saved successfully".to_string()),
        Err(_) => return Ok("Failed to save request".to_string())
    }
}

#[tauri::command]
fn get_all_saved_requests() -> Result<Vec<HttpRequest>, ()> {
    let conn = GLOBAL_SQLITE_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT uri, http_type, body, body_type FROM request").unwrap();
    let request_iter = stmt.query_map([], |row| {
        let request = HttpRequest {
            uri: row.get(0)?,
            http_type: row.get(1)?,
            body: row.get(2)?,
            body_type: row.get(3)?
        };
        return Ok(request);
    }).unwrap();

    let mut requests: Vec<HttpRequest> = Vec::new();
    for request in request_iter {
        let http_request = request.unwrap();
        requests.push(http_request);
    }

    if requests.len() > 0 {
        Ok(requests)
    } else {
        Err(())
    }
}

#[tauri::command]
async fn generate_http_request(uri: String, http_type: String, body: String, body_type: String) -> Result<String,()> {
    let client = reqwest::Client::new();
    let response = match http_type.as_str() {
        "GET" => client.get(uri).send().await,
        "POST" => {
            let request = client.post(uri);
            let request = match body_type.as_str() {
                "json" => request.json(&body),
                _ => request.body(body.to_string())
            };
            request.send().await
        },
        _ => client.get(uri).send().await
    };

    let text = response.unwrap().text().await;

    match text {
        Ok(text) => Ok(text),
        Err(_) => Err(())
    }
}