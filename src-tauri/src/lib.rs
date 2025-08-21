mod database;
mod export;
mod filter;

use database::{Database, GrassrootsCadreInfo, MidLevelCadreInfo};
use filter::FilterParams;
use export::{ExportConfig, export_to_excel};
use tauri::State;
use std::sync::Mutex;
use calamine::{open_workbook_auto, Reader, Sheets, Data, ExcelDateTime};
// 导入处理chrono日期所需的类型
use chrono::{NaiveDate, Duration, Local};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn export_grassroots_cadre_info_to_excel(
    db: State<'_, Mutex<Database>>,
    file_path: String,
    selected_fields: Vec<String>,
    cadre_ids: Option<Vec<i32>> // 新增参数，用于部分导出
) -> Result<(), String> {
    match db.lock() {
        Ok(db_guard) => {
            // 根据是否提供ID列表来决定导出全部还是部分数据
            let cadres = if let Some(ids) = cadre_ids {
                // 导出指定ID的数据
                let mut selected_cadres = Vec::new();
                for id in ids {
                    if let Ok(Some(cadre)) = db_guard.get_grassroots_cadre_by_id(id) {
                        selected_cadres.push(cadre);
                    }
                }
                selected_cadres
            } else {
                // 导出全部数据
                match db_guard.get_all_grassroots_cadres() {
                    Ok(cadres) => cadres,
                    Err(e) => {
                        return Err(format!("获取所有基层干部信息失败: {}", e));
                    }
                }
            };
            
            let mut config = ExportConfig::new(file_path);
            
            // 设置字段
            config.set_fields(selected_fields.clone());
            
            // 添加数据
            for (_index, cadre) in cadres.iter().enumerate() {
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
                        "grassroots_vice_position_date" => cadre.grassroots_vice_position_date.clone().unwrap_or_default(),
                        "grassroots_vice_tenure" => cadre.grassroots_vice_tenure.clone().unwrap_or_default(),
                        "grassroots_chief_position_date" => cadre.grassroots_chief_position_date.clone().unwrap_or_default(),
                        "grassroots_chief_tenure" => cadre.grassroots_chief_tenure.unwrap_or(0.0).to_string(),
                        "midlevel_assistant_date" => cadre.midlevel_assistant_date.clone().unwrap_or_default(),
                        "midlevel_assistant_tenure" => cadre.midlevel_assistant_tenure.unwrap_or(0.0).to_string(),
                        "midlevel_vice_date" => cadre.midlevel_vice_date.clone().unwrap_or_default(),
                        "midlevel_vice_tenure" => cadre.midlevel_vice_tenure.unwrap_or(0.0).to_string(),
                        "midlevel_chief_date" => cadre.midlevel_chief_date.clone().unwrap_or_default(),
                        "midlevel_chief_tenure" => cadre.midlevel_chief_tenure.unwrap_or(0.0).to_string(),
                        "same_department_date" => cadre.same_department_date.clone().unwrap_or_default(),
                        "same_department_tenure" => cadre.same_department_tenure.unwrap_or(0.0).to_string(),
                        _ => String::new(),
                    };
                    row_data.push(value);
                }
                config.add_data_row(row_data);
            }
            
            match export_to_excel(config) {
                Ok(_) => {
                    Ok(())
                },
                Err(e) => {
                    Err(format!("导出操作失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}






#[tauri::command]
fn export_midlevel_cadre_info_to_excel(
    db: State<'_, Mutex<Database>>,
    file_path: String,
    selected_fields: Vec<String>,
    cadre_ids: Option<Vec<i32>> // 新增参数，用于部分导出
) -> Result<(), String> {
    match db.lock() {
        Ok(db_guard) => {
            // 根据是否提供ID列表来决定导出全部还是部分数据
            let cadres = if let Some(ids) = cadre_ids {
                // 导出指定ID的数据
                let mut selected_cadres = Vec::new();
                for id in ids {
                    if let Ok(Some(cadre)) = db_guard.get_midlevel_cadre_by_id(id) {
                        selected_cadres.push(cadre);
                    }
                }
                selected_cadres
            } else {
                // 导出全部数据
                match db_guard.get_all_midlevel_cadres() {
                    Ok(cadres) => cadres,
                    Err(e) => {
                        return Err(format!("获取所有中层干部信息失败: {}", e));
                    }
                }
            };
            
            let mut config = ExportConfig::new(file_path);
            
            // 设置字段
            config.set_fields(selected_fields.clone());
            
            // 添加数据
            for (_index, cadre) in cadres.iter().enumerate() {
                let mut row_data = Vec::new();
                for field in &selected_fields {
                    let value = match field.as_str() {
                        "id" => cadre.id.unwrap_or(0).to_string(),
                        "serial_number" => cadre.serial_number.clone().unwrap_or_default(),
                        "name" => cadre.name.clone(),
                        "gender" => cadre.gender.clone().unwrap_or_default(),
                        "department" => cadre.department.clone().unwrap_or_default(),
                        "section" => "".to_string(), // 中层管理人员没有科室字段
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
                        "grassroots_vice_position_date" => cadre.grassroots_vice_position_date.clone().unwrap_or_default(),
                        "grassroots_vice_tenure" => cadre.grassroots_vice_tenure.clone().unwrap_or_default(),
                        "grassroots_chief_position_date" => cadre.grassroots_chief_position_date.clone().unwrap_or_default(),
                        "grassroots_chief_tenure" => cadre.grassroots_chief_tenure.unwrap_or(0.0).to_string(),
                        "midlevel_assistant_date" => cadre.midlevel_assistant_date.clone().unwrap_or_default(),
                        "midlevel_assistant_tenure" => cadre.midlevel_assistant_tenure.unwrap_or(0.0).to_string(),
                        "midlevel_vice_date" => cadre.midlevel_vice_date.clone().unwrap_or_default(),
                        "midlevel_vice_tenure" => cadre.midlevel_vice_tenure.unwrap_or(0.0).to_string(),
                        "midlevel_chief_date" => cadre.midlevel_chief_date.clone().unwrap_or_default(),
                        "midlevel_chief_tenure" => cadre.midlevel_chief_tenure.unwrap_or(0.0).to_string(),
                        "same_department_date" => cadre.same_department_date.clone().unwrap_or_default(),
                        "same_department_tenure" => cadre.same_department_tenure.unwrap_or(0.0).to_string(),
                        _ => String::new(),
                    };
                    row_data.push(value);
                }
                config.add_data_row(row_data);
            }
            
            match export_to_excel(config) {
                Ok(_) => {
                    Ok(())
                },
                Err(e) => {
                    Err(format!("导出操作失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}


#[tauri::command]
fn import_cadre_info_from_excel(db: State<'_, Mutex<Database>>, file_path: String, is_midlevel: bool) -> Result<String, String> {
    eprintln!("DEBUG: 开始导入Excel文件: {}", file_path);
    eprintln!("DEBUG: 是否为中层管理人员: {}", is_midlevel);
    
    // 读取Excel文件
    let mut workbook: Sheets<_> = open_workbook_auto(&file_path)
        .map_err(|e| format!("无法打开Excel文件: {}", e))?;
    
    // 获取第一个工作表
    let sheet_names = workbook.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err("Excel文件中没有工作表".to_string());
    }
    
    let first_sheet_name = sheet_names[0].clone();
    eprintln!("DEBUG: 工作表名称: {}", first_sheet_name);
    
    // 获取工作表数据
    let worksheet = workbook
        .worksheet_range(&first_sheet_name)
        .map_err(|e| format!("无法读取工作表: {}", e))?;
    
    eprintln!("DEBUG: 工作表总行数: {}", worksheet.rows().count());
    
    // 获取数据库连接
    let db_guard = db.lock()
        .map_err(|e| format!("无法获取数据库锁: {}", e))?;
    
    let mut imported_count = 0;
    let mut error_details = Vec::new();
    
    // 解析每一行数据（跳过第一行表头）
    for (row_index, row) in worksheet.rows().enumerate() {
        eprintln!("DEBUG: 处理第{}行，该行有{}列", row_index + 1, row.len());
        
        // 跳过第一行表头
        if row_index < 1 {
            continue;
        }
        
        // 检查是否是空行
        let is_empty_row = row.iter().all(|cell| matches!(cell, Data::Empty));
        
        if is_empty_row {
            eprintln!("DEBUG: 第{}行是空行，跳过", row_index + 1);
            continue;
        }
        
        // 解析每列数据
        if is_midlevel {
            // 导入中层管理人员数据
            match parse_midlevel_cadre_info_from_row(row, row_index + 1) {
                Ok(cadre_info) => {
                    eprintln!("DEBUG: 第{}行数据解析成功", row_index + 1);
                    // 插入数据库
                    match db_guard.add_midlevel_cadre(&cadre_info) {
                        Ok(_) => {
                            imported_count += 1;
                            eprintln!("DEBUG: 第{}行数据插入数据库成功", row_index + 1);
                        },
                        Err(e) => {
                            let error_msg = format!("第{}行数据插入数据库失败: {}", row_index + 1, e);
                            eprintln!("DEBUG: {}", error_msg);
                            error_details.push(error_msg);
                        }
                    }
                },
                Err(e) => {
                    let error_msg = format!("第{}行数据解析失败: {}", row_index + 1, e);
                    eprintln!("DEBUG: {}", error_msg);
                    error_details.push(error_msg);
                }
            }
        } else {
            // 导入基层管理人员数据
            match parse_cadre_info_from_row(row, row_index + 1) {
                Ok(cadre_info) => {
                    eprintln!("DEBUG: 第{}行数据解析成功", row_index + 1);
                    // 插入数据库
                    match db_guard.add_grassroots_cadre(&cadre_info) {
                        Ok(_) => {
                            imported_count += 1;
                            eprintln!("DEBUG: 第{}行数据插入数据库成功", row_index + 1);
                        },
                        Err(e) => {
                            let error_msg = format!("第{}行数据插入数据库失败: {}", row_index + 1, e);
                            eprintln!("DEBUG: {}", error_msg);
                            error_details.push(error_msg);
                        }
                    }
                },
                Err(e) => {
                    let error_msg = format!("第{}行数据解析失败: {}", row_index + 1, e);
                    eprintln!("DEBUG: {}", error_msg);
                    error_details.push(error_msg);
                }
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
    
    eprintln!("DEBUG: 导入完成，结果: {}", result_message);
    Ok(result_message)
}



// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// ... [greet function] ...

// 辅助函数：标准化日期格式，支持 yyyy-mm-dd 和 yyyy/mm/dd
// 同时处理 Excel 序列日期格式
fn normalize_date_format(date_str: &str) -> String {
    if date_str.is_empty() {
        return String::new();
    }
    
    // 如果是 yyyy/mm/dd 格式，转换为 yyyy-mm-dd
    if date_str.contains('/') {
        date_str.replace("/", "-")
    } else {
        // 如果已经是 yyyy-mm-dd 格式或其他格式，保持不变
        date_str.to_string()
    }
}

// 辅助函数：将 Excel 序列日期数转换为 yyyy-mm-dd 格式的字符串
fn convert_excel_date_to_string(excel_date: &ExcelDateTime) -> String {
    // 获取 Excel 序列日期数
    let serial_date = excel_date.as_f64();
    
    // Excel 的日期系统有两种：
    // 1. Windows 系统：以 1900 年 1 月 1 日为第 1 天 (但有一个错误，认为 1900 年是闰年)
    // 2. Mac 系统：以 1904 年 1 月 1 日为第 1 天
    // 这里我们假设使用 Windows 系统的日期系统 (1900-based)
    // 1900 年 1 月 1 日对应的序列日期数是 1
    // 但 chrono::NaiveDate 从 1970 年开始计算，所以我们需要先计算出距离 1970 年的天数
    
    // 1900 年 1 月 1 日距离 1970 年 1 月 1 日的天数 (负数)
    // 1970-01-01 是 Excel 日期序列 25569 (对于 1900-based 系统)
    let excel_epoch_days = 25569;
    
    // 将 Excel 序列日期数转换为距离 1970-01-01 的天数
    let days_since_epoch = serial_date - excel_epoch_days as f64;
    
    // 将天数转换为 chrono::Duration
    // Duration::days 需要一个 i64 类型的参数
    let days = days_since_epoch.round() as i64;
    
    // 1970 年 1 月 1 日
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    
    // 计算目标日期
    if let Some(target_date) = epoch.checked_add_signed(Duration::days(days)) {
        target_date.format("%Y-%m-%d").to_string()
    } else {
        // 如果日期计算失败，返回原始字符串
        excel_date.to_string()
    }
}


// 从Excel行数据解析基层干部信息
fn parse_cadre_info_from_row(row: &[Data], row_index: usize) -> Result<GrassrootsCadreInfo, String> {
    // 调试信息：打印行的总列数
    eprintln!("DEBUG: 第{}行总列数: {}", row_index, row.len());
    
    // 辅助函数：安全获取单元格值
    // 修改此函数以正确处理 Excel 的 DateTime 类型
    let get_cell_value = |index: usize| -> String {
        if index < row.len() {
            match &row[index] {
                Data::String(s) => s.clone(),
                Data::Float(f) => f.to_string(),
                Data::Int(i) => i.to_string(),
                // 处理 Excel 的 DateTime 类型
                Data::DateTime(d) => {
                    convert_excel_date_to_string(d)
                },
                Data::Bool(b) => b.to_string(),
                Data::Empty => String::new(),
                _ => String::new(),
            }
        } else {
            eprintln!("DEBUG: 尝试访问第{}列，但行只有{}列", index, row.len());
            String::new()
        }
    };
    
    // 解析各字段 (按照指定的字段顺序，保留自动计算字段在模板中但不读取其数据)
    let name = get_cell_value(0); // 姓名
    if name.trim().is_empty() {
        return Err(format!("第{}行姓名不能为空", row_index));
    }
    
    let gender = get_cell_value(1); // 性别
    let department = get_cell_value(2); // 部门
    let section = get_cell_value(3); // 科室
    let position1 = get_cell_value(4); // 职务1
    let position2 = get_cell_value(5); // 职务2
    let company_entry_date_raw = get_cell_value(6); // 入司日期
    let _company_tenure = get_cell_value(7); // 司龄（年） - 自动计算字段，导入时不使用
    let work_start_date_raw = get_cell_value(8); // 参加工作时间
    let _work_tenure = get_cell_value(9); // 工龄(年) - 自动计算字段，导入时不使用
    let current_level_date_raw = get_cell_value(10); // 任现职级时间
    let position_entry_date_raw = get_cell_value(11); // 任职时间
    let probation_period = get_cell_value(12); // 试用期
    let _probation_end_reminder = get_cell_value(13); // 试用期满到期提醒 - 自动计算字段，导入时不使用
    let id_number = get_cell_value(14); // 身份证号
    let birth_date_raw = get_cell_value(15); // 出生日期
    let _age = get_cell_value(16); // 年龄 - 自动计算字段，导入时不使用
    let native_place = get_cell_value(17); // 籍贯
    let birth_place = get_cell_value(18); // 出生地
    let ethnicity = get_cell_value(19); // 民族
    let technical_position = get_cell_value(20); // 专业技术职务
    let education = get_cell_value(21); // 学历
    let full_time_education = get_cell_value(22); // 全日制学历
    let full_time_school_major = get_cell_value(23); // 毕业院校系及专业1
    let part_time_education = get_cell_value(24); // 在职学历
    let part_time_school_phone = get_cell_value(25); // 毕业院校系及专业2
    let political_status = get_cell_value(26); // 政治面貌
    let party_entry_date_raw = get_cell_value(27); // 入党时间
    let phone = get_cell_value(28); // 联系电话
    let grassroots_vice_position_date_raw = get_cell_value(29); // 任基层副职时间
    let _grassroots_vice_tenure_str = get_cell_value(30); // 任基层副职年限
    let grassroots_chief_position_date_raw = get_cell_value(31); // 任基层正职时间
    let grassroots_chief_tenure_str = get_cell_value(32); // 任基层正职年限
    let midlevel_assistant_date_raw = get_cell_value(33); // 任中层助理时间
    let midlevel_assistant_tenure_str = get_cell_value(34); // 任中层助理年限
    let midlevel_vice_date_raw = get_cell_value(35); // 任中层副职时间
    let midlevel_vice_tenure_str = get_cell_value(36); // 任中层副职年限
    let midlevel_chief_date_raw = get_cell_value(37); // 任中层正职时间
    let midlevel_chief_tenure_str = get_cell_value(38); // 任中层正职年限
    let same_department_date_raw = get_cell_value(39); // 同部门任职时间
    let same_department_tenure_str = get_cell_value(40); // 同部门任职年限
    let remarks = get_cell_value(41); // 备注
    
    // 对日期字段应用标准化函数
    let company_entry_date = normalize_date_format(&company_entry_date_raw); // 入司日期
    let work_start_date = normalize_date_format(&work_start_date_raw); // 参加工作时间
    let current_level_date = normalize_date_format(&current_level_date_raw); // 任现职级时间
    let position_entry_date = normalize_date_format(&position_entry_date_raw); // 任职时间
    let party_entry_date = normalize_date_format(&party_entry_date_raw); // 入党时间
    let _birth_date = normalize_date_format(&birth_date_raw); // 出生日期
    let same_department_date = normalize_date_format(&same_department_date_raw); // 同部门任职时间
    
    // 标准化新增的日期字段
    let grassroots_vice_position_date = normalize_date_format(&grassroots_vice_position_date_raw);
    let grassroots_chief_position_date = normalize_date_format(&grassroots_chief_position_date_raw);
    let midlevel_assistant_date = normalize_date_format(&midlevel_assistant_date_raw);
    let midlevel_vice_date = normalize_date_format(&midlevel_vice_date_raw);
    let midlevel_chief_date = normalize_date_format(&midlevel_chief_date_raw);
    
    // 自动计算任基层副职年限
    let grassroots_vice_tenure = {
        let mut tenure = String::new();
        
        // 如果有任基层正职时间，则用正职时间减去副职时间
        if !grassroots_chief_position_date_raw.is_empty() && !grassroots_vice_position_date_raw.is_empty() {
            if let Ok(vice_date) = NaiveDate::parse_from_str(&normalize_date_format(&grassroots_vice_position_date_raw), "%Y-%m-%d") {
                if let Ok(chief_date) = NaiveDate::parse_from_str(&normalize_date_format(&grassroots_chief_position_date_raw), "%Y-%m-%d") {
                    if chief_date >= vice_date {
                        let diff_days = (chief_date - vice_date).num_days();
                        let diff_years = diff_days / 365;
                        let diff_months = (diff_days % 365) / 30;
                        tenure = format!("{}年{}月", diff_years, diff_months);
                    }
                }
            }
        } 
        // 如果没有基层正职时间，则按照当前时间减去副职时间
        else if !grassroots_vice_position_date_raw.is_empty() {
            if let Ok(vice_date) = NaiveDate::parse_from_str(&normalize_date_format(&grassroots_vice_position_date_raw), "%Y-%m-%d") {
                let today = Local::now().naive_local().date();
                let diff_days = (today - vice_date).num_days();
                let diff_years = diff_days / 365;
                let diff_months = (diff_days % 365) / 30;
                tenure = format!("{}年{}月", diff_years, diff_months);
            }
        }
        
        if !tenure.is_empty() {
            Some(tenure)
        } else {
            None
        }
    };
    
    let grassroots_chief_tenure = if !grassroots_chief_tenure_str.is_empty() {
        grassroots_chief_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    let midlevel_assistant_tenure = if !midlevel_assistant_tenure_str.is_empty() {
        midlevel_assistant_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    let midlevel_vice_tenure = if !midlevel_vice_tenure_str.is_empty() {
        midlevel_vice_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    let midlevel_chief_tenure = if !midlevel_chief_tenure_str.is_empty() {
        midlevel_chief_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    let same_department_tenure = if !same_department_tenure_str.is_empty() {
        same_department_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    // 自动计算字段
    // 1. 根据身份证号计算出生日期和年龄
    let (calculated_birth_date, calculated_age) = if !id_number.is_empty() && id_number.len() == 18 {
        // 提取出生日期 (第7-14位)
        let birth_year = &id_number[6..10];
        let birth_month = &id_number[10..12];
        let birth_day = &id_number[12..14];
        
        // 创建日期字符串
        let birth_date_str = format!("{}-{}-{}", birth_year, birth_month, birth_day);
        
        // 计算年龄
        if let Ok(birth_date) = NaiveDate::parse_from_str(&birth_date_str, "%Y-%m-%d") {
            let today = Local::now().naive_local().date();
            let age = (today - birth_date).num_days() / 365;
            (Some(birth_date_str), Some(age as i32))
        } else {
            (Some(birth_date_str), None)
        }
    } else {
        (None, None)
    };
    
    // 2. 根据入司日期计算司龄
    let calculated_company_tenure = if !company_entry_date.is_empty() {
        if let Ok(entry_date) = NaiveDate::parse_from_str(&company_entry_date, "%Y-%m-%d") {
            let today = Local::now().naive_local().date();
            let diff_days = (today - entry_date).num_days();
            let tenure = diff_days as f32 / 365.0;
            Some((tenure * 10.0).round() / 10.0) // 保留一位小数
        } else {
            None
        }
    } else {
        None
    };
    
    // 3. 根据参加工作时间计算工龄
    let calculated_work_tenure = if !work_start_date.is_empty() {
        if let Ok(start_date) = NaiveDate::parse_from_str(&work_start_date, "%Y-%m-%d") {
            let today = Local::now().naive_local().date();
            let diff_days = (today - start_date).num_days();
            let tenure = diff_days as f32 / 365.0;
            Some((tenure * 10.0).round() / 10.0) // 保留一位小数
        } else {
            None
        }
    } else {
        None
    };
    
    // 4. 根据任职时间和试用期计算试用期满到期提醒
    let calculated_probation_end_reminder = {
        if !position_entry_date.is_empty() && !probation_period.is_empty() {
            if let Ok(position_date) = NaiveDate::parse_from_str(&position_entry_date, "%Y-%m-%d") {
                if let Ok(probation_years) = probation_period.parse::<f32>() {
                    // 计算试用期结束日期
                    let end_date = position_date + Duration::days((probation_years * 365.0) as i64);
                    Some(end_date.format("%Y-%m-%d").to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };
    
    let cadre_info = GrassrootsCadreInfo {
        id: None,
        serial_number: None, // 序号字段已移除
        name: name,
        gender: Some(gender), // 性别
        department: Some(department), // 部门
        section: Some(section), // 科室
        position1: Some(position1), // 职务1
        position2: Some(position2), // 职务2
        company_entry_date: Some(company_entry_date), // 入司日期 (已标准化)
        company_tenure: calculated_company_tenure, // 司龄(年) (自动计算)
        work_start_date: Some(work_start_date), // 参加工作时间 (已标准化)
        work_tenure: calculated_work_tenure, // 工龄(年) (自动计算)
        current_level_date: Some(current_level_date), // 任现职级时间 (已标准化)
        position_entry_date: Some(position_entry_date), // 任职时间 (已标准化)
        probation_period: Some(probation_period), // 试用期(年)
        probation_end_reminder: calculated_probation_end_reminder, // 试用期满到期提醒 (自动计算)
        id_number: Some(id_number), // 身份证号
        birth_date: calculated_birth_date, // 出生日期 (已标准化)
        age: calculated_age, // 年龄 (自动计算)
        native_place: Some(native_place), // 籍贯
        birth_place: Some(birth_place), // 出生地
        ethnicity: Some(ethnicity), // 民族
        technical_position: Some(technical_position), // 专业技术职务
        education: Some(education), // 最高学历
        full_time_education: Some(full_time_education), // 全日制学历
        full_time_school_major: Some(full_time_school_major), // 全日制毕业院校系及专业
        part_time_education: Some(part_time_education), // 在职学历
        part_time_school_phone: Some(part_time_school_phone), // 在职毕业院校系及专业
        political_status: Some(political_status), // 政治面貌
        party_entry_date: Some(party_entry_date), // 入党时间 (已标准化)
        phone: Some(phone), // 联系电话
        grassroots_vice_position_date: Some(grassroots_vice_position_date), // 任基层副职时间
        grassroots_vice_tenure: grassroots_vice_tenure, // 任基层副职年限
        grassroots_chief_position_date: Some(grassroots_chief_position_date), // 任基层正职时间
        grassroots_chief_tenure: grassroots_chief_tenure, // 任基层正职年限
        midlevel_assistant_date: Some(midlevel_assistant_date), // 任中层助理时间
        midlevel_assistant_tenure: midlevel_assistant_tenure, // 任中层助理年限
        midlevel_vice_date: Some(midlevel_vice_date), // 任中层副职时间
        midlevel_vice_tenure: midlevel_vice_tenure, // 任中层副职年限
        midlevel_chief_date: Some(midlevel_chief_date), // 任中层正职时间
        midlevel_chief_tenure: midlevel_chief_tenure, // 任中层正职年限
        same_department_date: Some(same_department_date), // 同部门任职时间
        same_department_tenure: same_department_tenure, // 同部门任职年限
        remarks: Some(remarks), // 备注
        major: None, // 专业字段（未在Excel中使用）
        contact_date: None, // 联系日期字段（未在Excel中使用）
        special_date: None, // 特殊日期字段（未在Excel中使用）
    };
    
    Ok(cadre_info)
}

// 从Excel行数据解析中层干部信息
fn parse_midlevel_cadre_info_from_row(row: &[Data], row_index: usize) -> Result<MidLevelCadreInfo, String> {
    // 调试信息：打印行的总列数
    eprintln!("DEBUG: 第{}行总列数: {}", row_index, row.len());
    
    // 辅助函数：安全获取单元格值
    // 修改此函数以正确处理 Excel 的 DateTime 类型
    let get_cell_value = |index: usize| -> String {
        if index < row.len() {
            match &row[index] {
                Data::String(s) => s.clone(),
                Data::Float(f) => f.to_string(),
                Data::Int(i) => i.to_string(),
                // 处理 Excel 的 DateTime 类型
                Data::DateTime(d) => {
                    convert_excel_date_to_string(d)
                },
                Data::Bool(b) => b.to_string(),
                Data::Empty => String::new(),
                _ => String::new(),
            }
        } else {
            eprintln!("DEBUG: 尝试访问第{}列，但行只有{}列", index, row.len());
            String::new()
        }
    };
    
    // 解析各字段 (按照指定的字段顺序，保留自动计算字段在模板中但不读取其数据，注意中层管理人员没有科室字段)
    let name = get_cell_value(0); // 姓名
    if name.trim().is_empty() {
        return Err(format!("第{}行姓名不能为空", row_index));
    }
    
    let gender = get_cell_value(1); // 性别
    let department = get_cell_value(2); // 部门
    // 注意：中层管理人员没有科室字段，所以索引需要调整
    let position1 = get_cell_value(3); // 职务1
    let position2 = get_cell_value(4); // 职务2
    let company_entry_date_raw = get_cell_value(5); // 入司日期
    let _company_tenure = get_cell_value(6); // 司龄（年） - 自动计算字段，导入时不使用
    let work_start_date_raw = get_cell_value(7); // 参加工作时间
    let _work_tenure = get_cell_value(8); // 工龄(年) - 自动计算字段，导入时不使用
    let current_level_date_raw = get_cell_value(9); // 任现职级时间
    let position_entry_date_raw = get_cell_value(10); // 任职时间
    let probation_period = get_cell_value(11); // 试用期
    let _probation_end_reminder = get_cell_value(12); // 试用期满到期提醒 - 自动计算字段，导入时不使用
    let id_number = get_cell_value(13); // 身份证号
    let birth_date_raw = get_cell_value(14); // 出生日期
    let _age = get_cell_value(15); // 年龄 - 自动计算字段，导入时不使用
    let native_place = get_cell_value(16); // 籍贯
    let birth_place = get_cell_value(17); // 出生地
    let ethnicity = get_cell_value(18); // 民族
    let technical_position = get_cell_value(19); // 专业技术职务
    let education = get_cell_value(20); // 学历
    let full_time_education = get_cell_value(21); // 全日制学历
    let full_time_school_major = get_cell_value(22); // 毕业院校系及专业1
    let part_time_education = get_cell_value(23); // 在职学历
    let part_time_school_phone = get_cell_value(24); // 毕业院校系及专业2
    let political_status = get_cell_value(25); // 政治面貌
    let party_entry_date_raw = get_cell_value(26); // 入党时间
    let phone = get_cell_value(27); // 联系电话
    let grassroots_vice_position_date_raw = get_cell_value(28); // 任基层副职时间
    let _grassroots_vice_tenure_str = get_cell_value(29); // 任基层副职年限
    let grassroots_chief_position_date_raw = get_cell_value(30); // 任基层正职时间
    let grassroots_chief_tenure_str = get_cell_value(31); // 任基层正职年限
    let midlevel_assistant_date_raw = get_cell_value(32); // 任中层助理时间
    let midlevel_assistant_tenure_str = get_cell_value(33); // 任中层助理年限
    let midlevel_vice_date_raw = get_cell_value(34); // 任中层副职时间
    let midlevel_vice_tenure_str = get_cell_value(35); // 任中层副职年限
    let midlevel_chief_date_raw = get_cell_value(36); // 任中层正职时间
    let midlevel_chief_tenure_str = get_cell_value(37); // 任中层正职年限
    let same_department_date_raw = get_cell_value(38); // 同部门任职时间
    let same_department_tenure_str = get_cell_value(39); // 同部门任职年限
    let remarks = get_cell_value(40); // 备注
    
    // 对日期字段应用标准化函数
    let company_entry_date = normalize_date_format(&company_entry_date_raw); // 入司日期
    let work_start_date = normalize_date_format(&work_start_date_raw); // 参加工作时间
    let current_level_date = normalize_date_format(&current_level_date_raw); // 任现职级时间
    let position_entry_date = normalize_date_format(&position_entry_date_raw); // 任职时间
    let party_entry_date = normalize_date_format(&party_entry_date_raw); // 入党时间
    let _birth_date = normalize_date_format(&birth_date_raw); // 出生日期
    let same_department_date = normalize_date_format(&same_department_date_raw); // 同部门任职时间
    
    // 标准化新增的日期字段
    let grassroots_vice_position_date = normalize_date_format(&grassroots_vice_position_date_raw);
    let grassroots_chief_position_date = normalize_date_format(&grassroots_chief_position_date_raw);
    let midlevel_assistant_date = normalize_date_format(&midlevel_assistant_date_raw);
    let midlevel_vice_date = normalize_date_format(&midlevel_vice_date_raw);
    let midlevel_chief_date = normalize_date_format(&midlevel_chief_date_raw);
    
    // 自动计算任基层副职年限
    let grassroots_vice_tenure = {
        let mut tenure = String::new();
        
        // 如果有任基层正职时间，则用正职时间减去副职时间
        if !grassroots_chief_position_date_raw.is_empty() && !grassroots_vice_position_date_raw.is_empty() {
            if let Ok(vice_date) = NaiveDate::parse_from_str(&normalize_date_format(&grassroots_vice_position_date_raw), "%Y-%m-%d") {
                if let Ok(chief_date) = NaiveDate::parse_from_str(&normalize_date_format(&grassroots_chief_position_date_raw), "%Y-%m-%d") {
                    if chief_date >= vice_date {
                        let diff_days = (chief_date - vice_date).num_days();
                        let diff_years = diff_days / 365;
                        let diff_months = (diff_days % 365) / 30;
                        tenure = format!("{}年{}月", diff_years, diff_months);
                    }
                }
            }
        } 
        // 如果没有基层正职时间，则按照当前时间减去副职时间
        else if !grassroots_vice_position_date_raw.is_empty() {
            if let Ok(vice_date) = NaiveDate::parse_from_str(&normalize_date_format(&grassroots_vice_position_date_raw), "%Y-%m-%d") {
                let today = Local::now().naive_local().date();
                let diff_days = (today - vice_date).num_days();
                let diff_years = diff_days / 365;
                let diff_months = (diff_days % 365) / 30;
                tenure = format!("{}年{}月", diff_years, diff_months);
            }
        }
        
        if !tenure.is_empty() {
            Some(tenure)
        } else {
            None
        }
    };
    
    let grassroots_chief_tenure = if !grassroots_chief_tenure_str.is_empty() {
        grassroots_chief_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    let midlevel_assistant_tenure = if !midlevel_assistant_tenure_str.is_empty() {
        midlevel_assistant_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    let midlevel_vice_tenure = if !midlevel_vice_tenure_str.is_empty() {
        midlevel_vice_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    let midlevel_chief_tenure = if !midlevel_chief_tenure_str.is_empty() {
        midlevel_chief_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    let same_department_tenure = if !same_department_tenure_str.is_empty() {
        same_department_tenure_str.parse::<f32>().ok()
    } else {
        None
    };
    
    // 自动计算字段
    // 1. 根据身份证号计算出生日期和年龄
    let (calculated_birth_date, calculated_age) = if !id_number.is_empty() && id_number.len() == 18 {
        // 提取出生日期 (第7-14位)
        let birth_year = &id_number[6..10];
        let birth_month = &id_number[10..12];
        let birth_day = &id_number[12..14];
        
        // 创建日期字符串
        let birth_date_str = format!("{}-{}-{}", birth_year, birth_month, birth_day);
        
        // 计算年龄
        if let Ok(birth_date) = NaiveDate::parse_from_str(&birth_date_str, "%Y-%m-%d") {
            let today = Local::now().naive_local().date();
            let age = (today - birth_date).num_days() / 365;
            (Some(birth_date_str), Some(age as i32))
        } else {
            (Some(birth_date_str), None)
        }
    } else {
        (None, None)
    };
    
    // 2. 根据入司日期计算司龄
    let calculated_company_tenure = if !company_entry_date.is_empty() {
        if let Ok(entry_date) = NaiveDate::parse_from_str(&company_entry_date, "%Y-%m-%d") {
            let today = Local::now().naive_local().date();
            let diff_days = (today - entry_date).num_days();
            let tenure = diff_days as f32 / 365.0;
            Some((tenure * 10.0).round() / 10.0) // 保留一位小数
        } else {
            None
        }
    } else {
        None
    };
    
    // 3. 根据参加工作时间计算工龄
    let calculated_work_tenure = if !work_start_date.is_empty() {
        if let Ok(start_date) = NaiveDate::parse_from_str(&work_start_date, "%Y-%m-%d") {
            let today = Local::now().naive_local().date();
            let diff_days = (today - start_date).num_days();
            let tenure = diff_days as f32 / 365.0;
            Some((tenure * 10.0).round() / 10.0) // 保留一位小数
        } else {
            None
        }
    } else {
        None
    };
    
    // 4. 根据任职时间和试用期计算试用期满到期提醒
    let calculated_probation_end_reminder = {
        if !position_entry_date.is_empty() && !probation_period.is_empty() {
            if let Ok(position_date) = NaiveDate::parse_from_str(&position_entry_date, "%Y-%m-%d") {
                if let Ok(probation_years) = probation_period.parse::<f32>() {
                    // 计算试用期结束日期
                    let end_date = position_date + Duration::days((probation_years * 365.0) as i64);
                    Some(end_date.format("%Y-%m-%d").to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };
    
    let cadre_info = MidLevelCadreInfo {
        id: None,
        serial_number: None, // 序号字段已移除
        name: name,
        gender: Some(gender), // 性别
        department: Some(department), // 部门
        position1: Some(position1), // 职务1
        position2: Some(position2), // 职务2
        company_entry_date: Some(company_entry_date), // 入司日期 (已标准化)
        company_tenure: calculated_company_tenure, // 司龄(年) (自动计算)
        work_start_date: Some(work_start_date), // 参加工作时间 (已标准化)
        work_tenure: calculated_work_tenure, // 工龄(年) (自动计算)
        current_level_date: Some(current_level_date), // 任现职级时间 (已标准化)
        position_entry_date: Some(position_entry_date), // 任职时间 (已标准化)
        probation_period: Some(probation_period), // 试用期(年)
        probation_end_reminder: calculated_probation_end_reminder, // 试用期满到期提醒 (自动计算)
        id_number: Some(id_number), // 身份证号
        birth_date: calculated_birth_date, // 出生日期 (已标准化)
        age: calculated_age, // 年龄 (自动计算)
        native_place: Some(native_place), // 籍贯
        birth_place: Some(birth_place), // 出生地
        ethnicity: Some(ethnicity), // 民族
        technical_position: Some(technical_position), // 专业技术职务
        education: Some(education), // 最高学历
        full_time_education: Some(full_time_education), // 全日制学历
        full_time_school_major: Some(full_time_school_major), // 全日制毕业院校系及专业
        part_time_education: Some(part_time_education), // 在职学历
        part_time_school_phone: Some(part_time_school_phone), // 在职毕业院校系及专业
        political_status: Some(political_status), // 政治面貌
        party_entry_date: Some(party_entry_date), // 入党时间 (已标准化)
        phone: Some(phone), // 联系电话
        grassroots_vice_position_date: Some(grassroots_vice_position_date), // 任基层副职时间
        grassroots_vice_tenure: grassroots_vice_tenure, // 任基层副职年限
        grassroots_chief_position_date: Some(grassroots_chief_position_date), // 任基层正职时间
        grassroots_chief_tenure: grassroots_chief_tenure, // 任基层正职年限
        midlevel_assistant_date: Some(midlevel_assistant_date), // 任中层助理时间
        midlevel_assistant_tenure: midlevel_assistant_tenure, // 任中层助理年限
        midlevel_vice_date: Some(midlevel_vice_date), // 任中层副职时间
        midlevel_vice_tenure: midlevel_vice_tenure, // 任中层副职年限
        midlevel_chief_date: Some(midlevel_chief_date), // 任中层正职时间
        midlevel_chief_tenure: midlevel_chief_tenure, // 任中层正职年限
        same_department_date: Some(same_department_date), // 同部门任职时间
        same_department_tenure: same_department_tenure, // 同部门任职年限
        remarks: Some(remarks), // 备注
        major: None, // 专业字段（未在Excel中使用）
        contact_date: None, // 联系日期字段（未在Excel中使用）
        special_date: None, // 特殊日期字段（未在Excel中使用）
    };
    
    Ok(cadre_info)
}

// ... [其他函数] ...

#[tauri::command]
fn generate_import_template(is_midlevel: bool) -> Result<Vec<u8>, String> {
    // 创建内存中的工作簿
    let mut workbook = simple_excel_writer::Workbook::create_in_memory();
    
    // 创建工作表
    let mut sheet = workbook.create_sheet("干部信息导入模板");
    
    // 写入表头和示例数据
    workbook.write_sheet(&mut sheet, |sheet_writer| {
        use simple_excel_writer::*;
        
        if is_midlevel {
            // 中层管理人员模板 (不包含科室字段，但包含自动计算字段)
            sheet_writer.append_row(row!["姓名", "性别", "部门", "职务1", "职务2", 
                          "入司日期", "司龄（年）", "参加工作时间", "工龄(年)", 
                          "任现职级时间", "任职时间", "试用期", "试用期满到期提醒", 
                          "身份证号", "出生日期", "年龄", "籍贯", "出生地", 
                          "民族", "专业技术职务", "学历", "全日制学历", "毕业院校系及专业1", "在职学历", 
                          "毕业院校系及专业2", "政治面貌", "入党时间", "联系电话", 
                          "任基层副职时间", "任基层副职年限", "任基层正职时间", "任基层正职年限", 
                          "任中层助理时间", "任中层助理年限", "任中层副职时间", "任中层副职年限", 
                          "任中层正职时间", "任中层正职年限", "同部门任职年限", "备注"])?;
            
            // 写入数据规范说明行
            sheet_writer.append_row(row!["", "男/女", "部门名称", "职务名称", "职务名称",
                          "YYYY-MM-DD", "数字", "YYYY-MM-DD", "数字",
                          "YYYY-MM-DD", "YYYY-MM-DD", "数字", "YYYY-MM-DD",
                          "18位身份证号", "YYYY-MM-DD", "数字", "地区", 
                          "地区", "民族名称", "职务名称", "学历名称", "学历名称", "学校专业", 
                          "学历名称", "学校专业", "政治面貌选项", "YYYY-MM-DD", "手机号码", 
                          "YYYY-MM-DD", "文本(x年x月)", "YYYY-MM-DD", "数字", 
                          "YYYY-MM-DD", "数字", "YYYY-MM-DD", "数字", 
                          "YYYY-MM-DD", "数字", "数字", "备注信息"])?;
            
            // 写入示例行
            sheet_writer.append_row(row!["张三", "男", "人力资源部", "部长", "副主任",
                          "2020-01-15", "5.2", "2010-07-01", "12.5",
                          "2021-03-20", "2020-01-15", "1", "2021-01-15",
                          "110101199001011234", "1990-01-01", "33", "北京", 
                          "北京", "汉族", "高级工程师", "硕士研究生", "硕士研究生", 
                          "清华大学计算机系", "硕士研究生", "清华大学计算机系", "中共党员", 
                          "2015-06-15", "13800138000", 
                          "2018-05-10", "2年3月", "2020-01-15", "1年2月", 
                          "2019-03-20", "1年8月", "2021-06-01", "0年5月", 
                          "2022-01-10", "0年3月", "2年0月", "优秀员工"])?;
        } else {
            // 基层管理人员模板 (包含科室字段和自动计算字段)
            sheet_writer.append_row(row!["姓名", "性别", "部门", "科室", "职务1", "职务2", 
                          "入司日期", "司龄（年）", "参加工作时间", "工龄(年)", 
                          "任现职级时间", "任职时间", "试用期", "试用期满到期提醒", 
                          "身份证号", "出生日期", "年龄", "籍贯", "出生地", 
                          "民族", "专业技术职务", "学历", "全日制学历", "毕业院校系及专业1", "在职学历", 
                          "毕业院校系及专业2", "政治面貌", "入党时间", "联系电话", 
                          "任基层副职时间", "任基层副职年限", "任基层正职时间", "任基层正职年限", 
                          "任中层助理时间", "任中层助理年限", "任中层副职时间", "任中层副职年限", 
                          "任中层正职时间", "任中层正职年限", "同部门任职年限", "备注"])?;
            
            // 写入数据规范说明行
            sheet_writer.append_row(row!["", "男/女", "部门名称", "科室名称", "职务名称", "职务名称",
                          "YYYY-MM-DD", "数字", "YYYY-MM-DD", "数字",
                          "YYYY-MM-DD", "YYYY-MM-DD", "数字", "YYYY-MM-DD",
                          "18位身份证号", "YYYY-MM-DD", "数字", "地区", 
                          "地区", "民族名称", "职务名称", "学历名称", "学历名称", "学校专业", 
                          "学历名称", "学校专业", "政治面貌选项", "YYYY-MM-DD", "手机号码", 
                          "YYYY-MM-DD", "文本(x年x月)", "YYYY-MM-DD", "数字", 
                          "YYYY-MM-DD", "数字", "YYYY-MM-DD", "数字", 
                          "YYYY-MM-DD", "数字", "数字", "备注信息"])?;
            
            // 写入示例行
            sheet_writer.append_row(row!["张三", "男", "人力资源部", "招聘科", "部长", "副主任",
                          "2020-01-15", "5.2", "2010-07-01", "12.5",
                          "2021-03-20", "2020-01-15", "1", "2021-01-15",
                          "110101199001011234", "1990-01-01", "33", "北京", 
                          "北京", "汉族", "高级工程师", "硕士研究生", "硕士研究生", 
                          "清华大学计算机系", "硕士研究生", "清华大学计算机系", "中共党员", 
                          "2015-06-15", "13800138000", 
                          "2018-05-10", "2年3月", "2020-01-15", "1年2月", 
                          "2019-03-20", "1年8月", "2021-06-01", "0年5月", 
                          "2022-01-10", "0年3月", "2年0月", "优秀员工"])?;
        }
        
        // 写入政治面貌选项说明
        sheet_writer.append_row(row!["", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
                      "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
                      "可选值: 中共党员/预备党员/共青团员/民革党员/民盟盟员/民建会员/民进会员/农工党党员/致公党党员/九三学社社员/台盟盟员/无党派人士/群众", "", "", ""])?;
        
        // 写入学历选项说明
        sheet_writer.append_row(row!["", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
                      "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
                      "可选值: 博士研究生/硕士研究生/大学/大专/高中/中专/初中/职高", 
                      "可选值: 博士研究生/硕士研究生/大学/大专/高中/中专/初中/职高", "", 
                      "可选值: 博士研究生/硕士研究生/大学/大专/高中/中专/初中/职高", "", "", ""])?;
        
        Ok(())
    }).map_err(|e| format!("生成Excel模板失败: {}", e))?;
    
    // 将工作簿保存到内存中的字节向量
    let result = workbook.close();
    
    match result {
        Ok(Some(buffer)) => {
            Ok(buffer)
        },
        Ok(None) => {
            Ok(Vec::new())
        },
        Err(e) => {
            Err(format!("保存Excel模板失败: {}", e))
        }
    }
}

#[tauri::command]
fn save_import_template(file_path: String, is_midlevel: bool) -> Result<(), String> {
    // 生成模板数据
    let template_data = generate_import_template(is_midlevel)?;
    
    // 将模板数据保存到指定文件路径
    std::fs::write(&file_path, template_data)
        .map_err(|e| format!("保存Excel模板到文件失败: {}", e))?;
    
    Ok(())
}

// ==================== Grassroots Cadres Commands ====================

#[tauri::command]
fn add_grassroots_cadre_info(db: State<'_, Mutex<Database>>, cadre: GrassrootsCadreInfo) -> Result<(), String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.add_grassroots_cadre(&cadre) {
                Ok(_result) => {
                    Ok(())
                },
                Err(e) => {
                    Err(format!("添加基层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn get_all_grassroots_cadre_info(db: State<'_, Mutex<Database>>) -> Result<Vec<GrassrootsCadreInfo>, String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.get_all_grassroots_cadres() {
                Ok(cadres) => {
                    Ok(cadres)
                },
                Err(e) => {
                    Err(format!("获取所有基层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn get_grassroots_cadre_info_by_id(db: State<'_, Mutex<Database>>, id: i32) -> Result<Option<GrassrootsCadreInfo>, String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.get_grassroots_cadre_by_id(id) {
                Ok(cadre) => {
                    Ok(cadre)
                },
                Err(e) => {
                    Err(format!("根据ID获取基层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn update_grassroots_cadre_info(db: State<'_, Mutex<Database>>, cadre: GrassrootsCadreInfo) -> Result<(), String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.update_grassroots_cadre(&cadre) {
                Ok(_result) => {
                    Ok(())
                },
                Err(e) => {
                    Err(format!("更新基层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn delete_grassroots_cadre_info(db: State<'_, Mutex<Database>>, id: i32) -> Result<(), String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.delete_grassroots_cadre(id) {
                Ok(_result) => {
                    Ok(())
                },
                Err(e) => {
                    Err(format!("删除基层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

// ==================== Mid-Level Cadres Commands ====================

#[tauri::command]
fn add_midlevel_cadre_info(db: State<'_, Mutex<Database>>, cadre: MidLevelCadreInfo) -> Result<(), String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.add_midlevel_cadre(&cadre) {
                Ok(_result) => {
                    Ok(())
                },
                Err(e) => {
                    Err(format!("添加中层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn get_all_midlevel_cadre_info(db: State<'_, Mutex<Database>>) -> Result<Vec<MidLevelCadreInfo>, String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.get_all_midlevel_cadres() {
                Ok(cadres) => {
                    Ok(cadres)
                },
                Err(e) => {
                    Err(format!("获取所有中层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn get_midlevel_cadre_info_by_id(db: State<'_, Mutex<Database>>, id: i32) -> Result<Option<MidLevelCadreInfo>, String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.get_midlevel_cadre_by_id(id) {
                Ok(cadre) => {
                    Ok(cadre)
                },
                Err(e) => {
                    Err(format!("根据ID获取中层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn update_midlevel_cadre_info(db: State<'_, Mutex<Database>>, cadre: MidLevelCadreInfo) -> Result<(), String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.update_midlevel_cadre(&cadre) {
                Ok(_result) => {
                    Ok(())
                },
                Err(e) => {
                    Err(format!("更新中层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn delete_midlevel_cadre_info(db: State<'_, Mutex<Database>>, id: i32) -> Result<(), String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.delete_midlevel_cadre(id) {
                Ok(_result) => {
                    Ok(())
                },
                Err(e) => {
                    Err(format!("删除中层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[tauri::command]
fn get_distinct_field_values_for_table(db: State<'_, Mutex<Database>>, table_name: String, field_name: String) -> Result<Vec<String>, String> {
    match db.lock() {
        Ok(db_guard) => {
            match db_guard.get_distinct_field_values(&table_name, &field_name) {
                Ok(values) => {
                    Ok(values)
                },
                Err(e) => {
                    Err(format!("获取表{}字段{}的distinct值失败: {}", table_name, field_name, e))
                }
            }
        },
        Err(e) => {
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化数据库
    let database = match Database::new() {
        Ok(db) => {
            db
        },
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    
    tauri::Builder::default()
        .manage(Mutex::new(database))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            export_grassroots_cadre_info_to_excel,
            export_midlevel_cadre_info_to_excel,
            generate_import_template,
            import_cadre_info_from_excel,
            save_import_template,
            // Grassroots cadres commands
            add_grassroots_cadre_info,
            get_all_grassroots_cadre_info,
            get_grassroots_cadre_info_by_id,
            update_grassroots_cadre_info,
            delete_grassroots_cadre_info,
            // Mid-level cadres commands
            add_midlevel_cadre_info,
            get_all_midlevel_cadre_info,
            get_midlevel_cadre_info_by_id,
            update_midlevel_cadre_info,
            delete_midlevel_cadre_info,
            // Common commands
            get_distinct_field_values_for_table,
            // 新增的干部名册导出命令
            export_grassroots_cadre_roster,
            export_midlevel_cadre_roster
        ])
        .run(tauri::generate_context!())
        .map_err(|e| {
            Box::new(e) as Box<dyn std::error::Error>
        })
}

// 使用模板导出基层管理人员名册
#[tauri::command]
fn export_grassroots_cadre_roster(db: State<'_, Mutex<Database>>, output_path: String, filter_params: Option<FilterParams>) -> Result<(), String> {
    use std::fs;
    use edit_xlsx::Write;
    
    // 获取数据库连接
    let db_guard = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    
    // 根据是否有筛选条件获取数据
    let cadres = if let Some(params) = filter_params {
        db_guard.get_filtered_grassroots_cadres(&params).map_err(|e| format!("获取筛选后的基层干部信息失败: {}", e))?
    } else {
        db_guard.get_all_grassroots_cadres().map_err(|e| format!("获取基层干部信息失败: {}", e))?
    };
    
    // 获取当前可执行文件的目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取可执行文件路径失败: {}", e))?
        .parent()
        .ok_or("无法获取可执行文件所在目录")?
        .to_path_buf();
    
    // 构建模板文件的完整路径
    // 首先尝试在当前可执行文件目录的templates子目录中查找
    let mut template_path = exe_dir.join("templates").join("基层管理人员名册.xlsx");
    
    // 如果找不到，则尝试在上一级目录的templates子目录中查找（开发环境）
    if !template_path.exists() {
        template_path = exe_dir.join("..").join("templates").join("基层管理人员名册.xlsx");
    }
    
    // 如果还找不到，则尝试在src-tauri目录下的templates子目录中查找（开发环境）
    if !template_path.exists() {
        template_path = exe_dir.join("..").join("..").join("templates").join("基层管理人员名册.xlsx");
    }
    
    // 如果还找不到，则尝试使用Tauri的资源路径（打包后）
    if !template_path.exists() {
        // 尝试获取应用的资源目录
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(parent_dir) = current_exe.parent() {
                let resource_path = parent_dir.join("templates").join("基层管理人员名册.xlsx");
                if resource_path.exists() {
                    template_path = resource_path;
                }
            }
        }
    }
    
    // 检查模板是否存在
    if !template_path.exists() {
        // 获取当前工作目录，用于调试
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("unknown"));
        return Err(format!("模板文件不存在。当前工作目录: {:?}, 尝试查找路径: {:?}", current_dir, template_path));
    }
    
    // 复制模板文件到输出位置
    fs::copy(&template_path, &output_path)
        .map_err(|e| format!("复制模板文件失败: {}", e))?;
    
    // 使用edit-xlsx库打开复制的文件并添加数据
    let mut workbook = edit_xlsx::Workbook::from_path(&output_path)
        .map_err(|e| format!("打开Excel文件失败: {}", e))?;
    
    // 获取第一个工作表
    let worksheet = workbook.get_worksheet_mut(1)
        .map_err(|e| format!("获取工作表失败: {}", e))?;
    
    // 获取模板的行数，以确定从哪里开始添加数据
    let template_row_count = worksheet.max_row() as usize;
    let mut current_row = template_row_count + 1; // 从模板之后的行开始添加数据
    
    // 添加干部数据
    for (index, cadre) in cadres.iter().enumerate() {
        // 安全获取字段值
        let index_str = (index + 1).to_string();
        let name = cadre.name.clone();
        let gender = cadre.gender.as_ref().unwrap_or(&"".to_string()).clone();
        let position1 = cadre.position1.as_ref().unwrap_or(&"".to_string()).clone();
        let birth_date = cadre.birth_date.as_ref().unwrap_or(&"".to_string()).clone();
        let ethnicity = cadre.ethnicity.as_ref().unwrap_or(&"".to_string()).clone();
        let native_place = cadre.native_place.as_ref().unwrap_or(&"".to_string()).clone();
        let work_start_date = cadre.work_start_date.as_ref().unwrap_or(&"".to_string()).clone();
        let party_entry_date = cadre.party_entry_date.as_ref().unwrap_or(&"".to_string()).clone();
        let full_time_education = cadre.full_time_education.as_ref().unwrap_or(&"".to_string()).clone();
        let full_time_school_major = cadre.full_time_school_major.as_ref().unwrap_or(&"".to_string()).clone();
        let part_time_education = cadre.part_time_education.as_ref().unwrap_or(&"".to_string()).clone();
        let part_time_school_phone = cadre.part_time_school_phone.as_ref().unwrap_or(&"".to_string()).clone();
        let technical_position = cadre.technical_position.as_ref().unwrap_or(&"".to_string()).clone();
        let position_entry_date = cadre.position_entry_date.as_ref().unwrap_or(&"".to_string()).clone();
        let current_level_date = cadre.current_level_date.as_ref().unwrap_or(&"".to_string()).clone();
        let id_number = cadre.id_number.as_ref().unwrap_or(&"".to_string()).clone();
        let phone = cadre.phone.as_ref().unwrap_or(&"".to_string()).clone();
        let remarks = cadre.remarks.as_ref().unwrap_or(&"".to_string()).clone();
        
        // 写入数据行
        worksheet.write(&format!("A{}", current_row), index_str)
            .map_err(|e| format!("写入索引失败: {}", e))?;
        worksheet.write(&format!("B{}", current_row), name)
            .map_err(|e| format!("写入姓名失败: {}", e))?;
        worksheet.write(&format!("C{}", current_row), gender)
            .map_err(|e| format!("写入性别失败: {}", e))?;
        worksheet.write(&format!("D{}", current_row), position1)
            .map_err(|e| format!("写入职务失败: {}", e))?;
        worksheet.write(&format!("E{}", current_row), birth_date)
            .map_err(|e| format!("写入出生日期失败: {}", e))?;
        worksheet.write(&format!("F{}", current_row), ethnicity)
            .map_err(|e| format!("写入民族失败: {}", e))?;
        worksheet.write(&format!("G{}", current_row), native_place)
            .map_err(|e| format!("写入籍贯失败: {}", e))?;
        worksheet.write(&format!("H{}", current_row), work_start_date)
            .map_err(|e| format!("写入参加工作时间失败: {}", e))?;
        worksheet.write(&format!("I{}", current_row), party_entry_date)
            .map_err(|e| format!("写入入党时间失败: {}", e))?;
        worksheet.write(&format!("J{}", current_row), full_time_education)
            .map_err(|e| format!("写入全日制学历失败: {}", e))?;
        worksheet.write(&format!("K{}", current_row), full_time_school_major)
            .map_err(|e| format!("写入全日制毕业院校失败: {}", e))?;
        worksheet.write(&format!("L{}", current_row), part_time_education)
            .map_err(|e| format!("写入在职学历失败: {}", e))?;
        worksheet.write(&format!("M{}", current_row), part_time_school_phone)
            .map_err(|e| format!("写入在职毕业院校失败: {}", e))?;
        worksheet.write(&format!("N{}", current_row), technical_position)
            .map_err(|e| format!("写入专业技术职务失败: {}", e))?;
        worksheet.write(&format!("O{}", current_row), position_entry_date)
            .map_err(|e| format!("写入任职时间失败: {}", e))?;
        worksheet.write(&format!("P{}", current_row), current_level_date)
            .map_err(|e| format!("写入任现职级时间失败: {}", e))?;
        worksheet.write(&format!("Q{}", current_row), id_number)
            .map_err(|e| format!("写入身份证号失败: {}", e))?;
        worksheet.write(&format!("R{}", current_row), phone)
            .map_err(|e| format!("写入联系电话失败: {}", e))?;
        worksheet.write(&format!("S{}", current_row), remarks)
            .map_err(|e| format!("写入备注失败: {}", e))?;
        
        current_row += 1;
    }
    
    // 保存文件
    workbook.save()
        .map_err(|e| format!("保存Excel文件失败: {}", e))?;
    
    Ok(())
}

// 使用模板导出中层管理人员名册
#[tauri::command]
fn export_midlevel_cadre_roster(db: State<'_, Mutex<Database>>, output_path: String, filter_params: Option<FilterParams>) -> Result<(), String> {
    use std::fs;
    use edit_xlsx::Write;
    
    // 获取数据库连接
    let db_guard = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    
    // 根据是否有筛选条件获取数据
    let cadres = if let Some(params) = filter_params {
        db_guard.get_filtered_midlevel_cadres(&params).map_err(|e| format!("获取筛选后的中层干部信息失败: {}", e))?
    } else {
        db_guard.get_all_midlevel_cadres().map_err(|e| format!("获取中层干部信息失败: {}", e))?
    };
    
    // 获取当前可执行文件的目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取可执行文件路径失败: {}", e))?
        .parent()
        .ok_or("无法获取可执行文件所在目录")?
        .to_path_buf();
    
    // 构建模板文件的完整路径
    // 首先尝试在当前可执行文件目录的templates子目录中查找
    let mut template_path = exe_dir.join("templates").join("中层管理人员名册.xlsx");
    
    // 如果找不到，则尝试在上一级目录的templates子目录中查找（开发环境）
    if !template_path.exists() {
        template_path = exe_dir.join("..").join("templates").join("中层管理人员名册.xlsx");
    }
    
    // 如果还找不到，则尝试在src-tauri目录下的templates子目录中查找（开发环境）
    if !template_path.exists() {
        template_path = exe_dir.join("..").join("..").join("templates").join("中层管理人员名册.xlsx");
    }
    
    // 如果还找不到，则尝试使用Tauri的资源路径（打包后）
    if !template_path.exists() {
        // 尝试获取应用的资源目录
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(parent_dir) = current_exe.parent() {
                let resource_path = parent_dir.join("templates").join("中层管理人员名册.xlsx");
                if resource_path.exists() {
                    template_path = resource_path;
                }
            }
        }
    }
    
    // 检查模板是否存在
    if !template_path.exists() {
        // 获取当前工作目录，用于调试
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("unknown"));
        return Err(format!("模板文件不存在。当前工作目录: {:?}, 尝试查找路径: {:?}", current_dir, template_path));
    }
    
    // 复制模板文件到输出位置
    fs::copy(&template_path, &output_path)
        .map_err(|e| format!("复制模板文件失败: {}", e))?;
    
    // 使用edit-xlsx库打开复制的文件并添加数据
    let mut workbook = edit_xlsx::Workbook::from_path(&output_path)
        .map_err(|e| format!("打开Excel文件失败: {}", e))?;
    
    // 获取第一个工作表
    let worksheet = workbook.get_worksheet_mut(1)
        .map_err(|e| format!("获取工作表失败: {}", e))?;
    
    // 获取模板的行数，以确定从哪里开始添加数据
    let template_row_count = worksheet.max_row() as usize;
    let mut current_row = template_row_count + 1; // 从模板之后的行开始添加数据
    
    // 添加干部数据
    for (index, cadre) in cadres.iter().enumerate() {
        // 安全获取字段值
        let index_str = (index + 1).to_string();
        let name = cadre.name.clone();
        let gender = cadre.gender.as_ref().unwrap_or(&"".to_string()).clone();
        let position1 = cadre.position1.as_ref().unwrap_or(&"".to_string()).clone();
        let birth_date = cadre.birth_date.as_ref().unwrap_or(&"".to_string()).clone();
        let ethnicity = cadre.ethnicity.as_ref().unwrap_or(&"".to_string()).clone();
        let native_place = cadre.native_place.as_ref().unwrap_or(&"".to_string()).clone();
        let work_start_date = cadre.work_start_date.as_ref().unwrap_or(&"".to_string()).clone();
        let party_entry_date = cadre.party_entry_date.as_ref().unwrap_or(&"".to_string()).clone();
        let full_time_education = cadre.full_time_education.as_ref().unwrap_or(&"".to_string()).clone();
        let full_time_school_major = cadre.full_time_school_major.as_ref().unwrap_or(&"".to_string()).clone();
        let part_time_education = cadre.part_time_education.as_ref().unwrap_or(&"".to_string()).clone();
        let part_time_school_phone = cadre.part_time_school_phone.as_ref().unwrap_or(&"".to_string()).clone();
        let technical_position = cadre.technical_position.as_ref().unwrap_or(&"".to_string()).clone();
        let position_entry_date = cadre.position_entry_date.as_ref().unwrap_or(&"".to_string()).clone();
        let current_level_date = cadre.current_level_date.as_ref().unwrap_or(&"".to_string()).clone();
        let id_number = cadre.id_number.as_ref().unwrap_or(&"".to_string()).clone();
        let phone = cadre.phone.as_ref().unwrap_or(&"".to_string()).clone();
        let remarks = cadre.remarks.as_ref().unwrap_or(&"".to_string()).clone();
        
        // 写入数据行
        worksheet.write(&format!("A{}", current_row), index_str)
            .map_err(|e| format!("写入索引失败: {}", e))?;
        worksheet.write(&format!("B{}", current_row), name)
            .map_err(|e| format!("写入姓名失败: {}", e))?;
        worksheet.write(&format!("C{}", current_row), gender)
            .map_err(|e| format!("写入性别失败: {}", e))?;
        worksheet.write(&format!("D{}", current_row), position1)
            .map_err(|e| format!("写入职务失败: {}", e))?;
        worksheet.write(&format!("E{}", current_row), birth_date)
            .map_err(|e| format!("写入出生日期失败: {}", e))?;
        worksheet.write(&format!("F{}", current_row), ethnicity)
            .map_err(|e| format!("写入民族失败: {}", e))?;
        worksheet.write(&format!("G{}", current_row), native_place)
            .map_err(|e| format!("写入籍贯失败: {}", e))?;
        worksheet.write(&format!("H{}", current_row), work_start_date)
            .map_err(|e| format!("写入参加工作时间失败: {}", e))?;
        worksheet.write(&format!("I{}", current_row), party_entry_date)
            .map_err(|e| format!("写入入党时间失败: {}", e))?;
        worksheet.write(&format!("J{}", current_row), full_time_education)
            .map_err(|e| format!("写入全日制学历失败: {}", e))?;
        worksheet.write(&format!("K{}", current_row), full_time_school_major)
            .map_err(|e| format!("写入全日制毕业院校失败: {}", e))?;
        worksheet.write(&format!("L{}", current_row), part_time_education)
            .map_err(|e| format!("写入在职学历失败: {}", e))?;
        worksheet.write(&format!("M{}", current_row), part_time_school_phone)
            .map_err(|e| format!("写入在职毕业院校失败: {}", e))?;
        worksheet.write(&format!("N{}", current_row), technical_position)
            .map_err(|e| format!("写入专业技术职务失败: {}", e))?;
        worksheet.write(&format!("O{}", current_row), position_entry_date)
            .map_err(|e| format!("写入任职时间失败: {}", e))?;
        worksheet.write(&format!("P{}", current_row), current_level_date)
            .map_err(|e| format!("写入任现职级时间失败: {}", e))?;
        worksheet.write(&format!("Q{}", current_row), id_number)
            .map_err(|e| format!("写入身份证号失败: {}", e))?;
        worksheet.write(&format!("R{}", current_row), phone)
            .map_err(|e| format!("写入联系电话失败: {}", e))?;
        worksheet.write(&format!("S{}", current_row), remarks)
            .map_err(|e| format!("写入备注失败: {}", e))?;
        
        current_row += 1;
    }
    
    // 保存文件
    workbook.save()
        .map_err(|e| format!("保存Excel文件失败: {}", e))?;
    
    Ok(())
}