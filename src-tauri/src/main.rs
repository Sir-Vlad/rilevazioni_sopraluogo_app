#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(unused_must_use)]


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run();
}
