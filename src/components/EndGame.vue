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
import { ref, onMounted} from 'vue';
import { useRouter } from 'vue-router';
  
export default {
  name: 'EndGame',
  setup(props) {
    const isWin = ref(props.isWin);
    const roundNum = ref(props.roundNum);
    const bulletNum = ref('');
    const propNum = ref('');
  
    const router = useRouter();
   
    // 返回开始页面
    const goToStartPage = () => {
      router.push('/start');
    };  

    // 关闭页面
    const closePage = () => {
      window.open('about:blank','_self').close();
    };
  
    onMounted(() => {
      console.log('isWin:', isWin.value);
      console.log('roundNum:', roundNum.value);
    });

    return {
      isWin,
      roundNum,
      bulletNum,
      propNum,
      goToStartPage,
      closePage
    };
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
