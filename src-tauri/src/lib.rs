mod database;
mod export;

use database::{Database, GrassrootsCadreInfo, MidLevelCadreInfo};
use export::{ExportConfig, export_to_excel};
use tauri::State;
use std::sync::Mutex;
use calamine::{open_workbook_auto, Reader, Sheets, Data, ExcelDateTime};
// 导入处理chrono日期所需的类型
use chrono::{NaiveDate, Duration, Local};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    println!("执行greet命令，参数: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn export_grassroots_cadre_info_to_excel(
    db: State<'_, Mutex<Database>>,
    file_path: String,
    selected_fields: Vec<String>,
    cadre_ids: Option<Vec<i32>> // 新增参数，用于部分导出
) -> Result<(), String> {
    println!("开始执行export_grassroots_cadre_info_to_excel命令");
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
                    if let Ok(Some(cadre)) = db_guard.get_grassroots_cadre_by_id(id) {
                        selected_cadres.push(cadre);
                    }
                }
                selected_cadres
            } else {
                println!("导出全部数据");
                // 导出全部数据
                match db_guard.get_all_grassroots_cadres() {
                    Ok(cadres) => cadres,
                    Err(e) => {
                        eprintln!("获取所有基层干部信息失败: {}", e);
                        return Err(format!("获取所有基层干部信息失败: {}", e));
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
fn export_midlevel_cadre_info_to_excel(
    db: State<'_, Mutex<Database>>,
    file_path: String,
    selected_fields: Vec<String>,
    cadre_ids: Option<Vec<i32>> // 新增参数，用于部分导出
) -> Result<(), String> {
    println!("开始执行export_midlevel_cadre_info_to_excel命令");
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
                    if let Ok(Some(cadre)) = db_guard.get_midlevel_cadre_by_id(id) {
                        selected_cadres.push(cadre);
                    }
                }
                selected_cadres
            } else {
                println!("导出全部数据");
                // 导出全部数据
                match db_guard.get_all_midlevel_cadres() {
                    Ok(cadres) => cadres,
                    Err(e) => {
                        eprintln!("获取所有中层干部信息失败: {}", e);
                        return Err(format!("获取所有中层干部信息失败: {}", e));
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
                match db_guard.add_grassroots_cadre(&cadre_info) {
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
    // 但是 Excel 有一个著名的错误，它认为 1900 年是闰年（实际上不是）
    // 这意味着在 1900 年 3 月 1 日之前，Excel 的日期会比实际多一天
    // 我们需要处理这个错误
    
    // 计算从 1900-01-01 到 1970-01-01 的天数
    // 1970-01-01 是 Excel 日期序列 25569 (对于 1900-based 系统)
    // 但是由于 Excel 的 1900 闰年错误，实际的偏移量是 25568
    let excel_epoch_days = 25568;
    
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
    println!("解析第{}行数据", row_index);
    
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
            String::new()
        }
    };
    
    // 解析各字段 (按照新的列顺序)
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
    let current_level_date_raw = get_cell_value(7); // 任现职级时间
    let position_entry_date_raw = get_cell_value(8); // 任职时间
    let probation_period = get_cell_value(9); // 试用期(年)
    let probation_end_reminder_raw = get_cell_value(10); // 试用期满到期提醒
    let id_number = get_cell_value(11); // 身份证号
    let birth_date_raw = get_cell_value(12); // 出生日期
    let age_raw = get_cell_value(13); // 年龄
    let native_place = get_cell_value(14); // 籍贯
    let birth_place = get_cell_value(15); // 出生地
    let ethnicity = get_cell_value(16); // 民族
    let company_tenure_raw = get_cell_value(17); // 司龄(年)
    let work_start_date_raw = get_cell_value(18); // 参加工作时间
    let work_tenure_raw = get_cell_value(19); // 工龄(年)
    let technical_position = get_cell_value(20); // 专业技术职务
    let education = get_cell_value(21); // 最高学历
    let full_time_education = get_cell_value(22); // 全日制学历
    let full_time_school_major = get_cell_value(23); // 全日制毕业院校系及专业
    let part_time_education = get_cell_value(24); // 在职学历
    let part_time_school_phone = get_cell_value(25); // 在职毕业院校系及专业
    let political_status = get_cell_value(26); // 政治面貌
    let party_entry_date_raw = get_cell_value(27); // 入党时间
    let phone = get_cell_value(28); // 联系电话
    let remarks = get_cell_value(29); // 备注
    
    // 对日期字段应用标准化函数
    let company_entry_date = normalize_date_format(&company_entry_date_raw); // 入司日期
    let current_level_date = normalize_date_format(&current_level_date_raw); // 任现职级时间
    let position_entry_date = normalize_date_format(&position_entry_date_raw); // 任职时间
    let probation_end_reminder = normalize_date_format(&probation_end_reminder_raw); // 试用期满到期提醒
    let party_entry_date = normalize_date_format(&party_entry_date_raw); // 入党时间
    let birth_date = normalize_date_format(&birth_date_raw); // 出生日期
    let work_start_date = normalize_date_format(&work_start_date_raw); // 参加工作时间
    
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
            Some(tenure)
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
            Some(tenure)
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
        current_level_date: Some(current_level_date), // 任现职级时间 (已标准化)
        position_entry_date: Some(position_entry_date), // 任职时间 (已标准化)
        probation_period: Some(probation_period), // 试用期(年)
        probation_end_reminder: if let Some(date) = &calculated_probation_end_reminder {
            Some(date.clone())
        } else {
            Some(probation_end_reminder) // 如果没有计算出结果，则使用模板中的值
        }, // 试用期满到期提醒 (已标准化)
        id_number: Some(id_number), // 身份证号
        technical_position: Some(technical_position), // 专业技术职务
        education: Some(education), // 最高学历
        full_time_education: Some(full_time_education), // 全日制学历
        full_time_school_major: Some(full_time_school_major), // 全日制毕业院校系及专业
        part_time_education: Some(part_time_education), // 在职学历
        part_time_school_phone: Some(part_time_school_phone), // 在职毕业院校系及专业
        political_status: Some(political_status), // 政治面貌
        party_entry_date: Some(party_entry_date), // 入党时间 (已标准化)
        phone: Some(phone), // 联系电话
        birth_date: if let Some(date) = &calculated_birth_date {
            Some(date.clone())
        } else {
            Some(birth_date) // 如果没有计算出结果，则使用模板中的值
        }, // 出生日期 (已标准化)
        age: if let Some(age) = calculated_age {
            Some(age)
        } else {
            if !age_raw.is_empty() {
                age_raw.parse::<i32>().ok()
            } else {
                None
            }
        }, // 年龄
        native_place: Some(native_place), // 籍贯
        birth_place: Some(birth_place), // 出生地
        ethnicity: Some(ethnicity), // 民族
        company_tenure: if let Some(tenure) = calculated_company_tenure {
            Some(tenure)
        } else {
            if !company_tenure_raw.is_empty() {
                company_tenure_raw.parse::<f32>().ok()
            } else {
                None
            }
        }, // 司龄(年)
        work_start_date: Some(work_start_date), // 参加工作时间 (已标准化)
        work_tenure: if let Some(tenure) = calculated_work_tenure {
            Some(tenure)
        } else {
            if !work_tenure_raw.is_empty() {
                work_tenure_raw.parse::<f32>().ok()
            } else {
                None
            }
        }, // 工龄(年)
        remarks: Some(remarks), // 备注
        major: None, // 专业字段（未在Excel中使用）
        contact_date: None, // 联系日期字段（未在Excel中使用）
        special_date: None, // 特殊日期字段（未在Excel中使用）
    };
    
    println!("第{}行数据解析完成", row_index);
    Ok(cadre_info)
}

// ... [其他函数] ...

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
        
        // 写入表头 (按照新的列顺序)
        sheet_writer.append_row(row!["姓名", "性别", "部门", "科室", "职务1", "职务2", 
                      "入司日期", "任现职级时间", "任职时间", "试用期(年)", "试用期满到期提醒",
                      "身份证号", "出生日期", "年龄", "籍贯", "出生地", "民族", "司龄(年)", 
                      "参加工作时间", "工龄(年)", "专业技术职务", "最高学历", "全日制学历", 
                      "全日制毕业院校系及专业", "在职学历", "在职毕业院校系及专业", 
                      "政治面貌", "入党时间", "联系电话", "备注"])?;
        
        // 写入数据规范说明行
        sheet_writer.append_row(row!["", "男/女", "部门名称", "科室名称", "职务名称", "职务名称",
                      "YYYY-MM-DD", "YYYY-MM-DD", "YYYY-MM-DD", "数字", "YYYY-MM-DD",
                      "18位身份证号", "YYYY-MM-DD", "数字", "地区", "地区", "民族名称", 
                      "数字", "YYYY-MM-DD", "数字", "职务名称", "学历名称", "学历名称", 
                      "学校专业", "学历名称", "学校专业", "政治面貌选项", "YYYY-MM-DD", 
                      "手机号码", "备注信息"])?;
        
        // 写入示例行
        sheet_writer.append_row(row!["张三", "男", "人力资源部", "招聘科", "部长", "副主任",
                      "2020-01-15", "2021-03-20", "2020-01-15", "1", "2021-01-15",
                      "110101199001011234", "1990-01-01", "34", "北京", "北京", "汉族", "4.5", 
                      "2010-07-01", "13.5", "高级工程师", "硕士研究生", "硕士研究生", 
                      "清华大学计算机系", "硕士研究生", "清华大学计算机系", "中共党员", 
                      "2015-06-15", "13800138000", "优秀员工"])?;
        
        // 写入政治面貌选项说明
        sheet_writer.append_row(row!["", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
                      "", "", "", "", "", "", "", "", "", 
                      "可选值: 中共党员/预备党员/共青团员/民革党员/民盟盟员/民建会员/民进会员/农工党党员/致公党党员/九三学社社员/台盟盟员/无党派人士/群众", "", "", ""])?;
        
        // 写入学历选项说明
        sheet_writer.append_row(row!["", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
                      "", "", "可选值: 博士研究生/硕士研究生/大学/大专/高中/中专/初中/职高", 
                      "可选值: 博士研究生/硕士研究生/大学/大专/高中/中专/初中/职高", "", 
                      "可选值: 博士研究生/硕士研究生/大学/大专/高中/中专/初中/职高", "", "", ""])?;
        
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

// ==================== Grassroots Cadres Commands ====================

#[tauri::command]
fn add_grassroots_cadre_info(db: State<'_, Mutex<Database>>, cadre: GrassrootsCadreInfo) -> Result<(), String> {
    println!("开始执行add_grassroots_cadre_info命令");
    println!("接收到的基层干部信息: {:?}", cadre);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.add_grassroots_cadre(&cadre) {
                Ok(result) => {
                    println!("添加基层干部信息成功，影响行数: {}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("添加基层干部信息失败: {}", e);
                    Err(format!("添加基层干部信息失败: {}", e))
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
fn get_all_grassroots_cadre_info(db: State<'_, Mutex<Database>>) -> Result<Vec<GrassrootsCadreInfo>, String> {
    println!("开始执行get_all_grassroots_cadre_info命令");
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.get_all_grassroots_cadres() {
                Ok(cadres) => {
                    println!("获取所有基层干部信息成功，共{}条记录", cadres.len());
                    Ok(cadres)
                },
                Err(e) => {
                    eprintln!("获取所有基层干部信息失败: {}", e);
                    Err(format!("获取所有基层干部信息失败: {}", e))
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
fn get_grassroots_cadre_info_by_id(db: State<'_, Mutex<Database>>, id: i32) -> Result<Option<GrassrootsCadreInfo>, String> {
    println!("开始执行get_grassroots_cadre_info_by_id命令，ID: {}", id);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.get_grassroots_cadre_by_id(id) {
                Ok(cadre) => {
                    println!("根据ID获取基层干部信息完成");
                    Ok(cadre)
                },
                Err(e) => {
                    eprintln!("根据ID获取基层干部信息失败: {}", e);
                    Err(format!("根据ID获取基层干部信息失败: {}", e))
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
fn update_grassroots_cadre_info(db: State<'_, Mutex<Database>>, cadre: GrassrootsCadreInfo) -> Result<(), String> {
    println!("开始执行update_grassroots_cadre_info命令");
    println!("接收到的基层干部信息: {:?}", cadre);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.update_grassroots_cadre(&cadre) {
                Ok(result) => {
                    println!("更新基层干部信息成功，影响行数: {}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("更新基层干部信息失败: {}", e);
                    Err(format!("更新基层干部信息失败: {}", e))
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
fn delete_grassroots_cadre_info(db: State<'_, Mutex<Database>>, id: i32) -> Result<(), String> {
    println!("开始执行delete_grassroots_cadre_info命令，ID: {}", id);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.delete_grassroots_cadre(id) {
                Ok(result) => {
                    println!("删除基层干部信息成功，影响行数: {}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("删除基层干部信息失败: {}", e);
                    Err(format!("删除基层干部信息失败: {}", e))
                }
            }
        },
        Err(e) => {
            eprintln!("获取数据库锁失败: {}", e);
            Err(format!("获取数据库锁失败: {}", e))
        }
    }
}

// ==================== Mid-Level Cadres Commands ====================

#[tauri::command]
fn add_midlevel_cadre_info(db: State<'_, Mutex<Database>>, cadre: MidLevelCadreInfo) -> Result<(), String> {
    println!("开始执行add_midlevel_cadre_info命令");
    println!("接收到的中层干部信息: {:?}", cadre);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.add_midlevel_cadre(&cadre) {
                Ok(result) => {
                    println!("添加中层干部信息成功，影响行数: {}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("添加中层干部信息失败: {}", e);
                    Err(format!("添加中层干部信息失败: {}", e))
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
fn get_all_midlevel_cadre_info(db: State<'_, Mutex<Database>>) -> Result<Vec<MidLevelCadreInfo>, String> {
    println!("开始执行get_all_midlevel_cadre_info命令");
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.get_all_midlevel_cadres() {
                Ok(cadres) => {
                    println!("获取所有中层干部信息成功，共{}条记录", cadres.len());
                    Ok(cadres)
                },
                Err(e) => {
                    eprintln!("获取所有中层干部信息失败: {}", e);
                    Err(format!("获取所有中层干部信息失败: {}", e))
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
fn get_midlevel_cadre_info_by_id(db: State<'_, Mutex<Database>>, id: i32) -> Result<Option<MidLevelCadreInfo>, String> {
    println!("开始执行get_midlevel_cadre_info_by_id命令，ID: {}", id);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.get_midlevel_cadre_by_id(id) {
                Ok(cadre) => {
                    println!("根据ID获取中层干部信息完成");
                    Ok(cadre)
                },
                Err(e) => {
                    eprintln!("根据ID获取中层干部信息失败: {}", e);
                    Err(format!("根据ID获取中层干部信息失败: {}", e))
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
fn update_midlevel_cadre_info(db: State<'_, Mutex<Database>>, cadre: MidLevelCadreInfo) -> Result<(), String> {
    println!("开始执行update_midlevel_cadre_info命令");
    println!("接收到的中层干部信息: {:?}", cadre);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.update_midlevel_cadre(&cadre) {
                Ok(result) => {
                    println!("更新中层干部信息成功，影响行数: {}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("更新中层干部信息失败: {}", e);
                    Err(format!("更新中层干部信息失败: {}", e))
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
fn delete_midlevel_cadre_info(db: State<'_, Mutex<Database>>, id: i32) -> Result<(), String> {
    println!("开始执行delete_midlevel_cadre_info命令，ID: {}", id);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.delete_midlevel_cadre(id) {
                Ok(result) => {
                    println!("删除中层干部信息成功，影响行数: {}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("删除中层干部信息失败: {}", e);
                    Err(format!("删除中层干部信息失败: {}", e))
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
fn get_distinct_field_values_for_table(db: State<'_, Mutex<Database>>, table_name: String, field_name: String) -> Result<Vec<String>, String> {
    println!("开始执行get_distinct_field_values_for_table命令，表名: {}, 字段名: {}", table_name, field_name);
    
    match db.lock() {
        Ok(db_guard) => {
            println!("获取数据库锁成功");
            match db_guard.get_distinct_field_values(&table_name, &field_name) {
                Ok(values) => {
                    println!("获取表{}字段{}的distinct值成功，共{}个值", table_name, field_name, values.len());
                    Ok(values)
                },
                Err(e) => {
                    eprintln!("获取表{}字段{}的distinct值失败: {}", table_name, field_name, e);
                    Err(format!("获取表{}字段{}的distinct值失败: {}", table_name, field_name, e))
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
            get_distinct_field_values_for_table
        ])
        .run(tauri::generate_context!())
        .map_err(|e| {
            eprintln!("Tauri应用运行失败: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })
}