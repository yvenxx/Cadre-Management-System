 <template>
  <div class="app-container">
    <!-- 左侧菜单栏 -->
    <div class="sidebar">
      <div class="logo">
        <h2>干部管理系统</h2>
      </div>
      <nav class="menu">
        <button 
          class="menu-item" 
          :class="{ active: activeTab === 'cadre-list' }"
          @click="activeTab = 'cadre-list'"
        >
          <i class="menu-icon">📋</i>
          管理人员信息表
        </button>
        <button 
          class="menu-item" 
          :class="{ active: activeTab === 'statistics' }"
          @click="activeTab = 'statistics'"
        >
          <i class="menu-icon">📊</i>
          信息结构统计
        </button>
      </nav>
    </div>

    <!-- 主内容区域 -->
    <div class="main-content">
      <!-- 管理人员信息表 -->
      <div v-if="activeTab === 'cadre-list'" class="content-section">
        <div class="section-header">
          <h2>管理人员信息表</h2>
        </div>
        
        <!-- 筛选条件面板 -->
        <div class="filter-panel">
          <div class="filter-header">
            <h3>筛选条件</h3>
            <el-button @click="toggleFilterPanel" size="small" class="toggle-filter-button">
              {{ showFilterPanel ? '收起' : '展开' }}
            </el-button>
          </div>
          <div v-show="showFilterPanel" class="filter-content">
            <el-row :gutter="16">
              <el-col :span="6">
                <el-form-item label="姓名">
                  <el-input v-model="filters.name" placeholder="请输入姓名" clearable prefix-icon="Search" />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="性别">
                  <el-select v-model="filters.gender" placeholder="请选择" clearable>
                    <el-option label="全部" value="" />
                    <el-option label="男" value="男" />
                    <el-option label="女" value="女" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="部门">
                  <el-input v-model="filters.department" placeholder="请输入部门" clearable :prefix-icon="OfficeBuilding" />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="科室">
                  <el-input v-model="filters.section" placeholder="请输入科室" clearable :prefix-icon="OfficeBuilding" />
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-row :gutter="16">
              <el-col :span="6">
                <el-form-item label="职务1">
                  <el-input v-model="filters.position1" placeholder="请输入职务1" clearable prefix-icon="UserFilled" />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="职务2">
                  <el-input v-model="filters.position2" placeholder="请输入职务2" clearable prefix-icon="UserFilled" />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="学历">
                  <el-input v-model="filters.education" placeholder="请输入学历" clearable :prefix-icon="Medal" />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="政治面貌">
                  <el-select v-model="filters.politicalStatus" placeholder="请选择" clearable>
                    <el-option label="全部" value="" />
                    <el-option label="中共党员" value="中共党员" />
                    <el-option label="预备党员" value="预备党员" />
                    <el-option label="共青团员" value="共青团员" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-row :gutter="16">
              <el-col :span="6">
                <el-form-item label="身份证号">
                  <el-input v-model="filters.idNumber" placeholder="请输入身份证号" clearable />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="籍贯">
                  <el-input v-model="filters.nativePlace" placeholder="请输入籍贯" clearable />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="民族">
                  <el-input v-model="filters.ethnicity" placeholder="请输入民族" clearable />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="专业技术职务">
                  <el-input v-model="filters.technicalPosition" placeholder="请输入专业技术职务" clearable />
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-row :gutter="16">
              <el-col :span="8">
                <el-form-item label="联系电话">
                  <el-input v-model="filters.phone" placeholder="请输入联系电话" clearable />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="入司日期范围">
                  <el-date-picker
                    v-model="filterDateRanges.companyEntryDate"
                    type="daterange"
                    range-separator="至"
                    start-placeholder="开始日期"
                    end-placeholder="结束日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                    style="width: 100%"
                  />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="年龄范围">
                  <div class="age-range-container">
                    <el-input-number
                      v-model="filters.ageMin"
                      :min="18"
                      :max="70"
                      placeholder="最小年龄"
                      style="width: 45%"
                      controls-position="right"
                    />
                    <span class="age-range-separator">至</span>
                    <el-input-number
                      v-model="filters.ageMax"
                      :min="18"
                      :max="70"
                      placeholder="最大年龄"
                      style="width: 45%"
                      controls-position="right"
                    />
                    <span class="age-range-unit">岁</span>
                  </div>
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-row :gutter="16">
              <el-col :span="8">
                <el-form-item label="出生地">
                  <el-input v-model="filters.birthPlace" placeholder="请输入出生地" clearable />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="全日制学历">
                  <el-input v-model="filters.fullTimeEducation" placeholder="请输入全日制学历" clearable />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="在职学历">
                  <el-input v-model="filters.partTimeEducation" placeholder="请输入在职学历" clearable />
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-row :gutter="16">
              <el-col :span="8">
                <el-form-item label="出生日期范围">
                  <el-date-picker
                    v-model="filterDateRanges.birthDate"
                    type="daterange"
                    range-separator="至"
                    start-placeholder="开始日期"
                    end-placeholder="结束日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                    style="width: 100%"
                  />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="参加工作时间范围">
                  <el-date-picker
                    v-model="filterDateRanges.workStart"
                    type="daterange"
                    range-separator="至"
                    start-placeholder="开始日期"
                    end-placeholder="结束日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                    style="width: 100%"
                  />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="任现职级时间范围">
                  <el-date-picker
                    v-model="filterDateRanges.currentLevel"
                    type="daterange"
                    range-separator="至"
                    start-placeholder="开始日期"
                    end-placeholder="结束日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                    style="width: 100%"
                  />
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-row :gutter="16">
              <el-col :span="8">
                <el-form-item label="任职时间范围">
                  <el-date-picker
                    v-model="filterDateRanges.positionEntry"
                    type="daterange"
                    range-separator="至"
                    start-placeholder="开始日期"
                    end-placeholder="结束日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                    style="width: 100%"
                  />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="入党时间范围">
                  <el-date-picker
                    v-model="filterDateRanges.partyEntry"
                    type="daterange"
                    range-separator="至"
                    start-placeholder="开始日期"
                    end-placeholder="结束日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                    style="width: 100%"
                  />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="试用期满到期提醒范围">
                  <el-date-picker
                    v-model="filterDateRanges.probationEndReminder"
                    type="daterange"
                    range-separator="至"
                    start-placeholder="开始日期"
                    end-placeholder="结束日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                    style="width: 100%"
                  />
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-row :gutter="16">
              <el-col :span="8">
                <el-form-item label="全日制毕业院校系及专业">
                  <el-input v-model="filters.fullTimeSchoolMajor" placeholder="请输入全日制毕业院校系及专业" clearable />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="在职毕业院校系及专业">
                  <el-input v-model="filters.partTimeSchoolPhone" placeholder="请输入在职毕业院校系及专业" clearable />
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="试用期（年）">
                  <el-input-number
                    v-model="filters.probationPeriod"
                    :min="0"
                    :max="10"
                    :step="0.1"
                    placeholder="请输入试用期"
                    style="width: 100%"
                    controls-position="right"
                  />
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-row :gutter="16">
              <el-col :span="24">
                <el-form-item label="备注">
                  <el-input v-model="filters.remarks" placeholder="请输入备注" clearable />
                </el-form-item>
              </el-col>
            </el-row>
            
            <div class="filter-actions">
              <el-button type="primary" @click="applyFilters" :icon="Search">应用筛选</el-button>
              <el-button @click="clearFilters" :icon="RefreshRight">清空筛选</el-button>
            </div>
          </div>
        </div>
        
        <!-- 操作按钮组 -->
        <div class="export-buttons-container">
          <div class="export-buttons-info">
            <span class="export-count-info">
              已选择: {{ selectedCadres.length }} 条记录
            </span>
          </div>
          <div class="export-buttons-group">
            <button class="btn-primary" @click="openAddModal">
              <i>➕</i> 新增
            </button>
            <button class="btn-export btn-export-selected" @click="exportSelectedCadres">
              <i>📤</i> 导出选中
            </button>
            <button class="btn-export btn-export-full" @click="exportAllCadres">
              <i>📤</i> 导出全部
            </button>
          </div>
        </div>

        <!-- 数据表格 -->
        <div class="horizontal-table-container">
          <div class="table-header">
            <div class="table-row header-row">
              <div class="table-cell">
                <input 
                  type="checkbox" 
                  v-model="selectAll" 
                  @change="toggleSelectAll" 
                  class="row-checkbox"
                />
              </div>
              <div class="table-cell">序号</div>
              <div class="table-cell">姓名</div>
              <div class="table-cell">性别</div>
              <div class="table-cell">部门</div>
              <div class="table-cell">科室</div>
              <div class="table-cell">职务1</div>
              <div class="table-cell">职务2</div>
              <div class="table-cell">入司日期</div>
              <div class="table-cell">司龄（年）</div>
              <div class="table-cell">参加工作时间</div>
              <div class="table-cell">工龄(年)</div>
              <div class="table-cell">任现职级时间</div>
              <div class="table-cell">任职时间</div>
              <div class="table-cell">试用期</div>
              <div class="table-cell">试用期满到期提醒</div>
              <div class="table-cell">身份证号</div>
              <div class="table-cell">出生日期</div>
              <div class="table-cell">年龄</div>
              <div class="table-cell">籍贯</div>
              <div class="table-cell">出生地</div>
              <div class="table-cell">民族</div>
              <div class="table-cell">专业技术职务</div>
              <div class="table-cell">学历</div>
              <div class="table-cell">全日制学历</div>
              <div class="table-cell">毕业院校系及专业</div>
              <div class="table-cell">在职学历</div>
              <div class="table-cell">毕业院校系及专业</div>
              <div class="table-cell">政治面貌</div>
              <div class="table-cell">入党时间</div>
              <div class="table-cell">联系电话</div>
              <div class="table-cell">备注</div>
              <div class="table-cell">操作</div>
            </div>
          </div>
          <div class="table-body">
            <div v-for="(cadre, index) in cadreList" :key="cadre.id" class="table-row">
              <div class="table-cell">
                <input 
                  type="checkbox" 
                  :value="cadre" 
                  v-model="selectedCadres" 
                  class="row-checkbox"
                />
              </div>
              <div class="table-cell">{{ index + 1 }}</div>
              <div class="table-cell">{{ cadre.name }}</div>
              <div class="table-cell">{{ cadre.gender }}</div>
              <div class="table-cell">{{ cadre.department }}</div>
              <div class="table-cell">{{ cadre.section }}</div>
              <div class="table-cell">{{ cadre.position1 }}</div>
              <div class="table-cell">{{ cadre.position2 }}</div>
              <div class="table-cell">{{ formatDate(cadre.company_entry_date) }}</div>
              <div class="table-cell">{{ cadre.company_tenure }}</div>
              <div class="table-cell">{{ formatDate(cadre.work_start_date) }}</div>
              <div class="table-cell">{{ cadre.work_tenure }}</div>
              <div class="table-cell">{{ formatDate(cadre.current_level_date) }}</div>
              <div class="table-cell">{{ formatDate(cadre.position_entry_date) }}</div>
              <div class="table-cell">{{ cadre.probation_period }}</div>
              <div class="table-cell">{{ formatDate(cadre.probation_end_reminder) }}</div>
              <div class="table-cell">{{ cadre.id_number }}</div>
              <div class="table-cell">{{ formatDate(cadre.birth_date) }}</div>
              <div class="table-cell">{{ cadre.age }}</div>
              <div class="table-cell">{{ cadre.native_place }}</div>
              <div class="table-cell">{{ cadre.birth_place }}</div>
              <div class="table-cell">{{ cadre.ethnicity }}</div>
              <div class="table-cell">{{ cadre.technical_position }}</div>
              <div class="table-cell">{{ cadre.education }}</div>
              <div class="table-cell">{{ cadre.full_time_education }}</div>
              <div class="table-cell">{{ cadre.full_time_school_major }}</div>
              <div class="table-cell">{{ cadre.part_time_education }}</div>
              <div class="table-cell">{{ cadre.part_time_school_phone }}</div>
              <div class="table-cell">{{ cadre.political_status }}</div>
              <div class="table-cell">{{ formatDate(cadre.party_entry_date) }}</div>
              <div class="table-cell">{{ cadre.phone }}</div>
              <div class="table-cell">{{ cadre.remarks }}</div>
              <div class="table-cell actions-cell">
                <button @click="editCadre(cadre)" class="btn-small">编辑</button>
                <button @click="deleteCadre(cadre.id)" class="btn-small btn-danger">删除</button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 信息结构统计 -->
      <div v-if="activeTab === 'statistics'" class="content-section">
        <div class="section-header">
          <h2>信息结构统计</h2>
        </div>
        <div class="statistics-content">
          <div class="stat-card">
            <h3>📊 总人数统计</h3>
            <p class="stat-number">{{ cadreList.length }}</p>
          </div>
          <div class="stat-card">
            <h3>👨‍💼 按性别统计</h3>
            <p>男: {{ cadreList.filter(c => c.gender === '男').length }}</p>
            <p>女: {{ cadreList.filter(c => c.gender === '女').length }}</p>
          </div>
          <div class="stat-card">
            <h3>🏢 按部门统计</h3>
            <div v-for="(count, dept) in getDepartmentStats()" :key="dept">
              <p>{{ dept }}: {{ count }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 导出配置弹窗 -->
    <div v-if="showExportModal" class="modal-overlay" @click="closeExportModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>导出配置</h3>
          <button class="modal-close" @click="closeExportModal">×</button>
        </div>
        <div class="modal-body">
          <div class="export-config-section">
            <div class="filename-row-container">
              <label for="export-file-name" class="filename-label">文件名</label>
              <input 
                id="export-file-name" 
                v-model="exportConfig.fileName" 
                placeholder="请输入文件名" 
                autocomplete="off" 
                class="filename-input"
              />
            </div>
            
                      </div>
          
          <div class="export-config-section">
            <div class="field-section-header">
              <h4>选择导出字段</h4>
              <div class="field-actions">
                <label class="checkbox-label">
                  <input 
                    type="checkbox" 
                    v-model="exportConfig.selectAllFields" 
                    @change="toggleAllFields"
                    class="checkbox-input"
                  /> 全选
                </label>
                <button @click="resetFields" class="btn-reset">重置</button>
              </div>
            </div>
            <div class="field-selection">
              <div class="field-grid">
                <div 
                  v-for="field in exportFields" 
                  :key="field.key" 
                  class="field-item"
                >
                  <label class="checkbox-label">
                    <input 
                      type="checkbox" 
                      :value="field.key" 
                      v-model="exportConfig.selectedFields" 
                      class="checkbox-input"
                    /> {{ field.label }}
                  </label>
                </div>
              </div>
            </div>
          </div>
          
          <div class="form-actions">
            <button type="button" @click="closeExportModal" class="btn-modal-secondary">取消</button>
            <button type="button" @click="performExport" class="btn-modal-primary">导出</button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 新增/编辑弹窗 -->
    <div v-if="showModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>{{ currentCadre.id ? '编辑干部信息' : '新增干部信息' }}</h3>
          <button class="modal-close" @click="closeModal">×</button>
        </div>
        <div class="modal-body">
          <el-form 
            ref="cadreFormRef"
            :model="currentCadre"
            :rules="formRules"
            label-width="120px"
            size="default"
            @submit.prevent="saveCadreInfo"
          >
            <!-- 基本信息 -->
            <div class="form-section">
              <h4 class="form-section-title">基本信息</h4>
              <el-row :gutter="20">
                <el-col :span="8">
                  <el-form-item label="姓名 *" prop="name">
                    <el-input v-model="currentCadre.name" placeholder="请输入姓名" clearable />
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item label="性别" prop="gender">
                    <el-select v-model="currentCadre.gender" placeholder="请选择性别" clearable>
                      <el-option label="" value="" />
                      <el-option label="男" value="男" />
                      <el-option label="女" value="女" />
                    </el-select>
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item label="身份证号" prop="id_number">
                    <el-input v-model="currentCadre.id_number" @change="extractIdInfo" placeholder="请输入18位身份证号" clearable />
                  </el-form-item>
                </el-col>
              </el-row>
              
              <el-row :gutter="20">
                <el-col :span="8">
                  <el-form-item label="出生日期">
                    <el-date-picker 
                      v-model="currentCadre.birth_date" 
                      type="date"
                      placeholder="自动计算"
                      format="YYYY-MM-DD"
                      value-format="YYYY-MM-DD"
                      disabled
                      clearable
                    />
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item label="年龄">
                    <el-input v-model="currentCadre.age" readonly placeholder="自动计算" disabled />
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item label="民族" prop="ethnicity">
                    <el-select v-model="currentCadre.ethnicity" placeholder="请选择或输入民族" filterable allow-create default-first-option clearable>
                      <el-option v-for="option in ethnicityOptions" :key="option" :label="option" :value="option" />
                    </el-select>
                  </el-form-item>
                </el-col>
              </el-row>
              
              <el-row :gutter="20">
                <el-col :span="8">
                  <el-form-item label="籍贯" prop="native_place">
                    <el-input v-model="currentCadre.native_place" placeholder="请输入籍贯" clearable />
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item label="出生地" prop="birth_place">
                    <el-input v-model="currentCadre.birth_place" placeholder="请输入出生地" clearable />
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item label="联系电话" prop="phone">
                    <el-input v-model="currentCadre.phone" placeholder="请输入联系电话" clearable />
                  </el-form-item>
                </el-col>
              </el-row>
            </div>
            
            <!-- 工作信息 -->
            <div class="form-section">
              <h4 class="form-section-title">工作信息</h4>
              <div class="form-row">
                <div class="form-group">
                  <label for="modal-department">部门</label>
                  <el-select id="modal-department" v-model="currentCadre.department" placeholder="请选择或输入部门名称" filterable allow-create default-first-option clearable>
                    <el-option v-for="option in departmentOptions" :key="option" :label="option" :value="option" />
                  </el-select>
                </div>
                <div class="form-group">
                  <label for="modal-section">科室</label>
                  <el-input id="modal-section" v-model="currentCadre.section" placeholder="请输入科室名称" clearable />
                </div>
                <div class="form-group">
                  <label for="modal-position1">职务1</label>
                  <el-select id="modal-position1" v-model="currentCadre.position1" placeholder="请选择或输入职务" filterable allow-create default-first-option clearable>
                    <el-option v-for="option in positionOptions" :key="option" :label="option" :value="option" />
                  </el-select>
                </div>
              </div>
              
              <div class="form-row">
                <div class="form-group">
                  <label for="modal-position2">职务2</label>
                  <el-input id="modal-position2" v-model="currentCadre.position2" placeholder="请输入职务2" clearable />
                </div>
                <div class="form-group">
                  <label for="modal-companyEntryDate">入司日期</label>
                  <el-date-picker 
                    id="modal-companyEntryDate"
                    v-model="currentCadre.company_entry_date" 
                    type="date"
                    placeholder="请选择日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    @change="calculateCompanyTenure"
                    clearable
                  />
                </div>
                <div class="form-group">
                  <label for="modal-companyTenure">司龄（年）</label>
                  <el-input id="modal-companyTenure" v-model="currentCadre.company_tenure" placeholder="自动计算" disabled />
                </div>
              </div>
              
              <div class="form-row">
                <div class="form-group">
                  <label for="modal-workStartDate">参加工作时间</label>
                  <el-date-picker 
                    id="modal-workStartDate"
                    v-model="currentCadre.work_start_date" 
                    type="date"
                    placeholder="请选择日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    @change="calculateWorkTenure"
                    clearable
                  />
                </div>
                <div class="form-group">
                  <label for="modal-workTenure">工龄（年）</label>
                  <el-input id="modal-workTenure" v-model="currentCadre.work_tenure" placeholder="自动计算" disabled />
                </div>
                <div class="form-group">
                  <label for="modal-currentLevelDate">任现职级时间</label>
                  <el-date-picker 
                    id="modal-currentLevelDate"
                    v-model="currentCadre.current_level_date" 
                    type="date"
                    placeholder="请选择日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                  />
                </div>
              </div>
              
              <div class="form-row">
                <div class="form-group">
                  <label for="modal-positionEntryDate">任职时间</label>
                  <el-date-picker 
                    id="modal-positionEntryDate"
                    v-model="currentCadre.position_entry_date" 
                    type="date"
                    placeholder="请选择日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    @change="calculateProbationEnd"
                    clearable
                  />
                </div>
                <div class="form-group">
                  <label for="modal-probationPeriod">试用期（年）</label>
                  <el-input-number id="modal-probationPeriod" v-model="currentCadre.probation_period" :min="0" :max="10" :step="0.1" @change="calculateProbationEnd" placeholder="请输入试用期" />
                </div>
                <div class="form-group">
                  <label for="modal-probationEndReminder">试用期满到期提醒</label>
                  <el-date-picker 
                    id="modal-probationEndReminder"
                    v-model="currentCadre.probation_end_reminder" 
                    type="date"
                    placeholder="自动计算"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    disabled
                    clearable
                  />
                </div>
              </div>
            </div>
            
            <!-- 教育背景 -->
            <div class="form-section">
              <h4 class="form-section-title">教育背景</h4>
              <div class="form-row">
                <div class="form-group">
                  <label for="modal-education">最高学历</label>
                  <el-select id="modal-education" v-model="currentCadre.education" placeholder="请选择或输入学历" filterable allow-create default-first-option clearable>
                    <el-option v-for="option in educationOptions" :key="option" :label="option" :value="option" />
                  </el-select>
                </div>
                <div class="form-group">
                  <label for="modal-fullTimeEducation">全日制学历</label>
                  <el-select id="modal-fullTimeEducation" v-model="currentCadre.full_time_education" placeholder="请选择或输入学历" filterable allow-create default-first-option clearable>
                    <el-option v-for="option in educationOptions" :key="option" :label="option" :value="option" />
                  </el-select>
                </div>
                <div class="form-group">
                  <label for="modal-fullTimeSchoolMajor">全日制毕业院校系及专业</label>
                  <el-input id="modal-fullTimeSchoolMajor" v-model="currentCadre.full_time_school_major" placeholder="请输入毕业院校系及专业" clearable />
                </div>
              </div>
              
              <div class="form-row">
                <div class="form-group">
                  <label for="modal-partTimeEducation">在职学历</label>
                  <el-select id="modal-partTimeEducation" v-model="currentCadre.part_time_education" placeholder="请选择或输入学历" filterable allow-create default-first-option clearable>
                    <el-option v-for="option in educationOptions" :key="option" :label="option" :value="option" />
                  </el-select>
                </div>
                <div class="form-group">
                  <label for="modal-partTimeSchoolPhone">在职毕业院校系及专业</label>
                  <el-input id="modal-partTimeSchoolPhone" v-model="currentCadre.part_time_school_phone" placeholder="请输入在职毕业院校系及专业" clearable />
                </div>
                <div class="form-group">
                  <label for="modal-technicalPosition">专业技术职务</label>
                  <el-select id="modal-technicalPosition" v-model="currentCadre.technical_position" placeholder="请选择或输入专业技术职务" filterable allow-create default-first-option clearable>
                    <el-option v-for="option in technicalPositionOptions" :key="option" :label="option" :value="option" />
                  </el-select>
                </div>
              </div>
            </div>
            
            <!-- 政治面貌 -->
            <div class="form-section">
              <h4 class="form-section-title">政治面貌</h4>
              <div class="form-row">
                <div class="form-group">
                  <label for="modal-politicalStatus">政治面貌</label>
                  <el-select id="modal-politicalStatus" v-model="currentCadre.political_status" placeholder="请选择政治面貌" clearable>
                    <el-option label="" value="" />
                    <el-option label="中共党员" value="中共党员" />
                    <el-option label="预备党员" value="预备党员" />
                    <el-option label="共青团员" value="共青团员" />
                    <el-option label="民革党员" value="民革党员" />
                    <el-option label="民盟盟员" value="民盟盟员" />
                    <el-option label="民建会员" value="民建会员" />
                    <el-option label="民进会员" value="民进会员" />
                    <el-option label="农工党党员" value="农工党党员" />
                    <el-option label="致公党党员" value="致公党党员" />
                    <el-option label="九三学社社员" value="九三学社社员" />
                    <el-option label="台盟盟员" value="台盟盟员" />
                    <el-option label="无党派人士" value="无党派人士" />
                    <el-option label="群众" value="群众" />
                  </el-select>
                </div>
                <div class="form-group">
                  <label for="modal-partyEntryDate">入党时间</label>
                  <el-date-picker 
                    id="modal-partyEntryDate"
                    v-model="currentCadre.party_entry_date" 
                    type="date"
                    placeholder="请选择日期"
                    format="YYYY-MM-DD"
                    value-format="YYYY-MM-DD"
                    clearable
                  />
                </div>
                <div class="form-group full-width">
                  <label for="modal-remarks">备注</label>
                  <el-input id="modal-remarks" v-model="currentCadre.remarks" type="textarea" :rows="4" placeholder="请输入备注" clearable />
                </div>
              </div>
            </div>
            
            <div class="form-actions">
              <el-button type="button" @click="closeModal">取消</el-button>
              <el-button type="primary" @click="saveCadreInfo">保存</el-button>
            </div>
          </el-form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Search, RefreshRight, OfficeBuilding, UserFilled, Medal } from '@element-plus/icons-vue';

// 日期格式化辅助函数
function formatDate(date) {
  if (!date) return "";
  
  // 如果是字符串，尝试转换为日期对象
  const dateObj = typeof date === 'string' ? new Date(date) : date;
  
  // 检查是否是有效日期
  if (!(dateObj instanceof Date) || isNaN(dateObj.getTime())) {
    return "";
  }
  
  const year = dateObj.getFullYear();
  const month = String(dateObj.getMonth() + 1).padStart(2, '0');
  const day = String(dateObj.getDate()).padStart(2, '0');
  
  return `${year}-${month}-${day}`;
}

const activeTab = ref('cadre-list');
const showModal = ref(false);
const showExportModal = ref(false);
const selectedCadres = ref([]); // 用于存储选中的干部
const selectAll = ref(false); // 用于全选功能
const cadreFormRef = ref(null); // 表单引用

// 学历选项
const educationOptions = [
  "博士研究生",
  "硕士研究生",
  "大学",
  "大专",
  "高中",
  "中专",
  "初中",
  "职高"
];

// 专业技术职务选项
const technicalPositionOptions = [
  "助理政工师",
  "政工师",
  "高级工程师",
  "中级工程师",
  "工程师",
  "中级会计师",
  "助理工程师",
  "初级会计师",
  "初级经济师"
];

// 部门选项
const departmentOptions = [
  "党委工作部（文化宣传部）",
  "人力资源部",
  "财务部",
  "纪检监察部",
  "生产技术部",
  "安全管理部",
  "经营管理部",
  "招标采购部",
  "调度指挥中心",
  "站务中心",
  "车辆中心",
  "维修中心",
  "工会（团委）",
  "办公室",
  "项目管理部",
];

// 职务选项
const positionOptions = [
  "主任",
  "副主任",
  "区域站长",
  "区域副站长",
  "部长",
  "副部长",
  "部长助理",
  "经理",
  "副经理",
  "经理助理"
];

// 民族选项
const ethnicityOptions = [
  "汉族",
  "畲族",
  "侗族",
  "瑶族"
];

// 表单验证规则
const formRules = {
  name: [
    { required: true, message: '请输入姓名', trigger: 'blur' },
    { min: 2, max: 20, message: '姓名长度在 2 到 20 个字符', trigger: 'blur' }
  ],
  gender: [
    { required: true, message: '请选择性别', trigger: 'change' }
  ],
  id_number: [
    { pattern: /^[1-9]\d{5}(18|19|20)\d{2}((0[1-9])|(1[0-2]))(([0-2][1-9])|10|20|30|31)\d{3}[0-9Xx]$/, message: '请输入有效的18位身份证号', trigger: 'blur' }
  ],
  department: [
    { required: true, message: '请选择或输入部门', trigger: 'change' }
  ],
  section: [
    { required: true, message: '请输入科室', trigger: 'blur' }
  ],
  position1: [
    { required: true, message: '请选择或输入职务1', trigger: 'change' }
  ],
  phone: [
    { pattern: /^1[3-9]\d{9}$/, message: '请输入有效的手机号码', trigger: 'blur' }
  ]
};

// 所有可导出字段的配置
const exportFields = [
  { key: "name", label: "姓名" },
  { key: "gender", label: "性别" },
  { key: "department", label: "部门" },
  { key: "section", label: "科室" },
  { key: "position1", label: "职务1" },
  { key: "position2", label: "职务2" },
  { key: "company_entry_date", label: "入司日期" },
  { key: "company_tenure", label: "司龄（年）" },
  { key: "work_start_date", label: "参加工作时间" },
  { key: "work_tenure", label: "工龄(年)" },
  { key: "current_level_date", label: "任现职级时间" },
  { key: "position_entry_date", label: "任职时间" },
  { key: "probation_period", label: "试用期" },
  { key: "probation_end_reminder", label: "试用期满到期提醒" },
  { key: "id_number", label: "身份证号" },
  { key: "birth_date", label: "出生日期" },
  { key: "age", label: "年龄" },
  { key: "native_place", label: "籍贯" },
  { key: "birth_place", label: "出生地" },
  { key: "ethnicity", label: "民族" },
  { key: "technical_position", label: "专业技术职务" },
  { key: "education", label: "学历" },
  { key: "full_time_education", label: "全日制学历" },
  { key: "full_time_school_major", label: "全日制毕业院校系及专业" },
  { key: "part_time_education", label: "在职学历" },
  { key: "part_time_school_phone", label: "在职毕业院校系及专业" },
  { key: "political_status", label: "政治面貌" },
  { key: "party_entry_date", label: "入党时间" },
  { key: "phone", label: "联系电话" },
  { key: "remarks", label: "备注" }
];

// 导出配置
const exportConfig = ref({
  fileName: "干部信息",
  selectedFields: [], // 选中的字段
  selectAllFields: false
});

// 初始化导出字段（默认全选）
exportConfig.value.selectedFields = exportFields.map(field => field.key);

const currentCadre = ref({
  id: null,
  name: "",
  gender: "",
  department: "",
  section: "",
  position1: "",
  position2: "",
  company_entry_date: "",
  company_tenure: null,
  work_start_date: "",
  work_tenure: null,
  current_level_date: "",
  position_entry_date: "",
  probation_period: "",
  probation_end_reminder: "",
  id_number: "",
  birth_date: "",
  age: null,
  native_place: "",
  birth_place: "",
  ethnicity: "",
  technical_position: "",
  education: "",
  full_time_education: "",
  full_time_school_major: "",
  part_time_education: "",
  part_time_school_phone: "",
  political_status: "",
  party_entry_date: "",
  phone: "",
  remarks: "",
  major: "",
  contact_date: "",
  special_date: ""
});

const cadreList = ref([]);
const showFilterPanel = ref(false);
const filterDateRanges = ref({
  companyEntryDate: [],
  birthDate: [],
  workStart: [],
  currentLevel: [],
  positionEntry: [],
  partyEntry: [],
  probationEndReminder: []
});
const filters = ref({
  name: "",
  gender: "",
  department: "",
  section: "",
  position1: "",
  position2: "",
  education: "",
  politicalStatus: "",
  idNumber: "",
  nativePlace: "",
  ethnicity: "",
  technicalPosition: "",
  phone: "",
  ageMin: "",
  ageMax: "",
  birthPlace: "",
  fullTimeEducation: "",
  partTimeEducation: "",
  probationPeriod: "",
  fullTimeSchoolMajor: "",
  partTimeSchoolPhone: "",
  remarks: ""
});

// 计算筛选后的干部列表
const filteredCadreList = computed(() => {
  return cadreList.value.filter(cadre => {
    // 姓名筛选
    if (filters.value.name && !cadre.name.includes(filters.value.name)) {
      return false;
    }
    
    // 性别筛选
    if (filters.value.gender && cadre.gender !== filters.value.gender) {
      return false;
    }
    
    // 部门筛选
    if (filters.value.department && !cadre.department.includes(filters.value.department)) {
      return false;
    }
    
    // 科室筛选
    if (filters.value.section && !cadre.section.includes(filters.value.section)) {
      return false;
    }
    
    // 职务1筛选
    if (filters.value.position1 && !cadre.position1.includes(filters.value.position1)) {
      return false;
    }
    
    // 职务2筛选
    if (filters.value.position2 && !cadre.position2.includes(filters.value.position2)) {
      return false;
    }
    
    // 学历筛选
    if (filters.value.education && !cadre.education.includes(filters.value.education)) {
      return false;
    }
    
    // 政治面貌筛选
    if (filters.value.politicalStatus && cadre.political_status !== filters.value.politicalStatus) {
      return false;
    }
    
    // 身份证号筛选
    if (filters.value.idNumber && !cadre.id_number.includes(filters.value.idNumber)) {
      return false;
    }
    
    // 籍贯筛选
    if (filters.value.nativePlace && !cadre.native_place.includes(filters.value.nativePlace)) {
      return false;
    }
    
    // 民族筛选
    if (filters.value.ethnicity && !cadre.ethnicity.includes(filters.value.ethnicity)) {
      return false;
    }
    
    // 专业技术职务筛选
    if (filters.value.technicalPosition && !cadre.technical_position.includes(filters.value.technicalPosition)) {
      return false;
    }
    
    // 联系电话筛选
    if (filters.value.phone && !cadre.phone.includes(filters.value.phone)) {
      return false;
    }
    
    // 入司日期范围筛选
    if (filterDateRanges.value.companyEntryDate && filterDateRanges.value.companyEntryDate.length === 2) {
      const entryDate = new Date(cadre.company_entry_date);
      const [startDate, endDate] = filterDateRanges.value.companyEntryDate.map(date => new Date(date));
      
      if (entryDate < startDate) {
        return false;
      }
      
      if (entryDate > endDate) {
        return false;
      }
    }
    
      // 年龄范围筛选
    if (filters.value.ageMin || filters.value.ageMax || filters.value.ageRange) {
      const age = parseInt(cadre.age);
      
      if (filters.value.ageMin && age < parseInt(filters.value.ageMin)) {
        return false;
      }
      
      if (filters.value.ageMax && age > parseInt(filters.value.ageMax)) {
        return false;
      }
      
      // 通过范围滑块更新年龄范围
      if (filters.value.ageRange && !filters.value.ageMin && !filters.value.ageMax) {
        const range = parseInt(filters.value.ageRange);
        // 简单的逻辑：范围值作为最大值，最小值设为范围-10
        if (age > range || age < range - 10) {
          return false;
        }
      }
    }
    
    // 出生地筛选
    if (filters.value.birthPlace && !cadre.birth_place.includes(filters.value.birthPlace)) {
      return false;
    }
    
    // 全日制学历筛选
    if (filters.value.fullTimeEducation && !cadre.full_time_education.includes(filters.value.fullTimeEducation)) {
      return false;
    }
    
    // 在职学历筛选
    if (filters.value.partTimeEducation && !cadre.part_time_education.includes(filters.value.partTimeEducation)) {
      return false;
    }
    
    // 出生日期范围筛选
    if (filterDateRanges.value.birthDate && filterDateRanges.value.birthDate.length === 2) {
      const birthDate = new Date(cadre.birth_date);
      const [startDate, endDate] = filterDateRanges.value.birthDate.map(date => new Date(date));
      
      if (birthDate < startDate) {
        return false;
      }
      
      if (birthDate > endDate) {
        return false;
      }
    }
    
    // 参加工作时间范围筛选
    if (filterDateRanges.value.workStart && filterDateRanges.value.workStart.length === 2) {
      const workDate = new Date(cadre.work_start_date);
      const [startDate, endDate] = filterDateRanges.value.workStart.map(date => new Date(date));
      
      if (workDate < startDate) {
        return false;
      }
      
      if (workDate > endDate) {
        return false;
      }
    }
    
    // 任现职级时间范围筛选
    if (filterDateRanges.value.currentLevel && filterDateRanges.value.currentLevel.length === 2) {
      const currentDate = new Date(cadre.current_level_date);
      const [startDate, endDate] = filterDateRanges.value.currentLevel.map(date => new Date(date));
      
      if (currentDate < startDate) {
        return false;
      }
      
      if (currentDate > endDate) {
        return false;
      }
    }
    
    // 任职时间范围筛选
    if (filterDateRanges.value.positionEntry && filterDateRanges.value.positionEntry.length === 2) {
      const positionDate = new Date(cadre.position_entry_date);
      const [startDate, endDate] = filterDateRanges.value.positionEntry.map(date => new Date(date));
      
      if (positionDate < startDate) {
        return false;
      }
      
      if (positionDate > endDate) {
        return false;
      }
    }
    
    // 入党时间范围筛选
    if (filterDateRanges.value.partyEntry && filterDateRanges.value.partyEntry.length === 2) {
      const partyDate = new Date(cadre.party_entry_date);
      const [startDate, endDate] = filterDateRanges.value.partyEntry.map(date => new Date(date));
      
      if (partyDate < startDate) {
        return false;
      }
      
      if (partyDate > endDate) {
        return false;
      }
    }
    
    // 试用期满到期提醒范围筛选
    if (filterDateRanges.value.probationEndReminder && filterDateRanges.value.probationEndReminder.length === 2) {
      const reminderDate = new Date(cadre.probation_end_reminder);
      const [startDate, endDate] = filterDateRanges.value.probationEndReminder.map(date => new Date(date));
      
      if (reminderDate < startDate) {
        return false;
      }
      
      if (reminderDate > endDate) {
        return false;
      }
    }
    
    // 全日制毕业院校系及专业筛选
    if (filters.value.fullTimeSchoolMajor && !cadre.full_time_school_major.includes(filters.value.fullTimeSchoolMajor)) {
      return false;
    }
    
    // 在职毕业院校系及专业筛选
    if (filters.value.partTimeSchoolPhone && !cadre.part_time_school_phone.includes(filters.value.partTimeSchoolPhone)) {
      return false;
    }
    
    // 试用期筛选
    if (filters.value.probationPeriod && cadre.probation_period !== parseFloat(filters.value.probationPeriod)) {
      return false;
    }
    
    // 备注筛选
    if (filters.value.remarks && !cadre.remarks.includes(filters.value.remarks)) {
      return false;
    }
    
    return true;
  });
});

// 加载所有干部信息
async function loadCadreInfo() {
  try {
    cadreList.value = await invoke("get_all_cadre_info");
  } catch (error) {
    console.error("加载干部信息失败:", error);
  }
}

// 保存干部信息
async function saveCadreInfo() {
  try {
    // 表单验证
    await cadreFormRef.value.validate();
    
    // 创建一个新对象用于保存，确保日期字段被正确格式化
    const cadreToSave = { ...currentCadre.value };
    
    // 格式化所有日期字段为字符串
    if (cadreToSave.company_entry_date) {
      cadreToSave.company_entry_date = formatDate(cadreToSave.company_entry_date);
    }
    
    if (cadreToSave.work_start_date) {
      cadreToSave.work_start_date = formatDate(cadreToSave.work_start_date);
    }
    
    if (cadreToSave.current_level_date) {
      cadreToSave.current_level_date = formatDate(cadreToSave.current_level_date);
    }
    
    if (cadreToSave.position_entry_date) {
      cadreToSave.position_entry_date = formatDate(cadreToSave.position_entry_date);
    }
    
    if (cadreToSave.probation_end_reminder) {
      cadreToSave.probation_end_reminder = formatDate(cadreToSave.probation_end_reminder);
    }
    
    if (cadreToSave.birth_date) {
      cadreToSave.birth_date = formatDate(cadreToSave.birth_date);
    }
    
    if (cadreToSave.party_entry_date) {
      cadreToSave.party_entry_date = formatDate(cadreToSave.party_entry_date);
    }
    
    // 确保数字字段被正确处理
    if (cadreToSave.probation_period !== null && cadreToSave.probation_period !== undefined) {
      cadreToSave.probation_period = cadreToSave.probation_period.toString();
    } else {
      cadreToSave.probation_period = null;
    }
    
    // 其他数字字段保持原样，因为后端期望的是数字类型
    // age, company_tenure, work_tenure 在后端是数字类型，不需要转换为字符串
    
    if (cadreToSave.id) {
      // 更新现有记录
      await invoke("update_cadre_info", { cadre: cadreToSave });
    } else {
      // 添加新记录
      await invoke("add_cadre_info", { cadre: cadreToSave });
    }
    closeModal();
    loadCadreInfo();
  } catch (error) {
    console.error("保存干部信息失败:", error);
    if (error.name === 'ValidationError') {
      console.error("表单验证失败:", error);
    } else {
      console.error("保存干部信息失败:", error);
    }
  }
}

// 编辑干部信息
function editCadre(cadre) {
  // 创建深拷贝
  const cadreCopy = { ...cadre };
  
  // 处理日期字段，将字符串转换为日期对象
  if (cadreCopy.company_entry_date && typeof cadreCopy.company_entry_date === 'string') {
    cadreCopy.company_entry_date = new Date(cadreCopy.company_entry_date);
  }
  
  if (cadreCopy.work_start_date && typeof cadreCopy.work_start_date === 'string') {
    cadreCopy.work_start_date = new Date(cadreCopy.work_start_date);
  }
  
  if (cadreCopy.current_level_date && typeof cadreCopy.current_level_date === 'string') {
    cadreCopy.current_level_date = new Date(cadreCopy.current_level_date);
  }
  
  if (cadreCopy.position_entry_date && typeof cadreCopy.position_entry_date === 'string') {
    cadreCopy.position_entry_date = new Date(cadreCopy.position_entry_date);
  }
  
  if (cadreCopy.probation_end_reminder && typeof cadreCopy.probation_end_reminder === 'string') {
    cadreCopy.probation_end_reminder = new Date(cadreCopy.probation_end_reminder);
  }
  
  if (cadreCopy.birth_date && typeof cadreCopy.birth_date === 'string') {
    cadreCopy.birth_date = new Date(cadreCopy.birth_date);
  }
  
  if (cadreCopy.party_entry_date && typeof cadreCopy.party_entry_date === 'string') {
    cadreCopy.party_entry_date = new Date(cadreCopy.party_entry_date);
  }
  
  currentCadre.value = cadreCopy;
  showModal.value = true;
  
  // 重新计算司龄和工龄
  if (currentCadre.value.company_entry_date) {
    calculateCompanyTenure();
  }
  
  if (currentCadre.value.work_start_date) {
    calculateWorkTenure();
  }
  
  // 重新计算试用期满到期提醒
  if (currentCadre.value.probation_period && currentCadre.value.position_entry_date) {
    calculateProbationEnd();
  }
  
  // 如果有身份证号，重新提取信息
  if (currentCadre.value.id_number && currentCadre.value.id_number.length === 18) {
    extractIdInfo();
  }
}

// 删除干部信息
async function deleteCadre(id) {
  if (confirm("确定要删除这条记录吗？")) {
    try {
      await invoke("delete_cadre_info", { id });
      loadCadreInfo();
    } catch (error) {
      console.error("删除干部信息失败:", error);
    }
  }
}

// 打开新增弹窗
function openAddModal() {
  resetForm();
  showModal.value = true;
}

// 关闭弹窗
function closeModal() {
  showModal.value = false;
}

// 重置表单
function resetForm() {
  currentCadre.value = {
    id: null,
    name: "",
    gender: "",
    department: "",
    section: "",
    position1: "",
    position2: "",
    company_entry_date: "",
    company_tenure: null,
    work_start_date: "",
    work_tenure: null,
    current_level_date: "",
    position_entry_date: "",
    probation_period: null,
    probation_end_reminder: "",
    id_number: "",
    birth_date: "",
    age: null,
    native_place: "",
    birth_place: "",
    ethnicity: "",
    technical_position: "",
    education: "",
    full_time_education: "",
    full_time_school_major: "",
    part_time_education: "",
    part_time_school_phone: "",
    political_status: "",
    party_entry_date: "",
    phone: "",
    remarks: "",
    major: "",
    contact_date: "",
    special_date: ""
  };
}

// 打开导出配置弹窗
function openExportModal() {
  showExportModal.value = true;
}

// 全选/取消全选功能
function toggleSelectAll() {
  if (selectAll.value) {
    // 全选
    selectedCadres.value = [...cadreList.value];
  } else {
    // 取消全选
    selectedCadres.value = [];
  }
}

// 监听选中项的变化，更新全选状态
watch(selectedCadres, (newVal) => {
  if (newVal.length === cadreList.value.length && newVal.length > 0) {
    selectAll.value = true;
  } else {
    selectAll.value = false;
  }
}, { deep: true });

// 关闭导出配置弹窗
function closeExportModal() {
  showExportModal.value = false;
}

// 切换筛选面板显示状态
function toggleFilterPanel() {
  showFilterPanel.value = !showFilterPanel.value;
}


// 应用筛选条件
function applyFilters() {
  // 筛选逻辑将在模板中通过计算属性实现
  console.log("应用筛选条件:", filters.value);
}

// 清空筛选条件
function clearFilters() {
  filters.value = {
    name: "",
    gender: "",
    department: "",
    section: "",
    position1: "",
    position2: "",
    education: "",
    politicalStatus: "",
    idNumber: "",
    nativePlace: "",
    ethnicity: "",
    technicalPosition: "",
    phone: "",
    ageMin: "",
    ageMax: "",
    birthPlace: "",
    fullTimeEducation: "",
    partTimeEducation: "",
    probationPeriod: "",
    fullTimeSchoolMajor: "",
    partTimeSchoolPhone: "",
    remarks: ""
  };
  filterDateRanges.value = {
    companyEntryDate: [],
    birthDate: [],
    workStart: [],
    currentLevel: [],
    positionEntry: [],
    partyEntry: [],
    probationEndReminder: []
  };
}

// 切换全选字段
function toggleAllFields() {
  if (exportConfig.value.selectAllFields) {
    exportConfig.value.selectedFields = exportFields.map(field => field.key);
  } else {
    exportConfig.value.selectedFields = [];
  }
}

// 重置字段选择
function resetFields() {
  exportConfig.value.selectedFields = exportFields.map(field => field.key);
  exportConfig.value.selectAllFields = true;
}

// 执行导出操作
async function performExport() {
  try {
    // 构造文件路径
    const filePath = `${exportConfig.value.fileName}.xlsx`;
    
    // 获取要导出的数据ID列表（如果是选中导出）
    let cadreIds = null;
    if (selectedCadres.value.length > 0) {
      cadreIds = selectedCadres.value.map(cadre => cadre.id);
    }
    
    await invoke("export_cadre_info_to_excel", { 
      filePath, 
      selectedFields: exportConfig.value.selectedFields,
      cadreIds // 如果为null则导出全部
    });
    
    closeExportModal();
    alert("导出成功！文件已保存为: " + filePath);
  } catch (error) {
    console.error("导出失败:", error);
    alert("导出失败: " + error);
  }
}

// 原始的导出函数保持兼容性
async function exportToExcel() {
  try {
    // 使用固定文件路径进行测试
    const filePath = "干部信息.xlsx";
    
    // 导出所有字段
    const selectedFields = exportFields.map(field => field.key);
    
    await invoke("export_cadre_info_to_excel", { 
      filePath, 
      selectedFields: selectedFields 
    });
    
    alert("导出成功！文件已保存为: " + filePath);
  } catch (error) {
    console.error("导出失败:", error);
    alert("导出失败: " + error);
  }
}

// 导出选中干部信息
async function exportSelectedCadres() {
  if (selectedCadres.value.length === 0) {
    alert("请先选择要导出的干部信息");
    return;
  }
  
  try {
    // 打开导出配置弹窗，设置默认文件名为"选中干部信息"
    exportConfig.value.fileName = "选中干部信息";
    showExportModal.value = true;
  } catch (error) {
    console.error("导出选中干部信息失败:", error);
    alert("导出失败: " + error);
  }
}

// 导出全部干部信息
async function exportAllCadres() {
  try {
    // 打开导出配置弹窗，设置默认文件名为"全部干部信息"
    exportConfig.value.fileName = "全部干部信息";
    showExportModal.value = true;
  } catch (error) {
    console.error("导出全部干部信息失败:", error);
    alert("导出失败: " + error);
  }
}

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

// 计算司龄
function calculateCompanyTenure() {
  if (currentCadre.value.company_entry_date) {
    // 确保日期是Date对象
    const entryDate = currentCadre.value.company_entry_date instanceof Date 
      ? currentCadre.value.company_entry_date 
      : new Date(currentCadre.value.company_entry_date);
    
    if (isNaN(entryDate.getTime())) {
      currentCadre.value.company_tenure = null;
      return;
    }
    
    const today = new Date();
    const diffTime = Math.abs(today - entryDate);
    const diffYears = diffTime / (1000 * 60 * 60 * 24 * 365);
    currentCadre.value.company_tenure = parseFloat(diffYears.toFixed(1));
  } else {
    currentCadre.value.company_tenure = null;
  }
}

// 计算工龄
function calculateWorkTenure() {
  if (currentCadre.value.work_start_date) {
    // 确保日期是Date对象
    const startDate = currentCadre.value.work_start_date instanceof Date 
      ? currentCadre.value.work_start_date 
      : new Date(currentCadre.value.work_start_date);
    
    if (isNaN(startDate.getTime())) {
      currentCadre.value.work_tenure = null;
      return;
    }
    
    const today = new Date();
    const diffTime = Math.abs(today - startDate);
    const diffYears = diffTime / (1000 * 60 * 60 * 24 * 365);
    currentCadre.value.work_tenure = parseFloat(diffYears.toFixed(1));
  } else {
    currentCadre.value.work_tenure = null;
  }
}

// 计算试用期满到期提醒
function calculateProbationEnd() {
  // 只有当试用期和任职时间都存在时才计算
  if (currentCadre.value.probation_period && currentCadre.value.position_entry_date) {
    // 确保日期是Date对象
    const positionDate = currentCadre.value.position_entry_date instanceof Date 
      ? currentCadre.value.position_entry_date 
      : new Date(currentCadre.value.position_entry_date);
    
    if (isNaN(positionDate.getTime())) {
      currentCadre.value.probation_end_reminder = "";
      return;
    }
    
    const probationYears = parseFloat(currentCadre.value.probation_period);
    
    // 计算试用期结束日期
    const endDate = new Date(positionDate);
    endDate.setFullYear(endDate.getFullYear() + Math.floor(probationYears));
    endDate.setMonth(endDate.getMonth() + Math.round((probationYears % 1) * 12));
    
    // 直接设置日期对象
    currentCadre.value.probation_end_reminder = endDate;
  } else {
    currentCadre.value.probation_end_reminder = "";
  }
}

// 从身份证号提取出生日期和年龄
function extractIdInfo() {
  const idNumber = currentCadre.value.id_number;
  if (idNumber && idNumber.length === 18) {
    // 提取出生日期 (第7-14位)
    const birthYear = idNumber.substring(6, 10);
    const birthMonth = idNumber.substring(10, 12);
    const birthDay = idNumber.substring(12, 14);
    
    // 创建日期对象
    const birthDate = new Date(birthYear, birthMonth - 1, birthDay);
    
    // 检查日期是否有效
    if (!isNaN(birthDate.getTime())) {
      // 设置日期对象
      currentCadre.value.birth_date = birthDate;
      
      // 计算年龄
      const today = new Date();
      let age = today.getFullYear() - birthDate.getFullYear();
      const monthDiff = today.getMonth() - birthDate.getMonth();
      
      // 如果还没过生日，则年龄减1
      if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
        age--;
      }
      
      currentCadre.value.age = age;
    } else {
      currentCadre.value.birth_date = "";
      currentCadre.value.age = null;
    }
  } else {
    currentCadre.value.birth_date = "";
    currentCadre.value.age = null;
  }
}



// 组件挂载时加载数据
onMounted(() => {
  loadCadreInfo();
});
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

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  overflow: hidden;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4edf5 100%);
}

.sidebar {
  width: 250px;
  background: linear-gradient(180deg, #2d3748, #1a202c);
  color: #e2e8f0;
  box-shadow: 4px 0 20px rgba(0, 0, 0, 0.15);
  overflow-y: auto;
  height: 100vh;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #4a5568;
  padding: 20px 0;
}


.logo {
  padding: 20px 20px 25px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  margin-bottom: 0;
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
  padding: 20px 10px 0;
  overflow-y: auto;
  flex: 1;
}

.menu-item {
  display: flex;
  align-items: center;
  width: calc(100% - 20px);
  padding: 16px 20px;
  background: transparent;
  border: none;
  color: #cbd5e0;
  font-size: 1.05rem;
  text-align: left;
  cursor: pointer;
  border-radius: 10px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  margin: 0 10px 8px;
  position: relative;
  overflow: hidden;
  font-weight: 500;
}

.menu-item:last-child {
  margin-bottom: 20px;
}

.menu-item:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #ffffff;
  transform: translateX(3px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.menu-item.active {
  background: linear-gradient(90deg, var(--primary-color), #5a7dff);
  color: white;
  font-weight: 600;
  box-shadow: 0 6px 12px rgba(67, 97, 238, 0.25);
}

.menu-item.active:hover {
  transform: translateX(0);
  box-shadow: 0 8px 16px rgba(67, 97, 238, 0.3);
}

.menu-item::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  width: 3px;
  background: transparent;
  border-radius: 0 2px 2px 0;
  transition: all 0.3s ease;
}

.menu-item.active::before {
  background: #ffffff;
}

.menu-icon {
  margin-right: 12px;
  font-size: 1.2rem;
}

.main-content {
  flex: 1;
  padding: 25px;
  overflow-y: auto;
  background: linear-gradient(135deg, #f8fafc, #f1f5f9);
  min-height: 100vh;
}

.content-section {
  background: white;
  border-radius: 16px;
  padding: 30px;
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

.header-actions {
  display: flex;
  gap: 10px;
}

.section-header h2 {
  margin: 0;
  color: #1e293b;
  font-size: 2rem;
  font-weight: 700;
  letter-spacing: -0.5px;
  position: relative;
  padding-bottom: 10px;
}

.section-header h2::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 60px;
  height: 4px;
  background: linear-gradient(90deg, var(--primary-color), var(--secondary-color));
  border-radius: 2px;
}

.btn-primary {
  background: linear-gradient(135deg, var(--primary-color), var(--primary-dark));
  color: white;
  border: none;
  padding: 14px 28px;
  border-radius: 10px;
  cursor: pointer;
  font-size: 1.05rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 4px 12px rgba(67, 97, 238, 0.3);
  position: relative;
  overflow: hidden;
  letter-spacing: 0.3px;
}

.btn-primary:hover {
  background: linear-gradient(135deg, var(--primary-light), var(--primary-color));
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(67, 97, 238, 0.4);
}

.btn-primary:active {
  transform: translateY(-1px);
  box-shadow: 0 4px 10px rgba(67, 97, 238, 0.3);
}

.btn-primary::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  transition: left 0.6s ease;
}

.btn-primary:hover::before {
  left: 100%;
}

.btn-primary:focus {
  outline: none;
  box-shadow: 0 0 0 4px rgba(67, 97, 238, 0.2);
}

.horizontal-table-container {
  overflow-x: auto;
  border-radius: 12px;
  box-shadow: var(--card-shadow);
  background: white;
  border: 1px solid rgba(0, 0, 0, 0.05);
}

.table-header {
  position: sticky;
  top: 0;
  z-index: 10;
}

.table-row {
  display: flex;
  border-bottom: 1px solid #f1f5f9;
  transition: all 0.2s ease;
  position: relative;
}

.table-row:hover {
  background-color: #f8fafc;
}

.table-row:nth-child(even) {
  background-color: #fdfdfd;
}

.table-row:nth-child(even):hover {
  background-color: #f8fafc;
}

.header-row {
  background: linear-gradient(180deg, #f8fafc, #f1f5f9);
  font-weight: 700;
  color: #1e293b;
  position: sticky;
  top: 0;
  z-index: 10;
  box-shadow: 0 3px 6px rgba(0,0,0,0.05);
  border-bottom: 2px solid #e2e8f0;
}

.table-cell {
  flex: 1;
  min-width: 120px;
  padding: 14px 16px;
  text-align: left;
  border-right: 1px solid #f1f5f9;
  word-break: break-word;
  display: flex;
  align-items: center;
  transition: background-color 0.2s ease;
  position: relative;
  color: #334155;
  font-size: 0.95rem;
}

.table-cell:last-child {
  border-right: none;
}

/* 选择列样式 */
.table-cell:first-child {
  flex: 0 0 60px;
  justify-content: center;
  background-color: #f8fafc;
  font-weight: 500;
}

/* 序号列样式 */
.table-cell:nth-child(2) {
  flex: 0 0 60px;
  justify-content: center;
  background-color: #f8fafc;
  font-weight: 500;
}

/* 操作列样式 */
.actions-cell {
  flex: 0 0 120px;
  display: flex;
  gap: 8px;
  align-items: center;
  justify-content: center;
  background-color: #f8fafc;
}

.actions-cell .btn-small {
  margin: 0;
}

/* 表头单元格样式 */
.header-row .table-cell {
  background: linear-gradient(180deg, #f1f5f9, #e2e8f0);
  font-weight: 700;
  color: #1e293b;
  text-align: center;
  padding: 16px;
  border-bottom: 2px solid #cbd5e0;
}

/* 奇偶行样式 */
.table-row:nth-child(even) .table-cell {
  background-color: #fdfdfd;
}

.table-row:nth-child(odd) .table-cell {
  background-color: white;
}

.table-row:hover .table-cell {
  background-color: #f1f5f9;
}

.table-row:hover .table-cell:first-child,
.table-row:hover .table-cell:nth-child(2),
.table-row:hover .actions-cell {
  background-color: #e2e8f0;
}

.btn-small {
  padding: 8px 14px;
  margin-right: 6px;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
  position: relative;
  overflow: hidden;
  font-weight: 500;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.btn-small:not(.btn-danger) {
  background: linear-gradient(135deg, var(--primary-color), var(--primary-light));
  color: white;
}

.btn-small:not(.btn-danger):hover {
  background: linear-gradient(135deg, var(--primary-light), var(--primary-color));
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(67, 97, 238, 0.2);
}

.btn-small:not(.btn-danger):active {
  transform: translateY(0);
  box-shadow: 0 2px 4px rgba(67, 97, 238, 0.15);
}

.btn-danger {
  background: linear-gradient(135deg, var(--danger-color), #d90429);
  color: white;
}

.btn-danger:hover {
  background: linear-gradient(135deg, #d90429, var(--danger-color));
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(230, 57, 70, 0.25);
}

.btn-danger:active {
  transform: translateY(0);
  box-shadow: 0 2px 4px rgba(230, 57, 70, 0.2);
}

.btn-small::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.6s ease;
}

.btn-small:hover::before {
  left: 100%;
}

.btn-small:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(67, 97, 238, 0.2);
}

.btn-danger:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(230, 57, 70, 0.2);
}

/* 统计页面样式 */
.statistics-content {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 25px;
  margin-top: 25px;
}

.stat-card {
  background: linear-gradient(135deg, white, #f8fafc);
  border-radius: 18px;
  padding: 30px;
  box-shadow: var(--card-shadow);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
  border: 1px solid rgba(0, 0, 0, 0.03);
}

.stat-card:hover {
  transform: translateY(-5px);
  box-shadow: var(--hover-shadow);
}

.stat-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 5px;
  background: linear-gradient(90deg, var(--primary-color), var(--secondary-color));
}

.stat-card h3 {
  margin-top: 0;
  color: #1e293b;
  font-size: 1.3rem;
  font-weight: 700;
  position: relative;
  padding-bottom: 12px;
  letter-spacing: -0.3px;
}

.stat-card h3::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 50px;
  height: 3px;
  background: linear-gradient(90deg, var(--primary-color), var(--secondary-color));
  border-radius: 2px;
}

.stat-number {
  font-size: 2.8rem;
  font-weight: 800;
  color: var(--primary-color);
  margin: 15px 0;
  text-shadow: 0 2px 8px rgba(67, 97, 238, 0.15);
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

/* 弹窗样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(8px);
  animation: overlayFadeIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-content {
  background: linear-gradient(135deg, white, #f8fafc);
  border-radius: 16px;
  width: 95%;
  max-width: 1000px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 30px 60px -15px rgba(0, 0, 0, 0.3);
  animation: modalFadeIn 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

@keyframes overlayFadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes modalFadeIn {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(20px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 25px 30px;
  border-bottom: 1px solid #edf2f7;
  background-color: #f8fafc;
  border-radius: 16px 16px 0 0;
}

.modal-header h3 {
  margin: 0;
  color: #1a202c;
  font-size: 1.5rem;
  font-weight: 600;
}

.modal-close {
  background: none;
  border: none;
  font-size: 1.8rem;
  cursor: pointer;
  color: #a0aec0;
  padding: 0;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.2s ease;
}

.modal-close:hover {
  background: #e2e8f0;
  color: #1a202c;
}

.modal-body {
  padding: 30px;
  background-color: white;
}

/* 表单分组样式 */
.form-section {
  margin-bottom: 30px;
  padding: 20px;
  border-radius: 12px;
  background: linear-gradient(135deg, #f8fafc, #f1f5f9);
  border: 1px solid #e2e8f0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.03);
}

.form-section:last-child {
  margin-bottom: 0;
}

.form-section-title {
  font-size: 1.3rem;
  font-weight: 700;
  color: #1e293b;
  margin: 0 0 20px 0;
  padding-bottom: 12px;
  border-bottom: 3px solid #4299e1;
  display: inline-block;
  position: relative;
}

.form-section-title::after {
  content: '';
  position: absolute;
  bottom: -3px;
  left: 0;
  width: 60px;
  height: 3px;
  background: linear-gradient(90deg, #4299e1, #3182ce);
  border-radius: 2px;
}

.form-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-bottom: 25px;
}

.form-group {
  display: flex;
  flex-direction: column;
}

.form-group.full-width {
  grid-column: 1 / -1;
}

.form-group label {
  margin-bottom: 8px;
  font-weight: 600;
  color: #334155;
  font-size: 0.95rem;
  position: relative;
  padding-left: 8px;
}

.form-group label::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 16px;
  background: linear-gradient(180deg, #4299e1, #3182ce);
  border-radius: 2px;
}

/* 统一的输入框样式 */
.form-group input,
.form-group select,
.form-group textarea,
.datepicker-input {
  padding: 14px 18px !important;
  border: 2px solid #e2e8f0 !important;
  border-radius: 10px !important;
  font-size: 1.05rem !important;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1) !important;
  box-sizing: border-box !important;
  background-color: white !important;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05) !important;
  min-height: 46px !important;
  outline: none !important;
  position: relative;
  font-family: inherit !important;
  color: #1e293b !important;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus,
.datepicker-input:focus {
  outline: none !important;
  border-color: var(--primary-color) !important;
  box-shadow: 0 0 0 4px rgba(67, 97, 238, 0.2) !important;
  transform: translateY(-2px);
}

.form-group input:hover,
.form-group select:hover,
.form-group textarea:hover,
.datepicker-input:hover {
  border-color: #cbd5e0 !important;
  box-shadow: 0 3px 8px rgba(0, 0, 0, 0.08) !important;
}

/* 统一的只读样式 */
.form-group input[readonly],
.form-group input:disabled,
.form-group select:disabled,
.form-group textarea:disabled,
.datepicker-input:disabled,
:deep(.dp__input_wrap input:disabled) {
  background-color: #f8fafc !important;
  color: #64748b !important;
  cursor: not-allowed !important;
  opacity: 1 !important;
  border-color: #e2e8f0 !important;
}

.form-group input[readonly]:focus,
.form-group input:disabled:focus,
.form-group select:disabled:focus,
.form-group textarea:disabled:focus,
.datepicker-input:disabled:focus {
  outline: none !important;
  border-color: #e2e8f0 !important;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05) !important;
}

/* 特殊处理选择框 */
.form-group select {
  appearance: none !important;
  -webkit-appearance: none !important;
  -moz-appearance: none !important;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%234a5568' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e") !important;
  background-repeat: no-repeat !important;
  background-position: right 16px center !important;
  background-size: 18px !important;
  padding-right: 45px !important;
}

/* 特殊处理选择框以保持视觉一致性 */
.form-group select {
  appearance: none !important;
  -webkit-appearance: none !important;
  -moz-appearance: none !important;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%234a5568' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e") !important;
  background-repeat: no-repeat !important;
  background-position: right 16px center !important;
  background-size: 16px !important;
  padding-right: 40px !important; /* 为下拉箭头留出空间 */
}

/* 统一的只读样式 */
.form-group input[readonly],
.form-group input:disabled,
.form-group select:disabled,
.form-group textarea:disabled,
.datepicker-input:disabled,
:deep(.dp__input_wrap input:disabled) {
  background-color: #e2e8f0 !important;
  color: #4a5568 !important;
  cursor: not-allowed !important;
  opacity: 1 !important;
  border-color: #e2e8f0 !important;
}

.form-group input[readonly]:focus,
.form-group input:disabled:focus,
.form-group select:disabled:focus,
.form-group textarea:disabled:focus,
.datepicker-input:disabled:focus {
  outline: none !important;
  border-color: #e2e8f0 !important;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05) !important;
}

/* 日期选择器悬停样式 */
.datepicker-input:hover:not(:disabled) {
  border-color: #cbd5e0 !important;
}

/* 特殊处理textarea的高度 */
.form-group textarea {
  min-height: 120px !important;
  resize: vertical !important;
  font-family: inherit !important;
}

/* 全局日期选择器样式 */
:deep(.dp__theme_light) {
  --dp-primary-color: #4299e1;
  --dp-secondary-color: #3182ce;
  --dp-border-radius: 8px;
  --dp-menu-border-color: #e2e8f0;
  --dp-background-color: #ffffff;
  --dp-text-color: #1a202c;
  --dp-hover-color: #f1f5f9;
  --dp-selected-color: #4299e1;
}

:deep(.dp__action_buttons) {
  display: none;
}

:deep(.dp__menu) {
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  animation: datepickerFadeIn 0.2s ease;
  z-index: 1100;
  border: 1px solid #e2e8f0;
}

:deep(.dp__cell_inner) {
  border-radius: 50%;
  font-weight: 500;
  transition: all 0.2s ease;
}

:deep(.dp__cell_inner:hover) {
  background-color: #f7fafc;
}

:deep(.dp__today) {
  border: 1px solid var(--dp-primary-color);
  background-color: #ebf8ff;
}

:deep(.dp__active_date) {
  background-color: var(--dp-primary-color);
  color: white;
}

:deep(.dp__active_date:hover) {
  background-color: var(--dp-primary-color);
}

:deep(.dp__month_year_select) {
  font-weight: 600;
  border-radius: 6px;
  padding: 4px 8px;
}

:deep(.dp__month_year_select:hover) {
  background-color: #f7fafc;
}

:deep(.dp__overlay_cell_active) {
  background-color: var(--dp-primary-color);
  color: white;
}

@keyframes datepickerFadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.form-group textarea {
  min-height: 120px;
  resize: vertical;
  font-family: inherit;
}

/* 组合输入框样式 */
.combobox-input {
  width: 100%;
  padding: 12px 16px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  transition: all 0.2s ease;
  box-sizing: border-box;
  background-color: white;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

/* 表格复选框样式 */
.row-checkbox {
  width: 20px;
  height: 20px;
  cursor: pointer;
  accent-color: var(--primary-color);
  transform: scale(1.1);
  margin: 0 auto;
  border-radius: 5px;
  border: 2px solid #cbd5e0;
  transition: all 0.2s ease;
}

/* 文件名输入框样式 */
.filename-input {
  width: 100%;
  padding: 12px 16px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  transition: all 0.2s ease;
  box-sizing: border-box;
  background-color: white;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.filename-input:focus {
  outline: none;
  border-color: #4299e1;
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.1);
}

/* 文件名行样式 - 标签和输入框在同一行 */
.filename-row-container {
  display: flex;
  align-items: center;
  gap: 15px;
  margin-bottom: 20px;
}

.filename-label {
  white-space: nowrap;
  margin-bottom: 0;
  flex: 0 0 auto;
  font-weight: 500;
  color: #334155;
}

.filename-input {
  flex: 1;
  margin-bottom: 0;
  padding: 12px 16px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  transition: all 0.2s ease;
  box-sizing: border-box;
  background-color: white;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.filename-input:focus {
  outline: none;
  border-color: #4299e1;
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.1);
}


/* 字段选择部分样式 */
.field-section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.field-section-header h4 {
  margin: 0;
}

.field-actions {
  display: flex;
  gap: 15px;
  align-items: center;
}

/* 重置按钮样式 - 更小的尺寸 */
.btn-reset {
  background: linear-gradient(135deg, #f1f5f9, #e2e8f0);
  color: #4a5568;
  border: 1px solid #cbd5e0;
  padding: 6px 12px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 500;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
  position: relative;
  overflow: hidden;
  letter-spacing: 0.3px;
  white-space: nowrap;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.btn-reset:hover {
  background: linear-gradient(135deg, #e2e8f0, #cbd5e0);
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.12);
  color: #2d3748;
  border-color: #a0aec0;
}

.btn-reset:active {
  transform: translateY(0);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.08);
}

.btn-reset::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.6s ease;
}

.btn-reset:hover::before {
  left: 100%;
}

.btn-reset:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(203, 213, 224, 0.3);
}

/* 模态框按钮样式 - 更小的尺寸 */
.btn-modal-secondary {
  background: linear-gradient(135deg, #f1f5f9, #e2e8f0);
  color: #4a5568;
  border: 1px solid #cbd5e0;
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.95rem;
  font-weight: 500;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
  position: relative;
  overflow: hidden;
  letter-spacing: 0.3px;
}

.btn-modal-secondary:hover {
  background: linear-gradient(135deg, #e2e8f0, #cbd5e0);
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.12);
  color: #2d3748;
  border-color: #a0aec0;
}

.btn-modal-secondary:active {
  transform: translateY(0);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.08);
}

.btn-modal-secondary::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.6s ease;
}

.btn-modal-secondary:hover::before {
  left: 100%;
}

.btn-modal-secondary:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(203, 213, 224, 0.3);
}

.btn-modal-primary {
  background: linear-gradient(135deg, var(--primary-color), var(--primary-dark));
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.95rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(67, 97, 238, 0.3);
  position: relative;
  overflow: hidden;
  letter-spacing: 0.3px;
}

.btn-modal-primary:hover {
  background: linear-gradient(135deg, var(--primary-light), var(--primary-color));
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(67, 97, 238, 0.4);
}

.btn-modal-primary:active {
  transform: translateY(0);
  box-shadow: 0 1px 3px rgba(67, 97, 238, 0.3);
}

.btn-modal-primary::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  transition: left 0.6s ease;
}

.btn-modal-primary:hover::before {
  left: 100%;
}

.btn-modal-primary:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(67, 97, 238, 0.2);
}

/* 字段网格布局 - 多行横向排列 */
.field-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 15px;
  max-height: 300px;
  overflow-y: auto;
  padding: 10px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  background-color: #f8fafc;
}

.field-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  background-color: white;
  border-radius: 6px;
  border: 1px solid #e2e8f0;
  transition: all 0.2s ease;
}

.field-item:hover {
  background-color: #f1f5f9;
  border-color: #cbd5e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  width: 100%;
  margin-bottom: 0;
}

.checkbox-input {
  width: 18px;
  height: 18px;
  cursor: pointer;
  accent-color: var(--primary-color);
}

.row-checkbox:hover {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(67, 97, 238, 0.1);
}

/* 表头复选框样式 */
.header-row .row-checkbox {
  transform: scale(1.1);
  border-color: #a0aec0;
}

.header-row .row-checkbox:hover {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(67, 97, 238, 0.2);
}

.combobox-input:focus {
  outline: none;
  border-color: #4299e1;
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.1);
}

/* 筛选条件面板样式 */
.filter-panel {
  background: linear-gradient(135deg, white, #f8fafc);
  border-radius: 12px;
  padding: 20px;
  box-shadow: var(--card-shadow);
  margin-bottom: 20px;
  border: 1px solid rgba(0, 0, 0, 0.05);
}

/* 筛选条件展开/收起按钮样式优化 */
.toggle-filter-button {
  padding: 12px 24px !important; /* 增大内边距 */
  font-size: 1.1rem !important; /* 增大字体 */
  min-height: 44px !important; /* 设置最小高度 */
  border-radius: 10px !important; /* 增大圆角 */
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1) !important; /* 添加过渡动画 */
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1) !important; /* 添加阴影 */
}

.toggle-filter-button:hover {
  transform: translateY(-2px) !important; /* 悬停时上移 */
  box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15) !important; /* 悬停时增强阴影 */
}

.toggle-filter-button:active {
  transform: translateY(0) !important; /* 点击时恢复原位 */
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1) !important; /* 点击时减弱阴影 */
}

.filter-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.filter-header h3 {
  margin: 0;
  color: #1e293b;
  font-size: 1.3rem;
  font-weight: 600;
}

.filter-content {
  padding: 15px 0;
}

.filter-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 16px;
}

.filter-group {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 12px;
}

.filter-group label {
  flex: 0 0 auto;
  min-width: 80px;
  margin-bottom: 0;
  font-weight: 500;
  color: #334155;
  font-size: 0.95rem;
  white-space: nowrap;
}

.filter-input,
.filter-select {
  flex: 1;
  padding: 10px 14px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  transition: all 0.2s ease;
  box-sizing: border-box;
  background-color: white;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  min-height: 42px;
}

.filter-input:focus,
.filter-select:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(67, 97, 238, 0.2);
}

.filter-input:hover,
.filter-select:hover {
  border-color: #cbd5e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
}

/* 日期范围控件样式 */
.date-range-container {
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: white;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 4px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  min-height: 42px;
}

.date-range-input {
  flex: 1;
  border: none !important;
  padding: 10px 12px !important;
  font-size: 1rem !important;
  outline: none !important;
  background-color: transparent !important;
  box-shadow: none !important;
  min-height: auto !important;
}

.date-range-input:focus {
  outline: none !important;
  border: none !important;
  box-shadow: none !important;
}

.date-range-separator {
  color: #64748b;
  font-size: 0.9rem;
  font-weight: 500;
  white-space: nowrap;
  padding: 0 4px;
}

/* 年龄范围控件样式 */
.age-range-container {
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: white;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 4px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  min-height: 42px;
}

.age-range-input {
  flex: 1;
  border: none !important;
  padding: 10px 12px !important;
  font-size: 1rem !important;
  outline: none !important;
  background-color: transparent !important;
  box-shadow: none !important;
  min-height: auto !important;
  text-align: center;
}

.age-range-input:focus {
  outline: none !important;
  border: none !important;
  box-shadow: none !important;
}

.age-range-separator {
  color: #64748b;
  font-size: 0.9rem;
  font-weight: 500;
  white-space: nowrap;
  padding: 0 4px;
}

.age-range-unit {
  color: #64748b;
  font-size: 0.9rem;
  font-weight: 500;
  white-space: nowrap;
  padding: 0 4px;
}

/* 响应式设计 - 筛选控件 */
@media (max-width: 1200px) {
  .filter-row {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 992px) {
  .filter-row {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .filter-row {
    grid-template-columns: 1fr;
  }
  
  .filter-group {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .filter-group label {
    min-width: auto;
    margin-bottom: 0;
    white-space: normal;
  }
  
  .filter-input,
  .filter-select {
    width: 100%;
  }
  
  .date-range-container {
    flex-direction: column;
    gap: 6px;
  }
  
  .date-range-separator {
    width: 100%;
    text-align: center;
    padding: 4px 0;
  }
  
  .age-range-container {
    flex-direction: column;
    gap: 6px;
    padding: 6px;
  }
  
  .age-range-separator,
  .age-range-unit {
    width: 100%;
    text-align: center;
    padding: 4px 0;
  }
}

.filter-actions {
  display: flex;
  gap: 15px;
  justify-content: flex-end;
  padding-top: 15px;
  border-top: 1px solid #e2e8f0;
}


.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 15px;
  margin-top: 30px;
  padding-top: 25px;
  border-top: 2px solid #e2e8f0;
  background: linear-gradient(180deg, transparent, #f8fafc);
  border-radius: 0 0 12px 12px;
  padding: 25px;
}

.btn-secondary {
  background: linear-gradient(135deg, #f1f5f9, #e2e8f0);
  color: #4a5568;
  border: 1px solid #cbd5e0;
  padding: 14px 28px;
  border-radius: 10px;
  cursor: pointer;
  font-size: 1.05rem;
  font-weight: 500;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.08);
  position: relative;
  overflow: hidden;
  letter-spacing: 0.3px;
}

.btn-secondary:hover {
  background: linear-gradient(135deg, #e2e8f0, #cbd5e0);
  transform: translateY(-3px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
  color: #2d3748;
  border-color: #a0aec0;
}

.btn-secondary:active {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
}

.btn-secondary::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.6s ease;
}

.btn-secondary:hover::before {
  left: 100%;
}

.btn-secondary:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(203, 213, 224, 0.3);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .app-container {
    flex-direction: column;
  }
  
  .sidebar {
    width: 100%;
    padding: 10px 0;
    height: auto;
    max-height: 200px;
  }
  
  .menu {
    display: flex;
    padding: 0 10px;
    flex-wrap: wrap;
  }
  
  .menu-item {
    flex: 1 1 auto;
    margin: 0 5px 5px 0;
    padding: 12px;
    font-size: 0.9rem;
    min-width: 120px;
  }
  
  .menu-icon {
    margin-right: 5px;
  }
  
  .main-content {
    padding: 15px;
  }
  
  .content-section {
    padding: 20px;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 15px;
  }
  
  .header-actions {
    width: 100%;
    justify-content: space-between;
  }
  
  .section-header h2 {
    font-size: 1.5rem;
  }
  
  .form-row {
    grid-template-columns: 1fr;
  }
  
  .modal-content {
    width: 95%;
    margin: 10px;
    border-radius: 12px;
    max-height: 95vh;
  }
  
  .modal-header {
    padding: 20px 25px;
  }
  
  .modal-header h3 {
    font-size: 1.3rem;
  }
  
  .modal-body {
    padding: 25px;
  }
  
  .form-row {
    gap: 15px;
    margin-bottom: 20px;
  }
  
  .table-cell {
    min-width: 100px;
    padding: 10px;
    font-size: 0.9rem;
  }
  
  .horizontal-table-container {
    font-size: 0.9rem;
  }
  
  .form-actions {
    margin-top: 25px;
    padding-top: 20px;
    flex-direction: column;
  }
  
  .form-actions .btn-primary,
  .form-actions .btn-secondary {
    width: 100%;
    margin-bottom: 10px;
  }
  
  .statistics-content {
    grid-template-columns: 1fr;
  }
  
  .stat-card {
    padding: 20px;
  }
  
  .stat-number {
    font-size: 2rem;
  }
}

@media (max-width: 480px) {
  .table-cell {
    min-width: 80px;
    padding: 8px;
    font-size: 0.8rem;
  }
  
  .section-header h2 {
    font-size: 1.3rem;
  }
  
  .btn-primary {
    padding: 10px 15px;
    font-size: 0.9rem;
  }
  
  .modal-content {
    width: 98%;
    margin: 5px;
  }
  
  .modal-header {
    padding: 15px 20px;
  }
  
  .modal-body {
    padding: 20px;
  }
  
  .form-group label {
    font-size: 0.9rem;
  }
  
  .form-group input,
  .form-group select,
  .form-group textarea {
    padding: 10px 12px;
    font-size: 0.95rem;
  }
  
  .datepicker-input {
    padding: 10px 12px;
    font-size: 0.95rem;
  }
  
  .menu-item {
    min-width: 100px;
    padding: 10px;
    font-size: 0.85rem;
  }
  
  .stat-number {
    font-size: 1.8rem;
  }
  
  .stat-card h3 {
    font-size: 1.1rem;
  }
}

/* 导出按钮组样式 */
.export-buttons-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 25px 0 20px;
  padding: 20px;
  background: linear-gradient(135deg, white, #f8fafc);
  border-radius: 12px;
  box-shadow: var(--card-shadow);
  border: 1px solid rgba(0, 0, 0, 0.05);
}

.export-buttons-info {
  display: flex;
  align-items: center;
}

.export-count-info {
  font-size: 1.1rem;
  font-weight: 600;
  color: #4a5568;
  background: linear-gradient(135deg, #f1f5f9, #e2e8f0);
  padding: 12px 20px;
  border-radius: 10px;
  border: 1px solid #cbd5e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.export-buttons-group {
  display: flex;
  gap: 15px;
}

.btn-export {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border: none;
  border-radius: 10px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  letter-spacing: 0.3px;
  min-width: 140px;
  justify-content: center;
}

.btn-export-selected {
  background: linear-gradient(135deg, #48bb78, #38a169);
  color: white;
  box-shadow: 0 4px 12px rgba(72, 187, 120, 0.3);
}

.btn-export-selected:hover {
  background: linear-gradient(135deg, #38a169, #2f855a);
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(72, 187, 120, 0.4);
}

.btn-export-full {
  background: linear-gradient(135deg, #4299e1, #3182ce);
  color: white;
  box-shadow: 0 4px 12px rgba(66, 153, 225, 0.3);
}

.btn-export-full:hover {
  background: linear-gradient(135deg, #3182ce, #2c5282);
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(66, 153, 225, 0.4);
}

.btn-export:active {
  transform: translateY(-1px);
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
}

.btn-export::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  transition: left 0.6s ease;
}

.btn-export:hover::before {
  left: 100%;
}

.btn-export:focus {
  outline: none;
  box-shadow: 0 0 0 4px rgba(66, 153, 225, 0.2);
}

/* 响应式设计 - 导出按钮 */
@media (max-width: 768px) {
  .export-buttons-container {
    flex-direction: column;
    gap: 15px;
    margin: 20px 0 15px;
    padding: 15px;
  }
  
  .export-buttons-group {
    width: 100%;
    justify-content: center;
  }
  
  .btn-export {
    width: 100%;
    max-width: 200px;
    padding: 10px 20px;
    font-size: 0.95rem;
  }
  
  .export-count-info {
    font-size: 1rem;
    padding: 10px 16px;
  }
}

@media (max-width: 480px) {
  .export-buttons-container {
    margin: 15px 0;
    padding: 12px;
  }
  
  .btn-export {
    padding: 8px 16px;
    font-size: 0.9rem;
    min-width: 120px;
  }
  
  .export-count-info {
    font-size: 0.9rem;
    padding: 8px 12px;
  }
}
</style>
