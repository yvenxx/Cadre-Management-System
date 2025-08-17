<template>
  <div class="cadre-list-view">
    <el-card class="content-section">
      <template #header>
        <div class="section-header">
          <h2>管理人员信息表</h2>
        </div>
      </template>

      <!-- 筛选条件面板 -->
      <div class="filter-panel">
        <div class="filter-header">
          <div class="filter-title-container">
            <h3>筛选条件</h3>
            <el-button @click="toggleFilterPanel" class="toggle-filter-button">
              {{ showFilterPanel ? '收起' : '展开' }}
              <el-icon class="el-icon--right">
                <ArrowUp v-if="showFilterPanel" />
                <ArrowDown v-else />
              </el-icon>
            </el-button>
          </div>
        </div>
        <div v-show="showFilterPanel" class="filter-content">
          <el-row :gutter="16">
            <el-col :span="6">
              <el-form-item label="姓名">
                <el-input v-model="filters.name" placeholder="请输入姓名" clearable prefix-icon="Search" />
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="性别">
                <el-select v-model="filters.gender" placeholder="请选择" clearable>
                  <el-option label="全部" value="" />
                  <el-option label="男" value="男" />
                  <el-option label="女" value="女" />
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="部门">
                <el-input v-model="filters.department" placeholder="请输入部门" clearable :prefix-icon="OfficeBuilding" />
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="科室">
                <el-input v-model="filters.section" placeholder="请输入科室" clearable :prefix-icon="OfficeBuilding" />
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="16">
            <el-col :span="6">
              <el-form-item label="职务1">
                <el-input v-model="filters.position1" placeholder="请输入职务1" clearable prefix-icon="UserFilled" />
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="职务2">
                <el-input v-model="filters.position2" placeholder="请输入职务2" clearable prefix-icon="UserFilled" />
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="学历">
                <el-input v-model="filters.education" placeholder="请输入学历" clearable :prefix-icon="Medal" />
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="政治面貌">
                <el-select v-model="filters.politicalStatus" placeholder="请选择" clearable>
                  <el-option label="全部" value="" />
                  <el-option label="中共党员" value="中共党员" />
                  <el-option label="预备党员" value="预备党员" />
                  <el-option label="共青团员" value="共青团员" />
                </el-select>
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="16">
            <el-col :span="6">
              <el-form-item label="身份证号">
                <el-input v-model="filters.idNumber" placeholder="请输入身份证号" clearable />
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="籍贯">
                <el-input v-model="filters.nativePlace" placeholder="请输入籍贯" clearable />
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="民族">
                <el-input v-model="filters.ethnicity" placeholder="请输入民族" clearable />
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="专业技术职务">
                <el-input v-model="filters.technicalPosition" placeholder="请输入专业技术职务" clearable />
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="16">
              <el-col :span="8">
                <el-form-item label="联系电话">
                  <el-input v-model="filters.phone" placeholder="请输入联系电话" clearable />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="入司日期范围">
                  <el-date-picker
                    v-model="filterDateRanges.companyEntryDate"
                    type="daterange"
                    range-separator="至"
                    start-placeholder="开始日期"
                    end-placeholder="结束日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                    style="width: 100%"
                  />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="年龄范围">
                  <div class="age-range-container">
                    <el-input-number
                      v-model="filters.ageMin"
                      placeholder="最小年龄"
                      style="width: 45%"
                      controls-position="right"
                      :min="0"
                    />
                    <span class="age-range-separator">至</span>
                    <el-input-number
                      v-model="filters.ageMax"
                      placeholder="最大年龄"
                      style="width: 45%"
                      controls-position="right"
                      :min="0"
                    />
                    <span class="age-range-unit">岁</span>
                  </div>
                </el-form-item>
              </el-col>
            </el-row>

          <el-row :gutter="16">
            <el-col :span="8">
              <el-form-item label="出生地">
                <el-input v-model="filters.birthPlace" placeholder="请输入出生地" clearable />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="全日制学历">
                <el-input v-model="filters.fullTimeEducation" placeholder="请输入全日制学历" clearable />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="在职学历">
                <el-input v-model="filters.partTimeEducation" placeholder="请输入在职学历" clearable />
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="16">
            <el-col :span="8">
              <el-form-item label="出生日期范围">
                <el-date-picker
                  v-model="filterDateRanges.birthDate"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="开始日期"
                  end-placeholder="结束日期"
                  format="YYYY-MM-DD"
                  value-format="YYYY-MM-DD"
                  clearable
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="参加工作时间范围">
                <el-date-picker
                  v-model="filterDateRanges.workStart"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="开始日期"
                  end-placeholder="结束日期"
                  format="YYYY-MM-DD"
                  value-format="YYYY-MM-DD"
                  clearable
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="任现职级时间范围">
                <el-date-picker
                  v-model="filterDateRanges.currentLevel"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="开始日期"
                  end-placeholder="结束日期"
                  format="YYYY-MM-DD"
                  value-format="YYYY-MM-DD"
                  clearable
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="16">
            <el-col :span="8">
              <el-form-item label="任职时间范围">
                <el-date-picker
                  v-model="filterDateRanges.positionEntry"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="开始日期"
                  end-placeholder="结束日期"
                  format="YYYY-MM-DD"
                  value-format="YYYY-MM-DD"
                  clearable
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="入党时间范围">
                <el-date-picker
                  v-model="filterDateRanges.partyEntry"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="开始日期"
                  end-placeholder="结束日期"
                  format="YYYY-MM-DD"
                  value-format="YYYY-MM-DD"
                  clearable
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="试用期满到期提醒范围">
                <el-date-picker
                  v-model="filterDateRanges.probationEndReminder"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="开始日期"
                  end-placeholder="结束日期"
                  format="YYYY-MM-DD"
                  value-format="YYYY-MM-DD"
                  clearable
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="16">
            <el-col :span="8">
              <el-form-item label="全日制毕业院校系及专业">
                <el-input v-model="filters.fullTimeSchoolMajor" placeholder="请输入全日制毕业院校系及专业" clearable />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="在职毕业院校系及专业">
                <el-input v-model="filters.partTimeSchoolPhone" placeholder="请输入在职毕业院校系及专业" clearable />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="试用期（年）">
                <el-input-number
                  v-model="filters.probationPeriod"
                  placeholder="请输入试用期"
                  style="width: 100%"
                  controls-position="right"
                />
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="16">
            <el-col :span="24">
              <el-form-item label="备注">
                <el-input v-model="filters.remarks" placeholder="请输入备注" clearable />
              </el-form-item>
            </el-col>
          </el-row>

          <div class="filter-actions">
            <el-button type="primary" @click="applyFilters" :icon="Search">应用筛选</el-button>
            <el-button @click="clearFilters" :icon="RefreshRight">清空筛选</el-button>
          </div>
        </div>
      </div>

      <!-- 操作按钮组 -->
      <div class="export-buttons-container">
        <div class="export-buttons-info">
          <span class="export-count-info">
            已选择: {{ selectedCadres.length }} 条记录
          </span>
        </div>
        <div class="export-buttons-group">
          <el-button type="primary" @click="openAddModal" :icon="Plus">新增</el-button>
          <el-button type="success" @click="exportSelectedCadres" :icon="Upload" :disabled="selectedCadres.length === 0">导出选中</el-button>
          <el-button type="warning" @click="exportAllCadres" :icon="Download">导出全部</el-button>
        </div>
      </div>

      <!-- 数据表格 -->
      <el-table 
        :data="filteredCadreList" 
        style="width: 100%" 
        row-key="id"
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column type="index" label="序号" width="60" />
        <el-table-column prop="name" label="姓名" width="100" />
        <el-table-column prop="gender" label="性别" width="60" />
        <el-table-column prop="department" label="部门" width="150" />
        <el-table-column prop="section" label="科室" width="120" />
        <el-table-column prop="position1" label="职务1" width="120" />
        <el-table-column prop="position2" label="职务2" width="120" />
        <el-table-column prop="company_entry_date" label="入司日期" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.company_entry_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="company_tenure" label="司龄（年）" width="100" />
        <el-table-column prop="work_start_date" label="参加工作时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.work_start_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="work_tenure" label="工龄(年)" width="100" />
        <el-table-column prop="current_level_date" label="任现职级时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.current_level_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="position_entry_date" label="任职时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.position_entry_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="probation_period" label="试用期" width="80" />
        <el-table-column prop="probation_end_reminder" label="试用期满到期提醒" width="150">
          <template #default="scope">
            {{ formatDate(scope.row.probation_end_reminder) }}
          </template>
        </el-table-column>
        <el-table-column prop="id_number" label="身份证号" width="180" />
        <el-table-column prop="birth_date" label="出生日期" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.birth_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="age" label="年龄" width="60" />
        <el-table-column prop="native_place" label="籍贯" width="100" />
        <el-table-column prop="birth_place" label="出生地" width="100" />
        <el-table-column prop="ethnicity" label="民族" width="80" />
        <el-table-column prop="technical_position" label="专业技术职务" width="150" />
        <el-table-column prop="education" label="学历" width="100" />
        <el-table-column prop="full_time_education" label="全日制学历" width="120" />
        <el-table-column prop="full_time_school_major" label="全日制毕业院校系及专业" width="200" />
        <el-table-column prop="part_time_education" label="在职学历" width="120" />
        <el-table-column prop="part_time_school_phone" label="在职毕业院校系及专业" width="200" />
        <el-table-column prop="political_status" label="政治面貌" width="120" />
        <el-table-column prop="party_entry_date" label="入党时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.party_entry_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="phone" label="联系电话" width="120" />
        <el-table-column prop="remarks" label="备注" width="150" />
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="scope">
            <el-button size="small" @click="editCadre(scope.row)">编辑</el-button>
            <el-button size="small" type="danger" @click="deleteCadre(scope.row.id)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
    
    <!-- 新增/编辑弹窗 -->
    <CadreForm 
      v-model="showModal"
      :cadre="currentCadre"
      @save="saveCadreInfo"
    />
    
    <!-- 导出配置弹窗 -->
    <ExportConfig 
      v-model="showExportModal"
      :selected-cadres="selectedCadres"
      :export-fields="exportFields"
      @export="performExport"
    />
  </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Search, RefreshRight, OfficeBuilding, UserFilled, Medal, Plus, Upload, Download, ArrowUp, ArrowDown } from '@element-plus/icons-vue';
import CadreForm from '../components/CadreForm.vue';
import ExportConfig from '../components/ExportConfig.vue';

// 日期格式化辅助函数
function formatDate(date) {
  if (!date) return "";
  
  // 如果是字符串，尝试转换为日期对象
  const dateObj = typeof date === 'string' ? new Date(date) : date;
  
  // 检查是否是有效日期
  if (!(dateObj instanceof Date) || isNaN(dateObj.getTime())) {
    return "";
  }
  
  const year = dateObj.getFullYear();
  const month = String(dateObj.getMonth() + 1).padStart(2, '0');
  const day = String(dateObj.getDate()).padStart(2, '0');
  
  return `${year}-${month}-${day}`;
}

const showModal = ref(false);
const showExportModal = ref(false);
const selectedCadres = ref([]); // 用于存储选中的干部

// 学历选项
const educationOptions = [
  "博士研究生",
  "硕士研究生",
  "大学",
  "大专",
  "高中",
  "中专",
  "初中",
  "职高"
];

// 专业技术职务选项
const technicalPositionOptions = [
  "助理政工师",
  "政工师",
  "高级工程师",
  "中级工程师",
  "工程师",
  "中级会计师",
  "助理工程师",
  "初级会计师",
  "初级经济师"
];

// 部门选项
const departmentOptions = [
  "党委工作部（文化宣传部）",
  "人力资源部",
  "财务部",
  "纪检监察部",
  "生产技术部",
  "安全管理部",
  "经营管理部",
  "招标采购部",
  "调度指挥中心",
  "站务中心",
  "车辆中心",
  "维修中心",
  "工会（团委）",
  "办公室",
  "项目管理部",
];

// 职务选项
const positionOptions = [
  "主任",
  "副主任",
  "区域站长",
  "区域副站长",
  "部长",
  "副部长",
  "部长助理",
  "经理",
  "副经理",
  "经理助理"
];

// 民族选项
const ethnicityOptions = [
  "汉族",
  "畲族",
  "侗族",
  "瑶族"
];

// 表单验证规则
const formRules = {
  name: [
    { required: true, message: '请输入姓名', trigger: 'blur' },
    { min: 2, max: 20, message: '姓名长度在 2 到 20 个字符', trigger: 'blur' }
  ],
  gender: [
    { required: true, message: '请选择性别', trigger: 'change' }
  ],
  id_number: [
    { pattern: /^[1-9]\d{5}(18|19|20)\d{2}((0[1-9])|(1[0-2]))(([0-2][1-9])|10|20|30|31)\d{3}[0-9Xx]$/, message: '请输入有效的18位身份证号', trigger: 'blur' }
  ],
  department: [
    { required: true, message: '请选择或输入部门', trigger: 'change' }
  ],
  section: [
    { required: true, message: '请输入科室', trigger: 'blur' }
  ],
  position1: [
    { required: true, message: '请选择或输入职务1', trigger: 'change' }
  ],
  phone: [
    { pattern: /^1[3-9]\d{9}$/, message: '请输入有效的手机号码', trigger: 'blur' }
  ]
};

// 所有可导出字段的配置
const exportFields = [
  { key: "name", label: "姓名" },
  { key: "gender", label: "性别" },
  { key: "department", label: "部门" },
  { key: "section", label: "科室" },
  { key: "position1", label: "职务1" },
  { key: "position2", label: "职务2" },
  { key: "company_entry_date", label: "入司日期" },
  { key: "company_tenure", label: "司龄（年）" },
  { key: "work_start_date", label: "参加工作时间" },
  { key: "work_tenure", label: "工龄(年)" },
  { key: "current_level_date", label: "任现职级时间" },
  { key: "position_entry_date", label: "任职时间" },
  { key: "probation_period", label: "试用期" },
  { key: "probation_end_reminder", label: "试用期满到期提醒" },
  { key: "id_number", label: "身份证号" },
  { key: "birth_date", label: "出生日期" },
  { key: "age", label: "年龄" },
  { key: "native_place", label: "籍贯" },
  { key: "birth_place", label: "出生地" },
  { key: "ethnicity", label: "民族" },
  { key: "technical_position", label: "专业技术职务" },
  { key: "education", label: "学历" },
  { key: "full_time_education", label: "全日制学历" },
  { key: "full_time_school_major", label: "全日制毕业院校系及专业" },
  { key: "part_time_education", label: "在职学历" },
  { key: "part_time_school_phone", label: "在职毕业院校系及专业" },
  { key: "political_status", label: "政治面貌" },
  { key: "party_entry_date", label: "入党时间" },
  { key: "phone", label: "联系电话" },
  { key: "remarks", label: "备注" }
];

const currentCadre = ref({
  id: null,
  name: "",
  gender: "",
  department: "",
  section: "",
  position1: "",
  position2: "",
  company_entry_date: "",
  company_tenure: null,
  work_start_date: "",
  work_tenure: null,
  current_level_date: "",
  position_entry_date: "",
  probation_period: "",
  probation_end_reminder: "",
  id_number: "",
  birth_date: "",
  age: null,
  native_place: "",
  birth_place: "",
  ethnicity: "",
  technical_position: "",
  education: "",
  full_time_education: "",
  full_time_school_major: "",
  part_time_education: "",
  part_time_school_phone: "",
  political_status: "",
  party_entry_date: "",
  phone: "",
  remarks: "",
  major: "",
  contact_date: "",
  special_date: ""
});

const cadreList = ref([]);
const showFilterPanel = ref(false);
const filterDateRanges = ref({
  companyEntryDate: [],
  birthDate: [],
  workStart: [],
  currentLevel: [],
  positionEntry: [],
  partyEntry: [],
  probationEndReminder: []
});
const filters = ref({
  name: "",
  gender: "",
  department: "",
  section: "",
  position1: "",
  position2: "",
  education: "",
  politicalStatus: "",
  idNumber: "",
  nativePlace: "",
  ethnicity: "",
  technicalPosition: "",
  phone: "",
  ageMin: null,
  ageMax: null,
  birthPlace: "",
  fullTimeEducation: "",
  partTimeEducation: "",
  probationPeriod: null,
  fullTimeSchoolMajor: "",
  partTimeSchoolPhone: "",
  remarks: ""
});

// 计算筛选后的干部列表
const filteredCadreList = computed(() => {
  return cadreList.value.filter(cadre => {
    // 姓名筛选
    if (filters.value.name && !cadre.name.includes(filters.value.name)) {
      return false;
    }
    
    // 性别筛选
    if (filters.value.gender && cadre.gender !== filters.value.gender) {
      return false;
    }
    
    // 部门筛选
    if (filters.value.department && !cadre.department.includes(filters.value.department)) {
      return false;
    }
    
    // 科室筛选
    if (filters.value.section && !cadre.section.includes(filters.value.section)) {
      return false;
    }
    
    // 职务1筛选
    if (filters.value.position1 && !cadre.position1.includes(filters.value.position1)) {
      return false;
    }
    
    // 职务2筛选
    if (filters.value.position2 && !cadre.position2.includes(filters.value.position2)) {
      return false;
    }
    
    // 学历筛选
    if (filters.value.education && !cadre.education.includes(filters.value.education)) {
      return false;
    }
    
    // 政治面貌筛选
    if (filters.value.politicalStatus && cadre.political_status !== filters.value.politicalStatus) {
      return false;
    }
    
    // 身份证号筛选
    if (filters.value.idNumber && !cadre.id_number.includes(filters.value.idNumber)) {
      return false;
    }
    
    // 籍贯筛选
    if (filters.value.nativePlace && !cadre.native_place.includes(filters.value.nativePlace)) {
      return false;
    }
    
    // 民族筛选
    if (filters.value.ethnicity && !cadre.ethnicity.includes(filters.value.ethnicity)) {
      return false;
    }
    
    // 专业技术职务筛选
    if (filters.value.technicalPosition && !cadre.technical_position.includes(filters.value.technicalPosition)) {
      return false;
    }
    
    // 联系电话筛选
    if (filters.value.phone && !cadre.phone.includes(filters.value.phone)) {
      return false;
    }
    
    // 入司日期范围筛选
    if (filterDateRanges.value.companyEntryDate && filterDateRanges.value.companyEntryDate.length === 2) {
      const entryDate = new Date(cadre.company_entry_date);
      const [startDate, endDate] = filterDateRanges.value.companyEntryDate.map(date => new Date(date));
      
      if (entryDate < startDate) {
        return false;
      }
      
      if (entryDate > endDate) {
        return false;
      }
    }
    
      // 年龄范围筛选
    if ((filters.value.ageMin !== null && filters.value.ageMin !== undefined && filters.value.ageMin !== "") || 
        (filters.value.ageMax !== null && filters.value.ageMax !== undefined && filters.value.ageMax !== "")) {
      const age = parseInt(cadre.age);
      
      if (filters.value.ageMin !== null && filters.value.ageMin !== undefined && filters.value.ageMin !== "" && 
          age < parseInt(filters.value.ageMin)) {
        return false;
      }
      
      if (filters.value.ageMax !== null && filters.value.ageMax !== undefined && filters.value.ageMax !== "" && 
          age > parseInt(filters.value.ageMax)) {
        return false;
      }
    }
    
    // 出生地筛选
    if (filters.value.birthPlace && !cadre.birth_place.includes(filters.value.birthPlace)) {
      return false;
    }
    
    // 全日制学历筛选
    if (filters.value.fullTimeEducation && !cadre.full_time_education.includes(filters.value.fullTimeEducation)) {
      return false;
    }
    
    // 在职学历筛选
    if (filters.value.partTimeEducation && !cadre.part_time_education.includes(filters.value.partTimeEducation)) {
      return false;
    }
    
    // 出生日期范围筛选
    if (filterDateRanges.value.birthDate && filterDateRanges.value.birthDate.length === 2) {
      const birthDate = new Date(cadre.birth_date);
      const [startDate, endDate] = filterDateRanges.value.birthDate.map(date => new Date(date));
      
      if (birthDate < startDate) {
        return false;
      }
      
      if (birthDate > endDate) {
        return false;
      }
    }
    
    // 参加工作时间范围筛选
    if (filterDateRanges.value.workStart && filterDateRanges.value.workStart.length === 2) {
      const workDate = new Date(cadre.work_start_date);
      const [startDate, endDate] = filterDateRanges.value.workStart.map(date => new Date(date));
      
      if (workDate < startDate) {
        return false;
      }
      
      if (workDate > endDate) {
        return false;
      }
    }
    
    // 任现职级时间范围筛选
    if (filterDateRanges.value.currentLevel && filterDateRanges.value.currentLevel.length === 2) {
      const currentDate = new Date(cadre.current_level_date);
      const [startDate, endDate] = filterDateRanges.value.currentLevel.map(date => new Date(date));
      
      if (currentDate < startDate) {
        return false;
      }
      
      if (currentDate > endDate) {
        return false;
      }
    }
    
    // 任职时间范围筛选
    if (filterDateRanges.value.positionEntry && filterDateRanges.value.positionEntry.length === 2) {
      const positionDate = new Date(cadre.position_entry_date);
      const [startDate, endDate] = filterDateRanges.value.positionEntry.map(date => new Date(date));
      
      if (positionDate < startDate) {
        return false;
      }
      
      if (positionDate > endDate) {
        return false;
      }
    }
    
    // 入党时间范围筛选
    if (filterDateRanges.value.partyEntry && filterDateRanges.value.partyEntry.length === 2) {
      const partyDate = new Date(cadre.party_entry_date);
      const [startDate, endDate] = filterDateRanges.value.partyEntry.map(date => new Date(date));
      
      if (partyDate < startDate) {
        return false;
      }
      
      if (partyDate > endDate) {
        return false;
      }
    }
    
    // 试用期满到期提醒范围筛选
    if (filterDateRanges.value.probationEndReminder && filterDateRanges.value.probationEndReminder.length === 2) {
      const reminderDate = new Date(cadre.probation_end_reminder);
      const [startDate, endDate] = filterDateRanges.value.probationEndReminder.map(date => new Date(date));
      
      if (reminderDate < startDate) {
        return false;
      }
      
      if (reminderDate > endDate) {
        return false;
      }
    }
    
    // 全日制毕业院校系及专业筛选
    if (filters.value.fullTimeSchoolMajor && !cadre.full_time_school_major.includes(filters.value.fullTimeSchoolMajor)) {
      return false;
    }
    
    // 在职毕业院校系及专业筛选
    if (filters.value.partTimeSchoolPhone && !cadre.part_time_school_phone.includes(filters.value.partTimeSchoolPhone)) {
      return false;
    }
    
    // 试用期筛选
    if (filters.value.probationPeriod !== null && filters.value.probationPeriod !== undefined && filters.value.probationPeriod !== "" && 
        cadre.probation_period !== null && cadre.probation_period !== parseFloat(filters.value.probationPeriod)) {
      return false;
    }
    
    // 备注筛选
    if (filters.value.remarks && !cadre.remarks.includes(filters.value.remarks)) {
      return false;
    }
    
    return true;
  });
});

// 加载所有干部信息
async function loadCadreInfo() {
  try {
    cadreList.value = await invoke("get_all_cadre_info");
  } catch (error) {
    console.error("加载干部信息失败:", error);
  }
}

// 保存干部信息
async function saveCadreInfo(cadreData) {
  try {
    if (cadreData.id) {
      // 更新现有记录
      await invoke("update_cadre_info", { cadre: cadreData });
    } else {
      // 添加新记录
      await invoke("add_cadre_info", { cadre: cadreData });
    }
    closeModal();
    loadCadreInfo();
  } catch (error) {
    console.error("保存干部信息失败:", error);
    if (error.name === 'ValidationError') {
      console.error("表单验证失败:", error);
    } else {
      console.error("保存干部信息失败:", error);
    }
  }
}

// 编辑干部信息
function editCadre(cadre) {
  // 创建深拷贝
  const cadreCopy = { ...cadre };
  
  // 处理日期字段，将字符串转换为日期对象
  if (cadreCopy.company_entry_date && typeof cadreCopy.company_entry_date === 'string') {
    cadreCopy.company_entry_date = new Date(cadreCopy.company_entry_date);
  }
  
  if (cadreCopy.work_start_date && typeof cadreCopy.work_start_date === 'string') {
    cadreCopy.work_start_date = new Date(cadreCopy.work_start_date);
  }
  
  if (cadreCopy.current_level_date && typeof cadreCopy.current_level_date === 'string') {
    cadreCopy.current_level_date = new Date(cadreCopy.current_level_date);
  }
  
  if (cadreCopy.position_entry_date && typeof cadreCopy.position_entry_date === 'string') {
    cadreCopy.position_entry_date = new Date(cadreCopy.position_entry_date);
  }
  
  if (cadreCopy.probation_end_reminder && typeof cadreCopy.probation_end_reminder === 'string') {
    cadreCopy.probation_end_reminder = new Date(cadreCopy.probation_end_reminder);
  }
  
  if (cadreCopy.birth_date && typeof cadreCopy.birth_date === 'string') {
    cadreCopy.birth_date = new Date(cadreCopy.birth_date);
  }
  
  if (cadreCopy.party_entry_date && typeof cadreCopy.party_entry_date === 'string') {
    cadreCopy.party_entry_date = new Date(cadreCopy.party_entry_date);
  }
  
  // 先进行计算
  // 重新计算司龄和工龄
  if (cadreCopy.company_entry_date) {
    // 计算司龄
    const entryDate = cadreCopy.company_entry_date instanceof Date 
      ? cadreCopy.company_entry_date 
      : new Date(cadreCopy.company_entry_date);
    
    if (!isNaN(entryDate.getTime())) {
      const today = new Date();
      const diffTime = Math.abs(today - entryDate);
      const diffYears = diffTime / (1000 * 60 * 60 * 24 * 365);
      cadreCopy.company_tenure = parseFloat(diffYears.toFixed(1));
    } else {
      cadreCopy.company_tenure = null;
    }
  }
  
  if (cadreCopy.work_start_date) {
    // 计算工龄
    const startDate = cadreCopy.work_start_date instanceof Date 
      ? cadreCopy.work_start_date 
      : new Date(cadreCopy.work_start_date);
    
    if (!isNaN(startDate.getTime())) {
      const today = new Date();
      const diffTime = Math.abs(today - startDate);
      const diffYears = diffTime / (1000 * 60 * 60 * 24 * 365);
      cadreCopy.work_tenure = parseFloat(diffYears.toFixed(1));
    } else {
      cadreCopy.work_tenure = null;
    }
  }
  
  // 重新计算试用期满到期提醒
  if (cadreCopy.probation_period && cadreCopy.position_entry_date) {
    const positionDate = cadreCopy.position_entry_date instanceof Date 
      ? cadreCopy.position_entry_date 
      : new Date(cadreCopy.position_entry_date);
    
    if (!isNaN(positionDate.getTime())) {
      const probationYears = parseFloat(cadreCopy.probation_period);
      
      // 计算试用期结束日期
      const endDate = new Date(positionDate);
      endDate.setFullYear(endDate.getFullYear() + Math.floor(probationYears));
      endDate.setMonth(endDate.getMonth() + Math.round((probationYears % 1) * 12));
      
      cadreCopy.probation_end_reminder = endDate;
    }
  }
  
  // 如果有身份证号，重新提取信息
  if (cadreCopy.id_number && cadreCopy.id_number.length === 18) {
    // 提取出生日期和年龄
    const idNumber = cadreCopy.id_number;
    const birthYear = idNumber.substring(6, 10);
    const birthMonth = idNumber.substring(10, 12);
    const birthDay = idNumber.substring(12, 14);
    
    // 创建日期对象
    const birthDate = new Date(birthYear, birthMonth - 1, birthDay);
    
    // 检查日期是否有效
    if (!isNaN(birthDate.getTime())) {
      cadreCopy.birth_date = birthDate;
      
      // 计算年龄
      const today = new Date();
      let age = today.getFullYear() - birthDate.getFullYear();
      const monthDiff = today.getMonth() - birthDate.getMonth();
      
      // 如果还没过生日，则年龄减1
      if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
        age--;
      }
      
      cadreCopy.age = age;
    }
  }
  
  // 设置当前干部信息
  currentCadre.value = cadreCopy;
  showModal.value = true;
}

// 删除干部信息
async function deleteCadre(id) {
  if (confirm("确定要删除这条记录吗？")) {
    try {
      await invoke("delete_cadre_info", { id });
      loadCadreInfo();
    } catch (error) {
      console.error("删除干部信息失败:", error);
    }
  }
}

// 打开新增弹窗
function openAddModal() {
  resetForm();
  showModal.value = true;
}

// 关闭弹窗
function closeModal() {
  showModal.value = false;
}

// 重置表单
function resetForm() {
  currentCadre.value = {
    id: null,
    name: "",
    gender: "",
    department: "",
    section: "",
    position1: "",
    position2: "",
    company_entry_date: "",
    company_tenure: null,
    work_start_date: "",
    work_tenure: null,
    current_level_date: "",
    position_entry_date: "",
    probation_period: null,
    probation_end_reminder: "",
    id_number: "",
    birth_date: "",
    age: null,
    native_place: "",
    birth_place: "",
    ethnicity: "",
    technical_position: "",
    education: "",
    full_time_education: "",
    full_time_school_major: "",
    part_time_education: "",
    part_time_school_phone: "",
    political_status: "",
    party_entry_date: "",
    phone: "",
    remarks: "",
    major: "",
    contact_date: "",
    special_date: ""
  };
}

// 全选/取消全选功能
function handleSelectionChange(selection) {
  selectedCadres.value = selection;
}

// 关闭导出配置弹窗
function closeExportModal() {
  showExportModal.value = false;
}

// 切换筛选面板显示状态
function toggleFilterPanel() {
  showFilterPanel.value = !showFilterPanel.value;
}

// 应用筛选条件
function applyFilters() {
  // 筛选逻辑将在模板中通过计算属性实现
  console.log("应用筛选条件:", filters.value);
}

// 清空筛选条件
function clearFilters() {
  filters.value = {
    name: "",
    gender: "",
    department: "",
    section: "",
    position1: "",
    position2: "",
    education: "",
    politicalStatus: "",
    idNumber: "",
    nativePlace: "",
    ethnicity: "",
    technicalPosition: "",
    phone: "",
    ageMin: null,
    ageMax: null,
    birthPlace: "",
    fullTimeEducation: "",
    partTimeEducation: "",
    probationPeriod: null,
    fullTimeSchoolMajor: "",
    partTimeSchoolPhone: "",
    remarks: ""
  };
  filterDateRanges.value = {
    companyEntryDate: [],
    birthDate: [],
    workStart: [],
    currentLevel: [],
    positionEntry: [],
    partyEntry: [],
    probationEndReminder: []
  };
}

// 导出选中干部信息
async function exportSelectedCadres() {
  if (selectedCadres.value.length === 0) {
    alert("请先选择要导出的干部信息");
    return;
  }
  
  try {
    // 打开导出配置弹窗，设置默认文件名为"选中干部信息"
    exportConfig.value.fileName = "选中干部信息";
    showExportModal.value = true;
  } catch (error) {
    console.error("导出选中干部信息失败:", error);
    alert("导出失败: " + error);
  }
}

// 导出全部干部信息
async function exportAllCadres() {
  try {
    // 打开导出配置弹窗，设置默认文件名为"全部干部信息"
    exportConfig.value.fileName = "全部干部信息";
    showExportModal.value = true;
  } catch (error) {
    console.error("导出全部干部信息失败:", error);
    alert("导出失败: " + error);
  }
}

// 执行导出操作
async function performExport(exportData) {
  try {
    // 构造文件路径
    const filePath = `${exportData.fileName}.xlsx`;
    
    await invoke("export_cadre_info_to_excel", { 
      filePath, 
      selectedFields: exportData.selectedFields,
      cadreIds: exportData.cadreIds // 如果为null则导出全部
    });
    
    closeExportModal();
    alert("导出成功！文件已保存为: " + filePath);
  } catch (error) {
    console.error("导出失败:", error);
    alert("导出失败: " + error);
  }
}

// 获取部门统计信息
function getDepartmentStats() {
  const stats = {};
  cadreList.value.forEach(cadre => {
    if (cadre.department) {
      stats[cadre.department] = (stats[cadre.department] || 0) + 1;
    }
  });
  return stats;
}

// 计算司龄
function calculateCompanyTenure() {
  if (currentCadre.value.company_entry_date) {
    // 确保日期是Date对象
    const entryDate = currentCadre.value.company_entry_date instanceof Date 
      ? currentCadre.value.company_entry_date 
      : new Date(currentCadre.value.company_entry_date);
    
    if (isNaN(entryDate.getTime())) {
      currentCadre.value.company_tenure = null;
      return;
    }
    
    const today = new Date();
    const diffTime = Math.abs(today - entryDate);
    const diffYears = diffTime / (1000 * 60 * 60 * 24 * 365);
    currentCadre.value.company_tenure = parseFloat(diffYears.toFixed(1));
  } else {
    currentCadre.value.company_tenure = null;
  }
}

// 计算工龄
function calculateWorkTenure() {
  if (currentCadre.value.work_start_date) {
    // 确保日期是Date对象
    const startDate = currentCadre.value.work_start_date instanceof Date 
      ? currentCadre.value.work_start_date 
      : new Date(currentCadre.value.work_start_date);
    
    if (isNaN(startDate.getTime())) {
      currentCadre.value.work_tenure = null;
      return;
    }
    
    const today = new Date();
    const diffTime = Math.abs(today - startDate);
    const diffYears = diffTime / (1000 * 60 * 60 * 24 * 365);
    currentCadre.value.work_tenure = parseFloat(diffYears.toFixed(1));
  } else {
    currentCadre.value.work_tenure = null;
  }
}

// 计算试用期满到期提醒
function calculateProbationEnd() {
  // 只有当试用期和任职时间都存在时才计算
  if (currentCadre.value.probation_period && currentCadre.value.position_entry_date) {
    // 确保日期是Date对象
    const positionDate = currentCadre.value.position_entry_date instanceof Date 
      ? currentCadre.value.position_entry_date 
      : new Date(currentCadre.value.position_entry_date);
    
    if (isNaN(positionDate.getTime())) {
      currentCadre.value.probation_end_reminder = "";
      return;
    }
    
    const probationYears = parseFloat(currentCadre.value.probation_period);
    
    // 计算试用期结束日期
    const endDate = new Date(positionDate);
    endDate.setFullYear(endDate.getFullYear() + Math.floor(probationYears));
    endDate.setMonth(endDate.getMonth() + Math.round((probationYears % 1) * 12));
    
    // 直接设置日期对象
    currentCadre.value.probation_end_reminder = endDate;
  } else {
    currentCadre.value.probation_end_reminder = "";
  }
}

// 从身份证号提取出生日期和年龄
function extractIdInfo() {
  const idNumber = currentCadre.value.id_number;
  if (idNumber && idNumber.length === 18) {
    // 提取出生日期 (第7-14位)
    const birthYear = idNumber.substring(6, 10);
    const birthMonth = idNumber.substring(10, 12);
    const birthDay = idNumber.substring(12, 14);
    
    // 创建日期对象
    const birthDate = new Date(birthYear, birthMonth - 1, birthDay);
    
    // 检查日期是否有效
    if (!isNaN(birthDate.getTime())) {
      // 设置日期对象
      currentCadre.value.birth_date = birthDate;
      
      // 计算年龄
      const today = new Date();
      let age = today.getFullYear() - birthDate.getFullYear();
      const monthDiff = today.getMonth() - birthDate.getMonth();
      
      // 如果还没过生日，则年龄减1
      if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
        age--;
      }
      
      currentCadre.value.age = age;
    } else {
      currentCadre.value.birth_date = "";
      currentCadre.value.age = null;
    }
  } else {
    currentCadre.value.birth_date = "";
    currentCadre.value.age = null;
  }
}

// 组件挂载时加载数据
onMounted(() => {
  loadCadreInfo();
});
</script>

<style scoped>
.cadre-list-view {
  height: 100%;
}

.content-section {
  background: white;
  border-radius: 16px;
  box-shadow: var(--card-shadow);
  position: relative;
  overflow: auto;
  border: 1px solid rgba(0, 0, 0, 0.05);
  max-height: calc(100vh - 50px);
}

.content-section::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 4px;
  background: linear-gradient(90deg, var(--primary-color), var(--secondary-color));
  border-radius: 16px 16px 0 0;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
  padding-bottom: 20px;
}

.filter-header {
  margin-bottom: 20px;
}

.filter-header h3 {
  margin: 0;
  color: #333;
  font-size: 18px;
  font-weight: 600;
}

.filter-title-container {
  display: flex;
  align-items: center;
  gap: 15px;
}

.toggle-filter-button {
  padding: 10px 20px;
  font-size: 14px;
  border-radius: 6px;
  background-color: #409eff;
  color: white;
  border: none;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  gap: 5px;
}

.toggle-filter-button:hover {
  background-color: #337ecc;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}
</style>