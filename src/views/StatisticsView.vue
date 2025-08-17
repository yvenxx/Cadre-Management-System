<template>
  <div class="statistics-view">
    <el-card class="content-section">
      <template #header>
        <div class="section-header">
          <h2>信息结构统计</h2>
        </div>
      </template>
      
      <!-- 统计信息概览 -->
      <el-row :gutter="20" style="margin-bottom: 30px;">
        <el-col :span="6">
          <el-card class="overview-card">
            <div class="overview-content">
              <h3>总人数</h3>
              <p class="overview-number">{{ cadreList.length }}</p>
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card class="overview-card">
            <div class="overview-content">
              <h3>部门数</h3>
              <p class="overview-number">{{ Object.keys(departmentTableData.reduce((acc, item) => { acc[item.department] = true; return acc; }, {})).length }}</p>
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card class="overview-card">
            <div class="overview-content">
              <h3>职务数</h3>
              <p class="overview-number">{{ Object.keys(positionTableData.reduce((acc, item) => { acc[item.position] = true; return acc; }, {})).length }}</p>
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card class="overview-card">
            <div class="overview-content">
              <h3>最高学历类型</h3>
              <p class="overview-number">{{ educationTableData.length }}</p>
            </div>
          </el-card>
        </el-col>
      </el-row>
      
      <!-- 各项统计详情 -->
      <el-row :gutter="20">
        <!-- 性别分布 -->
        <el-col :span="12">
          <el-card class="stat-card">
            <template #header>
              <h3>性别分布</h3>
            </template>
            <el-table :data="genderTableData" style="width: 100%" stripe>
              <el-table-column prop="gender" label="性别" />
              <el-table-column prop="count" label="人数" />
              <el-table-column prop="percentage" label="百分比">
                <template #default="scope">
                  {{ scope.row.percentage }}%
                </template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
        
        <!-- 部门分布 -->
        <el-col :span="12">
          <el-card class="stat-card">
            <template #header>
              <h3>部门分布</h3>
            </template>
            <el-table :data="departmentTableData" style="width: 100%" stripe>
              <el-table-column prop="department" label="部门" />
              <el-table-column prop="count" label="人数" />
              <el-table-column prop="percentage" label="百分比">
                <template #default="scope">
                  {{ scope.row.percentage }}%
                </template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
      </el-row>
      
      <el-row :gutter="20" style="margin-top: 20px;">
        <!-- 年龄分布 -->
        <el-col :span="12">
          <el-card class="stat-card">
            <template #header>
              <h3>年龄分布</h3>
            </template>
            <el-table :data="ageTableData" style="width: 100%" stripe>
              <el-table-column prop="range" label="年龄范围" />
              <el-table-column prop="count" label="人数" />
              <el-table-column prop="percentage" label="百分比">
                <template #default="scope">
                  {{ scope.row.percentage }}%
                </template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
        
        <!-- 司龄分布 -->
        <el-col :span="12">
          <el-card class="stat-card">
            <template #header>
              <h3>司龄分布</h3>
            </template>
            <el-table :data="tenureTableData" style="width: 100%" stripe>
              <el-table-column prop="range" label="司龄范围" />
              <el-table-column prop="count" label="人数" />
              <el-table-column prop="percentage" label="百分比">
                <template #default="scope">
                  {{ scope.row.percentage }}%
                </template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
      </el-row>
      
      <el-row :gutter="20" style="margin-top: 20px;">
        <!-- 职务分布 -->
        <el-col :span="12">
          <el-card class="stat-card">
            <template #header>
              <h3>职务分布</h3>
            </template>
            <el-table :data="positionTableData" style="width: 100%" stripe>
              <el-table-column prop="position" label="职务" />
              <el-table-column prop="count" label="人数" />
              <el-table-column prop="percentage" label="百分比">
                <template #default="scope">
                  {{ scope.row.percentage }}%
                </template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
        
        <!-- 最高学历分布 -->
        <el-col :span="12">
          <el-card class="stat-card">
            <template #header>
              <h3>最高学历分布</h3>
            </template>
            <el-table :data="educationTableData" style="width: 100%" stripe>
              <el-table-column prop="education" label="最高学历" />
              <el-table-column prop="count" label="人数" />
              <el-table-column prop="percentage" label="百分比">
                <template #default="scope">
                  {{ scope.row.percentage }}%
                </template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
      </el-row>
      
      <el-row :gutter="20" style="margin-top: 20px;">
        <!-- 政治面貌分布 -->
        <el-col :span="12">
          <el-card class="stat-card">
            <template #header>
              <h3>政治面貌分布</h3>
            </template>
            <el-table :data="politicalTableData" style="width: 100%" stripe>
              <el-table-column prop="status" label="政治面貌" />
              <el-table-column prop="count" label="人数" />
              <el-table-column prop="percentage" label="百分比">
                <template #default="scope">
                  {{ scope.row.percentage }}%
                </template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
        
        <!-- 全日制学历分布 -->
        <el-col :span="12">
          <el-card class="stat-card">
            <template #header>
              <h3>全日制学历分布</h3>
            </template>
            <el-table :data="fullTimeTableData" style="width: 100%" stripe>
              <el-table-column prop="education" label="全日制学历" />
              <el-table-column prop="count" label="人数" />
              <el-table-column prop="percentage" label="百分比">
                <template #default="scope">
                  {{ scope.row.percentage }}%
                </template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
      </el-row>
      
      <el-row :gutter="20" style="margin-top: 20px;">
        <!-- 在职学历分布 -->
        <el-col :span="12">
          <el-card class="stat-card">
            <template #header>
              <h3>在职学历分布</h3>
            </template>
            <el-table :data="partTimeTableData" style="width: 100%" stripe>
              <el-table-column prop="education" label="在职学历" />
              <el-table-column prop="count" label="人数" />
              <el-table-column prop="percentage" label="百分比">
                <template #default="scope">
                  {{ scope.row.percentage }}%
                </template>
              </el-table-column>
            </el-table>
          </el-card>
        </el-col>
      </el-row>
      
      <!-- 页脚 -->
      <Footer />
    </el-card>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Footer from '../components/Footer.vue';

// 活动标签页
const activeTab = ref("gender");

// 干部数据
const cadreList = ref([]);

// 加载所有干部信息
async function loadCadreInfo() {
  try {
    const data = await invoke("get_all_cadre_info");
    // 添加数据验证
    if (Array.isArray(data)) {
      cadreList.value = data;
    } else {
      console.error("获取的数据格式不正确:", data);
      cadreList.value = [];
    }
  } catch (error) {
    console.error("加载干部信息失败:", error);
    // 确保即使出错也不会导致界面崩溃
    cadreList.value = [];
  }
}

// 获取性别统计表格数据
const genderTableData = computed(() => {
  const stats = {};
  const total = cadreList.value.length;
  
  cadreList.value.forEach(cadre => {
    if (cadre.gender) {
      stats[cadre.gender] = (stats[cadre.gender] || 0) + 1;
    }
  });
  
  // 转换为表格数据格式
  return Object.entries(stats).map(([gender, count]) => ({
    gender,
    count,
    percentage: total > 0 ? ((count / total) * 100).toFixed(2) : 0
  }));
});

// 获取部门统计表格数据
const departmentTableData = computed(() => {
  const stats = {};
  const total = cadreList.value.length;
  
  cadreList.value.forEach(cadre => {
    if (cadre.department) {
      stats[cadre.department] = (stats[cadre.department] || 0) + 1;
    }
  });
  
  // 转换为表格数据格式
  return Object.entries(stats).map(([department, count]) => ({
    department,
    count,
    percentage: total > 0 ? ((count / total) * 100).toFixed(2) : 0
  }));
});

// 获取年龄分布统计表格数据
const ageTableData = computed(() => {
  const stats = {
    "30岁以下": 0,
    "30-40岁": 0,
    "40-50岁": 0,
    "50-60岁": 0,
    "60岁以上": 0
  };
  const total = cadreList.value.length;
  
  cadreList.value.forEach(cadre => {
    if (cadre.age !== null && cadre.age !== undefined) {
      const age = parseInt(cadre.age);
      if (age < 30) {
        stats["30岁以下"]++;
      } else if (age < 40) {
        stats["30-40岁"]++;
      } else if (age < 50) {
        stats["40-50岁"]++;
      } else if (age < 60) {
        stats["50-60岁"]++;
      } else {
        stats["60岁以上"]++;
      }
    }
  });
  
  // 转换为表格数据格式
  return Object.entries(stats).map(([range, count]) => ({
    range,
    count,
    percentage: total > 0 ? ((count / total) * 100).toFixed(2) : 0
  }));
});

// 获取司龄分布统计表格数据
const tenureTableData = computed(() => {
  const stats = {
    "5年以下": 0,
    "5-10年": 0,
    "10-20年": 0,
    "20-30年": 0,
    "30年以上": 0
  };
  const total = cadreList.value.length;
  
  cadreList.value.forEach(cadre => {
    if (cadre.company_tenure !== null && cadre.company_tenure !== undefined) {
      const tenure = parseFloat(cadre.company_tenure);
      if (tenure < 5) {
        stats["5年以下"]++;
      } else if (tenure < 10) {
        stats["5-10年"]++;
      } else if (tenure < 20) {
        stats["10-20年"]++;
      } else if (tenure < 30) {
        stats["20-30年"]++;
      } else {
        stats["30年以上"]++;
      }
    }
  });
  
  // 转换为表格数据格式
  return Object.entries(stats).map(([range, count]) => ({
    range,
    count,
    percentage: total > 0 ? ((count / total) * 100).toFixed(2) : 0
  }));
});

// 获取职务分布统计表格数据
const positionTableData = computed(() => {
  const stats = {};
  const total = cadreList.value.length;
  
  cadreList.value.forEach(cadre => {
    // 统计职务1和职务2
    if (cadre.position1) {
      stats[cadre.position1] = (stats[cadre.position1] || 0) + 1;
    }
    if (cadre.position2) {
      stats[cadre.position2] = (stats[cadre.position2] || 0) + 1;
    }
  });
  
  // 转换为表格数据格式
  return Object.entries(stats).map(([position, count]) => ({
    position,
    count,
    percentage: total > 0 ? ((count / total) * 100).toFixed(2) : 0
  }));
});

// 获取最高学历分布统计表格数据
const educationTableData = computed(() => {
  const stats = {};
  const total = cadreList.value.length;
  
  cadreList.value.forEach(cadre => {
    if (cadre.education) {
      stats[cadre.education] = (stats[cadre.education] || 0) + 1;
    }
  });
  
  // 转换为表格数据格式
  return Object.entries(stats).map(([education, count]) => ({
    education,
    count,
    percentage: total > 0 ? ((count / total) * 100).toFixed(2) : 0
  }));
});

// 获取政治面貌分布统计表格数据
const politicalTableData = computed(() => {
  const stats = {};
  const total = cadreList.value.length;
  
  cadreList.value.forEach(cadre => {
    if (cadre.political_status) {
      stats[cadre.political_status] = (stats[cadre.political_status] || 0) + 1;
    }
  });
  
  // 转换为表格数据格式
  return Object.entries(stats).map(([status, count]) => ({
    status,
    count,
    percentage: total > 0 ? ((count / total) * 100).toFixed(2) : 0
  }));
});

// 获取全日制学历分布统计表格数据
const fullTimeTableData = computed(() => {
  const stats = {};
  const total = cadreList.value.length;
  
  cadreList.value.forEach(cadre => {
    if (cadre.full_time_education) {
      stats[cadre.full_time_education] = (stats[cadre.full_time_education] || 0) + 1;
    }
  });
  
  // 转换为表格数据格式
  return Object.entries(stats).map(([education, count]) => ({
    education,
    count,
    percentage: total > 0 ? ((count / total) * 100).toFixed(2) : 0
  }));
});

// 获取在职学历分布统计表格数据
const partTimeTableData = computed(() => {
  const stats = {};
  const total = cadreList.value.length;
  
  cadreList.value.forEach(cadre => {
    if (cadre.part_time_education) {
      stats[cadre.part_time_education] = (stats[cadre.part_time_education] || 0) + 1;
    }
  });
  
  // 转换为表格数据格式
  return Object.entries(stats).map(([education, count]) => ({
    education,
    count,
    percentage: total > 0 ? ((count / total) * 100).toFixed(2) : 0
  }));
});

// 组件挂载时加载数据
onMounted(() => {
  loadCadreInfo();
});
</script>

<style scoped>
.statistics-view {
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

.section-header h2 {
  margin: 0;
  color: #333;
  font-size: 24px;
  font-weight: 600;
}

.overview-card {
  text-align: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
}

.overview-content h3 {
  margin: 0 0 10px 0;
  font-size: 16px;
  font-weight: 500;
}

.overview-number {
  margin: 0;
  font-size: 28px;
  font-weight: 700;
}

.stat-card {
  margin-bottom: 20px;
}

.stat-card :deep(.el-card__header) {
  padding: 15px 20px;
  background-color: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
}

.stat-card :deep(.el-card__header) h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #495057;
}
</style>