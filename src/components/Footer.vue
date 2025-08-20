<template>
  <footer class="app-footer">
    <div class="footer-content">
      <p class="copyright-notice">&copy; {{ currentYear }} <a href="https://github.com/yvenxx" target="_blank" class="author-link">YVEN</a>. 版权所有.</p>
    </div>
  </footer>
</template>

<script setup>
import { ref } from 'vue'

const currentYear = ref(new Date().getFullYear())
</script>

<style scoped>
.app-footer {
  width: 100%;
}

.footer-content {
  text-align: center;
}

.copyright-notice {
  font-size: 12px;
  color: #6c757d;
  margin: 0;
}

.author-link {
  color: #4361ee;
  text-decoration: none;
}

.author-link:hover {
  text-decoration: underline;
}
</style>