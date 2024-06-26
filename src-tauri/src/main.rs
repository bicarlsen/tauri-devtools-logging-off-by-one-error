#![feature(stmt_expr_attributes)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    tracing::debug!(name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let mut builder = tauri::Builder::default();

    // --- Choose `tracing_subscriber` or `devtools` for logging.
    // ------ tracing
    // tracing_subscriber::registry()
    //     .with(fmt::layer())
    //     .with(EnvFilter::from_default_env())
    //     .init();

    // ----- devtools
    let devtools = tauri_plugin_devtools::init(); // initialize the plugin as early as possible
    builder = builder.plugin(devtools); // then register it with Tauri

    builder
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
