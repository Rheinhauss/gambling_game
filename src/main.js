import { createApp } from 'vue'
import App from './App.vue'
import { createRouter, createWebHistory } from 'vue-router'
import StartGame from './components/StartGame.vue'
import MatchPage from './components/MatchPage.vue'
import GamePage from './components/GamePage.vue'
import EndGame from './components/EndGame.vue'
import './socket/socket'

const app = createApp(App)

// 创建路由
const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/start' }, // 添加重定向到Start.vue
    { path: '/start', component: StartGame },
    { path: '/match', component: MatchPage },
    { path: '/game', component: GamePage },
    { path: '/end', component: EndGame },
  ]
})

app.use(router)
app.mount('#app')
