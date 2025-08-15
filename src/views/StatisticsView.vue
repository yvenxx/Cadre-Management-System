<template>
  <div class="statistics-view">
    <el-card class="content-section">
      <template #header>
        <div class="section-header">
          <h2>信息结构统计</h2>
        </div>
      </template>
      
      <div class="statistics-content">
        <el-row :gutter="20">
          <el-col :span="8">
            <el-card class="stat-card">
              <h3>📊 总人数统计</h3>
              <p class="stat-number">{{ cadreList.length }}</p>
            </el-card>
          </el-col>
          
          <el-col :span="8">
            <el-card class="stat-card">
              <h3>👨‍💼 按性别统计</h3>
              <p>男: {{ cadreList.filter(c => c.gender === '男').length }}</p>
              <p>女: {{ cadreList.filter(c => c.gender === '女').length }}</p>
            </el-card>
          </el-col>
          
          <el-col :span="8">
            <el-card class="stat-card">
              <h3>🏢 按部门统计</h3>
              <div v-for="(count, dept) in getDepartmentStats()" :key="dept">
                <p>{{ dept }}: {{ count }}</p>
              </div>
            </el-card>
          </el-col>
        </el-row>
      </div>
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const cadreList = ref([]);

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

// 加载所有干部信息
async function loadCadreInfo() {
  try {
    cadreList.value = await invoke("get_all_cadre_info");
  } catch (error) {
    console.error("加载干部信息失败:", error);
  }
}

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
  height: 100%;
  background: white;
  border-radius: 16px;
  box-shadow: var(--card-shadow);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
  border: 1px solid rgba(0, 0, 0, 0.05);
}

.content-section:hover {
  transform: translateY(-3px);
  box-shadow: var(--hover-shadow);
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
  border-bottom: 2px solid #f1f5f9;
}

.stat-card {
  margin-bottom: 20px;
}

.stat-card h3 {
  margin-top: 0;
}

.stat-number {
  font-size: 2rem;
  font-weight: bold;
  color: var(--primary-color);
}
</style>