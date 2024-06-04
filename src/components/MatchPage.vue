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
export default {
  name: 'MatchPage',
  data() {
      return {
        addRoomID: '', // 加入房间的房间号
        createRoomID: '123456', // 创建的房间号
        matchTimer: null, // 匹配计时器
        matchSuccessAlert: null, // 匹配成功提示对话框
        matchFailedAlert: null, // 匹配失败提示对话框
        addRoomDisabled: false // 加入房间按钮可用状态
      };
    },
  methods: {
    // 随机匹配
    randomMatch() {
      alert('匹配中……');
      // 延迟10s判定匹配结果，需要根据后端再修改
      this.matchTimer = setTimeout(() => {
        if (!matchedSuccessfully) { // 假设这里是后端返回的匹配结果
          alert('匹配失败！');
        } else {
          this.matchSuccessAlert = alert('匹配成功！');
          // 匹配成功后进行页面跳转到/game
          this.$router.push('/game');
        }
      }, 10000); // 10秒的毫秒数
        // 添加后端匹配的逻辑，并假设匹配成功的标志为 matchedSuccessfully
        const matchedSuccessfully = true;
    },

    // 创建房间
    createRoom(){
      alert('创建房间成功！房间号为：' + this.createRoomID);
      this.addRoomDisabled = true;
      // 创建房间默认为已加入该房间，无法再加入房间，加入房间按钮将不可用
    },

    // 加入房间
    addRoom() {
      if (this.addRoomID === '') {
        alert('请在输入框中输入房间ID再加入房间！');
      } else {
        alert('匹配中……');
        this.matchTimer = setTimeout(() => {
          if (!matchedSuccessfully) {
            alert('匹配失败！');
          } else {
            this.matchSuccessAlert = alert('匹配成功！');
            this.$router.push('/game');
          }
        }, 10000);
        // 添加后端匹配的逻辑，并假设匹配成功的标志为 matchedSuccessfully
        const matchedSuccessfully = true;
      }
    },
    goBackStart() {
      this.$router.push('/start');
    }
  },

  // 在离开当前页面时清除计时器和所有的对话框
  beforeRouteLeave(to, from, next) {
    clearTimeout(this.matchTimer);
    if (this.matchSuccessAlert) {
      this.matchSuccessAlert.close();
    }
    if (this.matchFailedAlert) {
      this.matchFailedAlert.close();
    }
    next();
  },

  mounted() {
    // 添加后端赋值的逻辑
    this.createRoomID = '123456';
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