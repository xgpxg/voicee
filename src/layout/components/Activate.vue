<script setup lang="ts">
import {onMounted, ref} from "vue";
import {call} from "../../utils/commands.ts";
import SvgIcon from "../../components/SvgIcon/index.vue";

const isShowActivate = ref(false);
const isActivate = ref(false);
const id = ref('')
const form = ref({
  code: ''
})

const get_unique_id = async () => {
  id.value = (await call('get_unique_id', {})) as string
}

onMounted(async () => {
  await get_unique_id()
})


const show = () => {
  isShowActivate.value = true
}

const updateVerifyCode = () => {
  call('update_verify_code', {
    code: form.value.code
  }).then(() => {
    isActivate.value = true
    setTimeout(() => {
      isShowActivate.value = false
    }, 3000)
  })
}

defineExpose({
  show,
})

const reset = () => {
  isActivate.value = false
}
</script>

<template>
  <el-dialog v-model="isShowActivate" title="激活Voicee" draggable destroy-on-close @closed="reset">
    <div v-if="!isActivate">
      <div class="flex-space-between" style="height: 60px">
        <el-image src="/wechat.png" style="width:115px"></el-image>
        <div class="fill-width">
          <el-text>
            扫描左侧二维码，添加开发者微信，获取激活码。
            激活码价格为6.6元，永久有效。开发不易，感谢您的支持。
          </el-text>
        </div>
      </div>
      <div class="mt20">
        <el-text>设备ID：{{ id }}</el-text>
        <el-input v-model="form.code" placeholder="请输入激活码" class="mt10"></el-input>
      </div>
    </div>
    <div v-else class="flex-center" style="height: 60px">
      <svg-icon icon-class="success" size="20"></svg-icon>
      <el-text class="ml10">激活成功，感谢您的支持！</el-text>
    </div>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="isShowActivate = false">取消</el-button>
        <el-button type="primary" @click="updateVerifyCode">永久激活</el-button>
      </div>
    </template>
  </el-dialog>

</template>

<style scoped>

</style>