<template>
  <el-config-provider :locale="zhCn">
    <div id="app">
      <Layout />
    </div>
  </el-config-provider>
</template>

<script setup>
import { ref, onMounted } from "vue";
import Layout from "./components/Layout.vue";
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
</script>

<style>
/* 全局样式修复 */
html, body {
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
}

#app {
  height: 100%;
}

/* 现代化配色方案 */
:root {
  --primary-color: #4361ee;
  --primary-light: #4895ef;
  --primary-dark: #3f37c9;
  --secondary-color: #7209b7;
  --success-color: #4cc9f0;
  --warning-color: #f72585;
  --danger-color: #e63946;
  --info-color: #4cc9f0;
  --light-bg: #f8f9fa;
  --dark-bg: #212529;
  --text-primary: #212529;
  --text-secondary: #6c757d;
  --border-color: #dee2e6;
  --card-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  --hover-shadow: 0 6px 16px rgba(0, 0, 0, 0.12);
}
</style>