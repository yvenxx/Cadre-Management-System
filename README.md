# 干部管理系统 (Cadre Management System)

干部管理系统是一个基于 Tauri 和 Vue 3 开发的桌面应用程序，旨在帮助组织高效地管理干部（或员工）信息。

## 功能特性 (Features)

- **信息管理**: 添加、查看、编辑和删除干部详细信息。
- **数据筛选**: 根据多种条件（如姓名、部门、职务、学历等）对干部信息进行筛选。
- **数据导入/导出**: 
  - 从 Excel 文件导入干部信息。
  - 将干部信息导出到 Excel 文件，支持自定义导出字段和筛选导出。
- **统计分析**: 提供部门、性别、年龄、司龄、职务、学历、政治面貌等维度的统计信息。
- **数据可视化**: 使用图表直观展示统计数据。
- **响应式设计**: 适配不同屏幕尺寸，提供良好的用户体验。

## 技术栈 (Tech Stack)

- **前端**: Vue 3 (Composition API, `<script setup>`), Element Plus UI 组件库
- **后端/桌面集成**: Tauri (Rust)
- **数据库**: SQLite (rusqlite)
- **构建工具**: Vite
- **样式**: CSS (SCSS)

## 本地开发 (Local Development)

### 环境准备 (Prerequisites)

- [Node.js](https://nodejs.org/) (推荐 LTS 版本)
- [Rust](https://www.rust-lang.org/tools/install) 工具链
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/) 所需的系统依赖项
- 代码编辑器 (推荐 [VS Code](https://code.visualstudio.com/) 配合 Tauri/Vue/Rust 插件)

### 安装依赖 (Installation)

```bash
# 克隆或下载项目代码到本地
git clone <repository-url> # 替换为你的仓库URL
cd Cadre-Management-System

# 安装前端依赖
npm install
```

### 运行开发服务器 (Run Dev Server)

#### 1. 运行前端开发服务器 (仅浏览器 UI)
```bash
# 启动 Vite 开发服务器，用于快速预览和调试 UI
npm run dev
```
此命令将在浏览器中启动应用，但不包含 Tauri 后端功能。

#### 2. 运行桌面应用 (完整功能)
```bash
# 启动 Tauri 开发服务器并运行桌面应用
npm run tauri dev
```
此命令将构建前端，启动 Tauri 开发服务器，并打开一个包含完整功能的桌面应用程序窗口。

### 构建生产版本 (Build for Production)

```bash
# 为当前平台构建生产版本的桌面应用
npm run tauri build
```
构建后的可执行文件将位于 `src-tauri/target/release` 目录下。

## 项目结构 (Project Structure)

```
Cadre-Management-System/
├── src/                    # Vue 前端源码
│   ├── assets/             # 静态资源 (图片, 字体等)
│   ├── components/         # Vue 组件
│   ├── views/              # 页面视图
│   ├── App.vue             # 根组件
│   └── main.js             # 应用入口文件
├── src-tauri/              # Tauri 后端/Rust 代码
│   ├── src/                # Rust 源码
│   │   ├── main.rs         # Tauri 应用入口
│   │   ├── lib.rs          # Tauri 命令和应用逻辑
│   │   ├── database.rs     # 数据库交互
│   │   └── export.rs       # 数据导出逻辑
│   ├── Cargo.toml          # Rust 项目配置和依赖
│   └── tauri.conf.json     # Tauri 配置文件
├── public/                 # 公共文件 (不会被构建处理)
├── index.html              # 主页面模板
├── package.json            # Node.js 项目配置和前端依赖
├── vite.config.js          # Vite 构建配置
└── README.md               # 本说明文件
```

## 使用说明 (Usage)

1. **启动应用**: 运行 `npm run tauri dev` 启动桌面应用。
2. **新增干部**: 点击 "新增" 按钮，填写干部信息表单并保存。
3. **查看/编辑/删除**: 在干部信息列表中，可以查看所有干部信息。点击 "编辑" 按钮修改信息，点击 "删除" 按钮删除记录。
4. **筛选数据**: 点击 "筛选条件" 展开面板，设置筛选项，然后点击 "应用筛选" 按钮。
5. **导出数据**:
   - 选择一条或多条记录，点击 "导出选中" 按钮。
   - 点击 "导出全部" 按钮导出所有记录。
   - 在弹出的配置窗口中选择要导出的字段，并指定文件保存路径。
6. **导入数据**:
   - 点击 "导入数据" 按钮。
   - 点击 "选择文件并导入"，在文件选择器中选择你的 Excel 文件。
   - 系统会自动解析并导入数据。
7. **统计分析**: 点击导航栏中的 "统计分析" 查看各项统计数据和图表。

## 贡献 (Contributing)

欢迎提交 Issue 或 Pull Request 来改进这个项目。

## 许可证 (License)

本项目采用 Apache License 2.0 开源许可证。

有关详细信息，请参阅 [LICENSE](LICENSE) 文件。

## 版权信息 (Copyright)

© 2025 yven. All rights reserved.