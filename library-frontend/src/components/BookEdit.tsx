import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { bookService } from '../services/api';
import type { Book } from '../services/api';
import BookForm from './BookForm';
import './BookForm.css';

interface BookEditProps {
  onSuccess?: () => void;
}

const BookEdit: React.FC<BookEditProps> = ({ onSuccess }) => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [book, setBook] = useState<Book | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchBook = async () => {
      if (!id) {
        setError('图书ID不存在');
        setLoading(false);
        return;
      }

      try {
        setLoading(true);
        const response = await bookService.getBook(parseInt(id));
        
        if (response.success && response.data) {
          setBook(response.data);
        } else {
          setError(response.message || '获取图书信息失败');
        }
      } catch (err: any) {
        setError(err.message || '网络错误，请稍后重试');
      } finally {
        setLoading(false);
      }
    };

    fetchBook();
  }, [id]);

  const handleSuccess = () => {
    if (onSuccess) {
      onSuccess();
    }
    // 返回上一页
    navigate(-1);
  };

  if (loading) {
    return <div className="loading">加载中...</div>;
  }

  if (error) {
    return (
      <div className="error-message">
        <span>{error}</span>
        <button onClick={() => navigate(-1)}>返回</button>
      </div>
    );
  }

  if (!book) {
    return (
      <div className="error-message">
        <span>图书不存在</span>
        <button onClick={() => navigate(-1)}>返回</button>
      </div>
    );
  }

  return (
    <div className="book-edit-container">
      <BookForm 
        book={book} 
        isEdit={true} 
        onSuccess={handleSuccess} 
      />
    </div>
  );
};

export default BookEdit;