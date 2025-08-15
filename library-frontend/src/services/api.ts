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

const api = axios.create({
  baseURL: API_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

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

export default api;