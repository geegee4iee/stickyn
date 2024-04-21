// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, generate_guid, generate_http_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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

#[tauri::command]
async fn generate_http_request(uri: &str, http_type: &str, body: &str, body_type: &str) -> Result<String,()> {
    let client = reqwest::Client::new();
    let response = match http_type {
        "GET" => client.get(uri).send().await,
        "POST" => {
            let request = client.post(uri);
            let request = match body_type {
                "json" => request.json(body),
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