use rusqlite::{Connection, Result, params};
use std::fs;
use crate::filter::FilterParams;

pub struct Database {
    conn: Connection,
}

// 基层管理人员信息结构体
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GrassrootsCadreInfo {
    pub id: Option<i32>,
    pub serial_number: Option<String>,
    pub name: String,
    pub gender: Option<String>,
    pub department: Option<String>,
    pub section: Option<String>, // 科室字段
    pub position1: Option<String>,
    pub position2: Option<String>,
    pub company_entry_date: Option<String>,
    pub company_tenure: Option<f32>,
    pub work_start_date: Option<String>,
    pub work_tenure: Option<f32>,
    pub current_level_date: Option<String>,
    pub position_entry_date: Option<String>,
    pub probation_period: Option<String>,
    pub probation_end_reminder: Option<String>,
    pub id_number: Option<String>,
    pub birth_date: Option<String>,
    pub age: Option<i32>,
    pub native_place: Option<String>,
    pub birth_place: Option<String>,
    pub ethnicity: Option<String>,
    pub technical_position: Option<String>,
    pub education: Option<String>,
    pub full_time_education: Option<String>,
    pub full_time_school_major: Option<String>,
    pub part_time_education: Option<String>,
    pub part_time_school_phone: Option<String>,
    pub political_status: Option<String>,
    pub party_entry_date: Option<String>,
    pub phone: Option<String>,
    pub grassroots_vice_position_date: Option<String>, // 任基层副职时间
    pub grassroots_vice_tenure: Option<String>, // 任基层副职年限 (改为文本类型)
    pub grassroots_chief_position_date: Option<String>, // 任基层正职时间
    pub grassroots_chief_tenure: Option<String>, // 任基层正职年限 (改为文本类型)
    pub midlevel_assistant_date: Option<String>, // 任中层助理层级时间
    pub midlevel_assistant_tenure: Option<String>, // 任中层助理年限
    pub midlevel_vice_date: Option<String>, // 任中层副职时间
    pub midlevel_vice_tenure: Option<String>, // 任中层副职年限
    pub midlevel_chief_date: Option<String>, // 任中层正职时间
    pub midlevel_chief_tenure: Option<String>, // 任中层正职年限
    pub same_department_date: Option<String>, // 同部门任职时间
    pub same_department_tenure: Option<String>, // 同部门任职年限
    pub remarks: Option<String>,
    pub major: Option<String>,
    pub contact_date: Option<String>,
    pub special_date: Option<String>,
}

// 中层管理人员信息结构体 (比基层少一个科室字段)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MidLevelCadreInfo {
    pub id: Option<i32>,
    pub serial_number: Option<String>,
    pub name: String,
    pub gender: Option<String>,
    pub department: Option<String>,
    // 注意：这里没有 section (科室) 字段
    pub position1: Option<String>,
    pub position2: Option<String>,
    pub company_entry_date: Option<String>,
    pub company_tenure: Option<f32>,
    pub work_start_date: Option<String>,
    pub work_tenure: Option<f32>,
    pub current_level_date: Option<String>,
    pub position_entry_date: Option<String>,
    pub probation_period: Option<String>,
    pub probation_end_reminder: Option<String>,
    pub id_number: Option<String>,
    pub birth_date: Option<String>,
    pub age: Option<i32>,
    pub native_place: Option<String>,
    pub birth_place: Option<String>,
    pub ethnicity: Option<String>,
    pub technical_position: Option<String>,
    pub education: Option<String>,
    pub full_time_education: Option<String>,
    pub full_time_school_major: Option<String>,
    pub part_time_education: Option<String>,
    pub part_time_school_phone: Option<String>,
    pub political_status: Option<String>,
    pub party_entry_date: Option<String>,
    pub phone: Option<String>,
    pub grassroots_vice_position_date: Option<String>, // 任基层副职时间
    pub grassroots_vice_tenure: Option<String>, // 任基层副职年限 (改为文本类型)
    pub grassroots_chief_position_date: Option<String>, // 任基层正职时间
    pub grassroots_chief_tenure: Option<String>, // 任基层正职年限 (改为文本类型)
    pub midlevel_assistant_date: Option<String>, // 任中层助理层级时间
    pub midlevel_assistant_tenure: Option<String>, // 任中层助理年限
    pub midlevel_vice_date: Option<String>, // 任中层副职时间
    pub midlevel_vice_tenure: Option<String>, // 任中层副职年限
    pub midlevel_chief_date: Option<String>, // 任中层正职时间
    pub midlevel_chief_tenure: Option<String>, // 任中层正职年限
    pub same_department_date: Option<String>, // 同部门任职时间
    pub same_department_tenure: Option<String>, // 同部门任职年限
    pub remarks: Option<String>,
    pub major: Option<String>,
    pub contact_date: Option<String>,
    pub special_date: Option<String>,
}

impl Database {
    pub fn new() -> Result<Self> {
        // Create the database directory if it doesn't exist
        let db_dir = "data";
        if let Err(e) = fs::metadata(db_dir) {
            if e.kind() == std::io::ErrorKind::NotFound {
                fs::create_dir(db_dir).map_err(|e| rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(1), 
                    Some(format!("Failed to create data directory: {}", e))
                ))?;
            } else {
                return Err(rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(1), 
                    Some(format!("Failed to check data directory: {}", e))
                ));
            }
        }
        
        let conn = Connection::open("data/cadre.db")?;
        
        // Create the grassroots_cadres table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS grassroots_cadres (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                serial_number TEXT,
                name TEXT NOT NULL,
                gender TEXT,
                department TEXT,
                section TEXT,
                position1 TEXT,
                position2 TEXT,
                company_entry_date TEXT,
                company_tenure REAL,
                work_start_date TEXT,
                work_tenure REAL,
                current_level_date TEXT,
                position_entry_date TEXT,
                probation_period TEXT,
                probation_end_reminder TEXT,
                id_number TEXT UNIQUE,
                birth_date TEXT,
                age INTEGER,
                native_place TEXT,
                birth_place TEXT,
                ethnicity TEXT,
                technical_position TEXT,
                education TEXT,
                full_time_education TEXT,
                full_time_school_major TEXT,
                part_time_education TEXT,
                part_time_school_phone TEXT,
                political_status TEXT,
                party_entry_date TEXT,
                phone TEXT,
                grassroots_vice_position_date TEXT,
                grassroots_vice_tenure TEXT,
                grassroots_chief_position_date TEXT,
                grassroots_chief_tenure TEXT,
                midlevel_assistant_date TEXT,
                midlevel_assistant_tenure TEXT,
                midlevel_vice_date TEXT,
                midlevel_vice_tenure TEXT,
                midlevel_chief_date TEXT,
                midlevel_chief_tenure TEXT,
                same_department_date TEXT,
                same_department_tenure TEXT,
                remarks TEXT,
                major TEXT,
                contact_date TEXT,
                special_date TEXT
            )",
            [],
        )?;
        
        // Create the midlevel_cadres table if it doesn't exist (without section field)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS midlevel_cadres (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                serial_number TEXT,
                name TEXT NOT NULL,
                gender TEXT,
                department TEXT,
                position1 TEXT,
                position2 TEXT,
                company_entry_date TEXT,
                company_tenure REAL,
                work_start_date TEXT,
                work_tenure REAL,
                current_level_date TEXT,
                position_entry_date TEXT,
                probation_period TEXT,
                probation_end_reminder TEXT,
                id_number TEXT UNIQUE,
                birth_date TEXT,
                age INTEGER,
                native_place TEXT,
                birth_place TEXT,
                ethnicity TEXT,
                technical_position TEXT,
                education TEXT,
                full_time_education TEXT,
                full_time_school_major TEXT,
                part_time_education TEXT,
                part_time_school_phone TEXT,
                political_status TEXT,
                party_entry_date TEXT,
                phone TEXT,
                grassroots_vice_position_date TEXT,
                grassroots_vice_tenure TEXT,
                grassroots_chief_position_date TEXT,
                grassroots_chief_tenure TEXT,
                midlevel_assistant_date TEXT,
                midlevel_assistant_tenure TEXT,
                midlevel_vice_date TEXT,
                midlevel_vice_tenure TEXT,
                midlevel_chief_date TEXT,
                midlevel_chief_tenure TEXT,
                same_department_date TEXT,
                same_department_tenure TEXT,
                remarks TEXT,
                major TEXT,
                contact_date TEXT,
                special_date TEXT
            )",
            [],
        )?;
        
        // Handle database migration - add part_time_school_phone column if it doesn't exist
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN part_time_school_phone TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN part_time_school_phone TEXT", []);
        
        // Handle database migration - add birth_date column if it doesn't exist
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN birth_date TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN birth_date TEXT", []);
        
        // Handle database migration - add new columns for grassroots cadre information
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN grassroots_vice_position_date TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN grassroots_vice_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN grassroots_chief_position_date TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN grassroots_chief_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN midlevel_assistant_date TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN midlevel_assistant_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN midlevel_vice_date TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN midlevel_vice_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN midlevel_chief_date TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN midlevel_chief_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN same_department_date TEXT", []);
        let _ = conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN same_department_tenure TEXT", []);
        
        // Handle database migration - add new columns for midlevel cadre information
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN grassroots_vice_position_date TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN grassroots_vice_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN grassroots_chief_position_date TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN grassroots_chief_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN midlevel_assistant_date TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN midlevel_assistant_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN midlevel_vice_date TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN midlevel_vice_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN midlevel_chief_date TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN midlevel_chief_tenure TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN same_department_date TEXT", []);
        let _ = conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN same_department_tenure TEXT", []);
        
        Ok(Database { conn })
    }
    
    // ==================== Grassroots Cadres Methods ====================
    
    pub fn add_grassroots_cadre(&self, cadre: &GrassrootsCadreInfo) -> Result<usize> {
        // 添加详细的调试信息
        eprintln!("DEBUG: Adding grassroots cadre - name: {}", cadre.name);
        eprintln!("DEBUG: Field count in INSERT statement: 46");
        eprintln!("DEBUG: Parameter count should be: 46");
        
        // 打印一些关键字段的值
        eprintln!("DEBUG: cadre.serial_number: {:?}", cadre.serial_number);
        eprintln!("DEBUG: cadre.name: {}", cadre.name);
        eprintln!("DEBUG: cadre.gender: {:?}", cadre.gender);
        eprintln!("DEBUG: cadre.department: {:?}", cadre.department);
        eprintln!("DEBUG: cadre.section: {:?}", cadre.section);
        eprintln!("DEBUG: cadre.position1: {:?}", cadre.position1);
        eprintln!("DEBUG: cadre.position2: {:?}", cadre.position2);
        eprintln!("DEBUG: cadre.company_entry_date: {:?}", cadre.company_entry_date);
        eprintln!("DEBUG: cadre.same_department_date: {:?}", cadre.same_department_date);
        eprintln!("DEBUG: cadre.same_department_tenure: {:?}", cadre.same_department_tenure);
        eprintln!("DEBUG: cadre.remarks: {:?}", cadre.remarks);
        eprintln!("DEBUG: cadre.major: {:?}", cadre.major);
        eprintln!("DEBUG: cadre.contact_date: {:?}", cadre.contact_date);
        eprintln!("DEBUG: cadre.special_date: {:?}", cadre.special_date);
        
        let result = self.conn.execute(
            "INSERT INTO grassroots_cadres (
                serial_number, name, gender, department, section, position1, position2,
                company_entry_date, company_tenure, work_start_date, work_tenure,
                current_level_date, position_entry_date, probation_period,
                probation_end_reminder, id_number, birth_date, age, native_place, birth_place,
                ethnicity, technical_position, education, full_time_education,
                full_time_school_major, part_time_education, part_time_school_phone,
                political_status, party_entry_date, phone, grassroots_vice_position_date,
                grassroots_vice_tenure, grassroots_chief_position_date, grassroots_chief_tenure,
                midlevel_assistant_date, midlevel_assistant_tenure, midlevel_vice_date,
                midlevel_vice_tenure, midlevel_chief_date, midlevel_chief_tenure,
                same_department_date, same_department_tenure, remarks, major, contact_date, special_date
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
                      ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29,
                      ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40, ?41, ?42, ?43, ?44, ?45, ?46)",
            params![
                cadre.serial_number, &cadre.name, cadre.gender, cadre.department, cadre.section,
                cadre.position1, cadre.position2, cadre.company_entry_date, cadre.company_tenure,
                cadre.work_start_date, cadre.work_tenure, cadre.current_level_date,
                cadre.position_entry_date, cadre.probation_period, cadre.probation_end_reminder,
                cadre.id_number, cadre.birth_date, cadre.age, cadre.native_place, cadre.birth_place,
                cadre.ethnicity, cadre.technical_position, cadre.education, cadre.full_time_education,
                cadre.full_time_school_major, cadre.part_time_education, cadre.part_time_school_phone,
                cadre.political_status, cadre.party_entry_date, cadre.phone, cadre.grassroots_vice_position_date,
                cadre.grassroots_vice_tenure, cadre.grassroots_chief_position_date, cadre.grassroots_chief_tenure,
                cadre.midlevel_assistant_date, cadre.midlevel_assistant_tenure, cadre.midlevel_vice_date,
                cadre.midlevel_vice_tenure, cadre.midlevel_chief_date, cadre.midlevel_chief_tenure,
                cadre.same_department_date, cadre.same_department_tenure, cadre.remarks, cadre.major, cadre.contact_date, cadre.special_date
            ],
        );
        
        match result {
            Ok(rows_affected) => {
                eprintln!("DEBUG: Grassroots cadre added successfully, rows affected: {}", rows_affected);
                Ok(rows_affected)
            },
            Err(e) => {
                eprintln!("DEBUG: Error adding grassroots cadre: {}", e);
                Err(e)
            },
        }
    }
    
    pub fn get_all_grassroots_cadres(&self) -> Result<Vec<GrassrootsCadreInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, serial_number, name, gender, department, section, position1, position2,
                    company_entry_date, company_tenure, work_start_date, work_tenure,
                    current_level_date, position_entry_date, probation_period,
                    probation_end_reminder, id_number, birth_date, age, native_place, birth_place,
                    ethnicity, technical_position, education, full_time_education,
                    full_time_school_major, part_time_education, part_time_school_phone,
                    political_status, party_entry_date, phone, grassroots_vice_position_date,
                    grassroots_vice_tenure, grassroots_chief_position_date, grassroots_chief_tenure,
                    midlevel_assistant_date, midlevel_assistant_tenure, midlevel_vice_date,
                    midlevel_vice_tenure, midlevel_chief_date, midlevel_chief_tenure,
                    same_department_date, same_department_tenure, remarks, major, contact_date, special_date
             FROM grassroots_cadres"
        )?;
        
        let cadre_iter = stmt.query_map([], |row| {
            Ok(GrassrootsCadreInfo {
                id: row.get(0)?,
                serial_number: row.get(1)?,
                name: row.get(2)?,
                gender: row.get(3)?,
                department: row.get(4)?,
                section: row.get(5)?,
                position1: row.get(6)?,
                position2: row.get(7)?,
                company_entry_date: row.get(8)?,
                company_tenure: row.get(9)?,
                work_start_date: row.get(10)?,
                work_tenure: row.get(11)?,
                current_level_date: row.get(12)?,
                position_entry_date: row.get(13)?,
                probation_period: row.get(14)?,
                probation_end_reminder: row.get(15)?,
                id_number: row.get(16)?,
                birth_date: row.get(17)?,
                age: row.get(18)?,
                native_place: row.get(19)?,
                birth_place: row.get(20)?,
                ethnicity: row.get(21)?,
                technical_position: row.get(22)?,
                education: row.get(23)?,
                full_time_education: row.get(24)?,
                full_time_school_major: row.get(25)?,
                part_time_education: row.get(26)?,
                part_time_school_phone: row.get(27)?,
                political_status: row.get(28)?,
                party_entry_date: row.get(29)?,
                phone: row.get(30)?,
                grassroots_vice_position_date: row.get(31)?,
                grassroots_vice_tenure: row.get(32)?,
                grassroots_chief_position_date: row.get(33)?,
                grassroots_chief_tenure: row.get(34)?,
                midlevel_assistant_date: row.get(35)?,
                midlevel_assistant_tenure: {
                    // Handle both TEXT and REAL types for backward compatibility
                    match row.get::<_, Option<String>>(36) {
                        Ok(val) => val,
                        Err(_) => {
                            // If it's stored as REAL, convert it to String
                            match row.get::<_, Option<f32>>(36) {
                                Ok(val) => val.map(|v| v.to_string()),
                                Err(_) => None
                            }
                        }
                    }
                },
                midlevel_vice_date: row.get(37)?,
                midlevel_vice_tenure: row.get(38)?,
                midlevel_chief_date: row.get(39)?,
                midlevel_chief_tenure: row.get(40)?,
                same_department_date: row.get(41)?,
                same_department_tenure: row.get(42)?,
                remarks: row.get(43)?,
                major: row.get(44)?,
                contact_date: row.get(44)?,
                special_date: row.get(45)?,
            })
        })?;
        
        let mut cadres = Vec::new();
        for cadre in cadre_iter {
            cadres.push(cadre?);
        }
        
        Ok(cadres)
    }
    
    pub fn get_grassroots_cadre_by_id(&self, id: i32) -> Result<Option<GrassrootsCadreInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, serial_number, name, gender, department, section, position1, position2,
                    company_entry_date, company_tenure, work_start_date, work_tenure,
                    current_level_date, position_entry_date, probation_period,
                    probation_end_reminder, id_number, birth_date, age, native_place, birth_place,
                    ethnicity, technical_position, education, full_time_education,
                    full_time_school_major, part_time_education, part_time_school_phone,
                    political_status, party_entry_date, phone, grassroots_vice_position_date,
                    grassroots_vice_tenure, grassroots_chief_position_date, grassroots_chief_tenure,
                    midlevel_assistant_date, midlevel_assistant_tenure, midlevel_vice_date,
                    midlevel_vice_tenure, midlevel_chief_date, midlevel_chief_tenure,
                    same_department_date, same_department_tenure, remarks, major, contact_date, special_date
             FROM grassroots_cadres WHERE id = ?1"
        )?;
        
        let mut cadre_iter = stmt.query_map([id], |row| {
            Ok(GrassrootsCadreInfo {
                id: row.get(0)?,
                serial_number: row.get(1)?,
                name: row.get(2)?,
                gender: row.get(3)?,
                department: row.get(4)?,
                section: row.get(5)?,
                position1: row.get(6)?,
                position2: row.get(7)?,
                company_entry_date: row.get(8)?,
                company_tenure: row.get(9)?,
                work_start_date: row.get(10)?,
                work_tenure: row.get(11)?,
                current_level_date: row.get(12)?,
                position_entry_date: row.get(13)?,
                probation_period: row.get(14)?,
                probation_end_reminder: row.get(15)?,
                id_number: row.get(16)?,
                birth_date: row.get(17)?,
                age: row.get(18)?,
                native_place: row.get(19)?,
                birth_place: row.get(20)?,
                ethnicity: row.get(21)?,
                technical_position: row.get(22)?,
                education: row.get(23)?,
                full_time_education: row.get(24)?,
                full_time_school_major: row.get(25)?,
                part_time_education: row.get(26)?,
                part_time_school_phone: row.get(27)?,
                political_status: row.get(28)?,
                party_entry_date: row.get(29)?,
                phone: row.get(30)?,
                grassroots_vice_position_date: row.get(31)?,
                grassroots_vice_tenure: row.get(32)?,
                grassroots_chief_position_date: row.get(33)?,
                grassroots_chief_tenure: row.get(34)?,
                midlevel_assistant_date: row.get(35)?,
                midlevel_assistant_tenure: {
                    // Handle both TEXT and REAL types for backward compatibility
                    match row.get::<_, Option<String>>(36) {
                        Ok(val) => val,
                        Err(_) => {
                            // If it's stored as REAL, convert it to String
                            match row.get::<_, Option<f32>>(36) {
                                Ok(val) => val.map(|v| v.to_string()),
                                Err(_) => None
                            }
                        }
                    }
                },
                midlevel_vice_date: row.get(37)?,
                midlevel_vice_tenure: row.get(38)?,
                midlevel_chief_date: row.get(39)?,
                midlevel_chief_tenure: row.get(40)?,
                same_department_date: row.get(41)?,
                same_department_tenure: row.get(42)?,
                remarks: row.get(43)?,
                major: row.get(44)?,
                contact_date: row.get(44)?,
                special_date: row.get(45)?,
            })
        })?;
        
        if let Some(cadre) = cadre_iter.next() {
            Ok(Some(cadre?))
        } else {
            Ok(None)
        }
    }
    
    pub fn update_grassroots_cadre(&self, cadre: &GrassrootsCadreInfo) -> Result<usize> {
        self.conn.execute(
            "UPDATE grassroots_cadres SET
                serial_number = ?1, name = ?2, gender = ?3, department = ?4, section = ?5,
                position1 = ?6, position2 = ?7, company_entry_date = ?8, company_tenure = ?9,
                work_start_date = ?10, work_tenure = ?11, current_level_date = ?12,
                position_entry_date = ?13, probation_period = ?14, probation_end_reminder = ?15,
                id_number = ?16, birth_date = ?17, age = ?18, native_place = ?19, birth_place = ?20,
                ethnicity = ?21, technical_position = ?22, education = ?23, full_time_education = ?24,
                full_time_school_major = ?25, part_time_education = ?26, part_time_school_phone = ?27,
                political_status = ?28, party_entry_date = ?29, phone = ?30,
                grassroots_vice_position_date = ?31, grassroots_vice_tenure = ?32,
                grassroots_chief_position_date = ?33, grassroots_chief_tenure = ?34,
                midlevel_assistant_date = ?35, midlevel_assistant_tenure = ?36,
                midlevel_vice_date = ?37, midlevel_vice_tenure = ?38,
                midlevel_chief_date = ?39, midlevel_chief_tenure = ?40,
                same_department_date = ?41, same_department_tenure = ?42, remarks = ?43, major = ?44, contact_date = ?45,
                special_date = ?46
             WHERE id = ?47",
            params![
                cadre.serial_number, &cadre.name, cadre.gender, cadre.department, cadre.section,
                cadre.position1, cadre.position2, cadre.company_entry_date, cadre.company_tenure,
                cadre.work_start_date, cadre.work_tenure, cadre.current_level_date,
                cadre.position_entry_date, cadre.probation_period, cadre.probation_end_reminder,
                cadre.id_number, cadre.birth_date, cadre.age, cadre.native_place, cadre.birth_place,
                cadre.ethnicity, cadre.technical_position, cadre.education, cadre.full_time_education,
                cadre.full_time_school_major, cadre.part_time_education, cadre.part_time_school_phone,
                cadre.political_status, cadre.party_entry_date, cadre.phone,
                cadre.grassroots_vice_position_date, cadre.grassroots_vice_tenure,
                cadre.grassroots_chief_position_date, cadre.grassroots_chief_tenure,
                cadre.midlevel_assistant_date, cadre.midlevel_assistant_tenure,
                cadre.midlevel_vice_date, cadre.midlevel_vice_tenure,
                cadre.midlevel_chief_date, cadre.midlevel_chief_tenure,
                cadre.same_department_date, cadre.same_department_tenure, cadre.remarks, cadre.major, cadre.contact_date,
                cadre.special_date, cadre.id
            ],
        )
    }
    
    pub fn delete_grassroots_cadre(&self, id: i32) -> Result<usize> {
        self.conn.execute("DELETE FROM grassroots_cadres WHERE id = ?1", [id])
    }
    
    // ==================== Mid-Level Cadres Methods ====================
    
    pub fn add_midlevel_cadre(&self, cadre: &MidLevelCadreInfo) -> Result<usize> {
        // 添加详细的调试信息
        eprintln!("DEBUG: Adding midlevel cadre - name: {}", cadre.name);
        eprintln!("DEBUG: Field count in INSERT statement: 45");
        eprintln!("DEBUG: Parameter count should be: 45");
        
        // 打印一些关键字段的值
        eprintln!("DEBUG: cadre.serial_number: {:?}", cadre.serial_number);
        eprintln!("DEBUG: cadre.name: {}", cadre.name);
        eprintln!("DEBUG: cadre.gender: {:?}", cadre.gender);
        eprintln!("DEBUG: cadre.department: {:?}", cadre.department);
        eprintln!("DEBUG: cadre.position1: {:?}", cadre.position1);
        eprintln!("DEBUG: cadre.position2: {:?}", cadre.position2);
        eprintln!("DEBUG: cadre.company_entry_date: {:?}", cadre.company_entry_date);
        eprintln!("DEBUG: cadre.same_department_date: {:?}", cadre.same_department_date);
        eprintln!("DEBUG: cadre.same_department_tenure: {:?}", cadre.same_department_tenure);
        eprintln!("DEBUG: cadre.remarks: {:?}", cadre.remarks);
        eprintln!("DEBUG: cadre.major: {:?}", cadre.major);
        eprintln!("DEBUG: cadre.contact_date: {:?}", cadre.contact_date);
        eprintln!("DEBUG: cadre.special_date: {:?}", cadre.special_date);
        
        let result = self.conn.execute(
            "INSERT INTO midlevel_cadres (
                serial_number, name, gender, department, position1, position2,
                company_entry_date, company_tenure, work_start_date, work_tenure,
                current_level_date, position_entry_date, probation_period,
                probation_end_reminder, id_number, birth_date, age, native_place, birth_place,
                ethnicity, technical_position, education, full_time_education,
                full_time_school_major, part_time_education, part_time_school_phone,
                political_status, party_entry_date, phone, grassroots_vice_position_date,
                grassroots_vice_tenure, grassroots_chief_position_date, grassroots_chief_tenure,
                midlevel_assistant_date, midlevel_assistant_tenure, midlevel_vice_date,
                midlevel_vice_tenure, midlevel_chief_date, midlevel_chief_tenure,
                same_department_date, same_department_tenure, remarks, major, contact_date, special_date
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
                      ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29,
                      ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40, ?41, ?42, ?43, ?44, ?45)",
            params![
                cadre.serial_number, &cadre.name, cadre.gender, cadre.department,
                cadre.position1, cadre.position2, cadre.company_entry_date, cadre.company_tenure,
                cadre.work_start_date, cadre.work_tenure, cadre.current_level_date,
                cadre.position_entry_date, cadre.probation_period, cadre.probation_end_reminder,
                cadre.id_number, cadre.birth_date, cadre.age, cadre.native_place, cadre.birth_place,
                cadre.ethnicity, cadre.technical_position, cadre.education, cadre.full_time_education,
                cadre.full_time_school_major, cadre.part_time_education, cadre.part_time_school_phone,
                cadre.political_status, cadre.party_entry_date, cadre.phone, cadre.grassroots_vice_position_date,
                cadre.grassroots_vice_tenure, cadre.grassroots_chief_position_date, cadre.grassroots_chief_tenure,
                cadre.midlevel_assistant_date, cadre.midlevel_assistant_tenure, cadre.midlevel_vice_date,
                cadre.midlevel_vice_tenure, cadre.midlevel_chief_date, cadre.midlevel_chief_tenure,
                cadre.same_department_date, cadre.same_department_tenure, cadre.remarks, cadre.major, cadre.contact_date, cadre.special_date
            ],
        );
        
        match result {
            Ok(rows_affected) => {
                eprintln!("DEBUG: Midlevel cadre added successfully, rows affected: {}", rows_affected);
                Ok(rows_affected)
            },
            Err(e) => {
                eprintln!("DEBUG: Error adding midlevel cadre: {}", e);
                Err(e)
            },
        }
    }
    
    pub fn get_all_midlevel_cadres(&self) -> Result<Vec<MidLevelCadreInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, serial_number, name, gender, department, position1, position2,
                    company_entry_date, company_tenure, work_start_date, work_tenure,
                    current_level_date, position_entry_date, probation_period,
                    probation_end_reminder, id_number, birth_date, age, native_place, birth_place,
                    ethnicity, technical_position, education, full_time_education,
                    full_time_school_major, part_time_education, part_time_school_phone,
                    political_status, party_entry_date, phone, grassroots_vice_position_date,
                    grassroots_vice_tenure, grassroots_chief_position_date, grassroots_chief_tenure,
                    midlevel_assistant_date, midlevel_assistant_tenure, midlevel_vice_date,
                    midlevel_vice_tenure, midlevel_chief_date, midlevel_chief_tenure,
                    same_department_date, same_department_tenure, remarks, major, contact_date, special_date
             FROM midlevel_cadres"
        )?;
        
        let cadre_iter = stmt.query_map([], |row| {
            Ok(MidLevelCadreInfo {
                id: row.get(0)?,
                serial_number: row.get(1)?,
                name: row.get(2)?,
                gender: row.get(3)?,
                department: row.get(4)?,
                position1: row.get(5)?,
                position2: row.get(6)?,
                company_entry_date: row.get(7)?,
                company_tenure: row.get(8)?,
                work_start_date: row.get(9)?,
                work_tenure: row.get(10)?,
                current_level_date: row.get(11)?,
                position_entry_date: row.get(12)?,
                probation_period: row.get(13)?,
                probation_end_reminder: row.get(14)?,
                id_number: row.get(15)?,
                birth_date: row.get(16)?,
                age: row.get(17)?,
                native_place: row.get(18)?,
                birth_place: row.get(19)?,
                ethnicity: row.get(20)?,
                technical_position: row.get(21)?,
                education: row.get(22)?,
                full_time_education: row.get(23)?,
                full_time_school_major: row.get(24)?,
                part_time_education: row.get(25)?,
                part_time_school_phone: row.get(26)?,
                political_status: row.get(27)?,
                party_entry_date: row.get(28)?,
                phone: row.get(29)?,
                grassroots_vice_position_date: row.get(30)?,
                grassroots_vice_tenure: row.get(31)?,
                grassroots_chief_position_date: row.get(32)?,
                grassroots_chief_tenure: row.get(33)?,
                midlevel_assistant_date: row.get(34)?,
                midlevel_assistant_tenure: {
                    // Handle both TEXT and REAL types for backward compatibility
                    match row.get::<_, Option<String>>(35) {
                        Ok(val) => val,
                        Err(_) => {
                            // If it's stored as REAL, convert it to String
                            match row.get::<_, Option<f32>>(35) {
                                Ok(val) => val.map(|v| v.to_string()),
                                Err(_) => None
                            }
                        }
                    }
                },
                midlevel_vice_date: row.get(36)?,
                midlevel_vice_tenure: row.get(37)?,
                midlevel_chief_date: row.get(38)?,
                midlevel_chief_tenure: row.get(39)?,
                same_department_date: row.get(40)?,
                same_department_tenure: row.get(41)?,
                remarks: row.get(42)?,
                major: row.get(43)?,
                contact_date: row.get(44)?,
                special_date: row.get(45)?,
            })
        })?;
        
        let mut cadres = Vec::new();
        for cadre in cadre_iter {
            cadres.push(cadre?);
        }
        
        Ok(cadres)
    }
    
    pub fn get_midlevel_cadre_by_id(&self, id: i32) -> Result<Option<MidLevelCadreInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, serial_number, name, gender, department, position1, position2,
                    company_entry_date, company_tenure, work_start_date, work_tenure,
                    current_level_date, position_entry_date, probation_period,
                    probation_end_reminder, id_number, birth_date, age, native_place, birth_place,
                    ethnicity, technical_position, education, full_time_education,
                    full_time_school_major, part_time_education, part_time_school_phone,
                    political_status, party_entry_date, phone, grassroots_vice_position_date,
                    grassroots_vice_tenure, grassroots_chief_position_date, grassroots_chief_tenure,
                    midlevel_assistant_date, midlevel_assistant_tenure, midlevel_vice_date,
                    midlevel_vice_tenure, midlevel_chief_date, midlevel_chief_tenure,
                    same_department_date, same_department_tenure, remarks, major, contact_date, special_date
             FROM midlevel_cadres WHERE id = ?1"
        )?;
        
        let mut cadre_iter = stmt.query_map([id], |row| {
            Ok(MidLevelCadreInfo {
                id: row.get(0)?,
                serial_number: row.get(1)?,
                name: row.get(2)?,
                gender: row.get(3)?,
                department: row.get(4)?,
                position1: row.get(5)?,
                position2: row.get(6)?,
                company_entry_date: row.get(7)?,
                company_tenure: row.get(8)?,
                work_start_date: row.get(9)?,
                work_tenure: row.get(10)?,
                current_level_date: row.get(11)?,
                position_entry_date: row.get(12)?,
                probation_period: row.get(13)?,
                probation_end_reminder: row.get(14)?,
                id_number: row.get(15)?,
                birth_date: row.get(16)?,
                age: row.get(17)?,
                native_place: row.get(18)?,
                birth_place: row.get(19)?,
                ethnicity: row.get(20)?,
                technical_position: row.get(21)?,
                education: row.get(22)?,
                full_time_education: row.get(23)?,
                full_time_school_major: row.get(24)?,
                part_time_education: row.get(25)?,
                part_time_school_phone: row.get(26)?,
                political_status: row.get(27)?,
                party_entry_date: row.get(28)?,
                phone: row.get(29)?,
                grassroots_vice_position_date: row.get(30)?,
                grassroots_vice_tenure: row.get(31)?,
                grassroots_chief_position_date: row.get(32)?,
                grassroots_chief_tenure: row.get(33)?,
                midlevel_assistant_date: row.get(34)?,
                midlevel_assistant_tenure: {
                    // Handle both TEXT and REAL types for backward compatibility
                    match row.get::<_, Option<String>>(35) {
                        Ok(val) => val,
                        Err(_) => {
                            // If it's stored as REAL, convert it to String
                            match row.get::<_, Option<f32>>(35) {
                                Ok(val) => val.map(|v| v.to_string()),
                                Err(_) => None
                            }
                        }
                    }
                },
                midlevel_vice_date: row.get(36)?,
                midlevel_vice_tenure: row.get(37)?,
                midlevel_chief_date: row.get(38)?,
                midlevel_chief_tenure: row.get(39)?,
                same_department_date: row.get(40)?,
                same_department_tenure: row.get(41)?,
                remarks: row.get(42)?,
                major: row.get(43)?,
                contact_date: row.get(44)?,
                special_date: row.get(45)?,
            })
        })?;
        
        if let Some(cadre) = cadre_iter.next() {
            Ok(Some(cadre?))
        } else {
            Ok(None)
        }
    }
    
    pub fn update_midlevel_cadre(&self, cadre: &MidLevelCadreInfo) -> Result<usize> {
        self.conn.execute(
            "UPDATE midlevel_cadres SET
                serial_number = ?1, name = ?2, gender = ?3, department = ?4,
                position1 = ?5, position2 = ?6, company_entry_date = ?7, company_tenure = ?8,
                work_start_date = ?9, work_tenure = ?10, current_level_date = ?11,
                position_entry_date = ?12, probation_period = ?13, probation_end_reminder = ?14,
                id_number = ?15, birth_date = ?16, age = ?17, native_place = ?18, birth_place = ?19,
                ethnicity = ?20, technical_position = ?21, education = ?22, full_time_education = ?23,
                full_time_school_major = ?24, part_time_education = ?25, part_time_school_phone = ?26,
                political_status = ?27, party_entry_date = ?28, phone = ?29,
                grassroots_vice_position_date = ?30, grassroots_vice_tenure = ?31,
                grassroots_chief_position_date = ?32, grassroots_chief_tenure = ?33,
                midlevel_assistant_date = ?34, midlevel_assistant_tenure = ?35,
                midlevel_vice_date = ?36, midlevel_vice_tenure = ?37,
                midlevel_chief_date = ?38, midlevel_chief_tenure = ?39,
                same_department_date = ?40, same_department_tenure = ?41, remarks = ?42, major = ?43, contact_date = ?44,
                special_date = ?45
             WHERE id = ?46",
            params![
                cadre.serial_number, &cadre.name, cadre.gender, cadre.department,
                cadre.position1, cadre.position2, cadre.company_entry_date, cadre.company_tenure,
                cadre.work_start_date, cadre.work_tenure, cadre.current_level_date,
                cadre.position_entry_date, cadre.probation_period, cadre.probation_end_reminder,
                cadre.id_number, cadre.birth_date, cadre.age, cadre.native_place, cadre.birth_place,
                cadre.ethnicity, cadre.technical_position, cadre.education, cadre.full_time_education,
                cadre.full_time_school_major, cadre.part_time_education, cadre.part_time_school_phone,
                cadre.political_status, cadre.party_entry_date, cadre.phone,
                cadre.grassroots_vice_position_date, cadre.grassroots_vice_tenure,
                cadre.grassroots_chief_position_date, cadre.grassroots_chief_tenure,
                cadre.midlevel_assistant_date, cadre.midlevel_assistant_tenure,
                cadre.midlevel_vice_date, cadre.midlevel_vice_tenure,
                cadre.midlevel_chief_date, cadre.midlevel_chief_tenure,
                cadre.same_department_date, cadre.same_department_tenure, cadre.remarks, cadre.major, cadre.contact_date,
                cadre.special_date, cadre.id
            ],
        )
    }
    
    pub fn delete_midlevel_cadre(&self, id: i32) -> Result<usize> {
        self.conn.execute("DELETE FROM midlevel_cadres WHERE id = ?1", [id])
    }
    
    // ==================== Common Methods ====================
    
    pub fn get_distinct_field_values(&self, table_name: &str, field_name: &str) -> Result<Vec<String>> {
        // 验证表名和字段名以防止SQL注入
        let allowed_tables = ["grassroots_cadres", "midlevel_cadres"];
        let allowed_fields = [
            "gender", "department", "section", "position1", "position2", "education", 
            "political_status", "technical_position", "ethnicity", "native_place", "birth_place",
            "full_time_education", "part_time_education"
        ];
        
        if !allowed_tables.contains(&table_name) {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some("Invalid table name".to_string())
            ));
        }
        
        if !allowed_fields.contains(&field_name) {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some("Invalid field name".to_string())
            ));
        }
        
        // 特殊处理: midlevel_cadres 表没有 section 字段
        if table_name == "midlevel_cadres" && field_name == "section" {
            return Ok(vec![]); // 返回空列表
        }
        
        let query = format!("SELECT DISTINCT {} FROM {} WHERE {} IS NOT NULL AND {} != '' ORDER BY {}", 
                           field_name, table_name, field_name, field_name, field_name);
        let mut stmt = self.conn.prepare(&query)?;
        
        let values: Result<Vec<String>, _> = stmt.query_map([], |row| {
            row.get(0)
        })?.collect();
        
        values
    }
    
    // ==================== Filtered Query Methods ====================
    
    // 根据筛选条件获取基层干部信息
    pub fn get_filtered_grassroots_cadres(&self, filter_params: &FilterParams) -> Result<Vec<GrassrootsCadreInfo>> {
        let mut query = "SELECT id, serial_number, name, gender, department, section, position1, position2,
                company_entry_date, company_tenure, work_start_date, work_tenure,
                current_level_date, position_entry_date, probation_period,
                probation_end_reminder, id_number, birth_date, age, native_place, birth_place,
                ethnicity, technical_position, education, full_time_education,
                full_time_school_major, part_time_education, part_time_school_phone,
                political_status, party_entry_date, phone, grassroots_vice_position_date,
                grassroots_vice_tenure, grassroots_chief_position_date, grassroots_chief_tenure,
                midlevel_assistant_date, midlevel_assistant_tenure, midlevel_vice_date,
                midlevel_vice_tenure, midlevel_chief_date, midlevel_chief_tenure,
                same_department_date, same_department_tenure, remarks, major, contact_date, special_date
         FROM grassroots_cadres WHERE 1=1".to_string();
        
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        let mut param_index = 1;
        
        // 处理普通筛选条件
        for filter in &filter_params.filters {
            match filter.operator.as_str() {
                "eq" => {
                    query.push_str(&format!(" AND {} = ?{}", filter.field, param_index));
                    params.push(Box::new(filter.value.clone()));
                    param_index += 1;
                },
                "ne" => {
                    query.push_str(&format!(" AND {} != ?{}", filter.field, param_index));
                    params.push(Box::new(filter.value.clone()));
                    param_index += 1;
                },
                "like" => {
                    query.push_str(&format!(" AND {} LIKE ?{}", filter.field, param_index));
                    params.push(Box::new(format!("%{}%", filter.value)));
                    param_index += 1;
                },
                _ => {}
            }
        }
        
        // 处理日期范围筛选
        for (field, date_range) in &filter_params.date_ranges {
            query.push_str(&format!(" AND {} >= ?{} AND {} <= ?{}", field, param_index, field, param_index + 1));
            params.push(Box::new(date_range.start.clone()));
            params.push(Box::new(date_range.end.clone()));
            param_index += 2;
        }
        
        // 处理年龄范围筛选
        if let Some(age_min) = filter_params.age_min {
            query.push_str(&format!(" AND age >= ?{}", param_index));
            params.push(Box::new(age_min));
            param_index += 1;
        }
        
        if let Some(age_max) = filter_params.age_max {
            query.push_str(&format!(" AND age <= ?{}", param_index));
            params.push(Box::new(age_max));
            // param_index += 1; // 不需要递增param_index，因为这是最后一个参数
        }
        
        query.push_str(" ORDER BY id");
        
        let mut stmt = self.conn.prepare(&query)?;
        
        // 执行查询
        let cadre_iter = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
            Ok(GrassrootsCadreInfo {
                id: row.get(0)?,
                serial_number: row.get(1)?,
                name: row.get(2)?,
                gender: row.get(3)?,
                department: row.get(4)?,
                section: row.get(5)?,
                position1: row.get(6)?,
                position2: row.get(7)?,
                company_entry_date: row.get(8)?,
                company_tenure: row.get(9)?,
                work_start_date: row.get(10)?,
                work_tenure: row.get(11)?,
                current_level_date: row.get(12)?,
                position_entry_date: row.get(13)?,
                probation_period: row.get(14)?,
                probation_end_reminder: row.get(15)?,
                id_number: row.get(16)?,
                birth_date: row.get(17)?,
                age: row.get(18)?,
                native_place: row.get(19)?,
                birth_place: row.get(20)?,
                ethnicity: row.get(21)?,
                technical_position: row.get(22)?,
                education: row.get(23)?,
                full_time_education: row.get(24)?,
                full_time_school_major: row.get(25)?,
                part_time_education: row.get(26)?,
                part_time_school_phone: row.get(27)?,
                political_status: row.get(28)?,
                party_entry_date: row.get(29)?,
                phone: row.get(30)?,
                grassroots_vice_position_date: row.get(31)?,
                grassroots_vice_tenure: row.get(32)?,
                grassroots_chief_position_date: row.get(33)?,
                grassroots_chief_tenure: row.get(34)?,
                midlevel_assistant_date: row.get(35)?,
                midlevel_assistant_tenure: {
                    // Handle both TEXT and REAL types for backward compatibility
                    match row.get::<_, Option<String>>(36) {
                        Ok(val) => val,
                        Err(_) => {
                            // If it's stored as REAL, convert it to String
                            match row.get::<_, Option<f32>>(36) {
                                Ok(val) => val.map(|v| v.to_string()),
                                Err(_) => None
                            }
                        }
                    }
                },
                midlevel_vice_date: row.get(37)?,
                midlevel_vice_tenure: row.get(38)?,
                midlevel_chief_date: row.get(39)?,
                midlevel_chief_tenure: row.get(40)?,
                same_department_date: row.get(41)?,
                same_department_tenure: row.get(42)?,
                remarks: row.get(43)?,
                major: row.get(44)?,
                contact_date: row.get(44)?,
                special_date: row.get(45)?,
            })
        })?;
        
        let mut cadres = Vec::new();
        for cadre in cadre_iter {
            cadres.push(cadre?);
        }
        
        Ok(cadres)
    }
    
    // 根据筛选条件获取中层干部信息
    pub fn get_filtered_midlevel_cadres(&self, filter_params: &FilterParams) -> Result<Vec<MidLevelCadreInfo>> {
        let mut query = "SELECT id, serial_number, name, gender, department, position1, position2,
                company_entry_date, company_tenure, work_start_date, work_tenure,
                current_level_date, position_entry_date, probation_period,
                probation_end_reminder, id_number, birth_date, age, native_place, birth_place,
                ethnicity, technical_position, education, full_time_education,
                full_time_school_major, part_time_education, part_time_school_phone,
                political_status, party_entry_date, phone, grassroots_vice_position_date,
                grassroots_vice_tenure, grassroots_chief_position_date, grassroots_chief_tenure,
                midlevel_assistant_date, midlevel_assistant_tenure, midlevel_vice_date,
                midlevel_vice_tenure, midlevel_chief_date, midlevel_chief_tenure,
                same_department_date, same_department_tenure, remarks, major, contact_date, special_date
         FROM midlevel_cadres WHERE 1=1".to_string();
        
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        let mut param_index = 1;
        
        // 处理普通筛选条件
        for filter in &filter_params.filters {
            match filter.operator.as_str() {
                "eq" => {
                    query.push_str(&format!(" AND {} = ?{}", filter.field, param_index));
                    params.push(Box::new(filter.value.clone()));
                    param_index += 1;
                },
                "ne" => {
                    query.push_str(&format!(" AND {} != ?{}", filter.field, param_index));
                    params.push(Box::new(filter.value.clone()));
                    param_index += 1;
                },
                "like" => {
                    query.push_str(&format!(" AND {} LIKE ?{}", filter.field, param_index));
                    params.push(Box::new(format!("%{}%", filter.value)));
                    param_index += 1;
                },
                _ => {}
            }
        }
        
        // 处理日期范围筛选
        for (field, date_range) in &filter_params.date_ranges {
            query.push_str(&format!(" AND {} >= ?{} AND {} <= ?{}", field, param_index, field, param_index + 1));
            params.push(Box::new(date_range.start.clone()));
            params.push(Box::new(date_range.end.clone()));
            param_index += 2;
        }
        
        // 处理年龄范围筛选
        if let Some(age_min) = filter_params.age_min {
            query.push_str(&format!(" AND age >= ?{}", param_index));
            params.push(Box::new(age_min));
            param_index += 1;
        }
        
        if let Some(age_max) = filter_params.age_max {
            query.push_str(&format!(" AND age <= ?{}", param_index));
            params.push(Box::new(age_max));
            // param_index += 1; // 不需要递增param_index，因为这是最后一个参数
        }
        
        query.push_str(" ORDER BY id");
        
        let mut stmt = self.conn.prepare(&query)?;
        
        // 执行查询
        let cadre_iter = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
            Ok(MidLevelCadreInfo {
                id: row.get(0)?,
                serial_number: row.get(1)?,
                name: row.get(2)?,
                gender: row.get(3)?,
                department: row.get(4)?,
                position1: row.get(5)?,
                position2: row.get(6)?,
                company_entry_date: row.get(7)?,
                company_tenure: row.get(8)?,
                work_start_date: row.get(9)?,
                work_tenure: row.get(10)?,
                current_level_date: row.get(11)?,
                position_entry_date: row.get(12)?,
                probation_period: row.get(13)?,
                probation_end_reminder: row.get(14)?,
                id_number: row.get(15)?,
                birth_date: row.get(16)?,
                age: row.get(17)?,
                native_place: row.get(18)?,
                birth_place: row.get(19)?,
                ethnicity: row.get(20)?,
                technical_position: row.get(21)?,
                education: row.get(22)?,
                full_time_education: row.get(23)?,
                full_time_school_major: row.get(24)?,
                part_time_education: row.get(25)?,
                part_time_school_phone: row.get(26)?,
                political_status: row.get(27)?,
                party_entry_date: row.get(28)?,
                phone: row.get(29)?,
                grassroots_vice_position_date: row.get(30)?,
                grassroots_vice_tenure: row.get(31)?,
                grassroots_chief_position_date: row.get(32)?,
                grassroots_chief_tenure: row.get(33)?,
                midlevel_assistant_date: row.get(34)?,
                midlevel_assistant_tenure: {
                    // Handle both TEXT and REAL types for backward compatibility
                    match row.get::<_, Option<String>>(35) {
                        Ok(val) => val,
                        Err(_) => {
                            // If it's stored as REAL, convert it to String
                            match row.get::<_, Option<f32>>(35) {
                                Ok(val) => val.map(|v| v.to_string()),
                                Err(_) => None
                            }
                        }
                    }
                },
                midlevel_vice_date: row.get(36)?,
                midlevel_vice_tenure: row.get(37)?,
                midlevel_chief_date: row.get(38)?,
                midlevel_chief_tenure: row.get(39)?,
                same_department_date: row.get(40)?,
                same_department_tenure: row.get(41)?,
                remarks: row.get(42)?,
                major: row.get(43)?,
                contact_date: row.get(44)?,
                special_date: row.get(45)?,
            })
        })?;
        
        let mut cadres = Vec::new();
        for cadre in cadre_iter {
            cadres.push(cadre?);
        }
        
        Ok(cadres)
    }
}