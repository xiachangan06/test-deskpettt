use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct LedgerEntry {
    id: u64,
    date: String,
    amount: f64,
    category: String,
    note: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct PushImage {
    id: String,
    url: String,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct WeatherData {
    city: String,
    temperature: f64,
    condition: String,
    humidity: u32,
    icon: String,
}

struct AppState {
    ledger: Mutex<Vec<LedgerEntry>>,
    current_image: Mutex<Option<PushImage>>,
}

#[tauri::command]
fn add_ledger_entry(state: tauri::State<AppState>, entry: LedgerEntry) -> Result<Vec<LedgerEntry>, String> {
    let mut ledger = state.ledger.lock().unwrap();
    ledger.push(entry.clone());
    
    let path = get_ledger_path();
    let _ = fs::write(path, serde_json::to_string(&*ledger).unwrap());
    
    Ok(ledger.clone())
}

#[tauri::command]
fn get_ledger(state: tauri::State<AppState>) -> Result<Vec<LedgerEntry>, String> {
    let ledger = state.ledger.lock().unwrap();
    Ok(ledger.clone())
}

#[tauri::command]
fn clear_ledger(state: tauri::State<AppState>) -> Result<(), String> {
    let mut ledger = state.ledger.lock().unwrap();
    ledger.clear();
    let path = get_ledger_path();
    let _ = fs::write(path, "[]");
    Ok(())
}

#[tauri::command]
async fn fetch_weather(city: String) -> Result<WeatherData, String> {
    Ok(WeatherData {
        city: city,
        temperature: 22.5,
        condition: "晴".to_string(),
        humidity: 65,
        icon: "☀️".to_string(),
    })
}

#[tauri::command]
fn receive_image(state: tauri::State<AppState>, image_data: PushImage) {
    let mut current_image = state.current_image.lock().unwrap();
    *current_image = Some(image_data);
}

#[tauri::command]
fn get_current_image(state: tauri::State<AppState>) -> Result<Option<PushImage>, String> {
    let current_image = state.current_image.lock().unwrap();
    Ok(current_image.clone())
}

fn get_ledger_path() -> PathBuf {
    let data_dir = dirs::document_dir().unwrap_or(PathBuf::from("."));
    data_dir.join("cat_ledger.json")
}

fn load_initial_data() -> Vec<LedgerEntry> {
    let path = get_ledger_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(entries) = serde_json::from_str(&content) {
                return entries;
            }
        }
    }
    Vec::new()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let initial_ledger = load_initial_data();
    
    let state = AppState {
        ledger: Mutex::new(initial_ledger),
        current_image: Mutex::new(None),
    };
    
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            add_ledger_entry,
            get_ledger,
            clear_ledger,
            fetch_weather,
            receive_image,
            get_current_image,
        ])
        .run(tauri::generate_context!())
        .expect("启动失败");
}