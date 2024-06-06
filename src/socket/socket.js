let socket = null;

// 创建 WebSocket 连接
function createWebSocketConnection() {
  const url = 'ws://localhost:3000'; // 待修改为后端 WebSocket 地址
  socket = new WebSocket(url);

  // 监听 WebSocket 连接建立事件
  socket.addEventListener('open', () => {
    console.log('Socket connected to backend!');
  });

  // 监听 WebSocket 连接关闭事件
  socket.addEventListener('close', () => {
    console.log('Socket disconnected from backend!');
  });

  // 监听 WebSocket 消息事件
  socket.addEventListener('message', (event) => {
    console.log('Received message:', event.data);
  });

  // 监听 WebSocket 错误事件
  socket.addEventListener('error', (error) => {
    console.error('Socket encountered error:', error);
  });
}

// 导出 WebSocket 对象
export function getSocket() {
  if (!socket) {
    createWebSocketConnection();
  }
  return socket;
}