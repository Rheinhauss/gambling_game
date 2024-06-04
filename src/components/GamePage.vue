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
        <!-- 抽牌窗口 -->  
        <div v-if="drawCardStatus" class="card-drawer">  
          <p>请在下方道具列表中抽取一个道具：</p>
          <button  
            v-for="(cardName, index) in drawableCards"  
            :key="index"  
            :class="{ selected: selectedCard === cardName }"  
            @click="selectDrawCard(cardName)"
            @mouseover="showTooltip(cardName)"  
            @mouseleave="hideTooltip"
          >  
            <!-- require动态导入图片 -->  
            <!-- <img :src="require(`@/assets/${card}.jpg`)" alt="" />   -->
            <img :src="cardImage(cardName)" :alt="cardName" />
            <div v-if="hoveredCard === cardName" class="tooltip">  
              {{ cardNote(cardName) }}
            </div>
          </button>
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
            <button  
              class="player-box"  
              v-for="(cardName, index) in playerCards"  
              :key="index"  
              :class="{ 'selected': selectedCard === index }"  
              @click="selectCard(index)"  
            >
            <!-- 待修改   -->
          </button>
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
        <button @click="tempEnd">临时退出游戏</button>
        <button @click="drawCards">临时抽牌</button>
      </div>
    </div>
  </div>
</template>

<script>

import { ref, watch } from 'vue';

const cardList = [  
  { name: 'Knife', imageUrl: require('@/assets/Knife.jpg'), note: '手锯：使下一次开枪的伤害翻倍'},
  { name: 'Cigarette', imageUrl: require('@/assets/Cigarette.jpg'), note: '香烟：回复玩家1点血量'},
  { name: 'Beer', imageUrl: require('@/assets/Beer.jpg'), note: '啤酒：弹出当前枪膛的1枚子弹'},
  { name: 'Handcuffs', imageUrl: require('@/assets/Handcuffs.jpg'), note: '手铐：对手下一回合无法行动' },
  { name: 'MagnifyingGlass', imageUrl: require('@/assets/MagnifyingGlass.jpg'), note:'放大镜：查看当前枪膛内的子弹类型'},
  { name: 'Reverser', imageUrl: require('@/assets/Reverser.jpg'), note:'逆转器：逆转当前枪膛内的子弹类型'},
  { name: 'Phone', imageUrl: require('@/assets/Phone.jpg'), note:'电话：若当前枪膛内有x颗子弹，随机查看第2颗子弹到第x颗子弹中的一颗子弹类型'},
  { name: 'UnknownMedicine', imageUrl: require('@/assets/UnknownMedicine.jpg'), note:'药盒：50%概率回复玩家2点血量，50%概率扣除玩家1点血量'},
];  

export default {
  name: 'GamePage',
  // {Knife, Cigarette, Beer, Handcuffs, MagnifyingGlass, Reverser, Phone, UnknownMedicine}

  setup() {
    // 使用 ref() 创建响应式引用  
    const playerHealth = ref(1);  
    const opponentHealth = ref(1);  
    const opponentImageUrl = ref(require("@/assets/opponent.jpg"));  
    const playerImageUrl = ref(require("@/assets/player.jpg"));  
    const gunImageUrl = ref(require("@/assets/gun.jpg"));  
    const showGunWindow = ref(false); // 开枪窗口显示状态  
    const playCardDisabled = ref(false); // 出牌按钮可用状态  
    const drawCardStatus = ref(false);  
    const drawableCards = ref(['Knife', 'Cigarette', 'Beer']); // 后端传来的待抽取牌列表  
    const selectedCard = ref(null); // 存储被选中的卡牌  
    const hoveredCard= ref(null); // 存储当前鼠标悬浮的卡牌名称
  
    // 抽牌函数  
    const drawCards = () => {  
      drawCardStatus.value = true; 
      togglePlayCardDisabled();
    };  
  
    // 抽牌中选择卡牌函数  
    const selectDrawCard = (card) => {  
      if (selectedCard.value === card) {  
        // 如果已经选择了该卡牌，则视为抽取
        drawCardStatus.value = false;
        togglePlayCardDisabled();
        // 待添加抽取卡牌逻辑
        console.log('抽取了卡牌:', card);  
        selectedCard.value = null; // 重置选择
      }
      else {  
        // 选择新的卡牌
        selectedCard.value = card;  
      }  
    };  
  
    // 监听drawCardStatus，当它为false时重置selectedCard  
    watch(drawCardStatus, (newVal) => {  
      if (!newVal) {  
        selectedCard.value = null;  
      }  
    });  
  
    // 根据卡牌名称获取图片URL  
    function cardImage(cardName) {  
      const card = cardList.find(card => card.name === cardName);  
      return card ? card.imageUrl : ''; // 如果找不到则返回空字符串  
    }  

    // 根据卡牌名称获取卡牌说明  
    function cardNote(cardName) {  
      const card = cardList.find(card => card.name === cardName);  
      return card ? card.note : '';  
    }  

    // 修改出牌按钮的状态  
    function togglePlayCardDisabled() {  
      playCardDisabled.value = !playCardDisabled.value;  
    }  
  
    function showTooltip(cardName) {  
      hoveredCard.value = cardName;    
    }  
    function hideTooltip() {  
      hoveredCard.value = null;  
    }

    // 返回响应式数据
    return {  
      drawCardStatus,  
      drawableCards,  
      selectedCard,  
      hoveredCard,
      drawCards,  
      selectDrawCard,  
      playerHealth,  
      opponentHealth,  
      opponentImageUrl,  
      playerImageUrl,  
      gunImageUrl,  
      showGunWindow,  
      playCardDisabled,  
      cardImage,
      cardNote,
      togglePlayCardDisabled,
      showTooltip,
      hideTooltip
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
.card-drawer {  
  position: absolute;  
  top: 50%;  
  left: 50%;  
  transform: translate(-50%, -50%);
  background-color:white;
  border: 1px solid black;
  font-size: 16px;
  font-weight: bold;  
  color: #333; 
}  

.card-drawer button {  
  flex: 1;
  max-width: 80%;
  max-height: 80%;
  border: none; 
  background: none; 
  padding: 20px 20px;
  cursor: pointer; 
}  
  
.card-drawer button.selected {  
  /* 被选中的按钮样式：边框加粗并变红 */  
  border: 3px solid red;
}  

.tooltip {  
  position: absolute;   
  z-index: 200;  
  background: #424242;  
  color: #fff;  
  padding: 5px 10px;  
  border-radius: 5px;  
  font-size: 16px;
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

.play-btn{
  width: 40%;
  height: 60%;
  padding: 10px;
  margin: 5px 0; /* 按钮之间的间距 */
  border: none;
  background-color: #007bff;
  color: white;
  font-size: 25px;
  cursor: pointer;
  font-weight: bold;
}

.play-btn:disabled{
  background-color: #dddddd; /* 不可用状态的背景色 */
  color: #555; /* 不可用状态的文字颜色 */
  cursor: not-allowed; /* 鼠标指针样式 */
}

.play-btn:hover {
  background-color: #ffff99;
  color: #333;
}

</style>
