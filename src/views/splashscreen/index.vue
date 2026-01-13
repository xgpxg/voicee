<script setup lang="ts">

import {call} from "@/utils/commands";
import {onMounted, ref} from "vue";
import {useRouter} from "vue-router";
import SvgIcon from "../../components/SvgIcon/index.vue";

const router = useRouter()
/**
 * 初始化状态
 * -1：检查中
 * 0：初始化失败
 * 1：初始化完成
 * 2：模型下载中
 * 3：模型加载中
 */
const init_status = ref(-1)
const check_initialized = () => {
  call<boolean>('is_initialized', {}).then((res: boolean) => {
    init_status.value = res ? 1 : 0

    if (res) {
      router.replace('/home')
    } else {
      init_status.value = 2
      call<void>('download_model', {}).then(() => {
        init_status.value = 3
        call<void>('load_model', {}).then(() => {
          init_status.value = 1
          setTimeout(() => {
            router.replace('/home')
          }, 1000)
        })
      })
    }
  }).catch((error: any) => {
    console.error("初始化检查失败:", error);
    init_status.value = 0;
  })
}

onMounted(() => {
  check_initialized()
})
</script>

<template>
  <div class="splash-container">
    <div class="logo-wrapper">
      <div class="logo-icon">
        <svg-icon icon-class="audio" :color="'var(--el-color-primary)'"></svg-icon>
      </div>
      <h1 class="app-title">克隆你的声音</h1>
    </div>

    <div class="status-content">
      <!-- 检查中状态 -->
      <div v-if="init_status === -1" class="status-item">
        <div class="loading-spinner"></div>
        <p>系统检查中...</p>
      </div>

      <!-- 下载模型状态 -->
      <div v-if="init_status === 2" class="status-item">
        <div class="loading-spinner"></div>
        <p>正在下载模型...</p>

      </div>

      <!-- 加载模型状态 -->
      <div v-if="init_status === 3" class="status-item">
        <div class="loading-spinner"></div>
        <p>正在加载模型...</p>
      </div>

      <!-- 初始化完成状态 -->
      <div v-if="init_status === 1" class="status-item success">
        <div class="success-checkmark">
          <div class="check-icon">
            <span class="icon-line line-tip"></span>
            <span class="icon-line line-long"></span>
            <div class="icon-circle"></div>
            <div class="icon-fix"></div>
          </div>
        </div>
        <p>初始化完成</p>
      </div>

      <!-- 初始化失败状态 -->
      <div v-if="init_status === 0" class="status-item error">
        <div class="error-icon">⚠️</div>
        <p>初始化失败，请重新启动程序</p>
      </div>
    </div>
    <!--
        <div class="footer-info">
          <p>无限音色克隆</p>
        </div>-->
  </div>

  <div class="app-container">
    <div class="version-info">v0.1.0</div>
  </div>
</template>

<style scoped>
.splash-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  font-family: 'PingFang SC', 'Microsoft YaHei', sans-serif;
  position: relative;
  overflow: hidden;
  padding-top: 50px;
}

.logo-wrapper {
  text-align: center;
  margin-bottom: 60px;
  z-index: 2;
}

.logo-icon {
  font-size: 48px;
  margin-bottom: 10px;
}

.app-title {
  font-size: 24px;
  color: var(--el-color-primary);
  margin: 0;
  font-weight: 500;
}

.status-content {
  text-align: center;
  z-index: 2;
  width: 100%;
  max-width: 300px;
}

.status-item {
  padding: 24px;
  border-radius: 8px;
  background: #ffffff;
  //box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
}

.status-item p {
  margin: 12px 0 0 0;
  color: #666;
  font-size: 14px;
}

/* 简约加载动画 */
.loading-spinner {
  width: 32px;
  height: 32px;
  border: 2px solid rgba(76, 175, 80, 0.2);
  border-top: 2px solid var(--el-color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto;
}

/* 成功对勾动画（简化） */
.success-checkmark {
  width: 40px;
  height: 40px;
  margin: 0 auto 16px;
  position: relative;
  zoom: 0.845;
}

.check-icon {
  width: 100%;
  height: 100%;
  position: relative;
  border-radius: 50%;
  box-sizing: content-box;
  border: 3px solid #4CAF50;
}

.icon-line {
  height: 2px;
  background-color: #4CAF50;
  display: block;
  border-radius: 1px;
  position: absolute;
  z-index: 10;
}

.line-tip {
  width: 12px;
  left: 6px;
  top: 18px;
  transform: rotate(45deg);
}

.line-long {
  width: 22px;
  right: 5px;
  top: 16px;
  transform: rotate(-45deg);
}

.icon-circle {
  width: 34px;
  height: 34px;
  border-radius: 50%;
  position: absolute;
  box-sizing: content-box;
  border: 3px solid rgba(76, 175, 80, 0.2);
}

.icon-fix {
  width: 2px;
  height: 12px;
  background-color: white;
  position: absolute;
  top: 13px;
  left: 18px;
  z-index: 1;
  transform: rotate(-45deg);
}

.error-icon {
  font-size: 36px;
  margin-bottom: 10px;
  display: block;
}

.footer-info {
  position: absolute;
  bottom: 20px;
  color: #ccc;
  font-size: 12px;
  z-index: 2;
}

/* 动画定义 */
@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

@keyframes linearProgress {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}


.version-info {
  position: fixed;
  bottom: 10px;
  left: 10px;
  color: #d6d6d6;
  font-size: 12px;
  z-index: 9999;
  pointer-events: none;
}
</style>
