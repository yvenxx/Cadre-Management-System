<template>
  <el-dialog
    v-model="visible"
    title="导出配置"
    width="50%"
    @close="handleClose"
  >
    <el-form label-position="top">
      <el-form-item label="文件名">
        <el-input v-model="config.fileName" placeholder="请输入文件名">
          <template #append>.xlsx</template>
        </el-input>
      </el-form-item>
      
      <el-form-item label="选择导出字段">
        <el-checkbox 
          v-model="config.selectAllFields" 
          @change="toggleAllFields"
        >
          全选
        </el-checkbox>
        <el-button @click="resetFields" style="margin-left: 10px;">重置</el-button>
        
        <div class="field-selection">
          <el-checkbox-group v-model="config.selectedFields">
            <el-row :gutter="20">
              <el-col 
                v-for="field in exportFields" 
                :key="field.key" 
                :span="8"
                style="margin-top: 10px;"
              >
                <el-checkbox :label="field.key">{{ field.label }}</el-checkbox>
              </el-col>
            </el-row>
          </el-checkbox-group>
        </div>
      </el-form-item>
    </el-form>
    
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button type="primary" @click="handleExport">导出</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, computed, watch } from "vue";

// 定义组件的属性
const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  },
  selectedCadres: {
    type: Array,
    default: () => []
  },
  exportFields: {
    type: Array,
    required: true
  },
  defaultFileName: {
    type: String,
    default: "干部信息"
  }
});

// 定义组件触发的事件
const emit = defineEmits(['update:modelValue', 'export']);

// 计算属性：控制弹窗显示/隐藏
const visible = computed({
  get() {
    return props.modelValue;
  },
  set(value) {
    emit('update:modelValue', value);
  }
});

// 导出配置
const config = ref({
  fileName: props.defaultFileName,
  selectedFields: [], // 选中的字段
  selectAllFields: false
});

// 监听exportFields变化，更新默认选中字段
watch(() => props.exportFields, (newFields) => {
  config.value.selectedFields = newFields.map(field => field.key);
  config.value.selectAllFields = true;
}, { immediate: true });

// 监听selectedFields变化，更新全选状态
watch(() => config.value.selectedFields, (newVal) => {
  if (newVal.length === props.exportFields.length && newVal.length > 0) {
    config.value.selectAllFields = true;
  } else {
    config.value.selectAllFields = false;
  }
}, { deep: true });

// 关闭弹窗
function handleClose() {
  visible.value = false;
}

// 切换全选字段
function toggleAllFields() {
  if (config.value.selectAllFields) {
    config.value.selectedFields = props.exportFields.map(field => field.key);
  } else {
    config.value.selectedFields = [];
  }
}

// 重置字段选择
function resetFields() {
  config.value.selectedFields = props.exportFields.map(field => field.key);
  config.value.selectAllFields = true;
}

// 处理导出
function handleExport() {
  // 获取要导出的数据ID列表（如果是选中导出）
  let cadreIds = null;
  if (props.selectedCadres.length > 0) {
    cadreIds = props.selectedCadres.map(cadre => cadre.id);
  }
  
  // 触发导出事件
  emit('export', {
    fileName: config.value.fileName,
    selectedFields: config.value.selectedFields,
    cadreIds // 如果为null则导出全部
  });
  
  // 关闭弹窗
  handleClose();
}
</script>

<style scoped>
.field-selection {
  margin-top: 15px;
  padding: 15px;
  border: 1px solid #ebeef5;
  border-radius: 4px;
  max-height: 300px;
  overflow-y: auto;
}
</style>