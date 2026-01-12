import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import 'virtual:svg-icons-register';
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'
import SvgIcon from '@/components/SvgIcon/index.vue'
// @ts-ignore
import router from './router'

import '@/styles/index.scss'

const app = createApp(App)

app.use(router)
app.use(ElementPlus, {locale: zhCn,})
app.component('SvgIcon', SvgIcon)


app.mount("#app")

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}