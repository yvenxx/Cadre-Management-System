<template>
  <div class="tenure-view">
    <el-card class="content-section">
      <template #header>
        <div class="section-header">
          <h2>中层任职年限表</h2>
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
                <el-select v-model="filters.department" placeholder="请选择部门" clearable filterable>
                  <el-option
                    v-for="dept in distinctDepartments"
                    :key="dept"
                    :label="dept"
                    :value="dept"
                  />
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="职务1">
                <el-select v-model="filters.position1" placeholder="请选择职务1" clearable filterable>
                  <el-option
                    v-for="position in distinctPositions1"
                    :key="position"
                    :label="position"
                    :value="position"
                  />
                </el-select>
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="16">
            <el-col :span="6">
              <el-form-item label="政治面貌">
                <el-select v-model="filters.politicalStatus" placeholder="请选择政治面貌" clearable filterable>
                  <el-option
                    v-for="status in distinctPoliticalStatuses"
                    :key="status"
                    :label="status"
                    :value="status"
                  />
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="专业技术职务">
                <el-select v-model="filters.technicalPosition" placeholder="请选择专业技术职务" clearable filterable>
                  <el-option
                    v-for="position in distinctTechnicalPositions"
                    :key="position"
                    :label="position"
                    :value="position"
                  />
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="6">
              <el-form-item label="民族">
                <el-select v-model="filters.ethnicity" placeholder="请选择民族" clearable filterable>
                  <el-option
                    v-for="ethnicity in distinctEthnicities"
                    :key="ethnicity"
                    :label="ethnicity"
                    :value="ethnicity"
                  />
                </el-select>
              </el-form-item>
            </el-col>
          </el-row>

          <div class="filter-actions">
            <el-button type="primary" @click="applyFilters" :icon="Search">应用筛选</el-button>
            <el-button @click="clearFilters" :icon="RefreshRight">清空筛选</el-button>
          </div>
        </div>
      </div>

      <!-- 导出按钮组 -->
      <div class="export-buttons-container">
        <div class="export-buttons-info">
          <span class="export-count-info">
            已选择: {{ selectedCadres.length }} 条记录
          </span>
        </div>
        <div class="export-buttons-group">
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
        max-height="600"
      >
        <el-table-column type="selection" width="55" fixed />
        <el-table-column type="index" label="序号" width="60" fixed />
        <el-table-column prop="name" label="姓名" width="100" fixed />
        <el-table-column prop="gender" label="性别" width="60" />
        <el-table-column prop="position1" label="现任职务" width="120" />
        <el-table-column prop="position2" label="层级" width="120" />
        <el-table-column prop="company_entry_date" label="入司时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.company_entry_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="company_tenure" label="司龄（年）" width="100" />
        <el-table-column prop="grassroots_vice_position_date" label="任基层副职时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.grassroots_vice_position_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="grassroots_vice_tenure" label="任基层副职年限" width="120" />
        <el-table-column prop="grassroots_chief_position_date" label="任基层正职时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.grassroots_chief_position_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="grassroots_chief_tenure" label="任基层正职年限" width="120" />
        <el-table-column prop="midlevel_assistant_date" label="任中层助理时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.midlevel_assistant_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="midlevel_assistant_tenure" label="任中层助理年限" width="120" />
        <el-table-column prop="midlevel_vice_date" label="任中层副职时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.midlevel_vice_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="midlevel_vice_tenure" label="任中层副职年限" width="120" />
        <el-table-column prop="midlevel_chief_date" label="任中层正职时间" width="120">
          <template #default="scope">
            {{ formatDate(scope.row.midlevel_chief_date) }}
          </template>
        </el-table-column>
        <el-table-column prop="midlevel_chief_tenure" label="任中层正职年限" width="120" />
        <el-table-column prop="same_department_tenure" label="同部门任职年限" width="120" />
        <el-table-column prop="remarks" label="备注" width="150" />
      </el-table>
    </el-card>
    
    <!-- 导出配置弹窗 -->
    <ExportConfig 
      v-model="showExportModal"
      :selected-cadres="selectedCadres"
      :export-fields="exportFields"
      :default-file-name="exportDefaultFileName"
      @export="performExport"
    />
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { save } from '@tauri-apps/plugin-dialog';
import { Search, RefreshRight, Upload, Download, ArrowUp, ArrowDown } from '@element-plus/icons-vue';
import ExportConfig from '../components/ExportConfig.vue';

// 日期格式化辅助函数
function formatDate(date) {
  try {
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
  } catch (error) {
    console.error("日期格式化错误:", error, "输入值:", date);
    return "";
  }
}

const showExportModal = ref(false);
const exportDefaultFileName = ref("中层任职年限信息");
const exportFilteredData = ref(false); // 标识是否导出筛选后的数据
const selectedCadres = ref([]); // 用于存储选中的干部

// 存储字段的distinct值
const distinctDepartments = ref([]); // 部门
const distinctPositions1 = ref([]); // 职务1
const distinctTechnicalPositions = ref([]); // 专业技术职务
const distinctEthnicities = ref([]); // 民族
const distinctPoliticalStatuses = ref([]); // 政治面貌

const showFilterPanel = ref(false);

const filters = ref({
  name: "",
  gender: "",
  department: "",
  position1: "",
  technicalPosition: "",
  ethnicity: "",
  politicalStatus: "",
});

const cadreList = ref([]);

// 切换筛选面板显示状态
function toggleFilterPanel() {
  showFilterPanel.value = !showFilterPanel.value;
}

// 应用筛选条件
function applyFilters() {
  // 筛选逻辑将在模板中通过计算属性实现
}

// 清空筛选条件
function clearFilters() {
  filters.value = {
    name: "",
    gender: "",
    department: "",
    position1: "",
    technicalPosition: "",
    ethnicity: "",
    politicalStatus: "",
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
    if (filters.value.department && cadre.department !== filters.value.department) {
      return false;
    }
    
    // 职务1筛选
    if (filters.value.position1 && cadre.position1 !== filters.value.position1) {
      return false;
    }
    
    // 专业技术职务筛选
    if (filters.value.technicalPosition && cadre.technical_position !== filters.value.technicalPosition) {
      return false;
    }
    
    // 民族筛选
    if (filters.value.ethnicity && cadre.ethnicity !== filters.value.ethnicity) {
      return false;
    }
    
    // 政治面貌筛选
    if (filters.value.politicalStatus && cadre.political_status !== filters.value.politicalStatus) {
      return false;
    }
    
    return true;
  });
});

// 所有可导出字段的配置
const exportFields = [
  { key: "name", label: "姓名" },
  { key: "gender", label: "性别" },
  { key: "position1", label: "现任职务" },
  { key: "position2", label: "层级" },
  { key: "company_entry_date", label: "入司时间" },
  { key: "company_tenure", label: "司龄（年）" },
  { key: "grassroots_vice_position_date", label: "任基层副职时间" },
  { key: "grassroots_vice_tenure", label: "任基层副职年限" },
  { key: "grassroots_chief_position_date", label: "任基层正职时间" },
  { key: "grassroots_chief_tenure", label: "任基层正职年限" },
  { key: "midlevel_assistant_date", label: "任中层助理时间" },
  { key: "midlevel_assistant_tenure", label: "任中层助理年限" },
  { key: "midlevel_vice_date", label: "任中层副职时间" },
  { key: "midlevel_vice_tenure", label: "任中层副职年限" },
  { key: "midlevel_chief_date", label: "任中层正职时间" },
  { key: "midlevel_chief_tenure", label: "任中层正职年限" },
  { key: "same_department_tenure", label: "同部门任职年限" },
  { key: "remarks", label: "备注" }
];

// 加载所有中层干部信息
async function loadCadreInfo() {
  try {
    const data = await invoke("get_all_midlevel_cadre_info");
    // 添加数据验证
    if (Array.isArray(data)) {
      cadreList.value = data;
    } else {
      console.error("获取的数据格式不正确:", data);
      cadreList.value = [];
    }
  } catch (error) {
    console.error("加载中层干部信息失败:", error);
    // 确保即使出错也不会导致界面崩溃
    cadreList.value = [];
  }
}

// 获取字段的distinct值
async function loadDistinctFieldValues() {
  try {
    // 获取部门的distinct值
    distinctDepartments.value = await invoke("get_distinct_field_values_for_table", { tableName: "midlevel_cadres", fieldName: "department" });
    
    // 获取职务1的distinct值
    distinctPositions1.value = await invoke("get_distinct_field_values_for_table", { tableName: "midlevel_cadres", fieldName: "position1" });
    
    // 获取专业技术职务的distinct值
    distinctTechnicalPositions.value = await invoke("get_distinct_field_values_for_table", { tableName: "midlevel_cadres", fieldName: "technical_position" });
    
    // 获取民族的distinct值
    distinctEthnicities.value = await invoke("get_distinct_field_values_for_table", { tableName: "midlevel_cadres", fieldName: "ethnicity" });
    
    // 获取政治面貌的distinct值
    distinctPoliticalStatuses.value = await invoke("get_distinct_field_values_for_table", { tableName: "midlevel_cadres", fieldName: "political_status" });
  } catch (error) {
    console.error("获取字段distinct值失败:", error);
  }
}

// 导出选中中层干部信息
async function exportSelectedCadres() {
  if (selectedCadres.value.length === 0) {
    alert("请先选择要导出的中层干部信息");
    return;
  }
  
  try {
    // 设置默认文件名
    exportDefaultFileName.value = "选中中层任职年限信息";
    // 重置导出筛选数据标识
    exportFilteredData.value = false;
    // 打开导出配置弹窗
    showExportModal.value = true;
  } catch (error) {
    console.error("导出选中中层干部信息失败:", error);
    alert("导出失败: " + error);
  }
}

// 导出全部中层干部信息
async function exportAllCadres() {
  try {
    // 设置默认文件名
    exportDefaultFileName.value = "全部中层任职年限信息";
    // 标识为导出筛选后的数据
    exportFilteredData.value = true;
    // 打开导出配置弹窗
    showExportModal.value = true;
  } catch (error) {
    console.error("导出全部中层干部信息失败:", error);
    alert("导出失败: " + error);
  }
}

// 执行导出操作
async function performExport(exportData) {
  try {
    // 弹出文件保存对话框，让用户选择保存位置
    const filePath = await save({
      filters: [{
        name: 'Excel Files',
        extensions: ['xlsx']
      }],
      defaultPath: `${exportData.fileName}.xlsx`
    });
    
    // 如果用户取消了保存对话框，则不执行导出
    if (!filePath) {
      closeExportModal();
      return;
    }
    
    // 根据导出类型确定要导出的数据ID列表
    let cadreIds = exportData.cadreIds; // 默认使用传入的ID列表（选中导出）
    
    // 如果是导出筛选后的全部数据且没有选中特定干部
    if (exportFilteredData.value && !exportData.cadreIds) {
      // 使用筛选后的数据ID列表
      cadreIds = filteredCadreList.value.map(cadre => cadre.id);
    }
    
    // 重置导出筛选数据标识
    exportFilteredData.value = false;
    
    await invoke("export_midlevel_cadre_info_to_excel", { 
      filePath, 
      selectedFields: exportData.selectedFields,
      cadreIds // 如果为null则导出全部
    });
    
    closeExportModal();
    alert("导出成功！文件已保存为: " + filePath);
  } catch (error) {
    console.error("导出失败:", error);
    alert("导出失败: " + error);
  }
}

// 组件挂载时加载数据
onMounted(() => {
  loadCadreInfo();
  loadDistinctFieldValues();
});
</script>

<style scoped>
.tenure-view {
  height: 100%;
}

.content-section {
  background: white;
  border-radius: 16px;
  box-shadow: var(--card-shadow);
  position: relative;
  overflow: visible;
  border: 1px solid rgba(0, 0, 0, 0.05);
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

/* 导出按钮容器样式 */
.export-buttons-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding: 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 12px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  color: white;
}

.export-buttons-info {
  flex: 1;
}

.export-count-info {
  font-size: 16px;
  font-weight: 500;
  color: white;
  background: rgba(255, 255, 255, 0.2);
  padding: 8px 16px;
  border-radius: 20px;
  backdrop-filter: blur(10px);
}

.export-buttons-group {
  display: flex;
  gap: 10px;
}

.filter-actions {
  margin-top: 20px;
  text-align: center;
}

.filter-actions .el-button {
  margin: 0 10px;
}

/* 筛选面板样式 */
.filter-content {
  padding: 20px;
  background: #f9f9f9;
  border-radius: 8px;
  margin-top: 15px;
}

.filter-content .el-form-item {
  margin-bottom: 18px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .export-buttons-container {
    flex-direction: column;
    gap: 15px;
  }
  
  .export-count-info {
    text-align: center;
  }
  
  .export-buttons-group {
    width: 100%;
    justify-content: center;
  }
}
</style>