<script setup lang="ts">
import {getCurrentWindow} from '@tauri-apps/api/window';
import SvgIcon from "@components/SvgIcon/index.vue";
import {onMounted} from "vue";
import {isDev} from "../../utils/commands.ts";

const appWindow = getCurrentWindow();

onMounted(() => {
  //在整个应用中禁用右键菜单
  document.addEventListener('contextmenu', (event: MouseEvent) => {
    event.preventDefault();
    return false;
  });

  // 生产环境下禁用F5
  if (!isDev()) {
    document.addEventListener('keydown', (event: KeyboardEvent) => {
      if (event.key === 'F5' && !event.ctrlKey) {
        event.preventDefault();
        return false;
      }
    });
  }

})

</script>

<template>
  <div class="title-bar">
    <div class="drag-region" data-tauri-drag-region>
      <div class="fill-width flex" data-tauri-drag-region>
        <div class="avatar" data-tauri-drag-region>
          <svg-icon icon-class="audio" size="20" class="mr5"></svg-icon>
          Voicee
        </div>
        <div class="menu fill-width" data-tauri-drag-region>
        </div>
      </div>

    </div>
    <div class="controls">
      <svg-icon icon-class="minsize" id="title-bar-minimize" class="control" @click="appWindow.minimize()">
      </svg-icon>
      <svg-icon icon-class="maxsize" id="title-bar-maximize" class="control" size="14"
                @click="appWindow.toggleMaximize()">
      </svg-icon>
      <svg-icon icon-class="close" id="title-bar-close" class="control" @click="appWindow.close()">
      </svg-icon>
    </div>
  </div>
  <user-settings ref="userSettingsRef"></user-settings>
</template>

<style scoped lang="scss">
.title-bar {
  height: 40px;
  background: linear-gradient(90deg, var(--el-color-primary-light-3), var(--el-color-primary));
  backdrop-filter: blur(10px);
  user-select: none;
  display: grid;
  grid-template-columns: auto max-content;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;

  .drag-region {
    height: 40px;
    width: calc(100vw - 200px);
  }

  .controls {
    display: flex;
    color: #ffffff;
    align-items: center; /* 垂直居中 */
    .control {
      padding: 0 20px;
      height: 40px;

      &:hover {
        background: var(--el-color-primary-light-3);
        //color: var(--el-color-primary);
      }
    }
  }

  .avatar {
    display: flex;
    align-items: center; /* 垂直居中 */
    height: 40px;
    padding: 0 20px;
    width: 200px;
    color: #ffffff;
  }

  .menu {
    display: flex;
    align-items: center;

    .main-menu {
      margin-left: 200px;
    }

    .main-menu-btn {
      background: transparent;
      border: unset;
      color: #ffffff;
    }
  }
}


</style>