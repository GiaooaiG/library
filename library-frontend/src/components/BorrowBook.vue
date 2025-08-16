<template>
  <div class="borrow-book">
    <div class="book-card">
      <div class="book-info">
        <h3>{{ book.title }}</h3>
        <p class="author">作者: {{ book.author }}</p>
        <p class="isbn">ISBN: {{ book.isbn }}</p>
        <p class="available">可借数量: {{ book.available_copies }}</p>
      </div>
      <div class="borrow-section">
        <button 
          @click="borrowBook" 
          :disabled="book.available_copies <= 0 || isBorrowing"
          class="borrow-btn"
        >
          {{ isBorrowing ? '借阅中...' : '借阅' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'

export default {
  name: 'BorrowBook',
  props: {
    book: {
      type: Object,
      required: true
    }
  },
  setup(props) {
    const authStore = useAuthStore()
    const router = useRouter()
    const isBorrowing = ref(false)

    const borrowBook = async () => {
      if (!authStore.isAuthenticated) {
        router.push('/login')
        return
      }

      if (props.book.available_copies <= 0) {
        alert('该图书库存不足')
        return
      }

      isBorrowing.value = true
      try {
        const response = await fetch('http://localhost:8080/api/v1/borrow', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${authStore.token}`
          },
          body: JSON.stringify({
            book_id: props.book.id
          })
        })

        const data = await response.json()
        
        if (data.success) {
          alert('借阅成功！请在7天内归还')
          // 更新图书信息
          props.book.available_copies -= 1
        } else {
          alert(data.message || '借阅失败')
        }
      } catch (error) {
        console.error('借阅失败:', error)
        alert('借阅失败，请稍后重试')
      } finally {
        isBorrowing.value = false
      }
    }

    return {
      isBorrowing,
      borrowBook
    }
  }
}
</script>

<style scoped>
.borrow-book {
  margin: 10px 0;
}

.book-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  background: white;
}

.book-info h3 {
  margin: 0 0 8px 0;
  color: #333;
}

.book-info p {
  margin: 4px 0;
  color: #666;
  font-size: 14px;
}

.borrow-btn {
  padding: 8px 20px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.borrow-btn:hover:not(:disabled) {
  background: #0056b3;
}

.borrow-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}
</style>
</template>