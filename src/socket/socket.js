import io from 'socket.io-client'

// 创建连接，待添加后端地址
const socket = io('http://localhost:3000')

socket.on('connect', () => {
  console.log('Socket connected to backend!')
})

// socket 对象
export default socket