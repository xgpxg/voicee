import {createRouter, createWebHistory} from 'vue-router'
import Layout from '@/layout/index.vue'

const routes = [
    {
        path: '/',
        component: Layout,
        redirect: 'welcome',
        children: [
            {
                path: '/',
                component: () => import('@/views/splashscreen/index.vue'),
                name: 'Welcome',
                meta: {title: '欢迎', icon: 'home', affix: true,}
            },
            {
                path: '/home',
                component: () => import('@/views/home/index.vue'),
                name: 'Home',
                meta: {title: '首页', icon: 'home', affix: true,}
            },
        ]
    }
]


const router = createRouter({
    //history: createWebHashHistory(),  // hash 模式
    history: createWebHistory(),  // history 模式
    routes: routes
})

// 全局路由守卫
// @ts-ignore
router.beforeEach((to, from, next) => {
    // console.log(to, from)
    if (to.meta.title) {
        document.title = `${to.meta.title}`;
    }
    next()
})

export default router
