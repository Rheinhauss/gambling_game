<template>
  <div class="match-page">
    <h1 class="title">匹配游戏</h1>
    <div class="content">
      <!-- <p>随机匹配将加入现有的任意一个房间</p> -->
      <p>玩家可以选择自己创建房间，或加入好友创建好的房间</p>
      <p>创建房间后可以将房间号分享给游戏好友</p>
      <p>你想加入的好友房间号：<input type="text" v-model="addRoomID" /></p>
    </div>
    <div class="buttons">
      <!-- <button @click="randomMatch">随机匹配</button> -->
      <button @click="createRoom">创建房间</button>
      <button @click="addRoom" :disabled="addRoomDisabled">加入房间</button>
      <button @click="leaveRoom" :disabled="leaveRoomDisabled">离开房间</button>
    </div>
    <div class="start-button">
      <button @click="goBackStart">返回开始页面</button>
    </div>
  </div>
</template>

<script>
import { ref, onMounted} from 'vue';
import { useRouter } from 'vue-router';
import { getSocket } from '@/socket/socket';

export default {
  name: 'MatchPage',
  setup() {
    const addRoomID = ref(''); // 加入的房间号
    const createRoomID = ref(''); // 创建的房间号
    const createRoomDisabled = ref(false); // 创建房间按钮可用状态
    const addRoomDisabled = ref(false); // 加入房间按钮可用状态
    const leaveRoomDisabled = ref(true); // 离开房间按钮可用状态
    const opponentJoin = ref(false); // 是否有对手加入房间

    const router = useRouter();
  
    // 获取 WebSocket 对象
    const socket = getSocket();

    // 创建房间
    const createRoom = () => {
      const message = {
        class: 'lobby',
        type: 'CreateRoom',
      };
      socket.send(JSON.stringify(message));
    };

    // 加入房间
    const addRoom = () => {
      if (addRoomID.value === '') {
        alert('请在输入框中输入房间ID再加入房间！');
      } else {
        const message = {
          class: 'lobby',
          type: 'JoinRoom',
          roomid: addRoomID.value
        };
        socket.send(JSON.stringify(message));
        alert('正在加入房间……');
      }
    };

    // 离开房间
    const leaveRoom = () => {
      const message = {
        class: 'lobby',
        type: 'LeaveRoom'
      };
      socket.send(JSON.stringify(message));
    };
  
    // 返回开始页面
    const goBackStart = () => {
      router.push('/start');
    };

    onMounted(() => {
      // 与后端建立连接握手
      const handShakeMessage = {
        class: 'lobby',
        type: 'Handshake'
      };
      socket.send(JSON.stringify(handShakeMessage));

      // 监听来自服务器的消息
      socket.addEventListener('message', (event) => {
        // 解析收到的数据
        const data = JSON.parse(event.data);
        if (data.class === 'lobby') {
          // 根据消息类型执行相应操作
          switch (data.type) {
            case 'CreateRoomSuccess':
              handleCreateRoomSuccess(data.roomid);
              break;
            case 'OpponentJoin':
              handleOpponentJoin();
              break;
            case 'OpponentLeave':
              handleOpponentLeave();
              break;
            case 'CreateRoomFail':
              handleCreateRoomFail();
              break;
            case 'HandshakeSuccess':
              handleHandshakeSuccess();
              break;
            case 'JoinRoomSuccess':
              handleJoinRoomSuccess();
              break;
            case 'JoinFail':
              handleJoinFail();
              break;
            default:
              console.log('Unknown message type:', data.type);
          }
        }
      });
    });

    // 房间创建成功
    const handleCreateRoomSuccess = (roomId) => {
      console.log('Room created successfully. Room ID:', roomId);
      createRoomID.value = roomId;
      addRoomDisabled.value = true;
      leaveRoomDisabled.value = false;
      createRoomDisabled.value = true;
      alert('房间创建成功！房间号为：' + createRoomID.value);
    };

    // 对手加入房间
    const handleOpponentJoin = () => {
      console.log('Opponent joined the room.');
      opponentJoin.value = true;
      alert('对手加入房间，即将开始游戏！');
      router.push('/game');
    };

    // 对手离开房间
    const handleOpponentLeave = () => {
      console.log('Opponent left the room.');
      opponentJoin.value = false;
      alert('对手离开房间！');
    };

    // 房间创建失败
    const handleCreateRoomFail = () => {
      console.log('Failed to create room.');
      alert('创建房间失败，请重试！');
    };

    // 握手成功
    const handleHandshakeSuccess = () => {
      console.log('Handshake successful.');
    };

    // 加入房间成功
    const handleJoinRoomSuccess = () => {
      console.log('Joined room successfully.');
      alert('加入房间成功，即将开始游戏！');
      router.push('/game');
    };

    // 加入房间失败
    const handleJoinFail = () => {
      console.log('Failed to join room.');
      alert('加入房间失败，请重试！');
    };

    return {
      addRoomID,
      createRoomID,
      createRoomDisabled,
      addRoomDisabled,
      leaveRoomDisabled,
      opponentJoin,
      createRoom,
      addRoom,
      leaveRoom,
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