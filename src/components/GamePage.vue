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
            <div  
              class="opponent-box"
              v-for="(cardName, index) in opponentHandCards"
              :key="index"
            >
            <img :src="cardImage(cardName)" :alt="cardName" />
            </div>
          </div>
        </div>
      </div>
      <!-- Gun 区域 -->
      <div class="gun-area">
        <!-- 开枪按钮 -->
        <button class="gun-button" @click="gunButtonClick" :disabled="gunButtonDisabled">
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
            @mouseover="showTooltip(index)"
            @mouseleave="hideTooltip"
            :disabled="cardButtonDisabled"
          >  
            <!-- require动态导入图片 -->  
            <!-- <img :src="require(`@/assets/${card}.jpg`)" alt="" />   -->
            <img :src="cardImage(cardName)" :alt="cardName" />
            <div v-if="hoveredCardIndex === index" class="tooltip">
              {{ cardNote(cardName) }}
            </div>
          </button>
        </div>
        <!-- 新轮次通知窗口 -->
        <div v-if="noteShowStatus" class="notification">
          <p>第{{ roundNum }}轮游戏开始！</p>
          <p>枪中有{{ bulletNum }}颗子弹：其中{{ realBulletNum }}颗实弹，{{ dummyBulletNum }}颗哑弹</p>
        </div>
        <!-- 卡牌效果通知窗口 -->
        <div v-if="cardEffetShowStatus" class="notification">
          <p >{{ itemUseText }}</p>
          <p >{{ itemEffetText }}</p>
        </div>
      </div>
      <!-- Player 区域 -->
      <div class="player-area">
        <div class="player-image">
          <img :src="playerImageUrl" alt="Player Image">
        </div>
        <div class="player-card">
          <div class="player-card-slot">
            <button  
              class="player-box"
              v-for="(cardName, index) in playerHandCards"
              :key="index"
              :class="{ 'selected': selectedCardIndex === index }"
              @click="drawCardStatus ? '' : selectCard(index)"
              @mouseover="drawCardStatus ? '' : showTooltip(index)"
              @mouseleave="drawCardStatus ? '' : hideTooltip"
            >
            <img :src="cardImage(cardName)" :alt="cardName" />
            <div v-if="hoveredCardIndex === index && !drawCardStatus" class="tooltip">
              {{ cardNote(cardName) }}
            </div>
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
        <button class="play-btn"
          @click="playCard" 
          :disabled="playCardDisabled || selectedCardIndex === null || playerHandCards.length === 0"
        >
        出牌
        </button>
      </div>
    </div>
  </div>
</template>

<script>

import { ref, watch, onMounted} from 'vue';
import { useRouter } from 'vue-router';
import { getSocket } from '@/socket/socket';

const cardList = [
  { name: 'Knife', call: '手锯', imageUrl: require('@/assets/knife.jpg'), note: '手锯：使下一次开枪的伤害翻倍'},
  { name: 'Cigarette', call: '香烟', imageUrl: require('@/assets/cigarette.jpg'), note: '香烟：回复玩家1点血量'},
  { name: 'Beer', call: '啤酒', imageUrl: require('@/assets/beer.jpg'), note: '啤酒：弹出当前枪膛的1枚子弹'},
  { name: 'Handcuffs', call: '手铐', imageUrl: require('@/assets/handcuffs.jpg'), note: '手铐：对手下一回合无法行动' },
  { name: 'MagnifyingGlass', call: '放大镜', imageUrl: require('@/assets/magnifyingGlass.jpg'), note:'放大镜：查看当前枪膛内的子弹类型'},
  { name: 'Reverser', call: '逆转器', imageUrl: require('@/assets/reverser.jpg'), note:'逆转器：逆转当前枪膛内的子弹类型'},
  { name: 'Phone', call: '电话', imageUrl: require('@/assets/phone.jpg'), note:'电话：若当前枪膛内有x颗子弹，随机查看第2颗子弹到第x颗子弹中的一颗子弹类型'},
  { name: 'UnknownMedicine', call: '药盒', imageUrl: require('@/assets/unknownMedicine.jpg'), note:'药盒：50%概率回复玩家2点血量，50%概率扣除玩家1点血量'},
];

export default {
  name: 'GamePage',
  // {Knife, Cigarette, Beer, Handcuffs, MagnifyingGlass, Reverser, Phone, UnknownMedicine}

  setup() {
    // 使用 ref() 创建响应式引用
    const playerHealth = ref(''); // 玩家血量
    const opponentHealth = ref(''); // 对手血量
    const roundNum = ref(''); // 本轮游戏回合数
    const bulletNum = ref(''); // 枪中子弹数
    const realBulletNum = ref(''); // 枪中实弹数
    const dummyBulletNum = ref(''); // 枪中实弹数
    const isWin = ref(false); // 游戏胜负
    const opponentImageUrl = ref(require("@/assets/opponent.jpg"));
    const playerImageUrl = ref(require("@/assets/player.jpg"));
    const gunImageUrl = ref(require("@/assets/gun.jpg"));
    const gunButtonDisabled = ref(false); // 开枪按钮可用状态
    const playCardDisabled = ref(false); // 出牌按钮可用状态
    const cardButtonDisabled = ref(false); // 手牌按钮可用状态
    const showGunWindow = ref(false); // 开枪窗口显示状态
    const drawCardStatus = ref(false); // 抽牌窗口显示状态
    const drawableCards = ref([]); // 待抽取牌名称列表
    const selectedCard = ref(null); // 抽牌时被选中的卡牌名称
    const hoveredCardIndex= ref(null); // 当前鼠标悬浮的卡牌名称
    const selectedCardIndex = ref(null); // 出牌时被选中的卡牌索引
    const playerHandCards = ref([]); //当前玩家手牌名称列表
    const opponentHandCards = ref([]); //对手手牌名称列表
    const noteShowStatus = ref(false); // 轮次提示框显示状态
    const cardEffetShowStatus = ref(false); // 卡牌效果提示框显示状态
    const itemUseText = ref(''); // 使用道具的文字提示
    const itemEffetText = ref(''); // 使用道具的效果提示

    const router = useRouter();

    // 获取 WebSocket 对象
    const socket = getSocket();
  
    // 点击手枪按钮
    const gunButtonClick = () => {
      showGunWindow.value = !showGunWindow.value;
      console.log('Gun button clicked!');
      playCardDisabled.value = true;
    };
  
    // 向对手开枪
    const shootOpponent = () => {
      console.log('向对手开枪');
      showGunWindow.value = false;
      playCardDisabled.value = false;
      const message = {
          class: 'game',
          type: 'shoot',
          shoot: false
      };
      socket.send(JSON.stringify(message));
    };
  
    // 向自己开枪
    const shootSelf = () => {
      console.log('向自己开枪');
      showGunWindow.value = false;
      playCardDisabled.value = false;
      const message = {
          class: 'game',
          type: 'shoot',
          shoot: true
      };
      socket.send(JSON.stringify(message));
    };

    // 出牌的选择卡牌函数
    function selectCard(index) {
      if (selectedCardIndex.value === index) {
        selectedCardIndex.value = null; // 重置选择
      }
      else if (!drawCardStatus.value){
        selectedCardIndex.value = index;
      }
    }

    // 点击出牌按钮
    const playCard = () => {
      if (selectedCardIndex.value !== null) {
        const cardName = playerHandCards.value[selectedCardIndex.value];
        console.log(`出牌: ${cardName}, 索引: ${selectedCardIndex.value}`);
        const message = {
          class: 'game',
          type: 'UseItem',
          use: selectedCardIndex.value
        };
        socket.send(JSON.stringify(message));
        // 重置已选卡牌
        selectedCardIndex.value = null;
      }
    };

    onMounted(() => {
      // 监听来自服务器的消息
      socket.addEventListener('message', (event) => {
        // 解析收到的数据
        const data = JSON.parse(event.data);
        if (data.class === 'game') {
          // 根据消息类型执行相应操作
          switch (data.type) {// NewRound, NewTurn, UseItem, DrawItemPool, GameEnd
            case 'NewRound':
              showNewRound(data);
              break;
            case 'NewTurn':
              handleNewTurn(data);
              break;
            case 'UseItem':
              useItem(data);
              break;
            case 'DrawItemPool':
              handleDrawItem(data);
              break;
            case 'UpdateCard':
              updateCard(data);
              break;
            case 'GameEnd':
              endGame(data);
              break;
          }
        }
      });
    });

    // 新的轮次开始
    const showNewRound = (data) => {
      console.log('New Round:', data);
      roundNum.value = data.open_state.round;
      bulletNum.value = data.hidden_state.num;
      realBulletNum.value = data.hidden_state.bullets.filter(bullet => bullet === 'real').length;
      dummyBulletNum.value = data.hidden_state.bullets.filter(bullet => bullet === 'dummy').length;
      // 初始化玩家/对手的血量/手牌
      playerHealth.value = data.open_state.hp_self;
      opponentHealth.value = data.open_state.hp_oppo;
      playerHandCards.value = data.items_self.filter(item => item !== 'empty');
      opponentHandCards.value = data.items_oppo.filter(item => item !== 'empty');
      showRoundNote();
    };

    const handleNewTurn = (data) => {
      console.log('New Turn:', data);
      // 更新玩家/对手的血量/手牌
      playerHealth.value = data.open_state.hp_self;
      opponentHealth.value = data.open_state.hp_oppo;
      playerHandCards.value = data.items_self.filter(item => item !== 'empty');
      opponentHandCards.value = data.items_oppo.filter(item => item !== 'empty');
      // 判断不同玩家的回合决定按钮操作的可用性
      if(data.open_state.playing === true) {
        playerTurn(data);
      } else {
        opponentTurn(data);
      }
    };

    const playerTurn = (data) => {
      console.log('Player Turn:', data);
      playCardDisabled.value = false;
      gunButtonDisabled.value = false;
      cardButtonDisabled.value = false;
    };

    const opponentTurn = (data) => {
      console.log('Opponent Turn:', data);
      playCardDisabled.value = true;
      gunButtonDisabled.value = true;
      cardButtonDisabled.value = true;
    };

    // 使用卡牌的效果
    const useItem = (data) => {
      console.log('Use Item:', data);
      if(data.last_use.user === 'self') {
        playerUseItem(data);
      }
      else if(data.last_use.user === 'oppo'){
        opponentUseItem(data);
      }
    };

    const playerUseItem = (data) => {
      console.log('Player Use Item:', data);
      let beerBulletType = '';
      let glassBulletType = '';
      let phoneBulletType = '';
      let phoneBulletID = '';
      let medicineHP = '';
      let shootBulletType = '';
      cardEffetShowStatus.value = true;
      switch(data.last_use.item)
      {
        case 'knife':
          itemUseText.value = `你使用了手锯！`;
          itemEffetText.value = `下一次开枪的伤害将会翻倍`;
          break;
        case 'cigarette':
          itemUseText.value = `你使用了香烟！`;
          itemEffetText.value = `回复了1点血量`;
          break;
        case 'beer':
          itemUseText.value = `你使用了啤酒！`;
          if (data.last_use.result === 'dummy') {
            beerBulletType = '哑弹';
          } else if (data.last_use.result === 'real') {
            beerBulletType = '实弹';
          }
          itemEffetText.value = `弹出了当前枪膛的1枚子弹：${beerBulletType}`;
          break;
        case 'handcuffs':
          itemUseText.value = `你使用了手铐！`;
          itemEffetText.value = `对手下一回合无法行动`;
          break;
        case 'magnifier':
          itemUseText.value = `你使用了放大镜！`;
          if (data.last_use.result === 'dummy') {
            beerBulletType = '哑弹';
          } else if (data.last_use.result === 'real') {
            beerBulletType = '实弹';
          }
          itemEffetText.value = `查看当前枪膛内的子弹类型：${glassBulletType}`;
          break;
        case 'reverser':
          itemUseText.value = `你使用了逆转器！`;
          itemEffetText.value = `逆转当前枪膛内的子弹类型`;
          break;
        case 'phone':
          itemUseText.value = `你使用了电话！`;
          phoneBulletID = data.last_use.effect_num;
          if (data.last_use.result === 'dummy') {
            phoneBulletType = '哑弹';
          } else if (data.last_use.result === 'real') {
            phoneBulletType = '实弹';
          }
          itemEffetText.value = `查看第${phoneBulletID}颗子弹类型：${phoneBulletType}`;
          break;
        case 'medicine':
          itemUseText.value = `你使用了药盒！`;
          medicineHP = data.last_use.effect_num;
          if (medicineHP == 2) {
            itemEffetText.value = `回复了2点血量`;
          } else if (medicineHP == -1) {
            itemEffetText.value = `扣除了1点血量`;
          }
          break;
        case 'shoot':
          itemUseText.value = `你开枪了！`;
          if (data.last_use.result === 'dummy') {
            shootBulletType = '哑弹';
          } else if (data.last_use.result === 'real') {
            shootBulletType = '实弹';
          }
          itemEffetText.value = `枪中是：${shootBulletType}`;
          break;
      }
      // 更新玩家/对手的血量/手牌
      playerHealth.value = data.open_state.hp_self;
      opponentHealth.value = data.open_state.hp_oppo;
      playerHandCards.value = data.items_self.filter(item => item !== 'empty');
      opponentHandCards.value = data.items_oppo.filter(item => item !== 'empty');
      setTimeout(() => {
        cardEffetShowStatus.value = false;
      }, 2000);
    };

    const opponentUseItem = (data) => {
      console.log('Opponent Use Item:', data);
      let beerBulletType = '';
      let medicineHP = '';
      let shootBulletType = '';
      cardEffetShowStatus.value = true;
      switch(data.last_use.item)
      {
        case 'knife':
          itemUseText.value = `对手使用了手锯！`;
          itemEffetText.value = `下一次开枪的伤害将会翻倍`;
          break;
        case 'cigarette':
          itemUseText.value = `对手使用了香烟！`;
          itemEffetText.value = `对手回复了1点血量`;
          break;
        case 'beer':
          itemUseText.value = `对手使用了啤酒！`;
          if (data.last_use.result === 'dummy') {
            beerBulletType = '哑弹';
          } else if (data.last_use.result === 'real') {
            beerBulletType = '实弹';
          }
          itemEffetText.value = `弹出了当前枪膛的1枚子弹：${beerBulletType}`;
          break;
        case 'handcuffs':
          itemUseText.value = `对手使用了手铐！`;
          itemEffetText.value = `你下一回合无法行动`;
          break;
        case 'magnifier':
          itemUseText.value = `对手使用了放大镜！`;
          itemEffetText.value = `查看当前枪膛内的子弹类型`;
          break;
        case 'reverser':
          itemUseText.value = `对手使用了逆转器！`;
          itemEffetText.value = `逆转当前枪膛内的子弹类型`;
          break;
        case 'phone':
          itemUseText.value = `对手使用了电话！`;
          break;
        case 'medicine':
          itemUseText.value = `对手使用了药盒！`;
          if (medicineHP == 2) {
            itemEffetText.value = `回复了2点血量`;
          } else if (medicineHP == -1) {
            itemEffetText.value = `扣除了1点血量`;
          }
          break;
        case 'shoot':
          itemUseText.value = `对手开枪了！`;
          if (data.last_use.result === 'dummy') {
            shootBulletType = '哑弹';
          } else if (data.last_use.result === 'real') {
            shootBulletType = '实弹';
          }
          itemEffetText.value = `枪中是：${shootBulletType}`;
          break;
      }
      // 更新玩家/对手的血量/手牌
      playerHealth.value = data.open_state.hp_self;
      opponentHealth.value = data.open_state.hp_oppo;
      playerHandCards.value = data.items_self.filter(item => item !== 'empty');
      opponentHandCards.value = data.items_oppo.filter(item => item !== 'empty');
      setTimeout(() => {
        cardEffetShowStatus.value = false;
      }, 2000);
    };

    // 更新手牌
    const updateCard = (data) => {
      console.log('Update Card:', data);
      playerHandCards.value = data.items_self.filter(item => item !== 'empty');
      opponentHandCards.value = data.items_oppo.filter(item => item !== 'empty');
    };

    // 抽取卡牌池中的卡牌
    const handleDrawItem = (data) => {
      console.log('Draw Item:', data);
      if(data.open_state.playing === true) {
        drawableCards.value = data.item_pool;
        drawCards();
      }
      // 待添加更新手牌
    };

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
        const message = {
          class: 'game',
          type: 'draw',
          draw: selectedCard.value
        };
        socket.send(JSON.stringify(message));
        console.log('Draw card:', card);
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

    const endGame = (data) => {
      console.log('Game End:', data);
      isWin.value = data.hidden_state.win;
      roundNum.value = data.open_state.round;
      router.push({
        path: '/end',
        query: {
          isWin: isWin.value,
          roundNum: roundNum.value
        }
      });
    };

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
  
    // 显示悬浮卡牌说明
    function showTooltip(index) {
      hoveredCardIndex.value = index;
    }

    // 隐藏悬浮卡牌说明
    function hideTooltip() {
      hoveredCardIndex.value = null;
    }

    // 控制轮次提示框显示2s后隐藏
    const showRoundNote = () => {
      noteShowStatus.value = true;
      setTimeout(() => {
        noteShowStatus.value = false;
      }, 2000);
    };

    return {
      playerHealth,
      opponentHealth,
      roundNum,
      bulletNum,
      realBulletNum,
      dummyBulletNum,
      isWin,
      opponentImageUrl,
      playerImageUrl,
      gunImageUrl,
      drawCardStatus,
      drawableCards,
      selectedCard,
      hoveredCardIndex,
      showGunWindow,
      gunButtonDisabled,
      playCardDisabled,
      cardButtonDisabled,
      playerHandCards,
      opponentHandCards,
      selectedCardIndex,
      noteShowStatus,
      cardEffetShowStatus,
      itemUseText,
      itemEffetText,
      gunButtonClick,
      shootOpponent,
      shootSelf,
      cardImage,
      cardNote,
      drawCards,
      selectDrawCard,
      selectCard,
      playCard,
      togglePlayCardDisabled,
      showTooltip,
      hideTooltip,
      showRoundNote,
      showNewRound,
      handleNewTurn,
      useItem,
      handleDrawItem,
      updateCard,
      endGame,
    };
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
  max-width: 50%;
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
  
.card-drawer button.selected,
.player-box.selected {
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
  justify-content: flex-start;
  border: 3px solid black;
  height: 200px;
  padding: 10px;
  gap: 10px; /* 减小间隙以适应图片大小 */
}  
  
.opponent-box,
.player-box {
  width: 23%;
  box-sizing: border-box; /* 边框和内边距包含在宽度内 */
  border: 1px solid black;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
  /* 添加一个最小宽度或固定宽度，以确保空框也有相同的宽度 */
  min-width: 0; /* 防止容器收缩到小于内容的最小尺寸 */
}

.opponent-box button,
.player-box button {
  flex: 1;
  width: 100%;
  height: 100%;
  border: none;
  background: none;
  padding: 0;
  cursor: pointer;
}  

.opponent-box img,
.player-box img {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
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
.notification {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  padding: 10px 20px;
  background-color: #f0f0f0;
  border: 1px solid #ccc;
  border-radius: 5px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
  font-size: 16px;
  font-weight: bold;
  color: #333;
}

</style>
