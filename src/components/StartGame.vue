<template>
  <div class="background-image">
    <div class="container">
      <div class="title">恶魔轮盘</div>
      <div class="buttons">
        <button @click="goToOnline">在线游戏</button>
        <button @click="goToOffline">离线游戏</button>
        <button @click="closePage">退出</button>
      </div>
    </div>
  </div>
</template>

<script>
import { getCurrentInstance, onMounted, onBeforeUnmount} from "vue";
import { useRouter } from 'vue-router';

export default {
  name: 'StartGame',

  setup() {
    const router = useRouter();
  
    // 在线游戏
    const goToOnline = () => {
      router.push('/match');
    };
  
    // 离线游戏
    const goToOffline = () => {
      router.push('/game');
    };
  
    // 关闭页面
    const closePage = () => {
      window.open('about:blank','_self').close();
    };

    // 获取当前实例对象
    const { proxy } = getCurrentInstance();
    // 定义监听node事件
    const sockets = {
      welcome(data) {
        console.log(data);
      },
    };
    proxy.$socket.io.emit("send", "client send some data to node Serve.");
    // 注册node事件
    onMounted(() => {
      proxy.$addSockets(sockets, proxy);
    });
    // 注销node事件
    onBeforeUnmount(() => {
      proxy.$removeSockets(sockets, proxy);
    });

    return {
      goToOnline,
      goToOffline,
      closePage,
    };
  }
}
</script>

<style scoped>

/* 组件区域 */
.container {
  width: 80%;
  margin: 20% auto 0; /* 调整上边距以使内容垂直居中 */
  text-align: center;
}

/* 背景图片 */
.background-image {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: url('@/assets/background.jpg');
  background-size: cover;
  background-repeat: no-repeat;
  z-index: -1; /* 将背景图片放置在其他内容的下方 */
}

/* 标题 */
.title {
  font-size: 3em;
  margin-bottom: 75px;
  color: white;
  font-weight: bold;
}

/* 按钮 */
.buttons {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px; /* 按钮之间的垂直间距 */
}

.buttons button {
  width: 40%; /* 按钮宽度 */
  padding: 15px 20px; /* 按钮内边距 */
  font-size: 1.5em; /* 按钮字体大小 */
  background-color: #007bff; /* 按钮默认背景色 */
  color: white; /* 按钮文字颜色 */
  border: none;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.buttons button:hover {
  background-color: #ffff99; /* 悬停时按钮背景色 */
  color: #333; /* 悬停时文字颜色 */
}
</style>
