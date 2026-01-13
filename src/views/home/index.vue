<script setup lang="ts">
import {open} from '@tauri-apps/plugin-dialog'
import {call} from "@/utils/commands";
import {computed, ref} from "vue";
import {openPath} from "@tauri-apps/plugin-opener";
import {sep} from '@tauri-apps/api/path';
import SvgIcon from "../../components/SvgIcon/index.vue";
import {ElMessage} from "element-plus";


const openFileDialog = async () => {
  const file = await open({
    multiple: false, // 允许选择多个文件
    filters: [
      {
        name: 'All Files',
        extensions: ['mp3', 'wav', 'ogg', 'm4a'],
      },
    ],
  })

  if (file !== null) {
    input.value.voice_file = file
  }

}


const input = ref<{
  voice_file: string,
  voice_text: string,
  input: string
}>({
  voice_file: '',
  voice_text: '',
  input: ''
})

const output = ref<string>('')

const loading = ref(false)

const verifyRef = ref()

const voice_clone = () => {
  loading.value = true
  call<{ save_file: string }>('voice_clone', {
    input: {
      voice_file: input.value.voice_file,
      voice_text: input.value.voice_text,
      input: input.value.input.replace(/\n/g, ' ')
    }
  }, {hideError: true}).then((save_file: any) => {
    output.value = save_file
    loading.value = false
  }).catch((error: any) => {
    loading.value = false
    if (error.includes('未激活')) {
      verifyRef.value.show()
    } else {
      ElMessage.error(error)
    }
  })
}

const disable = computed(() => {
  return !input.value.voice_file || !input.value.voice_text || !input.value.input
})


</script>

<template>
  <div class="pd10">
    <div class="flex">
      <div class="import-area" @click="openFileDialog">
        <el-text v-if="!input.voice_file">选择一个音频文件</el-text>
        <el-text v-else>
          <svg-icon icon-class="audio" size="16" class="mr5"></svg-icon>
          {{ input.voice_file?.substring(input.voice_file.lastIndexOf(sep()) + 1) }}
        </el-text>
      </div>
    </div>

    <div class="mt10">
      <el-input v-model="input.voice_text" placeholder="请输入音色对应的文本" class="voice-text"></el-input>
    </div>

    <div class="mt10">
      <el-input v-model="input.input" type="textarea" rows="10"
                placeholder="请输入要克隆的文本"
                class="input-text"></el-input>
    </div>
    <div class="mt10 flex-space-between">
      <el-button v-if="output.length" link @click="openPath(output)">
        <div class="flex">
          <svg-icon icon-class="audio" size="16" class="mr5"></svg-icon>
          {{ output.substring(output.lastIndexOf(sep()) + 1) }}
        </div>
      </el-button>
      <span v-else> </span>
      <el-button @click="voice_clone" :loading="loading" :disabled="disable" type="primary">
        <template v-if="!loading">
          <svg-icon icon-class="voice-clone" size="16" class="mr5"></svg-icon>
          开始克隆
        </template>
        <template v-else>
          正在克隆
        </template>
      </el-button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.import-area {
  width: 100%;
  text-align: center;
  line-height: 60px;
  color: #ccc;
  cursor: pointer;
  background: #fafafa;
  border-radius: 10px;
  display: flex;
  justify-content: center;

  &:hover {
    color: #409eff;
  }
}

.record {
  width: 60px;
  line-height: 60px;
  display: flex;
  justify-content: center;
  margin-left: 10px;
  border-radius: 10px;
  background: #fafafa;
}

:deep(.el-input) {
  border: none;
}

:deep(.voice-text) {
  .el-input__wrapper {
    background: #fafafa !important;
    border-radius: 10px;
    box-shadow: none;
    padding: 10px;

    .el-input__inner {
      font-weight: 500;
    }

  }

}


:deep(.input-text .el-textarea__inner) {
  background: #fafafa !important;
  border-radius: 10px;
  box-shadow: none;
  padding: 10px;
  height: calc(100vh - 235px);
  font-weight: 500;

}
</style>