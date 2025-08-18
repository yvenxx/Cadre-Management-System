<template>
  <footer class="app-footer">
    <div class="footer-content">
      <p>&copy; {{ currentYear }} 干部管理系统. All rights reserved. Developed by 
        <a href="https://github.com/yven" target="_blank" rel="noopener noreferrer" class="developer-link">yven</a>
      </p>
    </div>
  </footer>
</template>

<script setup>
import { ref } from 'vue'

const currentYear = ref(new Date().getFullYear())
</script>

<style scoped>
.app-footer {
  background-color: #f8f9fa;
  border-top: 1px solid #e9ecef;
  padding: 10px 0;
  text-align: center;
  margin-top: auto;
  width: 100%;
}

.footer-content p {
  margin: 0;
  font-size: 12px;
  color: #6c757d;
}

.developer-link {
  color: #409eff;
  text-decoration: none;
  font-weight: 500;
}

.developer-link:hover {
  text-decoration: underline;
  color: #66b1ff;
}
</style>