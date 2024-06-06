<template>
  <div class="match-page">
    <h1 class="title">匹配游戏</h1>
    <div class="content">
      <p>随机匹配将加入现有的任意一个房间</p>
      <p>创建房间后可以将房间号分享给游戏好友</p>
      <p>你想加入的好友房间号：<input type="text" v-model="addRoomID" /></p>
    </div>
    <div class="buttons">
      <button @click="randomMatch">随机匹配</button>
      <button @click="createRoom">创建房间</button>
      <button @click="addRoom" :disabled="addRoomDisabled">加入房间</button>
    </div>
    <div class="start-button">
      <button @click="goBackStart">返回开始页面</button>
    </div>
  </div>
</template>

<script>
import { ref, onMounted} from 'vue';
import { useRouter } from 'vue-router';

export default {
  name: 'MatchPage',
  setup() {
    const addRoomID = ref(''); // 加入的房间号
    const createRoomID = ref(''); // 创建的房间号
    const matchTimer = ref(null); // 匹配计时器
    const addRoomDisabled = ref(false); // 加入房间按钮可用状态
  
    const router = useRouter();
  
    // 随机匹配
    const randomMatch = () => {
      alert('匹配中……');
      matchTimer.value = setTimeout(() => {
        // 假设 matchedSuccessfully 是从后端获取的
        const matchedSuccessfully = true; // 异步调用后端API的逻辑
        if (!matchedSuccessfully) {
          alert('匹配失败！');
        } else {  
          alert('匹配成功！');
          router.push('/game');
        }
      }, 10000);
    };
  
    // 创建房间
    const createRoom = () => {
      alert('创建房间成功！房间号为：' + createRoomID.value);
      addRoomDisabled.value = true;
    };
  
    // 加入房间
    const addRoom = () => {
      if (addRoomID.value === '') {
        alert('请在输入框中输入房间ID再加入房间！');
      } else {
        alert('匹配中……');
        matchTimer.value = setTimeout(() => {
          // 假设 matchedSuccessfully 是从后端获取的
          const matchedSuccessfully = true; // 这里应该是一个异步调用后端API的逻辑
          if (!matchedSuccessfully) {
            alert('匹配失败！');
          } else {
            alert('匹配成功！'); // 注意：这里使用原生alert，不考虑matchSuccessAlert
            router.push('/game');
          }
        }, 10000);
      }
    };
  
    // 返回开始页面
    const goBackStart = () => {
      router.push('/start');
    };
  
    // 组件挂载后执行
    onMounted(() => {
      createRoomID.value = '123456';
      // 这里添加后端赋值的逻辑
    });
  
    // 返回响应式状态和方法
    return {
      addRoomID,
      createRoomID,
      addRoomDisabled,
      randomMatch,
      createRoom,
      addRoom,
      goBackStart,
    };
  }
};
</script>

/* 界面样式 */
<style scoped>
.match-page {
  text-align: center;
  margin-top: 100px;
}

.title {
  font-size: 3em;
  font-weight: bold;
  margin-bottom: 20px;
}

.content {
  font-size: 1.2em;
  margin-bottom: 40px;
}

.buttons {
  display: flex;
  justify-content: center;
  gap: 20px; /* 按钮之间的水平间距 */
}

.buttons button {
  width: 10%; /* 按钮宽度 */
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
.buttons button:disabled {
  background-color: #dddddd; /* 不可用状态的背景色 */
  color: #555; /* 不可用状态的文字颜色 */
  cursor: not-allowed; /* 鼠标指针样式 */
}

.start-button{
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px; /* 按钮之间的垂直间距 */
}

.start-button button {
  /* 除宽度外与button一致 */
  width: 20%;
  padding: 15px 20px;
  font-size: 1.5em;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.start-button button:hover {
  background-color: #ffff99;
  color: #333;
}

.buttons button, .start-button button {
  margin-bottom: 30px; /* 调整按钮之间的垂直间距 */
}

</style>