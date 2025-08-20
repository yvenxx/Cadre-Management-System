<template>
  <el-container class="layout-container">
    <el-aside width="250px" class="sidebar">
      <div class="logo">
        <h2>干部管理系统</h2>
      </div>
      <el-menu
        :default-active="activeMenu"
        class="menu"
        @select="handleMenuSelect"
      >
        <el-menu-item index="grassroots-cadre-list">
          <i class="menu-icon">📋</i>
          <span>基层管理人员信息表</span>
        </el-menu-item>
        <el-menu-item index="midlevel-cadre-list">
          <i class="menu-icon">📋</i>
          <span>中层管理人员信息表</span>
        </el-menu-item>
        <el-menu-item index="statistics">
          <i class="menu-icon">📊</i>
          <span>基层信息结构统计</span>
        </el-menu-item>
        <el-menu-item index="midlevel-statistics">
          <i class="menu-icon">📊</i>
          <span>中层信息结构统计</span>
        </el-menu-item>
        <el-menu-item index="grassroots-tenure">
          <i class="menu-icon">📅</i>
          <span>基层任职年限表</span>
        </el-menu-item>
        <el-menu-item index="midlevel-tenure">
          <i class="menu-icon">📅</i>
          <span>中层任职年限表</span>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <el-container>
      <el-main class="main-content">
        <GrassrootsCadreListView v-if="activeMenu === 'grassroots-cadre-list'" />
        <MidLevelCadreListView v-else-if="activeMenu === 'midlevel-cadre-list'" />
        <StatisticsView v-else-if="activeMenu === 'statistics'" />
        <MidlevelStatisticsView v-else-if="activeMenu === 'midlevel-statistics'" />
        <GrassrootsTenureView v-else-if="activeMenu === 'grassroots-tenure'" />
        <MidlevelTenureView v-else-if="activeMenu === 'midlevel-tenure'" />
      </el-main>
      <el-footer class="app-footer-container">
        <Footer />
      </el-footer>
    </el-container>
  </el-container>
</template>

<script setup>
import { ref } from 'vue'
import GrassrootsCadreListView from '../views/GrassrootsCadreListView.vue'
import MidLevelCadreListView from '../views/MidLevelCadreListView.vue'
import StatisticsView from '../views/StatisticsView.vue'
import MidlevelStatisticsView from '../views/MidlevelStatisticsView.vue'
import GrassrootsTenureView from '../views/GrassrootsTenureView.vue'
import MidlevelTenureView from '../views/MidlevelTenureView.vue'
import Footer from './Footer.vue'

const activeMenu = ref('grassroots-cadre-list')

const handleMenuSelect = (index) => {
  activeMenu.value = index
}
</script>

<style scoped>
.layout-container {
  height: 100vh;
}

.sidebar {
  background: linear-gradient(180deg, #2d3748, #1a202c);
  color: #e2e8f0;
  box-shadow: 4px 0 20px rgba(0, 0, 0, 0.15);
  border-right: 1px solid #4a5568;
}

.logo {
  padding: 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  text-align: center;
}

.logo h2 {
  margin: 0;
  font-size: 1.6rem;
  font-weight: 700;
  color: #ffffff;
  letter-spacing: 0.5px;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.menu {
  border-right: none;
}

.menu-icon {
  margin-right: 12px;
  font-size: 1.2rem;
}

.main-content {
  background: linear-gradient(135deg, #f8fafc, #f1f5f9);
  padding: 25px;
  height: calc(100vh - 100px);
  overflow-y: auto;
}

.app-footer-container {
  background-color: #ffffff;
  padding: 10px 0;
  text-align: center;
  height: 50px;
  border-top: 1px solid #e9ecef;
}
</style>