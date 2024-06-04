<template>
  <div class="game-page">
    <!-- 左侧窗口 -->
    <div class="left-window">
      <!-- Opponent 区域 -->
      <div class="opponent-area">
        <div class="opponent-image">
          <img :src="opponentImageUrl" alt="Opponent Image">
        </div>
        <div class="opponent-card">
          <div class="opponent-card-slot">
            <div class="opponent-box" v-for="box in 4" :key="box"></div>
          </div>
        </div>
      </div>
      <!-- Gun 区域 -->
      <div class="gun-area">
        <!-- 开枪按钮 -->
        <button class="gun-button" @click="gunButtonClick">
          <img :src="gunImageUrl" alt="Gun Button">
        </button>
        <!-- 开枪窗口 -->
        <div v-if="showGunWindow" class="gun-window">
          <p>请选择开枪的对象：</p>
          <button @click="shootOpponent">对手</button>
          <button @click="shootSelf">自己</button>
        </div>
      </div>
      <!-- Player 区域 -->
      <div class="player-area">
        <div class="player-image">
          <img :src="playerImageUrl" alt="Player Image">
        </div>
        <div class="player-card">
          <div class="player-card-slot">
            <div class="player-box" v-for="box in 4" :key="box"></div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 右侧窗口 -->
    <div class="right-window">
      <!-- Status 区域 -->
      <div class="status-area">
        <div class="status-box">
          <p>血量</p>
          <p>玩家：{{ playerHealth }}</p>
          <p>对手：{{ opponentHealth }}</p>
        </div>
      </div>
      <!-- Buttons 区域 -->
      <div class="buttons-area">
        <button class="play-btn" @click="playCard" :disabled="playCardDisabled">出牌</button>
        <button class="exit-btn" @click="tempEnd">临时退出游戏</button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'GamePage',
  data() {
    return {
      playerHealth: 1,
      opponentHealth: 1,
      opponentImageUrl: require("@/assets/opponent.jpg"),
      playerImageUrl: require("@/assets/player.jpg"),
      gunImageUrl: require("@/assets/gun.jpg"),
      showGunWindow: false, // 开枪窗口显示状态
      playCardDisabled: false // 出牌按钮可用状态
    };
  },
  methods: {
    tempEnd() {
      this.$router.push('/end');
    },
    // 手枪按钮点击事件
    gunButtonClick() {
      // 待添加按钮点击后的逻辑
      this.showGunWindow = !this.showGunWindow;
      console.log('Gun button clicked!');
      this.playCardDisabled = true;
    },
    // 向对手开枪
    shootOpponent() {
      // 待添加向对手开枪的逻辑
      console.log('向对手开枪');
      this.showGunWindow = false;
      this.playCardDisabled = false;
    },
    // 向自己开枪
    shootSelf() {
      // 待添加向自己开枪的逻辑
      console.log('向自己开枪');
      this.showGunWindow = false;
      this.playCardDisabled = false;
    }
  }
}
</script>

<style scoped>
.game-page {
  display: flex;
  height: 100dvh;
}

.left-window {
  flex: 8;
  display: flex;
  flex-direction: column;
}

.right-window {
  flex: 2;
  display: flex;
  flex-direction: column;
}

.gun-area {
  flex: 1;
  width: 100%;
  height: auto;
  display: flex;
  justify-content: center;
  align-items: center;
}

.gun-button {
  flex: 1;
  max-width: 100%;
  max-height: 100%;
  border: none; 
  background: none; 
  padding: 0;
  cursor: pointer; 
}

.gun-window {
  display: flex;
  flex-direction: column; 
  justify-content: center;
  align-items: center;
  position: absolute; /* 浮动窗口 */ 
  top: 50%; 
  left: 50%; 
  transform: translate(-50%, -50%); /* 居中对齐 */  
  z-index: 100; /* 窗口在其他元素之上 */
  padding: 20px;
  background-color: white;
  border: 1px solid black;
  font-size: 16px;
  font-weight: bold;  
  color: #333; 
}
.gun-window button {
  margin-bottom: 5px;
  padding: 10px 20px; 
  font-size: 16px;
  font-weight: bold;
  background-color: #007bff;
  border: none;
  color: white;
  border-radius: 4px; /* 边框圆角 */
}  

.gun-window button:hover {
  background-color: #ffff99;
  color: #333;
}

.opponent-area,
.player-area {
  flex: 1;
  display: flex;
  align-items: center;
}

.opponent-image,
.player-image{
  flex: 1;
}

.opponent-image img,
.player-image img {
  width: 100%;
  height: auto;
}

.opponent-card,
.player-card {
  flex: 5;
  display: flex;
  justify-content: center;
  padding: 20px;
}

.opponent-card-slot,
.player-card-slot {
  width: 80%;
  display: flex;
  justify-content: space-between;
  border: 3px solid black; 
  height: 200px; 
  padding: 10px;
}

.opponent-box,
.player-box {
  width: 20%;
  border: 1px solid black;
}

.status-area {
  flex: 3;
  display: flex;
  justify-content: center;
  align-items: center;
}

.status-box {
  font-size: 1.2em;
  border: 2px solid black;
  padding: 10px;
  text-align: center;
  width: 50%;
  height: 50%;
}

.buttons-area {
  flex: 1;
  display: flex;
  align-items: center;
  flex-direction: column;
}

.play-btn,
.exit-btn {
  width: 40%;
  padding: 10px;
  margin: 5px 0; /* 按钮之间的间距 */
  border: none;
  background-color: #007bff;
  color: white;
  font-size: 16px;
  cursor: pointer;
  font-weight: bold;
}

.play-btn:disabled,
.exit-btn:disabled {
  background-color: #dddddd; /* 不可用状态的背景色 */
  color: #555; /* 不可用状态的文字颜色 */
  cursor: not-allowed; /* 鼠标指针样式 */
}

.play-btn:hover,
.exit-btn:hover {
  background-color: #ffff99;
  color: #333;
}

</style>
