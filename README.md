<div align="center">
  <img src="public/tauri.svg" alt="Cadre Management System Logo" width="120" height="120">
  
  <h1 align="center">干部管理系统 (Cadre Management System)</h1>

  <p align="center">
    一个基于 Tauri 和 Vue 3 开发的现代化桌面应用程序，用于高效管理干部（或员工）信息。
    <br />
    <a href="#功能特性-features"><strong>浏览功能 »</strong></a>
    <br />
    <br />
    <a href="#快速开始-getting-started">快速开始</a>
    ·
    <a href="#截图-screenshots">截图</a>
    ·
    <a href="#许可证-license">许可证</a>
  </p>
  
  <p align="center">
    <img src="https://img.shields.io/badge/Tauri-2.0-blue?style=flat&logo=tauri" alt="Tauri Version">
    <img src="https://img.shields.io/badge/Vue.js-3-green?style=flat&logo=vue.js" alt="Vue.js Version">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    <img src="https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey" alt="Supported Platforms">
  </p>
</div>

<br />

## 目录

- [功能特性 (Features)](#功能特性-features)
- [截图 (Screenshots)](#截图-screenshots)
- [快速开始 (Getting Started)](#快速开始-getting-started)
  - [环境准备 (Prerequisites)](#环境准备-prerequisites)
  - [安装 (Installation)](#安装-installation)
  - [开发 (Development)](#开发-development)
  - [构建 (Build)](#构建-build)
- [项目结构 (Project Structure)](#项目结构-project-structure)
- [技术栈 (Tech Stack)](#技术栈-tech-stack)
- [许可证 (License)](#许可证-license)
- [版权信息 (Copyright)](#版权信息-copyright)

## 功能特性 (Features)

- **信息管理**:
  - 添加、查看、编辑和删除干部详细信息。
- **高级筛选**:
  - 根据姓名、性别、部门、科室、职务、学历、政治面貌、年龄范围、日期范围（入司日期、出生日期等）等多种条件进行精确筛选。
- **数据导入/导出**:
  - **Excel 导入**: 通过标准模板从 Excel 文件批量导入干部信息，支持多种日期格式。
  - **Excel 导出**: 将干部信息导出到 Excel 文件，支持自定义导出字段和筛选导出。
  - **模板下载**: 提供标准的 Excel 导入模板下载。
- **统计分析**:
  - 提供部门、性别、年龄、司龄、职务、学历、政治面貌、全日制学历、在职学历等多个维度的统计信息和图表。
- **用户界面**:
  - 现代化、响应式设计，适配不同屏幕尺寸。
  - 使用 Element Plus UI 组件库，提供流畅的用户体验。
- **数据存储**:
  - 本地 SQLite 数据库存储，确保数据安全和离线可用性。

## 截图 (Screenshots)

> 在此处放置你的应用程序截图。由于这是一个本地运行的桌面应用，你可以运行应用后截图并替换以下链接。
> 示例:
> ![Main Interface](docs/screenshots/main_interface.png)
> ![Statistics View](docs/screenshots/statistics_view.png)
> ![Import Data](docs/screenshots/import_data.png)

## 快速开始 (Getting Started)

要设置并运行该项目，请遵循以下步骤。

### 环境准备 (Prerequisites)

在开始之前，请确保您的开发环境已安装以下工具：

- [Node.js](https://nodejs.org/) (推荐 LTS 版本)
- [Rust](https://www.rust-lang.org/tools/install) 工具链
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/) 所需的系统依赖项
- 代码编辑器 (推荐 [VS Code](https://code.visualstudio.com/) 配合 [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) / [Vue](https://marketplace.visualstudio.com/items?itemName=Vue.volar) / [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) 插件)

### 安装 (Installation)

1. 克隆或下载项目代码到本地：

   ```bash
   git clone https://github.com/your-username/Cadre-Management-System.git
   cd Cadre-Management-System
   ```

2. 安装前端依赖：

   ```bash
   npm install
   ```

### 开发 (Development)

#### 运行前端开发服务器 (仅浏览器 UI)

```bash
# 启动 Vite 开发服务器，用于快速预览和调试 UI
npm run dev
```
此命令将在浏览器中启动应用，但不包含 Tauri 后端功能（如文件系统访问、数据库操作等）。

#### 运行桌面应用 (完整功能)

```bash
# 启动 Tauri 开发服务器并运行桌面应用
npm run tauri dev
```
此命令将构建前端，启动 Tauri 开发服务器，并打开一个包含完整功能的桌面应用程序窗口。

### 构建 (Build)

为当前平台构建生产版本的桌面应用：

```bash
npm run tauri build
```
构建后的可执行文件将位于 `src-tauri/target/release/bundle/` 目录下，具体路径取决于你的操作系统和配置。

## 项目结构 (Project Structure)

```
Cadre-Management-System/
├── src/                    # Vue 前端源码
│   ├── assets/             # 静态资源 (图片, 字体等)
│   ├── components/         # 可复用的 Vue 组件
│   ├── views/              # 页面视图组件
│   ├── App.vue             # 根组件
│   └── main.js             # 应用入口文件
├── src-tauri/              # Tauri 后端/Rust 代码
│   ├── src/                # Rust 源码
│   │   ├── main.rs         # Tauri 应用入口
│   │   ├── lib.rs          # Tauri 命令和应用逻辑
│   │   ├── database.rs     # 数据库交互
│   │   └── export.rs       # 数据导入/导出逻辑
│   ├── Cargo.toml          # Rust 项目配置和依赖
│   └── tauri.conf.json     # Tauri 配置文件
├── public/                 # 公共文件 (不会被构建处理)
├── index.html              # 主页面模板
├── package.json            # Node.js 项目配置和前端依赖
├── vite.config.js          # Vite 构建配置
├── LICENSE                 # Apache 2.0 许可证文件
└── README.md               # 本说明文件
```

## 技术栈 (Tech Stack)

- **前端**:
  - [Vue 3](https://v3.vuejs.org/) (Composition API, `<script setup>`)
  - [Vite](https://vitejs.dev/)
  - [Element Plus](https://element-plus.org/)
- **后端/桌面集成**:
  - [Tauri](https://tauri.app/) (Rust)
- **数据库**:
  - [SQLite](https://www.sqlite.org/index.html) (通过 [rusqlite](https://github.com/rusqlite/rusqlite))
- **数据处理**:
  - [calamine](https://github.com/tafia/calamine) (用于读取 Excel 文件)
  - [simple_excel_writer](https://github.com/outersky/simple-excel-writer) (用于写入 Excel 文件)
- **样式**:
  - CSS

## 贡献 (Contributing)

欢迎提交 Issue 或 Pull Request 来改进这个项目。对于重大更改，请先开 issue 讨论您想要改变的内容。

## 联系方式 (Contact)

yven - [@your_twitter](https://twitter.com/your_twitter) - your-email@example.com

项目地址: [https://github.com/your-username/Cadre-Management-System](https://github.com/your-username/Cadre-Management-System)

## 许可证 (License)

本项目采用 Apache License 2.0 开源许可证。有关详细信息，请参阅 [LICENSE](LICENSE) 文件。

## 版权信息 (Copyright)

© 2025 yven. All rights reserved.