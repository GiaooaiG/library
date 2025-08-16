import { useState, useEffect } from 'react';
import { bookService, authService } from '../services/api';
import { borrowService } from '../services/api';
import type { Book, PaginatedResponse, PaginationParams } from '../services/api';
import BookDetail from './BookDetail';
import './BookList.css';

interface BookListProps {
  onBookSelect?: (book: Book) => void;
  showAdminActions?: boolean;
}

const BookList: React.FC<BookListProps> = ({ showAdminActions = false }) => {
  const [books, setBooks] = useState<Book[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [pagination, setPagination] = useState<PaginatedResponse<Book> | null>(null);
  const [selectedBook, setSelectedBook] = useState<Book | null>(null);
  const [showDetail, setShowDetail] = useState(false);
  
  // 搜索和分页状态
  const [searchInput, setSearchInput] = useState('');
  const [categoryInput, setCategoryInput] = useState('');
  const [searchTerm, setSearchTerm] = useState('');
  const [categoryFilter, setCategoryFilter] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const [perPage] = useState(20);
  const [categories, setCategories] = useState<string[]>([]);

  const fetchBooks = async (params: PaginationParams = {}) => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await bookService.getBooks(params);
      
      if (response.success && response.data) {
        setBooks(response.data.data);
        setPagination(response.data);
        
        // 提取分类列表
        const uniqueCategories = [...new Set(response.data.data
          .map(book => book.category)
          .filter(Boolean)
        )] as string[];
        setCategories(uniqueCategories.sort());
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
    fetchBooks({
      page: currentPage,
      per_page: perPage,
      search: searchTerm || undefined,
      category: categoryFilter || undefined,
    });
  }, [currentPage, searchTerm, categoryFilter, perPage]);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    setSearchTerm(searchInput);
    setCategoryFilter(categoryInput);
    setCurrentPage(1);
  };

  const handleReset = () => {
    setSearchInput('');
    setCategoryInput('');
    setSearchTerm('');
    setCategoryFilter('');
    setCurrentPage(1);
  };

  const handleBookClick = (book: Book) => {
    setSelectedBook(book);
    setShowDetail(true);
  };

  const handleCloseDetail = () => {
    setShowDetail(false);
    setSelectedBook(null);
  };

  const handleBorrowSuccess = (updatedBook: Book) => {
    // 更新图书列表中的对应图书信息
    setBooks(prevBooks =>
      prevBooks.map(book =>
        book.id === updatedBook.id ? updatedBook : book
      )
    );
    
    // 如果分页数据也需要更新
    if (pagination) {
      setPagination(prev => prev ? {
        ...prev,
        data: prev.data.map(book =>
          book.id === updatedBook.id ? updatedBook : book
        )
      } : null);
    }
  };

  const handlePageChange = (page: number) => {
    setCurrentPage(page);
  };

  const handleBorrowBook = async (bookId: number) => {
    if (!authService.isAuthenticated()) {
      alert('请先登录');
      return;
    }

    try {
      const response = await borrowService.borrowBook(bookId);
      
      if (response.success) {
        alert('借阅成功！请在7天内归还');
        // 更新本地图书数据，避免重新获取整个列表
        setBooks(prevBooks =>
          prevBooks.map(book =>
            book.id === bookId
              ? { ...book, available_copies: book.available_copies - 1 }
              : book
          )
        );
        
        // 更新分页数据
        if (pagination) {
          setPagination(prev => prev ? {
            ...prev,
            data: prev.data.map(book =>
              book.id === bookId
                ? { ...book, available_copies: book.available_copies - 1 }
                : book
            )
          } : null);
        }
      } else {
        alert(response.message || '借阅失败');
      }
    } catch (error) {
      console.error('借阅失败:', error);
      alert('借阅失败，请稍后重试');
    }
  };

  const getStockStatus = (available: number, total: number) => {
    if (available === 0) {
      return { text: '已借完', className: 'status-out-of-stock' };
    } else if (available < total * 0.3) {
      return { text: '库存紧张', className: 'status-low-stock' };
    } else {
      return { text: '可借阅', className: 'status-available' };
    }
  };

  const renderPagination = () => {
    if (!pagination || pagination.total_pages <= 1) return null;

    const pages = [];
    const maxVisiblePages = 5;
    let startPage = Math.max(1, currentPage - Math.floor(maxVisiblePages / 2));
    let endPage = Math.min(pagination.total_pages, startPage + maxVisiblePages - 1);
    
    if (endPage - startPage < maxVisiblePages - 1) {
      startPage = Math.max(1, endPage - maxVisiblePages + 1);
    }

    for (let i = startPage; i <= endPage; i++) {
      pages.push(
        <button
          key={i}
          className={`page-btn ${i === currentPage ? 'active' : ''}`}
          onClick={() => handlePageChange(i)}
          disabled={i === currentPage}
        >
          {i}
        </button>
      );
    }

    return (
      <div className="pagination">
        <button
          className="page-btn"
          onClick={() => handlePageChange(currentPage - 1)}
          disabled={currentPage === 1}
        >
          上一页
        </button>
        {pages}
        <button
          className="page-btn"
          onClick={() => handlePageChange(currentPage + 1)}
          disabled={currentPage === pagination.total_pages}
        >
          下一页
        </button>
        <span className="page-info">
          第 {currentPage} 页 / 共 {pagination.total_pages} 页
        </span>
      </div>
    );
  };

  if (loading) {
    return <div className="loading">加载中...</div>;
  }

  if (error) {
    return (
      <div className="error-message">
        <span>{error}</span>
        <button onClick={() => fetchBooks()}>重试</button>
      </div>
    );
  }

  return (
    <div className="book-list-container">
      <div className="book-list-header">
        <h2>图书列表</h2>
        
        <form onSubmit={handleSearch} className="search-form">
          <div className="search-row">
            <input
              type="text"
              placeholder="搜索书名、作者或ISBN..."
              value={searchInput}
              onChange={(e) => setSearchInput(e.target.value)}
              className="search-input"
            />
            <select
              value={categoryInput}
              onChange={(e) => setCategoryInput(e.target.value)}
              className="category-select"
            >
              <option value="">全部分类</option>
              {categories.map((cat) => (
                <option key={cat} value={cat}>
                  {cat}
                </option>
              ))}
            </select>
            <button type="submit" className="search-btn">
              搜索
            </button>
            <button
              type="button"
              onClick={handleReset}
              className="reset-btn"
            >
              重置
            </button>
          </div>
        </form>
      </div>

      {books.length === 0 ? (
        <div className="empty-state">
          <p>暂无图书，请添加新图书或调整搜索条件</p>
        </div>
      ) : (
        <>
          <div className="books-grid">
            {books.map((book) => {
              const stockStatus = getStockStatus(book.available_copies, book.total_copies);
              
              return (
                <div
                  key={book.id}
                  className="book-card"
                  onClick={() => handleBookClick(book)}
                >
                  <div className="book-header">
                    <h3 className="book-title">{book.title}</h3>
                    <span className={`stock-status ${stockStatus.className}`}>
                      {stockStatus.text}
                    </span>
                  </div>
                  
                  <p className="book-author">作者: {book.author}</p>
                  <p className="book-isbn">ISBN: {book.isbn}</p>
                  
                  {book.category && (
                    <p className="book-category">分类: {book.category}</p>
                  )}
                  
                  {book.publisher && (
                    <p className="book-publisher">出版社: {book.publisher}</p>
                  )}
                  
                  <div className="book-stats">
                    <span>总数: {book.total_copies}</span>
                    <span>可用: {book.available_copies}</span>
                  </div>

                  {!showAdminActions && book.available_copies > 0 && (
                    <button
                      type="button"
                      className="btn btn-small btn-primary"
                      onClick={(e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        handleBorrowBook(book.id);
                      }}
                    >
                      借阅
                    </button>
                  )}
                  {showAdminActions && (
                    <div className="book-actions">
                      <button className="btn btn-small">编辑</button>
                      <button className="btn btn-small btn-danger">删除</button>
                    </div>
                  )}
                </div>
              );
            })}
          </div>

          {renderPagination()}
        </>
      )}

      {showDetail && selectedBook && (
        <BookDetail
          book={selectedBook}
          onClose={handleCloseDetail}
          isAdmin={showAdminActions}
          onBorrowSuccess={handleBorrowSuccess}
        />
      )}
    </div>
  );
};

export default BookList;