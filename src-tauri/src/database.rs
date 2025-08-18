use rusqlite::{Connection, Result, params};
use std::fs;

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
    pub current_level_date: Option<String>,
    pub position_entry_date: Option<String>,
    pub probation_period: Option<String>,
    pub probation_end_reminder: Option<String>,
    pub id_number: Option<String>,
    pub technical_position: Option<String>,
    pub education: Option<String>,
    pub full_time_education: Option<String>,
    pub full_time_school_major: Option<String>,
    pub part_time_education: Option<String>,
    pub part_time_school_phone: Option<String>,
    pub phone: Option<String>,
    pub remarks: Option<String>,
    pub major: Option<String>,
    pub political_status: Option<String>,
    pub party_entry_date: Option<String>,
    pub contact_date: Option<String>,
    pub age: Option<i32>,
    pub native_place: Option<String>,
    pub birth_place: Option<String>,
    pub birth_date: Option<String>,
    pub ethnicity: Option<String>,
    pub special_date: Option<String>,
    pub company_tenure: Option<f32>,
    pub work_start_date: Option<String>,
    pub work_tenure: Option<f32>,
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
    pub current_level_date: Option<String>,
    pub position_entry_date: Option<String>,
    pub probation_period: Option<String>,
    pub probation_end_reminder: Option<String>,
    pub id_number: Option<String>,
    pub technical_position: Option<String>,
    pub education: Option<String>,
    pub full_time_education: Option<String>,
    pub full_time_school_major: Option<String>,
    pub part_time_education: Option<String>,
    pub part_time_school_phone: Option<String>,
    pub phone: Option<String>,
    pub remarks: Option<String>,
    pub major: Option<String>,
    pub political_status: Option<String>,
    pub party_entry_date: Option<String>,
    pub contact_date: Option<String>,
    pub age: Option<i32>,
    pub native_place: Option<String>,
    pub birth_place: Option<String>,
    pub birth_date: Option<String>,
    pub ethnicity: Option<String>,
    pub special_date: Option<String>,
    pub company_tenure: Option<f32>,
    pub work_start_date: Option<String>,
    pub work_tenure: Option<f32>,
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
                current_level_date TEXT,
                position_entry_date TEXT,
                probation_period TEXT,
                probation_end_reminder TEXT,
                id_number TEXT UNIQUE,
                technical_position TEXT,
                education TEXT,
                full_time_education TEXT,
                full_time_school_major TEXT,
                part_time_education TEXT,
                part_time_school_phone TEXT,
                phone TEXT,
                remarks TEXT,
                major TEXT,
                political_status TEXT,
                party_entry_date TEXT,
                contact_date TEXT,
                age INTEGER,
                native_place TEXT,
                birth_place TEXT,
                birth_date TEXT,
                ethnicity TEXT,
                special_date TEXT,
                company_tenure REAL,
                work_start_date TEXT,
                work_tenure REAL
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
                current_level_date TEXT,
                position_entry_date TEXT,
                probation_period TEXT,
                probation_end_reminder TEXT,
                id_number TEXT UNIQUE,
                technical_position TEXT,
                education TEXT,
                full_time_education TEXT,
                full_time_school_major TEXT,
                part_time_education TEXT,
                part_time_school_phone TEXT,
                phone TEXT,
                remarks TEXT,
                major TEXT,
                political_status TEXT,
                party_entry_date TEXT,
                contact_date TEXT,
                age INTEGER,
                native_place TEXT,
                birth_place TEXT,
                birth_date TEXT,
                ethnicity TEXT,
                special_date TEXT,
                company_tenure REAL,
                work_start_date TEXT,
                work_tenure REAL
            )",
            [],
        )?;
        
        // Handle database migration - add part_time_school_phone column if it doesn't exist
        match conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN part_time_school_phone TEXT", []) {
            Ok(_) => println!("Successfully added part_time_school_phone column to grassroots_cadres"),
            Err(_) => println!("part_time_school_phone column already exists or error occurred in grassroots_cadres"),
        }
        
        match conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN part_time_school_phone TEXT", []) {
            Ok(_) => println!("Successfully added part_time_school_phone column to midlevel_cadres"),
            Err(_) => println!("part_time_school_phone column already exists or error occurred in midlevel_cadres"),
        }
        
        // Handle database migration - add birth_date column if it doesn't exist
        match conn.execute("ALTER TABLE grassroots_cadres ADD COLUMN birth_date TEXT", []) {
            Ok(_) => println!("Successfully added birth_date column to grassroots_cadres"),
            Err(_) => println!("birth_date column already exists or error occurred in grassroots_cadres"),
        }
        
        match conn.execute("ALTER TABLE midlevel_cadres ADD COLUMN birth_date TEXT", []) {
            Ok(_) => println!("Successfully added birth_date column to midlevel_cadres"),
            Err(_) => println!("birth_date column already exists or error occurred in midlevel_cadres"),
        }
        
        Ok(Database { conn })
    }
    
    // ==================== Grassroots Cadres Methods ====================
    
    pub fn add_grassroots_cadre(&self, cadre: &GrassrootsCadreInfo) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO grassroots_cadres (
                serial_number, name, gender, department, section, position1, position2,
                company_entry_date, current_level_date, position_entry_date, probation_period,
                probation_end_reminder, id_number, technical_position, education,
                full_time_education, full_time_school_major, part_time_education,
                part_time_school_phone, phone, remarks, major, political_status,
                party_entry_date, contact_date, age, native_place, birth_place, birth_date,
                ethnicity, special_date, company_tenure, work_start_date, work_tenure
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
                      ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29,
                      ?30, ?31, ?32, ?33, ?34)",
            params![
                cadre.serial_number, &cadre.name, cadre.gender, cadre.department, cadre.section,
                cadre.position1, cadre.position2, cadre.company_entry_date, cadre.current_level_date,
                cadre.position_entry_date, cadre.probation_period, cadre.probation_end_reminder,
                cadre.id_number, cadre.technical_position, cadre.education, cadre.full_time_education,
                cadre.full_time_school_major, cadre.part_time_education, cadre.part_time_school_phone,
                cadre.phone, cadre.remarks, cadre.major, cadre.political_status, cadre.party_entry_date,
                cadre.contact_date, cadre.age, cadre.native_place, cadre.birth_place, cadre.birth_date,
                cadre.ethnicity, cadre.special_date, cadre.company_tenure, cadre.work_start_date, cadre.work_tenure
            ],
        )
    }
    
    pub fn get_all_grassroots_cadres(&self) -> Result<Vec<GrassrootsCadreInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, serial_number, name, gender, department, section, position1, position2,
                    company_entry_date, current_level_date, position_entry_date, probation_period,
                    probation_end_reminder, id_number, technical_position, education,
                    full_time_education, full_time_school_major, part_time_education,
                    part_time_school_phone, phone, remarks, major, political_status,
                    party_entry_date, contact_date, age, native_place, birth_place, birth_date,
                    ethnicity, special_date, company_tenure, work_start_date, work_tenure
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
                current_level_date: row.get(9)?,
                position_entry_date: row.get(10)?,
                probation_period: row.get(11)?,
                probation_end_reminder: row.get(12)?,
                id_number: row.get(13)?,
                technical_position: row.get(14)?,
                education: row.get(15)?,
                full_time_education: row.get(16)?,
                full_time_school_major: row.get(17)?,
                part_time_education: row.get(18)?,
                part_time_school_phone: row.get(19)?,
                phone: row.get(20)?,
                remarks: row.get(21)?,
                major: row.get(22)?,
                political_status: row.get(23)?,
                party_entry_date: row.get(24)?,
                contact_date: row.get(25)?,
                age: row.get(26)?,
                native_place: row.get(27)?,
                birth_place: row.get(28)?,
                birth_date: row.get(29)?,
                ethnicity: row.get(30)?,
                special_date: row.get(31)?,
                company_tenure: row.get(32)?,
                work_start_date: row.get(33)?,
                work_tenure: row.get(34)?,
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
                    company_entry_date, current_level_date, position_entry_date, probation_period,
                    probation_end_reminder, id_number, technical_position, education,
                    full_time_education, full_time_school_major, part_time_education,
                    part_time_school_phone, phone, remarks, major, political_status,
                    party_entry_date, contact_date, age, native_place, birth_place, birth_date,
                    ethnicity, special_date, company_tenure, work_start_date, work_tenure
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
                current_level_date: row.get(9)?,
                position_entry_date: row.get(10)?,
                probation_period: row.get(11)?,
                probation_end_reminder: row.get(12)?,
                id_number: row.get(13)?,
                technical_position: row.get(14)?,
                education: row.get(15)?,
                full_time_education: row.get(16)?,
                full_time_school_major: row.get(17)?,
                part_time_education: row.get(18)?,
                part_time_school_phone: row.get(19)?,
                phone: row.get(20)?,
                remarks: row.get(21)?,
                major: row.get(22)?,
                political_status: row.get(23)?,
                party_entry_date: row.get(24)?,
                contact_date: row.get(25)?,
                age: row.get(26)?,
                native_place: row.get(27)?,
                birth_place: row.get(28)?,
                birth_date: row.get(29)?,
                ethnicity: row.get(30)?,
                special_date: row.get(31)?,
                company_tenure: row.get(32)?,
                work_start_date: row.get(33)?,
                work_tenure: row.get(34)?,
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
                position1 = ?6, position2 = ?7, company_entry_date = ?8, current_level_date = ?9,
                position_entry_date = ?10, probation_period = ?11, probation_end_reminder = ?12,
                id_number = ?13, technical_position = ?14, education = ?15, full_time_education = ?16,
                full_time_school_major = ?17, part_time_education = ?18, part_time_school_phone = ?19,
                phone = ?20, remarks = ?21, major = ?22, political_status = ?23, party_entry_date = ?24,
                contact_date = ?25, age = ?26, native_place = ?27, birth_place = ?28, birth_date = ?29, ethnicity = ?30,
                special_date = ?31, company_tenure = ?32, work_start_date = ?33, work_tenure = ?34
             WHERE id = ?35",
            params![
                cadre.serial_number, &cadre.name, cadre.gender, cadre.department, cadre.section,
                cadre.position1, cadre.position2, cadre.company_entry_date, cadre.current_level_date,
                cadre.position_entry_date, cadre.probation_period, cadre.probation_end_reminder,
                cadre.id_number, cadre.technical_position, cadre.education, cadre.full_time_education,
                cadre.full_time_school_major, cadre.part_time_education, cadre.part_time_school_phone,
                cadre.phone, cadre.remarks, cadre.major, cadre.political_status, cadre.party_entry_date,
                cadre.contact_date, cadre.age, cadre.native_place, cadre.birth_place, cadre.birth_date, cadre.ethnicity,
                cadre.special_date, cadre.company_tenure, cadre.work_start_date, cadre.work_tenure,
                cadre.id
            ],
        )
    }
    
    pub fn delete_grassroots_cadre(&self, id: i32) -> Result<usize> {
        self.conn.execute("DELETE FROM grassroots_cadres WHERE id = ?1", [id])
    }
    
    // ==================== Mid-Level Cadres Methods ====================
    
    pub fn add_midlevel_cadre(&self, cadre: &MidLevelCadreInfo) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO midlevel_cadres (
                serial_number, name, gender, department, position1, position2,
                company_entry_date, current_level_date, position_entry_date, probation_period,
                probation_end_reminder, id_number, technical_position, education,
                full_time_education, full_time_school_major, part_time_education,
                part_time_school_phone, phone, remarks, major, political_status,
                party_entry_date, contact_date, age, native_place, birth_place, birth_date,
                ethnicity, special_date, company_tenure, work_start_date, work_tenure
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
                      ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29,
                      ?30, ?31, ?32, ?33)",
            params![
                cadre.serial_number, &cadre.name, cadre.gender, cadre.department,
                cadre.position1, cadre.position2, cadre.company_entry_date, cadre.current_level_date,
                cadre.position_entry_date, cadre.probation_period, cadre.probation_end_reminder,
                cadre.id_number, cadre.technical_position, cadre.education, cadre.full_time_education,
                cadre.full_time_school_major, cadre.part_time_education, cadre.part_time_school_phone,
                cadre.phone, cadre.remarks, cadre.major, cadre.political_status, cadre.party_entry_date,
                cadre.contact_date, cadre.age, cadre.native_place, cadre.birth_place, cadre.birth_date,
                cadre.ethnicity, cadre.special_date, cadre.company_tenure, cadre.work_start_date, cadre.work_tenure
            ],
        )
    }
    
    pub fn get_all_midlevel_cadres(&self) -> Result<Vec<MidLevelCadreInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, serial_number, name, gender, department, position1, position2,
                    company_entry_date, current_level_date, position_entry_date, probation_period,
                    probation_end_reminder, id_number, technical_position, education,
                    full_time_education, full_time_school_major, part_time_education,
                    part_time_school_phone, phone, remarks, major, political_status,
                    party_entry_date, contact_date, age, native_place, birth_place, birth_date,
                    ethnicity, special_date, company_tenure, work_start_date, work_tenure
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
                current_level_date: row.get(8)?,
                position_entry_date: row.get(9)?,
                probation_period: row.get(10)?,
                probation_end_reminder: row.get(11)?,
                id_number: row.get(12)?,
                technical_position: row.get(13)?,
                education: row.get(14)?,
                full_time_education: row.get(15)?,
                full_time_school_major: row.get(16)?,
                part_time_education: row.get(17)?,
                part_time_school_phone: row.get(18)?,
                phone: row.get(19)?,
                remarks: row.get(20)?,
                major: row.get(21)?,
                political_status: row.get(22)?,
                party_entry_date: row.get(23)?,
                contact_date: row.get(24)?,
                age: row.get(25)?,
                native_place: row.get(26)?,
                birth_place: row.get(27)?,
                birth_date: row.get(28)?,
                ethnicity: row.get(29)?,
                special_date: row.get(30)?,
                company_tenure: row.get(31)?,
                work_start_date: row.get(32)?,
                work_tenure: row.get(33)?,
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
                    company_entry_date, current_level_date, position_entry_date, probation_period,
                    probation_end_reminder, id_number, technical_position, education,
                    full_time_education, full_time_school_major, part_time_education,
                    part_time_school_phone, phone, remarks, major, political_status,
                    party_entry_date, contact_date, age, native_place, birth_place, birth_date,
                    ethnicity, special_date, company_tenure, work_start_date, work_tenure
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
                current_level_date: row.get(8)?,
                position_entry_date: row.get(9)?,
                probation_period: row.get(10)?,
                probation_end_reminder: row.get(11)?,
                id_number: row.get(12)?,
                technical_position: row.get(13)?,
                education: row.get(14)?,
                full_time_education: row.get(15)?,
                full_time_school_major: row.get(16)?,
                part_time_education: row.get(17)?,
                part_time_school_phone: row.get(18)?,
                phone: row.get(19)?,
                remarks: row.get(20)?,
                major: row.get(21)?,
                political_status: row.get(22)?,
                party_entry_date: row.get(23)?,
                contact_date: row.get(24)?,
                age: row.get(25)?,
                native_place: row.get(26)?,
                birth_place: row.get(27)?,
                birth_date: row.get(28)?,
                ethnicity: row.get(29)?,
                special_date: row.get(30)?,
                company_tenure: row.get(31)?,
                work_start_date: row.get(32)?,
                work_tenure: row.get(33)?,
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
                position1 = ?5, position2 = ?6, company_entry_date = ?7, current_level_date = ?8,
                position_entry_date = ?9, probation_period = ?10, probation_end_reminder = ?11,
                id_number = ?12, technical_position = ?13, education = ?14, full_time_education = ?15,
                full_time_school_major = ?16, part_time_education = ?17, part_time_school_phone = ?18,
                phone = ?19, remarks = ?20, major = ?21, political_status = ?22, party_entry_date = ?23,
                contact_date = ?24, age = ?25, native_place = ?26, birth_place = ?27, birth_date = ?28, ethnicity = ?29,
                special_date = ?30, company_tenure = ?31, work_start_date = ?32, work_tenure = ?33
             WHERE id = ?34",
            params![
                cadre.serial_number, &cadre.name, cadre.gender, cadre.department,
                cadre.position1, cadre.position2, cadre.company_entry_date, cadre.current_level_date,
                cadre.position_entry_date, cadre.probation_period, cadre.probation_end_reminder,
                cadre.id_number, cadre.technical_position, cadre.education, cadre.full_time_education,
                cadre.full_time_school_major, cadre.part_time_education, cadre.part_time_school_phone,
                cadre.phone, cadre.remarks, cadre.major, cadre.political_status, cadre.party_entry_date,
                cadre.contact_date, cadre.age, cadre.native_place, cadre.birth_place, cadre.birth_date, cadre.ethnicity,
                cadre.special_date, cadre.company_tenure, cadre.work_start_date, cadre.work_tenure,
                cadre.id
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
}