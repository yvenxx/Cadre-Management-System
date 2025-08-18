import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
// 导入Element Plus的中文语言包
import zhCn from 'element-plus/es/locale/lang/zh-cn';

const app = createApp(App);

// 注册所有图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component);
}

// 配置Element Plus使用中文语言包
app.use(ElementPlus, {
  locale: zhCn,
});
app.mount("#app");
