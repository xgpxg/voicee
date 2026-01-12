import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import 'virtual:svg-icons-register';
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'
import SvgIcon from '@/components/SvgIcon/index.vue'
import router from './router'

import '@/styles/index.scss'

const app = createApp(App)

app.use(router)
app.use(ElementPlus, {locale: zhCn,})
app.use(SvgIcon)


app.mount("#app")

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}