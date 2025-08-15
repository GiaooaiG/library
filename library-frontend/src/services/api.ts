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

  async getBooks(): Promise<ApiResponse<Book[]>> {
    const response = await api.get('/books');
    return response.data;
  },

  async getBook(id: number): Promise<ApiResponse<Book>> {
    const response = await api.get(`/books/${id}`);
    return response.data;
  },
};

export default api;