const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  devServer: {
    proxy: {
      '/socket.io': {
        // 待添加后端地址
        target: 'http://localhost:3000',
        changeOrigin: true
      }
    }
  }
})
