<template>
  <el-config-provider>
    <div id="app">
      <Layout />
    </div>
  </el-config-provider>
</template>

<script setup>
import Layout from "./components/Layout.vue";
</script>

<style>
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

/* 修改Element Plus表格的横向滚动条大小 */
.el-scrollbar__bar.is-horizontal {
  height: 16px !important;
}

.el-scrollbar__bar.is-horizontal .el-scrollbar__thumb {
  height: 16px !important;
  border-radius: 8px !important;
  background-color: #c1c1c1 !important;
}

.el-scrollbar__bar.is-horizontal .el-scrollbar__thumb:hover {
  background-color: #a8a8a8 !important;
}
</style>