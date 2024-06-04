import { createApp } from 'vue'
import App from './App.vue'
import { createRouter, createWebHistory } from 'vue-router'
import StartGame from './components/StartGame.vue'
import MatchPage from './components/MatchPage.vue'
import GamePage from './components/GamePage.vue'
import EndGame from './components/EndGame.vue'
// 引入 socket.io
import VueSocketIO from 'vue-socket.io'
import { registerSockets, destroySockets } from "./sockets/sockets.js";

// 创建 Socket 连接
const socket = new VueSocketIO({
  debug: false, 
  connection: "http://localhost:8080",
});

const app = createApp(App)

// 在任意位置获取到 socket 对象
app.config.globalProperties.$socket = socket;
// 监听事件
app.config.globalProperties.$addSockets = registerSockets;
// 移除事件
app.config.globalProperties.$removeSockets = destroySockets;

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
