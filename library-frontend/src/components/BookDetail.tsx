import { useEffect } from 'react';
import type { Book } from '../services/api';
import './BookDetail.css';

interface BookDetailProps {
  book: Book;
  onClose: () => void;
  isAdmin?: boolean;
}

const BookDetail: React.FC<BookDetailProps> = ({ book, onClose, isAdmin = false }) => {
  const getStockStatus = (available: number, total: number) => {
    if (available === 0) {
      return { text: '已借完', className: 'status-out-of-stock' };
    } else if (available < total * 0.3) {
      return { text: '库存紧张', className: 'status-low-stock' };
    } else {
      return { text: '可借阅', className: 'status-available' };
    }
  };

  const stockStatus = getStockStatus(book.available_copies, book.total_copies);

  // 处理点击模态框外部关闭
  const handleBackdropClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  // 处理ESC键关闭
  useEffect(() => {
    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', handleEsc);
    return () => window.removeEventListener('keydown', handleEsc);
  }, [onClose]);

  return (
    <div className="book-detail-modal" onClick={handleBackdropClick}>
      <div className="book-detail-content">
        <div className="book-detail-header">
          <h2>图书详情</h2>
          <button className="close-btn" onClick={onClose}>×</button>
        </div>

        <div className="book-detail-body">
          <div className="book-info-section">
            <div className="book-title-section">
              <h3 className="book-title">{book.title}</h3>
              <span className={`stock-badge ${stockStatus.className}`}>
                {stockStatus.text}
              </span>
            </div>

            <div className="book-details">
              <div className="detail-row">
                <label>ISBN:</label>
                <span>{book.isbn}</span>
              </div>

              <div className="detail-row">
                <label>作者:</label>
                <span>{book.author}</span>
              </div>

              {book.category && (
                <div className="detail-row">
                  <label>分类:</label>
                  <span>{book.category}</span>
                </div>
              )}

              {book.publisher && (
                <div className="detail-row">
                  <label>出版社:</label>
                  <span>{book.publisher}</span>
                </div>
              )}

              <div className="detail-row">
                <label>总数量:</label>
                <span>{book.total_copies} 本</span>
              </div>

              <div className="detail-row">
                <label>可用数量:</label>
                <span>{book.available_copies} 本</span>
              </div>

              <div className="detail-row">
                <label>已借出:</label>
                <span>{book.total_copies - book.available_copies} 本</span>
              </div>

              <div className="detail-row">
                <label>添加时间:</label>
                <span>{book.created_at ? new Date(book.created_at).toLocaleDateString() : '未知'}</span>
              </div>
            </div>

            <div className="book-actions">
              {book.available_copies > 0 ? (
                <button className="btn btn-primary">
                  借阅图书
                </button>
              ) : (
                <button className="btn btn-secondary" disabled>
                  暂无库存
                </button>
              )}
              
              {isAdmin && (
                <>
                  <button className="btn btn-warning">
                    编辑信息
                  </button>
                  <button className="btn btn-danger">
                    删除图书
                  </button>
                </>
              )}
            </div>
          </div>

          {isAdmin && (
            <div className="borrow-history">
              <h3>借阅历史</h3>
              <div className="history-placeholder">
                <p>借阅历史功能即将上线</p>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default BookDetail;