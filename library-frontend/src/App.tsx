import { useState, useEffect } from 'react';
import BookForm from './components/BookForm';
import { bookService } from './services/api';
import type { Book } from './services/api';
import './App.css';

function App() {
  const [books, setBooks] = useState<Book[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchBooks = async () => {
    try {
      setLoading(true);
      const response = await bookService.getBooks();
      if (response.success && response.data) {
        setBooks(response.data);
      } else {
        setError(response.message || '获取图书列表失败');
      }
    } catch (err: any) {
      setError(err.message || '网络错误，请稍后重试');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchBooks();
  }, []);

  const handleBookAdded = () => {
    fetchBooks();
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>📚 图书馆管理系统</h1>
        <p>欢迎使用图书管理系统，您可以在这里添加和管理图书</p>
      </header>

      <main className="app-main">
        <section className="add-book-section">
          <BookForm onSuccess={handleBookAdded} />
        </section>

        <section className="books-section">
          <h2>图书列表</h2>
          
          {error && (
            <div className="alert alert-error">
              <span>{error}</span>
            </div>
          )}

          {loading ? (
            <div className="loading">加载中...</div>
          ) : books.length === 0 ? (
            <div className="empty-state">
              <p>暂无图书，请添加新图书</p>
            </div>
          ) : (
            <div className="books-grid">
              {books.map((book) => (
                <div key={book.id} className="book-card">
                  <h3>{book.title}</h3>
                  <p className="book-author">作者: {book.author}</p>
                  <p className="book-isbn">ISBN: {book.isbn}</p>
                  {book.category && <p className="book-category">分类: {book.category}</p>}
                  {book.publisher && <p className="book-publisher">出版社: {book.publisher}</p>}
                  <div className="book-stats">
                    <span>总数: {book.total_copies}</span>
                    <span>可用: {book.available_copies}</span>
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>
      </main>
    </div>
  );
}

export default App;
