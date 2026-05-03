<template>
  <div class="app-container" data-tauri-drag-region>
    <!-- 猫咪容器 -->
    <div 
      class="cat-wrapper"
      @mouseenter="onMouseEnter"
      @mouseleave="onMouseLeave"
    >
      <canvas 
        ref="canvasRef"
        width="350"
        height="400"
        class="cat-canvas"
      />
      
      <!-- 姿势切换按钮 -->
      <div class="pose-buttons">
        <button @click="setPose('dance')" class="pose-btn">💃 跳舞</button>
        <button @click="setPose('happy')" class="pose-btn">😊 开心</button>
        <button @click="setPose('sleep')" class="pose-btn">😴 睡觉</button>
        <button @click="setPose('idle')" class="pose-btn">😺 正常</button>
      </div>
    </div>
    
    <!-- 天气显示 -->
    <div class="weather-widget" v-if="weather">
      <div class="weather-icon">{{ weather.icon }}</div>
      <div class="weather-info">
        <div>{{ weather.city }}</div>
        <div class="temp">{{ weather.temperature }}°C</div>
        <div>{{ weather.condition }}</div>
      </div>
    </div>
    
    <!-- 记账本 -->
    <div class="ledger-panel" :class="{ collapsed: ledgerCollapsed }">
      <div class="ledger-header" @click="ledgerCollapsed = !ledgerCollapsed">
        <span>📒 记账本</span>
        <span>{{ ledgerCollapsed ? '▼' : '▲' }}</span>
      </div>
      
      <div v-if="!ledgerCollapsed" class="ledger-content">
        <div class="add-form">
          <input v-model.number="newAmount" type="number" placeholder="金额" />
          <select v-model="newCategory">
            <option>餐饮</option>
            <option>购物</option>
            <option>交通</option>
            <option>娱乐</option>
          </select>
          <input v-model="newNote" placeholder="备注" />
          <button @click="addLedger">添加</button>
        </div>
        
        <div class="ledger-list">
          <div v-for="item in ledger" :key="item.id" class="ledger-item">
            <span class="date">{{ item.date.slice(5, 16) }}</span>
            <span class="category">{{ item.category }}</span>
            <span class="amount">¥{{ item.amount }}</span>
            <span class="note">{{ item.note }}</span>
          </div>
        </div>
        
        <div class="ledger-footer">
          <span>总计: ¥{{ total }}</span>
          <button @click="clearLedger" class="clear-btn">清空</button>
        </div>
      </div>
    </div>
    
    <!-- 图片推送（控制端功能整合） -->
    <div class="image-control">
      <input type="file" @change="onImageSelect" accept="image/*" />
      <button @click="sendImage">发送图片到猫咪</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

// 猫咪相关
const canvasRef = ref<HTMLCanvasElement | null>(null)
let ctx: CanvasRenderingContext2D | null = null
let animationId: number
const currentPose = ref('idle')
const isMouseOver = ref(false)
let idleTimer: number

// 天气
const weather = ref<any>(null)

// 记账
const ledger = ref<any[]>([])
const ledgerCollapsed = ref(false)
const newAmount = ref(0)
const newCategory = ref('餐饮')
const newNote = ref('')

const total = computed(() => ledger.value.reduce((sum, i) => sum + i.amount, 0))

// 图片
const selectedImage = ref<string | null>(null)

// 绘图函数
const drawCat = () => {
  if (!ctx) return
  
  ctx.clearRect(0, 0, 350, 400)
  
  // 布偶猫配色
  const bodyColor = '#F5E6D3'
  const pointsColor = '#8B5A2B'
  const eyeColor = '#6B9FD2'
  
  // 根据姿势调整参数
  let earRotate = 0
  let eyeSize = 12
  let mouthArc = 0
  let tailSwing = 0
  
  switch(currentPose.value) {
    case 'touching':
      earRotate = 15
      eyeSize = 14
      mouthArc = 0.3
      tailSwing = 30
      break
    case 'happy':
      earRotate = -10
      eyeSize = 10
      mouthArc = 0.5
      tailSwing = -20
      break
    case 'sleep':
      earRotate = 5
      eyeSize = 8
      mouthArc = 0
      tailSwing = 0
      break
    case 'dance':
      const t = Date.now() / 200
      earRotate = Math.sin(t) * 15
      tailSwing = Math.sin(t) * 40
      mouthArc = 0.4
      break
    default:
      earRotate = 0
      eyeSize = 12
      mouthArc = 0.1
      tailSwing = 0
  }
  
  // 身体
  ctx.fillStyle = bodyColor
  ctx.beginPath()
  ctx.ellipse(175, 240, 75, 95, 0, 0, Math.PI * 2)
  ctx.fill()
  
  // 头部
  ctx.fillStyle = bodyColor
  ctx.beginPath()
  ctx.ellipse(175, 155, 65, 60, 0, 0, Math.PI * 2)
  ctx.fill()
  
  // 左耳
  ctx.save()
  ctx.translate(125, 105)
  ctx.rotate(earRotate * Math.PI / 180)
  ctx.fillStyle = pointsColor
  ctx.beginPath()
  ctx.moveTo(0, 0)
  ctx.lineTo(-25, -40)
  ctx.lineTo(25, -40)
  ctx.fill()
  ctx.fillStyle = '#FFB6C1'
  ctx.beginPath()
  ctx.moveTo(0, -5)
  ctx.lineTo(-15, -32)
  ctx.lineTo(15, -32)
  ctx.fill()
  ctx.restore()
  
  // 右耳
  ctx.save()
  ctx.translate(225, 105)
  ctx.rotate(-earRotate * Math.PI / 180)
  ctx.fillStyle = pointsColor
  ctx.beginPath()
  ctx.moveTo(0, 0)
  ctx.lineTo(-25, -40)
  ctx.lineTo(25, -40)
  ctx.fill()
  ctx.fillStyle = '#FFB6C1'
  ctx.beginPath()
  ctx.moveTo(0, -5)
  ctx.lineTo(-15, -32)
  ctx.lineTo(15, -32)
  ctx.fill()
  ctx.restore()
  
  // 面部色块
  ctx.fillStyle = pointsColor
  ctx.globalAlpha = 0.25
  ctx.beginPath()
  ctx.ellipse(175, 170, 40, 30, 0, 0, Math.PI * 2)
  ctx.fill()
  ctx.globalAlpha = 1
  
  // 眼睛
  if (currentPose.value !== 'sleep') {
    ctx.fillStyle = eyeColor
    ctx.beginPath()
    ctx.arc(150, 145, eyeSize, 0, Math.PI * 2)
    ctx.fill()
    ctx.beginPath()
    ctx.arc(200, 145, eyeSize, 0, Math.PI * 2)
    ctx.fill()
    
    ctx.fillStyle = '#2C3E50'
    ctx.beginPath()
    ctx.arc(150, 145, eyeSize * 0.4, 0, Math.PI * 2)
    ctx.fill()
    ctx.beginPath()
    ctx.arc(200, 145, eyeSize * 0.4, 0, Math.PI * 2)
    ctx.fill()
    
    ctx.fillStyle = 'white'
    ctx.beginPath()
    ctx.arc(147, 142, 3, 0, Math.PI * 2)
    ctx.fill()
    ctx.beginPath()
    ctx.arc(197, 142, 3, 0, Math.PI * 2)
    ctx.fill()
  } else {
    ctx.strokeStyle = eyeColor
    ctx.lineWidth = 3
    ctx.beginPath()
    ctx.arc(150, 150, 10, 0, Math.PI)
    ctx.stroke()
    ctx.beginPath()
    ctx.arc(200, 150, 10, 0, Math.PI)
    ctx.stroke()
  }
  
  // 鼻子
  ctx.fillStyle = '#FF8B94'
  ctx.beginPath()
  ctx.arc(175, 165, 5, 0, Math.PI * 2)
  ctx.fill()
  
  // 嘴巴
  ctx.beginPath()
  ctx.arc(175, 175, 15, 0.1 + mouthArc, Math.PI - 0.1 - mouthArc)
  ctx.stroke()
  
  // 胡须
  ctx.beginPath()
  ctx.moveTo(120, 160)
  ctx.lineTo(90, 155)
  ctx.stroke()
  ctx.beginPath()
  ctx.moveTo(120, 168)
  ctx.lineTo(85, 168)
  ctx.stroke()
  ctx.beginPath()
  ctx.moveTo(230, 160)
  ctx.lineTo(260, 155)
  ctx.stroke()
  ctx.beginPath()
  ctx.moveTo(230, 168)
  ctx.lineTo(265, 168)
  ctx.stroke()
  
  // 围脖
  ctx.fillStyle = '#FFF0F5'
  ctx.beginPath()
  ctx.ellipse(175, 200, 45, 25, 0, 0, Math.PI * 2)
  ctx.fill()
  
  // 尾巴
  ctx.save()
  ctx.translate(110, 285)
  ctx.rotate(tailSwing * Math.PI / 180)
  ctx.fillStyle = pointsColor
  ctx.beginPath()
  ctx.moveTo(0, 0)
  ctx.lineTo(-35, -25)
  ctx.lineTo(-25, 15)
  ctx.fill()
  ctx.restore()
  
  // 爪子
  ctx.fillStyle = '#FFF0F5'
  ctx.beginPath()
  ctx.ellipse(120, 270, 18, 22, 0, 0, Math.PI * 2)
  ctx.fill()
  ctx.beginPath()
  ctx.ellipse(230, 270, 18, 22, 0, 0, Math.PI * 2)
  ctx.fill()
}

// 动画循环
const animate = () => {
  drawCat()
  animationId = requestAnimationFrame(animate)
}

// 鼠标事件
const onMouseEnter = () => {
  isMouseOver.value = true
  if (currentPose.value !== 'dance') {
    currentPose.value = 'touching'
  }
  if (idleTimer) clearTimeout(idleTimer)
}

const onMouseLeave = () => {
  isMouseOver.value = false
  idleTimer = setTimeout(() => {
    if (currentPose.value !== 'dance') {
      currentPose.value = 'idle'
    }
  }, 2000) as unknown as number
}

const setPose = (pose: string) => {
  currentPose.value = pose
  if (idleTimer) clearTimeout(idleTimer)
  if (pose !== 'dance') {
    setTimeout(() => {
      if (!isMouseOver.value) {
        currentPose.value = 'idle'
      }
    }, 5000)
  }
}

// 天气
const loadWeather = async () => {
  try {
    const data = await invoke('fetch_weather', { city: '北京' })
    weather.value = data
  } catch (error) {
    console.error('天气加载失败', error)
  }
}

// 记账
const loadLedger = async () => {
  try {
    ledger.value = await invoke('get_ledger')
  } catch (error) {
    console.error('加载记账失败', error)
  }
}

const addLedger = async () => {
  if (newAmount.value <= 0) return
  
  const entry = {
    id: Date.now(),
    date: new Date().toLocaleString(),
    amount: newAmount.value,
    category: newCategory.value,
    note: newNote.value
  }
  
  try {
    ledger.value = await invoke('add_ledger_entry', { entry })
    newAmount.value = 0
    newNote.value = ''
  } catch (error) {
    console.error('添加失败', error)
  }
}

const clearLedger = async () => {
  if (confirm('清空所有记账记录？')) {
    try {
      await invoke('clear_ledger')
      ledger.value = []
    } catch (error) {
      console.error('清空失败', error)
    }
  }
}

// 图片功能
const onImageSelect = (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    const reader = new FileReader()
    reader.onload = (e) => {
      selectedImage.value = e.target?.result as string
    }
    reader.readAsDataURL(input.files[0])
  }
}

const sendImage = async () => {
  if (!selectedImage.value) {
    alert('请先选择图片')
    return
  }
  
  const imageData = {
    id: Date.now().toString(),
    url: selectedImage.value,
    timestamp: new Date().toISOString()
  }
  
  try {
    await invoke('receive_image', { imageData })
    alert('图片已发送到猫咪！')
    selectedImage.value = null
  } catch (error) {
    console.error('发送失败', error)
  }
}

// 初始化
onMounted(() => {
  if (canvasRef.value) {
    ctx = canvasRef.value.getContext('2d')
    animate()
  }
  loadWeather()
  loadLedger()
  
  // 每30分钟刷新天气
  setInterval(loadWeather, 30 * 60 * 1000)
})

onUnmounted(() => {
  if (animationId) cancelAnimationFrame(animationId)
  if (idleTimer) clearTimeout(idleTimer)
})
</script>

<style scoped>
.app-container {
  width: 350px;
  height: 400px;
  position: relative;
  background: transparent;
}

.cat-wrapper {
  width: 100%;
  height: 100%;
  position: relative;
  -webkit-app-region: drag;
}

.cat-canvas {
  width: 100%;
  height: 100%;
  -webkit-app-region: no-drag;
  cursor: pointer;
}

.pose-buttons {
  position: absolute;
  bottom: 10px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 5px;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px);
  padding: 5px 10px;
  border-radius: 20px;
  -webkit-app-region: no-drag;
}

.pose-btn {
  background: rgba(255, 255, 255, 0.9);
  border: none;
  padding: 4px 10px;
  border-radius: 15px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.pose-btn:hover {
  transform: scale(1.05);
  background: white;
}

.weather-widget {
  position: fixed;
  top: 10px;
  right: 10px;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(10px);
  padding: 8px 12px;
  border-radius: 15px;
  display: flex;
  gap: 10px;
  color: white;
  font-size: 14px;
  z-index: 1000;
  pointer-events: none;
}

.weather-icon {
  font-size: 32px;
}

.temp {
  font-size: 18px;
  font-weight: bold;
}

.ledger-panel {
  position: fixed;
  bottom: 10px;
  left: 10px;
  width: 280px;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(10px);
  border-radius: 10px;
  color: white;
  z-index: 1000;
  transition: all 0.3s;
}

.ledger-panel.collapsed {
  width: auto;
}

.ledger-header {
  padding: 8px 12px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  font-weight: bold;
}

.ledger-content {
  padding: 10px;
  font-size: 12px;
  max-height: 300px;
  overflow-y: auto;
}

.add-form {
  display: flex;
  gap: 5px;
  flex-wrap: wrap;
  margin-bottom: 10px;
}

.add-form input, .add-form select, .add-form button {
  padding: 4px 6px;
  font-size: 11px;
  border-radius: 4px;
  border: none;
}

.ledger-list {
  max-height: 200px;
  overflow-y: auto;
}

.ledger-item {
  display: flex;
  gap: 8px;
  padding: 4px 0;
  border-bottom: 1px solid rgba(255,255,255,0.1);
  font-size: 11px;
}

.date { width: 70px; }
.category { width: 40px; }
.amount { width: 50px; color: #4CAF50; }
.note { flex: 1; }

.ledger-footer {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid rgba(255,255,255,0.2);
}

.clear-btn {
  background: #f44336;
  color: white;
  border: none;
  padding: 2px 8px;
  border-radius: 4px;
  cursor: pointer;
}

.image-control {
  position: fixed;
  bottom: 10px;
  right: 10px;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(10px);
  padding: 8px;
  border-radius: 10px;
  display: flex;
  gap: 5px;
  z-index: 1000;
}

.image-control input, .image-control button {
  font-size: 11px;
  padding: 4px 8px;
  border-radius: 4px;
  border: none;
}

.image-control button {
  background: #2196F3;
  color: white;
  cursor: pointer;
}
</style>