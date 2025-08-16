import React, { useState } from 'react';
import { borrowService } from '../services/api';
import './BorrowBook.css';

interface Book {
  id: number;
  title: string;
  author: string;
  isbn: string;
  available_copies: number;
}

interface BorrowBookProps {
  book: Book;
  onBorrowSuccess: () => void;
}

const BorrowBook: React.FC<BorrowBookProps> = ({ book, onBorrowSuccess }) => {
  const [isBorrowing, setIsBorrowing] = useState(false);

  const borrowBook = async () => {
    if (book.available_copies <= 0) {
      alert('该图书库存不足');
      return;
    }

    setIsBorrowing(true);
    try {
      const response = await borrowService.borrowBook(book.id);
      
      if (response.success) {
        alert('借阅成功！请在7天内归还');
        onBorrowSuccess();
      } else {
        alert(response.message || '借阅失败');
      }
    } catch (error) {
      console.error('借阅失败:', error);
      alert('借阅失败，请稍后重试');
    } finally {
      setIsBorrowing(false);
    }
  };

  return (
    <div className="borrow-book">
      <div className="book-card">
        <div className="book-info">
          <h3>{book.title}</h3>
          <p className="author">作者: {book.author}</p>
          <p className="isbn">ISBN: {book.isbn}</p>
          <p className="available">可借数量: {book.available_copies}</p>
        </div>
        <div className="borrow-section">
          <button 
            onClick={borrowBook} 
            disabled={book.available_copies <= 0 || isBorrowing}
            className="borrow-btn"
          >
            {isBorrowing ? '借阅中...' : '借阅'}
          </button>
        </div>
      </div>
    </div>
  );
};

export default BorrowBook;