mod database;
mod export;

use database::{Database, CadreInfo};
use export::{ExportConfig, export_to_excel};
use tauri::State;
use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    println!("执行greet命令，参数: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_cadre_info(db: State<'_, Mutex<Database>>, cadre: CadreInfo) -> Result<(), String> {
    println!("开始执行add_cadre_info命令");
    println!("接收到的干部信息: {:?}", cadre);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.add_cadre(&cadre) {
                Ok(result) => {
                    println!("添加干部信息成功，影响行数: {}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("添加干部信息失败: {}", e);
                    Err(format!("添加干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            eprintln!("获取数据库锁失败: {}", e);
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn get_all_cadre_info(db: State<'_, Mutex<Database>>) -> Result<Vec<CadreInfo>, String> {
    println!("开始执行get_all_cadre_info命令");
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.get_all_cadres() {
                Ok(cadres) => {
                    println!("获取所有干部信息成功，共{}条记录", cadres.len());
                    Ok(cadres)
                },
                Err(e) => {
                    eprintln!("获取所有干部信息失败: {}", e);
                    Err(format!("获取所有干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            eprintln!("获取数据库锁失败: {}", e);
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn get_cadre_info_by_id(db: State<'_, Mutex<Database>>, id: i32) -> Result<Option<CadreInfo>, String> {
    println!("开始执行get_cadre_info_by_id命令，ID: {}", id);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.get_cadre_by_id(id) {
                Ok(cadre) => {
                    println!("根据ID获取干部信息完成");
                    Ok(cadre)
                },
                Err(e) => {
                    eprintln!("根据ID获取干部信息失败: {}", e);
                    Err(format!("根据ID获取干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            eprintln!("获取数据库锁失败: {}", e);
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn update_cadre_info(db: State<'_, Mutex<Database>>, cadre: CadreInfo) -> Result<(), String> {
    println!("开始执行update_cadre_info命令");
    println!("接收到的干部信息: {:?}", cadre);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.update_cadre(&cadre) {
                Ok(result) => {
                    println!("更新干部信息成功，影响行数: {}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("更新干部信息失败: {}", e);
                    Err(format!("更新干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            eprintln!("获取数据库锁失败: {}", e);
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn delete_cadre_info(db: State<'_, Mutex<Database>>, id: i32) -> Result<(), String> {
    println!("开始执行delete_cadre_info命令，ID: {}", id);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.delete_cadre(id) {
                Ok(result) => {
                    println!("删除干部信息成功，影响行数: {}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("删除干部信息失败: {}", e);
                    Err(format!("删除干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            eprintln!("获取数据库锁失败: {}", e);
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn export_cadre_info_to_excel(
    db: State<'_, Mutex<Database>>,
    file_path: String,
    selected_fields: Vec<String>,
    cadre_ids: Option<Vec<i32>> // 新增参数，用于部分导出
) -> Result<(), String> {
    println!("开始执行export_cadre_info_to_excel命令");
    println!("文件路径: {}, 选择字段数: {}, 挌定ID数: {:?}", 
             file_path, selected_fields.len(), cadre_ids.as_ref().map(|v| v.len()));
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            
            // 根据是否提供ID列表来决定导出全部还是部分数据
            let cadres = if let Some(ids) = cadre_ids {
                println!("导出指定ID的数据，共{}个ID", ids.len());
                // 导出指定ID的数据
                let mut selected_cadres = Vec::new();
                for id in ids {
                    if let Ok(Some(cadre)) = db_guard.get_cadre_by_id(id) {
                        selected_cadres.push(cadre);
                    }
                }
                selected_cadres
            } else {
                println!("导出全部数据");
                // 导出全部数据
                match db_guard.get_all_cadres() {
                    Ok(cadres) => cadres,
                    Err(e) => {
                        eprintln!("获取所有干部信息失败: {}", e);
                        return Err(format!("获取所有干部信息失败: {}", e));
                    }
                }
            };
            
            println!("准备导出{}条记录", cadres.len());
            
            let mut config = ExportConfig::new(file_path);
            
            // 设置字段
            config.set_fields(selected_fields.clone());
            
            // 添加数据
            for (index, cadre) in cadres.iter().enumerate() {
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
                if index % 100 == 0 {
                    println!("已处理{}条记录", index);
                }
            }
            
            println!("开始执行导出操作");
            match export_to_excel(config) {
                Ok(_) => {
                    println!("导出操作完成");
                    Ok(())
                },
                Err(e) => {
                    eprintln!("导出操作失败: {}", e);
                    Err(format!("导出操作失败: {}", e))
                }
            }
        },
        Err(e) => {
            eprintln!("获取数据库锁失败: {}", e);
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("初始化Tauri应用");
    
    // 初始化数据库
    println!("初始化数据库");
    let database = match Database::new() {
        Ok(db) => {
            println!("数据库初始化成功");
            db
        },
        Err(e) => {
            eprintln!("数据库初始化失败: {}", e);
            return Err(Box::new(e));
        }
    };
    
    println!("启动Tauri Builder");
    tauri::Builder::default()
        .manage(Mutex::new(database))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
        .map_err(|e| {
            eprintln!("Tauri应用运行失败: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })
}