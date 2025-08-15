mod database;
mod export;

use database::{Database, CadreInfo};
use export::{ExportConfig, export_to_excel};
use tauri::State;
use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_cadre_info(db: State<'_, Mutex<Database>>, cadre: CadreInfo) -> Result<(), String> {
    let db = db.lock().unwrap();
    db.add_cadre(&cadre).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_all_cadre_info(db: State<'_, Mutex<Database>>) -> Result<Vec<CadreInfo>, String> {
    let db = db.lock().unwrap();
    db.get_all_cadres().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_cadre_info_by_id(db: State<'_, Mutex<Database>>, id: i32) -> Result<Option<CadreInfo>, String> {
    let db = db.lock().unwrap();
    db.get_cadre_by_id(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_cadre_info(db: State<'_, Mutex<Database>>, cadre: CadreInfo) -> Result<(), String> {
    let db = db.lock().unwrap();
    db.update_cadre(&cadre).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn delete_cadre_info(db: State<'_, Mutex<Database>>, id: i32) -> Result<(), String> {
    let db = db.lock().unwrap();
    db.delete_cadre(id).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn export_cadre_info_to_excel(
    db: State<'_, Mutex<Database>>,
    file_path: String,
    selected_fields: Vec<String>,
    cadre_ids: Option<Vec<i32>> // 新增参数，用于部分导出
) -> Result<(), String> {
    let db = db.lock().unwrap();
    
    // 根据是否提供ID列表来决定导出全部还是部分数据
    let cadres = if let Some(ids) = cadre_ids {
        // 导出指定ID的数据
        let mut selected_cadres = Vec::new();
        for id in ids {
            if let Ok(Some(cadre)) = db.get_cadre_by_id(id) {
                selected_cadres.push(cadre);
            }
        }
        selected_cadres
    } else {
        // 导出全部数据
        db.get_all_cadres().map_err(|e| e.to_string())?
    };
    
    let mut config = ExportConfig::new(file_path);
    
    // 设置字段
    config.set_fields(selected_fields.clone());
    
    // 添加数据
    for cadre in cadres {
        let mut row_data = Vec::new();
        for field in &selected_fields {
            let value = match field.as_str() {
                "id" => cadre.id.unwrap_or(0).to_string(),
                "serial_number" => cadre.serial_number.clone().unwrap_or_default(),
                "name" => cadre.name.clone(),
                "gender" => cadre.gender.clone().unwrap_or_default(),
                "department" => cadre.department.clone().unwrap_or_default(),
                "section" => cadre.section.clone().unwrap_or_default(),
                "position1" => cadre.position1.clone().unwrap_or_default(),
                "position2" => cadre.position2.clone().unwrap_or_default(),
                "company_entry_date" => cadre.company_entry_date.clone().unwrap_or_default(),
                "current_level_date" => cadre.current_level_date.clone().unwrap_or_default(),
                "position_entry_date" => cadre.position_entry_date.clone().unwrap_or_default(),
                "probation_period" => cadre.probation_period.clone().unwrap_or_default(),
                "probation_end_reminder" => cadre.probation_end_reminder.clone().unwrap_or_default(),
                "id_number" => cadre.id_number.clone().unwrap_or_default(),
                "technical_position" => cadre.technical_position.clone().unwrap_or_default(),
                "education" => cadre.education.clone().unwrap_or_default(),
                "full_time_education" => cadre.full_time_education.clone().unwrap_or_default(),
                "full_time_school_major" => cadre.full_time_school_major.clone().unwrap_or_default(),
                "part_time_education" => cadre.part_time_education.clone().unwrap_or_default(),
                "part_time_school_phone" => cadre.part_time_school_phone.clone().unwrap_or_default(),
                "phone" => cadre.phone.clone().unwrap_or_default(),
                "remarks" => cadre.remarks.clone().unwrap_or_default(),
                "major" => cadre.major.clone().unwrap_or_default(),
                "political_status" => cadre.political_status.clone().unwrap_or_default(),
                "party_entry_date" => cadre.party_entry_date.clone().unwrap_or_default(),
                "contact_date" => cadre.contact_date.clone().unwrap_or_default(),
                "age" => cadre.age.unwrap_or(0).to_string(),
                "native_place" => cadre.native_place.clone().unwrap_or_default(),
                "birth_place" => cadre.birth_place.clone().unwrap_or_default(),
                "birth_date" => cadre.birth_date.clone().unwrap_or_default(),
                "ethnicity" => cadre.ethnicity.clone().unwrap_or_default(),
                "special_date" => cadre.special_date.clone().unwrap_or_default(),
                "company_tenure" => cadre.company_tenure.unwrap_or(0.0).to_string(),
                "work_start_date" => cadre.work_start_date.clone().unwrap_or_default(),
                "work_tenure" => cadre.work_tenure.unwrap_or(0.0).to_string(),
                _ => String::new(),
            };
            row_data.push(value);
        }
        config.add_data_row(row_data);
    }
    
    export_to_excel(config).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Database::new().expect("Failed to initialize database")))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            add_cadre_info,
            get_all_cadre_info,
            get_cadre_info_by_id,
            update_cadre_info,
            delete_cadre_info,
            export_cadre_info_to_excel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}