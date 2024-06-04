<template>
  <div class="end-game">
    <div class="title">游戏结束</div>
    <div class="message">
      <p v-if="isWin">你赢了！</p>
      <p v-else>你输了！</p>
      <p>本轮游戏回合数为：{{ roundNum }}</p>
      <p>本轮游戏你打出的子弹数：{{ bulletNum }}</p>
      <p>本轮游戏你使用的道具数：{{ propNum }}</p>
    </div>
    <div class="buttons">
      <button @click="goToStartPage">返回开始页面</button>
      <button @click="closePage">退出</button>
    </div>
  </div>
</template>

<script>
  export default {
    name: 'EndGame',
    data() {
      return {
        isWin : true,
        roundNum: 1,
        bulletNum: 1,
        propNum: 1
      }
    },
    methods: {
      // 跳转到开始界面
      goToStartPage() {
        this.$router.push('/start');
      },
      // 关闭窗口
      closePage() {
        window.open('about:blank','_self').close();
      }
    }
  }
</script>

/* 界面样式 */
<style scoped>
.end-game {
  text-align: center;
  margin-top: 100px;
}

.title {
  font-size: 3em;
  font-weight: bold;
  margin-bottom: 20px;
}

.message {
  margin-bottom: 40px;
}

.message p {
  font-size: 1.5em;
}

/* 按钮 */
.buttons {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}

.buttons button {
  width: 30%;
  padding: 15px 20px;
  font-size: 1.5em;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.buttons button:hover {
  background-color: #ffff99;
  color: #333;
}
</style>
