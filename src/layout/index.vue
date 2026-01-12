<template>
  <div class="layout">
    <Header></Header>
    <div class="app-main">
      <router-view></router-view>
    </div>
  </div>
</template>


<script>

import Header from "@/layout/components/Header.vue";

export default {
  computed: {},
  components: {Header},
  data() {
    return {
      defaultMenuWidth: 250,
      menuWidth: 0,
      headerHeight: '50px',
      top: 0,
      showLogin: false,
      collapse: false,
    }
  },
  provide() {
    return {
      scrollTop: this.scrollTop,
      scrollTo: this.scrollTo,
      leftMenuWidth: this.getMenuWidth,
      hideMenu: this.hideMenu,
      showMenu: this.showMenu
    }
  },
  created() {
    this.menuWidth = this.defaultMenuWidth
  },
  mounted() {

  },
  methods: {
    getKey(route) {
      return route.fullPath;
    },
    switchCollapse(isCollapse) {
      this.collapse = isCollapse
      if (isCollapse) {
        this.menuWidth = 65
      } else {
        this.menuWidth = 130
      }
    },
    /**
     * 滚动事件
     * @param scrollTop
     */
    scroll({scrollTop}) {
      this.top = scrollTop
    },
    /**
     * 设置滚动距离顶部的位置
     * @param top
     */
    scrollTo(top) {
      this.$refs['scrollbar'].setScrollTop(top)
    },
    /**
     * 获取滚动距离顶部的位置
     * @returns {number}
     */
    scrollTop() {
      return this.top
    },
    getMenuWidth() {
      return this.menuWidth
    },
    hideMenu() {
      this.menuWidth = 0
    },
    showMenu() {
      this.menuWidth = this.defaultMenuWidth
    }
  }
}
</script>

<style scoped lang="scss">
.layout {
  display: flex;
  flex-direction: column;

  .app-main {
    margin-top: 40px;
    min-height: calc(100vh - 40px);
  }
}
</style>