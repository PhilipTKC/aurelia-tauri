#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

use std::sync::Mutex;
use tauri::State;

struct Counter(Mutex<i32>);

fn main() {
    tauri::Builder::default()
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![
            return_os,
            play_sine,
            counter,
            return_counter_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn return_os(name: String) -> String {
    return format!("Hello {0}, You're running {1}", name, env::consts::OS).into();
}

#[tauri::command]
async fn play_sine(duration: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(duration))
        .amplify(0.20);
    sink.append(source);
    sink.sleep_until_end();
}

#[tauri::command]
fn counter(count_val: i32, counter: State<Counter>) -> i32 {
    let mut ct = counter.0.lock().unwrap();
    *ct += count_val;
    *ct
}

#[tauri::command]
fn return_counter_state(counter: State<Counter>) -> i32 {
    *counter.0.lock().unwrap()
}
