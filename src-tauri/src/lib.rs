mod database;
mod export;

use database::{Database, CadreInfo};
use export::{ExportConfig, export_to_excel};
use tauri::State;
use std::sync::Mutex;
use calamine::{open_workbook_auto, Reader, Sheets, Data};

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

#[tauri::command]
fn get_distinct_field_values(db: State<'_, Mutex<Database>>, field_name: String) -> Result<Vec<String>, String> {
    println!("开始执行get_distinct_field_values命令，字段名: {}", field_name);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.get_distinct_field_values(&field_name) {
                Ok(values) => {
                    println!("获取字段{}的distinct值成功，共{}个值", field_name, values.len());
                    Ok(values)
                },
                Err(e) => {
                    eprintln!("获取字段{}的distinct值失败: {}", field_name, e);
                    Err(format!("获取字段{}的distinct值失败: {}", field_name, e))
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
fn import_cadre_info_from_excel(db: State<'_, Mutex<Database>>, file_path: String) -> Result<String, String> {
    println!("开始从Excel文件导入干部信息: {}", file_path);
    
    // 读取Excel文件
    let mut workbook: Sheets<_> = open_workbook_auto(&file_path)
        .map_err(|e| format!("无法打开Excel文件: {}", e))?;
    
    // 获取第一个工作表
    let sheet_names = workbook.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err("Excel文件中没有工作表".to_string());
    }
    
    let first_sheet_name = sheet_names[0].clone();
    println!("读取工作表: {}", first_sheet_name);
    
    // 获取工作表数据
    let worksheet = workbook
        .worksheet_range(&first_sheet_name)
        .map_err(|e| format!("无法读取工作表: {}", e))?;
    
    // 打印工作表的行数和列数
    println!("工作表尺寸: {}行 x {}列", worksheet.height(), worksheet.width());
    
    // 获取数据库连接
    let db_guard = db.lock()
        .map_err(|e| format!("无法获取数据库锁: {}", e))?;
    
    let mut imported_count = 0;
    let mut error_details = Vec::new();
    
    // 解析每一行数据（跳过第一行表头）
    for (row_index, row) in worksheet.rows().enumerate() {
        println!("读取第{}行数据，行数据: {:?}", row_index + 1, row);
        
        // 跳过第一行表头
        if row_index < 1 {
            println!("跳过第{}行（表头行）", row_index + 1);
            continue;
        }
        
        // 检查是否是空行
        let is_empty_row = row.iter().all(|cell| matches!(cell, Data::Empty));
        println!("第{}行是否为空行: {}", row_index + 1, is_empty_row);
        
        if is_empty_row {
            println!("跳过第{}行（空行）", row_index + 1);
            continue;
        }
        
        println!("处理第{}行数据", row_index + 1);
        
        // 解析每列数据
        match parse_cadre_info_from_row(row, row_index + 1) {
            Ok(cadre_info) => {
                println!("第{}行数据解析成功: {:?}", row_index + 1, cadre_info.name);
                // 插入数据库
                match db_guard.add_cadre(&cadre_info) {
                    Ok(_) => {
                        imported_count += 1;
                        println!("成功导入第{}行数据", row_index + 1);
                    },
                    Err(e) => {
                        let error_msg = format!("第{}行数据插入数据库失败: {}", row_index + 1, e);
                        error_details.push(error_msg);
                        eprintln!("导入第{}行数据失败: {}", row_index + 1, e);
                    }
                }
            },
            Err(e) => {
                let error_msg = format!("第{}行数据解析失败: {}", row_index + 1, e);
                error_details.push(error_msg);
                eprintln!("解析第{}行数据失败: {}", row_index + 1, e);
            }
        }
    }
    
    let error_count = error_details.len();
    let mut result_message = format!("导入完成！成功导入{}条记录，{}条记录失败", imported_count, error_count);
    
    // 如果有错误详情，添加到结果消息中
    if !error_details.is_empty() {
        result_message.push_str("\n\n错误详情:");
        for error in error_details {
            result_message.push_str(&format!("\n- {}", error));
        }
    }
    
    println!("{}", result_message);
    
    Ok(result_message)
}

// 从Excel行数据解析干部信息
fn parse_cadre_info_from_row(row: &[Data], row_index: usize) -> Result<CadreInfo, String> {
    println!("解析第{}行数据", row_index);
    
    // 辅助函数：安全获取单元格值
    let get_cell_value = |index: usize| -> String {
        if index < row.len() {
            match &row[index] {
                Data::String(s) => s.clone(),
                Data::Float(f) => f.to_string(),
                Data::Int(i) => i.to_string(),
                Data::DateTime(d) => d.to_string(),
                Data::Bool(b) => b.to_string(),
                Data::Empty => String::new(),
                _ => String::new(),
            }
        } else {
            String::new()
        }
    };
    
    // 解析各字段
    let name = get_cell_value(0); // 姓名
    if name.trim().is_empty() {
        return Err(format!("第{}行姓名不能为空", row_index));
    }
    
    let cadre_info = CadreInfo {
        id: None,
        serial_number: None, // 序号字段已移除
        name: name,
        gender: Some(get_cell_value(1)), // 性别
        department: Some(get_cell_value(2)), // 部门
        section: Some(get_cell_value(3)), // 科室
        position1: Some(get_cell_value(4)), // 职务1
        position2: Some(get_cell_value(5)), // 职务2
        company_entry_date: Some(get_cell_value(6)), // 入司日期
        current_level_date: Some(get_cell_value(7)), // 任现职级时间
        position_entry_date: Some(get_cell_value(8)), // 任职时间
        probation_period: Some(get_cell_value(9)), // 试用期(年)
        probation_end_reminder: Some(get_cell_value(10)), // 试用期满到期提醒
        id_number: Some(get_cell_value(11)), // 身份证号
        technical_position: Some(get_cell_value(12)), // 专业技术职务
        education: Some(get_cell_value(13)), // 最高学历
        full_time_education: Some(get_cell_value(14)), // 全日制学历
        full_time_school_major: Some(get_cell_value(15)), // 全日制毕业院校系及专业
        part_time_education: Some(get_cell_value(16)), // 在职学历
        part_time_school_phone: Some(get_cell_value(17)), // 在职毕业院校系及专业
        political_status: Some(get_cell_value(18)), // 政治面貌
        party_entry_date: Some(get_cell_value(19)), // 入党时间
        phone: Some(get_cell_value(20)), // 联系电话
        birth_date: Some(get_cell_value(21)), // 出生日期
        age: {
            let age_str = get_cell_value(22); // 年龄
            if !age_str.is_empty() {
                age_str.parse::<i32>().ok()
            } else {
                None
            }
        },
        native_place: Some(get_cell_value(23)), // 籍贯
        birth_place: Some(get_cell_value(24)), // 出生地
        ethnicity: Some(get_cell_value(25)), // 民族
        company_tenure: {
            let tenure_str = get_cell_value(26); // 司龄(年)
            if !tenure_str.is_empty() {
                tenure_str.parse::<f32>().ok()
            } else {
                None
            }
        },
        work_start_date: Some(get_cell_value(27)), // 参加工作时间
        work_tenure: {
            let tenure_str = get_cell_value(28); // 工龄(年)
            if !tenure_str.is_empty() {
                tenure_str.parse::<f32>().ok()
            } else {
                None
            }
        },
        remarks: Some(get_cell_value(29)), // 备注
        major: None, // 专业字段（未在Excel中使用）
        contact_date: None, // 联系日期字段（未在Excel中使用）
        special_date: None, // 特殊日期字段（未在Excel中使用）
    };
    
    println!("第{}行数据解析完成", row_index);
    Ok(cadre_info)
}

#[tauri::command]
fn generate_import_template() -> Result<Vec<u8>, String> {
    println!("开始生成Excel导入模板");
    
    // 创建内存中的工作簿
    let mut workbook = simple_excel_writer::Workbook::create_in_memory();
    
    // 创建工作表
    let mut sheet = workbook.create_sheet("干部信息导入模板");
    
    // 写入表头和示例数据
    workbook.write_sheet(&mut sheet, |sheet_writer| {
        use simple_excel_writer::*;
        
        // 写入表头
        sheet_writer.append_row(row!["姓名", "性别", "部门", "科室", "职务1", "职务2", 
                      "入司日期", "任现职级时间", "任职时间", "试用期(年)", "试用期满到期提醒",
                      "身份证号", "专业技术职务", "最高学历", "全日制学历", "全日制毕业院校系及专业",
                      "在职学历", "在职毕业院校系及专业", "政治面貌", "入党时间", "联系电话",
                      "出生日期", "年龄", "籍贯", "出生地", "民族", "司龄(年)", "参加工作时间", "工龄(年)", "备注"])?;
        
        // 写入数据规范说明行
        sheet_writer.append_row(row!["", "男/女", "部门名称", "科室名称", "职务名称", "职务名称",
                      "YYYY-MM-DD", "YYYY-MM-DD", "YYYY-MM-DD", "数字", "YYYY-MM-DD",
                      "18位身份证号", "职务名称", "学历名称", "学历名称", "学校专业",
                      "学历名称", "学校专业", "政治面貌选项", "YYYY-MM-DD", "手机号码",
                      "YYYY-MM-DD", "数字", "地区", "地区", "民族名称", "数字", "YYYY-MM-DD", "数字", "备注信息"])?;
        
        // 写入示例行
        sheet_writer.append_row(row!["张三", "男", "人力资源部", "招聘科", "部长", "副主任",
                      "2020-01-15", "2021-03-20", "2020-01-15", "1", "2021-01-15",
                      "110101199001011234", "高级工程师", "硕士研究生", "硕士研究生", "清华大学计算机系",
                      "硕士研究生", "清华大学计算机系", "中共党员", "2015-06-15", "13800138000",
                      "1990-01-01", "34", "北京", "北京", "汉族", "4.5", "2010-07-01", "13.5", "优秀员工"])?;
        
        // 写入政治面貌选项说明
        sheet_writer.append_row(row!["", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
                      "", "", "可选值: 中共党员/预备党员/共青团员/民革党员/民盟盟员/民建会员/民进会员/农工党党员/致公党党员/九三学社社员/台盟盟员/无党派人士/群众", "", "", "", "", "", "", "", "", ""])?;
        
        // 写入学历选项说明
        sheet_writer.append_row(row!["", "", "", "", "", "", "", "", "", "", "", "", "", "", 
                      "可选值: 博士研究生/硕士研究生/大学/大专/高中/中专/初中/职高", 
                      "可选值: 博士研究生/硕士研究生/大学/大专/高中/中专/初中/职高", "", 
                      "可选值: 博士研究生/硕士研究生/大学/大专/高中/中专/初中/职高", "", "", "", "", "", "", "", "", ""])?;
        
        Ok(())
    }).map_err(|e| format!("生成Excel模板失败: {}", e))?;
    
    // 将工作簿保存到内存中的字节向量
    let result = workbook.close();
    
    match result {
        Ok(Some(buffer)) => {
            println!("Excel导入模板生成成功，大小: {} 字节", buffer.len());
            Ok(buffer)
        },
        Ok(None) => {
            println!("Excel导入模板生成成功，但没有数据");
            Ok(Vec::new())
        },
        Err(e) => {
            eprintln!("保存Excel模板失败: {}", e);
            Err(format!("保存Excel模板失败: {}", e))
        }
    }
}

#[tauri::command]
fn save_import_template(file_path: String) -> Result<(), String> {
    println!("开始生成并保存Excel导入模板到: {}", file_path);
    
    // 生成模板数据
    let template_data = generate_import_template()?;
    
    // 将模板数据保存到指定文件路径
    std::fs::write(&file_path, template_data)
        .map_err(|e| format!("保存Excel模板到文件失败: {}", e))?;
    
    println!("Excel导入模板已保存到: {}", file_path);
    Ok(())
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
            export_cadre_info_to_excel,
            get_distinct_field_values,
            generate_import_template,
            import_cadre_info_from_excel,
            save_import_template
        ])
        .run(tauri::generate_context!())
        .map_err(|e| {
            eprintln!("Tauri应用运行失败: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })
}