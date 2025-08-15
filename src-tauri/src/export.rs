use simple_excel_writer::*;
use std::io;

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

pub fn export_to_excel(config: ExportConfig) -> Result<(), io::Error> {
    let mut workbook = Workbook::create(&config.file_path);
    let mut sheet = workbook.create_sheet("干部信息");
    
    workbook.write_sheet(&mut sheet, |sheet_writer| {
        // 写入字段名作为列标题
        let mut header_row = Row::new();
        for field in &config.fields {
            header_row.add_cell(field.clone());
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

