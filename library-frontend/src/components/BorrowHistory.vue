<template>
  <div class="borrow-history">
    <h2>我的借阅历史</h2>
    
    <div v-if="loading" class="loading">
      加载中...
    </div>
    
    <div v-else-if="records.length === 0" class="empty">
      暂无借阅记录
    </div>
    
    <div v-else class="records">
      <div v-for="record in records" :key="record.id" class="record-card">
        <div class="record-info">
          <h4>{{ record.book_title }}</h4>
          <p class="date">借阅日期: {{ formatDate(record.borrow_date) }}</p>
          <p class="date">应还日期: {{ formatDate(record.due_date) }}</p>
          <p class="status" :class="record.status">
            状态: {{ getStatusText(record.status) }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'

export default {
  name: 'BorrowHistory',
  setup() {
    const authStore = useAuthStore()
    const records = ref([])
    const loading = ref(true)

    const fetchBorrowHistory = async () => {
      try {
        const response = await fetch('http://localhost:8080/api/v1/borrow/history', {
          headers: {
            'Authorization': `Bearer ${authStore.token}`
          }
        })

        const data = await response.json()
        
        if (data.success) {
          records.value = data.data
        } else {
          console.error('获取借阅历史失败:', data.message)
        }
      } catch (error) {
        console.error('获取借阅历史失败:', error)
      } finally {
        loading.value = false
      }
    }

    const formatDate = (dateString) => {
      const date = new Date(dateString)
      return date.toLocaleDateString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit'
      })
    }

    const getStatusText = (status) => {
      const statusMap = {
        'borrowed': '借阅中',
        'returned': '已归还',
        'overdue': '已逾期'
      }
      return statusMap[status] || status
    }

    onMounted(() => {
      if (authStore.isAuthenticated) {
        fetchBorrowHistory()
      }
    })

    return {
      records,
      loading,
      formatDate,
      getStatusText
    }
  }
}
</script>

<style scoped>
.borrow-history {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

h2 {
  color: #333;
  margin-bottom: 20px;
}

.loading, .empty {
  text-align: center;
  color: #666;
  padding: 40px;
}

.records {
  display: grid;
  gap: 15px;
}

.record-card {
  padding: 15px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  background: white;
}

.record-info h4 {
  margin: 0 0 8px 0;
  color: #333;
}

.record-info p {
  margin: 4px 0;
  color: #666;
  font-size: 14px;
}

.status.borrowed {
  color: #007bff;
}

.status.returned {
  color: #28a745;
}

.status.overdue {
  color: #dc3545;
}
</style>
</template>