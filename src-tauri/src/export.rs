use simple_excel_writer::*;
use std::io;
use std::collections::HashMap;
use once_cell::sync::Lazy;

// 静态HashMap存储字段名和中文标签的映射
static FIELD_LABELS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    [
        ("id", "ID"),
        ("serial_number", "序号"),
        ("name", "姓名"),
        ("gender", "性别"),
        ("department", "部门"),
        ("section", "科室"),
        ("position1", "职务1"),
        ("position2", "职务2"),
        ("company_entry_date", "入司日期"),
        ("company_tenure", "司龄（年）"),
        ("work_start_date", "参加工作时间"),
        ("work_tenure", "工龄(年)"),
        ("current_level_date", "任现职级时间"),
        ("position_entry_date", "任职时间"),
        ("probation_period", "试用期"),
        ("probation_end_reminder", "试用期满到期提醒"),
        ("id_number", "身份证号"),
        ("birth_date", "出生日期"),
        ("age", "年龄"),
        ("native_place", "籍贯"),
        ("birth_place", "出生地"),
        ("ethnicity", "民族"),
        ("technical_position", "专业技术职务"),
        ("education", "学历"),
        ("full_time_education", "全日制学历"),
        ("full_time_school_major", "全日制毕业院校系及专业"),
        ("part_time_education", "在职学历"),
        ("part_time_school_phone", "在职毕业院校系及专业"),
        ("political_status", "政治面貌"),
        ("party_entry_date", "入党时间"),
        ("phone", "联系电话"),
        ("remarks", "备注"),
        ("major", "专业"),
        ("contact_date", "联系日期"),
        ("special_date", "特殊日期"),
        // 新增的字段映射
        ("grassroots_vice_position_date", "任基层副职时间"),
        ("grassroots_vice_tenure", "任基层副职年限"),
        ("grassroots_chief_position_date", "任基层正职时间"),
        ("grassroots_chief_tenure", "任基层正职年限"),
        ("midlevel_assistant_date", "任中层助理时间"),
        ("midlevel_assistant_tenure", "任中层助理年限"),
        ("midlevel_vice_date", "任中层副职时间"),
        ("midlevel_vice_tenure", "任中层副职年限"),
        ("midlevel_chief_date", "任中层正职时间"),
        ("midlevel_chief_tenure", "任中层正职年限"),
        ("same_department_date", "同部门任职时间"),
        ("same_department_tenure", "同部门任职年限"),
    ].iter().cloned().collect()
});

pub struct ExportConfig {
    pub file_path: String,
    pub fields: Vec<String>,       // 要导出的字段名
    pub data: Vec<Vec<String>>,    // 数据，每一行是一个记录
}

impl ExportConfig {
    pub fn new(file_path: String) -> Self {
        ExportConfig {
            file_path,
            fields: Vec::new(),
            data: Vec::new(),
        }
    }
    
    pub fn set_fields(&mut self, fields: Vec<String>) {
        self.fields = fields;
    }
    
    pub fn add_data_row(&mut self, row: Vec<String>) {
        self.data.push(row);
    }
}

// 获取字段名对应的中文标题
fn get_field_label(field: &str) -> String {
    FIELD_LABELS.get(field).unwrap_or(&field).to_string()
}

pub fn export_to_excel(config: ExportConfig) -> Result<(), io::Error> {
    let mut workbook = Workbook::create(&config.file_path);
    let mut sheet = workbook.create_sheet("干部信息");
    
    workbook.write_sheet(&mut sheet, |sheet_writer| {
        // 写入字段名作为列标题（使用中文标题）
        let mut header_row = Row::new();
        for field in &config.fields {
            let label = get_field_label(field);
            header_row.add_cell(label);
        }
        sheet_writer.append_row(header_row)?;
        
        // 写入数据
        for data_row in config.data {
            let mut row = Row::new();
            for cell_data in data_row {
                row.add_cell(cell_data);
            }
            sheet_writer.append_row(row)?;
        }
        
        Ok(())
    })?;
    
    workbook.close()?;
    Ok(())
}

