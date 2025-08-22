<template>
  <el-dialog
    v-model="visible"
    :title="title"
    width="80%"
    @close="handleClose"
  >
    <el-form 
      ref="formRef"
      :model="formData"
      :rules="formRules"
      label-position="top"
      size="default"
    >
      <!-- 基本信息 -->
      <div class="form-section">
        <h4 class="form-section-title">基本信息</h4>
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="姓名 *" prop="name">
              <el-input v-model="formData.name" placeholder="请输入姓名" clearable />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="性别" prop="gender">
              <el-select v-model="formData.gender" placeholder="请选择性别" clearable>
                <el-option label="" value="" />
                <el-option label="男" value="男" />
                <el-option label="女" value="女" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="身份证号" prop="id_number">
              <el-input v-model="formData.id_number" @change="extractIdInfo" placeholder="请输入18位身份证号" clearable />
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="出生日期">
              <el-date-picker 
                v-model="formData.birth_date" 
                type="date"
                placeholder="自动计算"
                format="YYYY-MM-DD"
                disabled
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="年龄">
              <el-input v-model="formData.age" readonly placeholder="自动计算" disabled />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="民族" prop="ethnicity">
              <el-select v-model="formData.ethnicity" placeholder="请选择或输入民族" filterable allow-create default-first-option clearable>
                <el-option v-for="option in ethnicityOptions" :key="option" :label="option" :value="option" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="籍贯" prop="native_place">
              <el-input v-model="formData.native_place" placeholder="请输入籍贯" clearable />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="出生地" prop="birth_place">
              <el-input v-model="formData.birth_place" placeholder="请输入出生地" clearable />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="联系电话" prop="phone">
              <el-input v-model="formData.phone" placeholder="请输入联系电话" clearable />
            </el-form-item>
          </el-col>
        </el-row>
      </div>
      
      <!-- 工作信息 -->
      <div class="form-section">
        <h4 class="form-section-title">工作信息</h4>
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="部门">
              <el-select v-model="formData.department" placeholder="请选择或输入部门名称" filterable allow-create default-first-option clearable>
                <el-option v-for="option in departmentOptions" :key="option" :label="option" :value="option" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col v-if="!isMidlevel" :span="8">
            <el-form-item label="科室">
              <el-input v-model="formData.section" placeholder="请输入科室名称" clearable />
            </el-form-item>
          </el-col>
          <el-col :span="isMidlevel ? 16 : 8">
            <el-form-item label="职务1">
              <el-select v-model="formData.position1" placeholder="请选择或输入职务" filterable allow-create default-first-option clearable>
                <el-option v-for="option in positionOptions" :key="option" :label="option" :value="option" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="职务2">
              <el-input v-model="formData.position2" placeholder="请输入职务2" clearable />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="入司日期">
              <el-date-picker 
                v-model="formData.company_entry_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                @change="calculateCompanyTenure"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="司龄（年）">
              <el-input v-model="formData.company_tenure" placeholder="自动计算" disabled />
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="参加工作时间">
              <el-date-picker 
                v-model="formData.work_start_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                @change="calculateWorkTenure"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="工龄（年）">
              <el-input v-model="formData.work_tenure" placeholder="自动计算" disabled />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="任现职级时间">
              <el-date-picker 
                v-model="formData.current_level_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="任职时间">
              <el-date-picker 
                v-model="formData.position_entry_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                @change="calculateProbationEnd"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="试用期（年）">
              <el-input-number v-model="formData.probation_period" :min="0" :max="10" :step="0.1" @change="calculateProbationEnd" placeholder="请输入试用期" controls-position="right" style="width: 100%" />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="试用期满到期提醒">
              <el-date-picker 
                v-model="formData.probation_end_reminder" 
                type="date"
                placeholder="自动计算"
                format="YYYY-MM-DD"
                disabled
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
        </el-row>
      </div>
      
      <!-- 教育背景 -->
      <div class="form-section">
        <h4 class="form-section-title">教育背景</h4>
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="最高学历">
              <el-select v-model="formData.education" placeholder="请选择或输入学历" filterable allow-create default-first-option clearable>
                <el-option v-for="option in educationOptions" :key="option" :label="option" :value="option" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="全日制学历">
              <el-select v-model="formData.full_time_education" placeholder="请选择或输入学历" filterable allow-create default-first-option clearable>
                <el-option v-for="option in educationOptions" :key="option" :label="option" :value="option" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="全日制毕业院校系及专业">
              <el-input v-model="formData.full_time_school_major" placeholder="请输入毕业院校系及专业" clearable />
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="在职学历">
              <el-select v-model="formData.part_time_education" placeholder="请选择或输入学历" filterable allow-create default-first-option clearable>
                <el-option v-for="option in educationOptions" :key="option" :label="option" :value="option" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="在职毕业院校系及专业">
              <el-input v-model="formData.part_time_school_phone" placeholder="请输入在职毕业院校系及专业" clearable />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="专业技术职务">
              <el-select v-model="formData.technical_position" placeholder="请选择或输入专业技术职务" filterable allow-create default-first-option clearable>
                <el-option v-for="option in technicalPositionOptions" :key="option" :label="option" :value="option" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
      </div>
      
      <!-- 政治面貌 -->
      <div class="form-section">
        <h4 class="form-section-title">政治面貌</h4>
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="政治面貌">
              <el-select v-model="formData.political_status" placeholder="请选择政治面貌" clearable>
                <el-option label="" value="" />
                <el-option label="中共党员" value="中共党员" />
                <el-option label="预备党员" value="预备党员" />
                <el-option label="共青团员" value="共青团员" />
                <el-option label="民革党员" value="民革党员" />
                <el-option label="民盟盟员" value="民盟盟员" />
                <el-option label="民建会员" value="民建会员" />
                <el-option label="民进会员" value="民进会员" />
                <el-option label="农工党党员" value="农工党党员" />
                <el-option label="致公党党员" value="致公党党员" />
                <el-option label="九三学社社员" value="九三学社社员" />
                <el-option label="台盟盟员" value="台盟盟员" />
                <el-option label="无党派人士" value="无党派人士" />
                <el-option label="群众" value="群众" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="入党时间">
              <el-date-picker 
                v-model="formData.party_entry_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="备注">
              <el-input v-model="formData.remarks" placeholder="请输入备注" clearable />
            </el-form-item>
          </el-col>
        </el-row>
      </div>
      
      <!-- 任职信息 -->
      <div class="form-section">
        <h4 class="form-section-title">任职信息</h4>
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="任基层副职时间">
              <el-date-picker 
                v-model="formData.grassroots_vice_position_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                @change="calculateGrassrootsViceTenure"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="任基层副职年限">
              <el-input v-model="formData.grassroots_vice_tenure" placeholder="自动计算" disabled />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="任基层正职时间">
              <el-date-picker 
                v-model="formData.grassroots_chief_position_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                @change="calculateGrassrootsChiefTenure"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="任基层正职年限">
              <el-input v-model="formData.grassroots_chief_tenure" placeholder="自动计算" disabled />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="任中层助理层级时间">
              <el-date-picker 
                v-model="formData.midlevel_assistant_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                @change="calculateMidlevelAssistantTenure"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="任中层助理年限">
              <el-input v-model="formData.midlevel_assistant_tenure" placeholder="自动计算" disabled />
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="任中层副职时间">
              <el-date-picker 
                v-model="formData.midlevel_vice_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                @change="calculateMidlevelViceTenure"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="任中层副职年限">
              <el-input v-model="formData.midlevel_vice_tenure" placeholder="自动计算" disabled />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="任中层正职时间">
              <el-date-picker 
                v-model="formData.midlevel_chief_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                @change="calculateMidlevelChiefTenure"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
        </el-row>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="任中层正职年限">
              <el-input v-model="formData.midlevel_chief_tenure" placeholder="自动计算" disabled />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="同部门任职时间">
              <el-date-picker 
                v-model="formData.same_department_date" 
                type="date"
                placeholder="请选择日期"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
                @change="calculateSameDepartmentTenure"
                clearable
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="同部门任职年限">
              <el-input v-model="formData.same_department_tenure" placeholder="自动计算" disabled />
            </el-form-item>
          </el-col>
        </el-row>
      </div>
    </el-form>
    
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button type="primary" @click="handleSubmit">保存</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 定义组件的属性
const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  },
  cadre: {
    type: Object,
    default: () => ({
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
      grassroots_vice_position_date: "",
      grassroots_vice_tenure: null,
      grassroots_chief_position_date: "",
      grassroots_chief_tenure: null,
      midlevel_assistant_date: "",
      midlevel_assistant_tenure: null,
      midlevel_vice_date: "",
      midlevel_vice_tenure: null,
      midlevel_chief_date: "",
      midlevel_chief_tenure: null,
      same_department_date: "",
      same_department_tenure: null,
      remarks: "",
      major: "",
      contact_date: "",
      special_date: ""
    })
  },
  isMidlevel: {
    type: Boolean,
    default: false
  }
});

// 定义组件触发的事件
const emit = defineEmits(['update:modelValue', 'save']);

// 表单引用
const formRef = ref(null);

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
const formRules = computed(() => {
  const rules = {
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
    position1: [
      { required: true, message: '请选择或输入职务1', trigger: 'change' }
    ],
    phone: [
      { pattern: /^1[3-9]\d{9}$/, message: '请输入有效的手机号码', trigger: 'blur' }
    ]
  };
  
  // 如果不是中层管理人员，则科室字段为必填
  if (!props.isMidlevel) {
    rules.section = [
      { required: true, message: '请输入科室', trigger: 'blur' }
    ];
  }
  
  return rules;
});

// 计算属性：控制弹窗显示/隐藏
const visible = computed({
  get() {
    return props.modelValue;
  },
  set(value) {
    emit('update:modelValue', value);
  }
});

// 计算属性：弹窗标题
const title = computed(() => {
  return props.cadre.id ? '编辑干部信息' : '新增干部信息';
});

// 表单数据
const formData = ref({ ...props.cadre });

// 监听props.cadre变化，更新formData
watch(() => props.cadre, (newCadre) => {
  // 直接使用父组件传递的数据，包括已计算的字段
  formData.value = { ...newCadre };
  
  // 处理日期字段，确保它们是正确的格式
  const dateFields = [
    'company_entry_date',
    'work_start_date',
    'current_level_date',
    'position_entry_date',
    'probation_end_reminder',
    'birth_date',
    'party_entry_date',
    'grassroots_vice_position_date',
    'grassroots_chief_position_date',
    'midlevel_assistant_date',
    'midlevel_vice_date',
    'midlevel_chief_date'
  ];
  
  dateFields.forEach(field => {
    if (formData.value[field] && typeof formData.value[field] === 'string') {
      // 如果是字符串日期，转换为Date对象
      const date = new Date(formData.value[field]);
      if (!isNaN(date.getTime())) {
        formData.value[field] = date;
      }
    }
  });
}, { deep: true });

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

// 关闭弹窗
function handleClose() {
  visible.value = false;
}

// 提交表单
async function handleSubmit() {
  try {
    // 表单验证
    await formRef.value.validate();
    
    // 创建一个新对象用于保存，确保日期字段被正确格式化
    const cadreToSave = { ...formData.value };
    
    // 格式化所有日期字段为字符串
    const dateFields = [
      'company_entry_date',
      'work_start_date',
      'current_level_date',
      'position_entry_date',
      'probation_end_reminder',
      'birth_date',
      'party_entry_date',
      'grassroots_vice_position_date',
      'grassroots_chief_position_date',
      'midlevel_assistant_date',
      'midlevel_vice_date',
      'midlevel_chief_date'
    ];
    
    dateFields.forEach(field => {
      if (cadreToSave[field] instanceof Date) {
        // 如果是Date对象，格式化为字符串
        const year = cadreToSave[field].getFullYear();
        const month = String(cadreToSave[field].getMonth() + 1).padStart(2, '0');
        const day = String(cadreToSave[field].getDate()).padStart(2, '0');
        cadreToSave[field] = `${year}-${month}-${day}`;
      } else if (cadreToSave[field]) {
        // 如果是字符串，直接使用formatDate函数处理
        cadreToSave[field] = formatDate(cadreToSave[field]);
      } else {
        // 如果是空值，保持为null或空字符串
        cadreToSave[field] = null;
      }
    });
    
    // 确保数字字段被正确处理
    if (cadreToSave.probation_period !== null && cadreToSave.probation_period !== undefined && cadreToSave.probation_period !== "") {
      cadreToSave.probation_period = cadreToSave.probation_period.toString();
    } else {
      cadreToSave.probation_period = null;
    }
    
    // 其他数字字段保持原样，因为后端期望的是数字类型
    // age, company_tenure, work_tenure 在后端是数字类型，不需要转换为字符串
    
    // 触发保存事件
    emit('save', cadreToSave);
    
    // 关闭弹窗
    handleClose();
  } catch (error) {
    console.error("保存干部信息失败:", error);
    if (error.name === 'ValidationError') {
      console.error("表单验证失败:", error);
    } else {
      console.error("保存干部信息失败:", error);
    }
  }
}

// 计算司龄
function calculateCompanyTenure() {
  if (formData.value.company_entry_date) {
    // 确保日期是Date对象
    const entryDate = formData.value.company_entry_date instanceof Date 
      ? formData.value.company_entry_date 
      : new Date(formData.value.company_entry_date);
    
    if (isNaN(entryDate.getTime())) {
      formData.value.company_tenure = null;
      return;
    }
    
    const today = new Date();
    const diffTime = Math.abs(today - entryDate);
    const diffYears = diffTime / (1000 * 60 * 60 * 24 * 365);
    formData.value.company_tenure = parseFloat(diffYears.toFixed(1));
  } else {
    formData.value.company_tenure = null;
  }
}

// 计算工龄
function calculateWorkTenure() {
  if (formData.value.work_start_date) {
    // 确保日期是Date对象
    const startDate = formData.value.work_start_date instanceof Date 
      ? formData.value.work_start_date 
      : new Date(formData.value.work_start_date);
    
    if (isNaN(startDate.getTime())) {
      formData.value.work_tenure = null;
      return;
    }
    
    const today = new Date();
    const diffTime = Math.abs(today - startDate);
    const diffYears = diffTime / (1000 * 60 * 60 * 24 * 365);
    formData.value.work_tenure = parseFloat(diffYears.toFixed(1));
  } else {
    formData.value.work_tenure = null;
  }
}

// 计算试用期满到期提醒
function calculateProbationEnd() {
  // 只有当试用期和任职时间都存在时才计算
  if (formData.value.probation_period && formData.value.position_entry_date) {
    // 确保日期是Date对象
    const positionDate = formData.value.position_entry_date instanceof Date 
      ? formData.value.position_entry_date 
      : new Date(formData.value.position_entry_date);
    
    if (isNaN(positionDate.getTime())) {
      formData.value.probation_end_reminder = "";
      return;
    }
    
    const probationYears = parseFloat(formData.value.probation_period);
    
    // 计算试用期结束日期
    const endDate = new Date(positionDate);
    endDate.setFullYear(endDate.getFullYear() + Math.floor(probationYears));
    endDate.setMonth(endDate.getMonth() + Math.round((probationYears % 1) * 12));
    
    // 直接设置日期对象
    formData.value.probation_end_reminder = endDate;
  } else {
    formData.value.probation_end_reminder = "";
  }
}

// 从身份证号提取出生日期和年龄
function extractIdInfo() {
  const idNumber = formData.value.id_number;
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
      formData.value.birth_date = birthDate;
      
      // 计算年龄
      const today = new Date();
      let age = today.getFullYear() - birthDate.getFullYear();
      const monthDiff = today.getMonth() - birthDate.getMonth();
      
      // 如果还没过生日，则年龄减1
      if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
        age--;
      }
      
      formData.value.age = age;
    } else {
      formData.value.birth_date = "";
      formData.value.age = null;
    }
  } else {
    formData.value.birth_date = "";
    formData.value.age = null;
  }
}

// 计算任基层副职年限
function calculateGrassrootsViceTenure() {
  // 如果有任基层正职时间，则用正职时间减去副职时间
  if (formData.value.grassroots_chief_position_date && formData.value.grassroots_vice_position_date) {
    const viceDate = formData.value.grassroots_vice_position_date instanceof Date 
      ? formData.value.grassroots_vice_position_date 
      : new Date(formData.value.grassroots_vice_position_date);
      
    const chiefDate = formData.value.grassroots_chief_position_date instanceof Date 
      ? formData.value.grassroots_chief_position_date 
      : new Date(formData.value.grassroots_chief_position_date);
    
    if (!isNaN(viceDate.getTime()) && !isNaN(chiefDate.getTime()) && chiefDate >= viceDate) {
      const diffTime = chiefDate - viceDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.grassroots_vice_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有基层正职时间，则按照当前时间减去副职时间
  if (formData.value.grassroots_vice_position_date) {
    const viceDate = formData.value.grassroots_vice_position_date instanceof Date 
      ? formData.value.grassroots_vice_position_date 
      : new Date(formData.value.grassroots_vice_position_date);
      
    if (!isNaN(viceDate.getTime())) {
      const today = new Date();
      const diffTime = today - viceDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.grassroots_vice_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有足够的信息计算，则清空年限
  formData.value.grassroots_vice_tenure = "";
}

// 计算任基层正职年限
function calculateGrassrootsChiefTenure() {
  // 如果有任中层助理层级时间，则用任中层助理层级时间减去任基层正职时间
  if (formData.value.midlevel_assistant_date && formData.value.grassroots_chief_position_date) {
    const chiefDate = formData.value.grassroots_chief_position_date instanceof Date 
      ? formData.value.grassroots_chief_position_date 
      : new Date(formData.value.grassroots_chief_position_date);
      
    const assistantDate = formData.value.midlevel_assistant_date instanceof Date 
      ? formData.value.midlevel_assistant_date 
      : new Date(formData.value.midlevel_assistant_date);
    
    if (!isNaN(chiefDate.getTime()) && !isNaN(assistantDate.getTime()) && assistantDate >= chiefDate) {
      const diffTime = assistantDate - chiefDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.grassroots_chief_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有任中层助理层级时间，则使用当前时间减去任基层正职时间
  if (formData.value.grassroots_chief_position_date) {
    const chiefDate = formData.value.grassroots_chief_position_date instanceof Date 
      ? formData.value.grassroots_chief_position_date 
      : new Date(formData.value.grassroots_chief_position_date);
      
    if (!isNaN(chiefDate.getTime())) {
      const today = new Date();
      const diffTime = today - chiefDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.grassroots_chief_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有足够的信息计算，则清空年限
  formData.value.grassroots_chief_tenure = "";
}

// 计算任中层助理年限
function calculateMidlevelAssistantTenure() {
  // 如果有任中层副职时间，则用任中层副职时间减去任中层助理层级时间
  if (formData.value.midlevel_vice_date && formData.value.midlevel_assistant_date) {
    const assistantDate = formData.value.midlevel_assistant_date instanceof Date 
      ? formData.value.midlevel_assistant_date 
      : new Date(formData.value.midlevel_assistant_date);
      
    const viceDate = formData.value.midlevel_vice_date instanceof Date 
      ? formData.value.midlevel_vice_date 
      : new Date(formData.value.midlevel_vice_date);
    
    if (!isNaN(assistantDate.getTime()) && !isNaN(viceDate.getTime()) && viceDate >= assistantDate) {
      const diffTime = viceDate - assistantDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.midlevel_assistant_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有任中层副职时间，则使用当前时间减去任中层助理层级时间
  if (formData.value.midlevel_assistant_date) {
    const assistantDate = formData.value.midlevel_assistant_date instanceof Date 
      ? formData.value.midlevel_assistant_date 
      : new Date(formData.value.midlevel_assistant_date);
      
    if (!isNaN(assistantDate.getTime())) {
      const today = new Date();
      const diffTime = today - assistantDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.midlevel_assistant_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有足够的信息计算，则清空年限
  formData.value.midlevel_assistant_tenure = "";
}

// 计算任中层副职年限
function calculateMidlevelViceTenure() {
  // 如果有任中层正职时间，则用任中层正职时间减去任中层副职时间
  if (formData.value.midlevel_chief_date && formData.value.midlevel_vice_date) {
    const viceDate = formData.value.midlevel_vice_date instanceof Date 
      ? formData.value.midlevel_vice_date 
      : new Date(formData.value.midlevel_vice_date);
      
    const chiefDate = formData.value.midlevel_chief_date instanceof Date 
      ? formData.value.midlevel_chief_date 
      : new Date(formData.value.midlevel_chief_date);
    
    if (!isNaN(viceDate.getTime()) && !isNaN(chiefDate.getTime()) && chiefDate >= viceDate) {
      const diffTime = chiefDate - viceDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.midlevel_vice_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有任中层正职时间，则使用当前时间减去任中层副职时间
  if (formData.value.midlevel_vice_date) {
    const viceDate = formData.value.midlevel_vice_date instanceof Date 
      ? formData.value.midlevel_vice_date 
      : new Date(formData.value.midlevel_vice_date);
      
    if (!isNaN(viceDate.getTime())) {
      const today = new Date();
      const diffTime = today - viceDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.midlevel_vice_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有足够的信息计算，则清空年限
  formData.value.midlevel_vice_tenure = "";
}

// 计算任中层正职年限
function calculateMidlevelChiefTenure() {
  // 直接使用当前时间减去任中层正职时间
  if (formData.value.midlevel_chief_date) {
    const chiefDate = formData.value.midlevel_chief_date instanceof Date 
      ? formData.value.midlevel_chief_date 
      : new Date(formData.value.midlevel_chief_date);
      
    if (!isNaN(chiefDate.getTime())) {
      const today = new Date();
      const diffTime = today - chiefDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.midlevel_chief_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有足够的信息计算，则清空年限
  formData.value.midlevel_chief_tenure = "";
}

// 计算同部门任职年限
function calculateSameDepartmentTenure() {
  // 使用当前时间减去同部门任职时间
  if (formData.value.same_department_date) {
    const sameDepartmentDate = formData.value.same_department_date instanceof Date 
      ? formData.value.same_department_date 
      : new Date(formData.value.same_department_date);
      
    if (!isNaN(sameDepartmentDate.getTime())) {
      const today = new Date();
      const diffTime = today - sameDepartmentDate;
      const diffYears = Math.floor(diffTime / (1000 * 60 * 60 * 24 * 365));
      const diffDays = Math.floor((diffTime % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24));
      const diffMonths = Math.floor(diffDays / 30);
      
      formData.value.same_department_tenure = `${diffYears}年${diffMonths}月`;
      return;
    }
  }
  
  // 如果没有足够的信息计算，则清空年限
  formData.value.same_department_tenure = "";
}
</script>

<style scoped>
.form-section {
  margin-bottom: 30px;
  padding: 20px;
  background: #f8f9fa;
  border-radius: 8px;
}

.form-section-title {
  margin-top: 0;
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 1px solid #dee2e6;
  color: #495057;
}

.dialog-footer {
  text-align: right;
}
</style>