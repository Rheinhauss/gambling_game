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
        <button class="gun-button" @click="gunButtonClick">
          <img :src="gunImageUrl" alt="Gun Button">
        </button>
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
        <button class="play-btn">出牌</button>
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
      gunImageUrl: require("@/assets/gun.jpg")
    };
  },
  methods: {
    tempEnd() {
      this.$router.push('/end');
    },
    gunButtonClick() {
      // 在这里添加按钮点击后的逻辑
      console.log('Gun button clicked!');
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
  display: flex;
  justify-content: center;
  align-items: center;
}

.gun-button {
  max-width: 100%;
  max-height: 100%;
  border: none; 
  background: none; 
  padding: 0;
  cursor: pointer; 
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
}

.play-btn:hover,
.exit-btn:hover {
  background-color: #ffff99;
  color: #333;
}

</style>
