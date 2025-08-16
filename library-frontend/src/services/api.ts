import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL;

export interface Book {
  id: number;
  isbn: string;
  title: string;
  author: string;
  category?: string;
  publisher?: string;
  total_copies: number;
  available_copies: number;
  created_at?: string;
  updated_at?: string;
}

export interface NewBook {
  isbn: string;
  title: string;
  author: string;
  category?: string;
  publisher?: string;
  total_copies?: number;
  available_copies?: number;
}

export interface User {
  id: number;
  username: string;
  role?: string;
  phone?: string;
  created_at?: string;
}

export interface RegisterData {
  username: string;
  password: string;
  phone?: string;
}

export interface LoginData {
  username: string;
  password: string;
}

export interface AuthResponse {
  user: User;
  token: string;
}

export interface ApiResponse<T> {
  success: boolean;
  message: string;
  data?: T;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export interface PaginationParams {
  page?: number;
  per_page?: number;
  search?: string;
  category?: string;
}

export interface BorrowRecord {
  id: number;
  book_id: number;
  book_title: string;
  borrow_date: string;
  due_date: string;
  status: string;
  renewal_count?: number;
}

export interface PopularBook {
  book_id: number;
  title: string;
  author: string;
  category?: string;
  publisher?: string;
  borrow_count: number;
  total_copies: number;
  available_copies: number;
}

export interface PopularBooksParams {
  start_date?: string;
  end_date?: string;
  limit?: number;
}

export interface InventoryStats {
  total_books: number;
  total_copies: number;
  available_copies: number;
  borrowed_copies: number;
  category_stats: CategoryStats[];
}

export interface CategoryStats {
  category?: string;
  book_count: number;
  total_copies: number;
  available_copies: number;
}

const api = axios.create({
  baseURL: API_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 请求拦截器，添加token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// 响应拦截器，处理401错误
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export const bookService = {
  async createBook(book: NewBook): Promise<ApiResponse<Book>> {
    const response = await api.post('/books', book);
    return response.data;
  },

  async getBooks(params?: PaginationParams): Promise<ApiResponse<PaginatedResponse<Book>>> {
    const response = await api.get('/books', { params });
    return response.data;
  },

  async getBook(id: number): Promise<ApiResponse<Book>> {
    const response = await api.get(`/books/${id}`);
    return response.data;
  },
};

export const authService = {
  async register(data: RegisterData): Promise<ApiResponse<AuthResponse>> {
    const response = await api.post('/auth/register', data);
    return response.data;
  },

  async login(data: LoginData): Promise<ApiResponse<AuthResponse>> {
    const response = await api.post('/auth/login', data);
    return response.data;
  },

  logout() {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
  },

  getCurrentUser(): User | null {
    const userData = localStorage.getItem('user');
    return userData ? JSON.parse(userData) : null;
  },

  isAuthenticated(): boolean {
    return !!localStorage.getItem('token');
  }
};

export const borrowService = {
  async borrowBook(bookId: number): Promise<ApiResponse<BorrowRecord>> {
    const response = await api.post('/borrow', { book_id: bookId });
    return response.data;
  },

  async getBorrowHistory(): Promise<ApiResponse<BorrowRecord[]>> {
    const response = await api.get('/borrow/history');
    return response.data;
  },

  async returnBook(borrowId: number): Promise<ApiResponse<BorrowRecord>> {
    const response = await api.post(`/return/${borrowId}`);
    return response.data;
  },

  async renewBook(borrowId: number): Promise<ApiResponse<BorrowRecord>> {
    const response = await api.post(`/renew/${borrowId}`);
    return response.data;
  }
};

export const statisticsService = {
  async getPopularBooks(params?: PopularBooksParams): Promise<ApiResponse<PopularBook[]>> {
    const response = await api.get('/statistics/popular-books', { params });
    return response.data;
  },

  async getInventoryStats(): Promise<ApiResponse<InventoryStats>> {
    const response = await api.get('/statistics/inventory');
    return response.data;
  }
};

// 管理员服务
export const adminService = {
  async getAllUsers(params?: PaginationParams): Promise<ApiResponse<PaginatedResponse<User>>> {
    const response = await api.get('/admin/users', { params });
    return response.data;
  },

  async getAllBorrowRecords(params?: PaginationParams): Promise<ApiResponse<PaginatedResponse<BorrowRecord>>> {
    const response = await api.get('/admin/borrow-records', { params });
    return response.data;
  },

  async getUserBorrowRecords(userId: number): Promise<ApiResponse<BorrowRecord[]>> {
    const response = await api.get(`/admin/users/${userId}/borrow-records`);
    return response.data;
  },

  async createBook(book: NewBook): Promise<ApiResponse<Book>> {
    const response = await api.post('/books', book);
    return response.data;
  }
};

// 权限工具
export const authUtils = {
  isAdmin(): boolean {
    const user = authService.getCurrentUser();
    return user?.role === 'admin';
  },

  isUser(): boolean {
    const user = authService.getCurrentUser();
    return user?.role === 'user';
  },

  canAccessUserData(targetUserId: number): boolean {
    const user = authService.getCurrentUser();
    if (!user) return false;
    
    // 管理员可以访问所有用户数据
    if (user.role === 'admin') return true;
    
    // 普通用户只能访问自己的数据
    return user.id === targetUserId;
  }
};

export default api;