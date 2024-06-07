const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  devServer: {
    proxy: {
      '/websocket': {
        // 待修改为后端 WebSocket 地址
        target: 'ws://localhost:6444',
        ws: true,
        changeOrigin: true,
      }
    }
  }
})
