import { useState, useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { bookService } from '../services/api';
import type { NewBook, Book } from '../services/api';
import { useNavigate, useParams } from 'react-router-dom';
import './BookForm.css';

const bookSchema = z.object({
  isbn: z.string()
    .length(13, 'ISBN必须是13位数字')
    .regex(/^\d+$/, 'ISBN只能包含数字'),
  title: z.string()
    .min(1, '书名不能为空')
    .max(255, '书名不能超过255个字符'),
  author: z.string()
    .min(1, '作者不能为空')
    .max(255, '作者名不能超过255个字符'),
  category: z.string()
    .max(100, '分类不能超过100个字符')
    .optional(),
  publisher: z.string()
    .max(255, '出版社不能超过255个字符')
    .optional(),
  total_copies: z.number()
    .min(1, '图书数量必须大于0')
    .max(1000, '图书数量不能超过1000')
    .default(1),
  available_copies: z.number()
    .min(0, '可用数量不能为负数')
    .max(1000, '可用数量不能超过1000')
    .default(1),
});

type BookFormData = z.infer<typeof bookSchema>;

interface BookFormProps {
  onSuccess?: () => void;
  book?: Book;
  isEdit?: boolean;
}

const BookForm: React.FC<BookFormProps> = ({ onSuccess, book, isEdit = false }) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
    watch,
    setValue,
  } = useForm<BookFormData>({
    resolver: zodResolver(bookSchema),
    defaultValues: {
      total_copies: 1,
      available_copies: 1,
    },
  });

  // 如果是编辑模式，加载图书数据
  useEffect(() => {
    if (isEdit && book) {
      setValue('isbn', book.isbn);
      setValue('title', book.title);
      setValue('author', book.author);
      setValue('category', book.category || '');
      setValue('publisher', book.publisher || '');
      setValue('total_copies', book.total_copies);
      setValue('available_copies', book.available_copies);
    }
  }, [isEdit, book, setValue]);

  const totalCopies = watch('total_copies', 1);

  const onSubmit = async (data: BookFormData) => {
    setLoading(true);
    setError(null);
    setSuccess(false);

    try {
      const bookData: NewBook = {
        ...data,
        category: data.category || undefined,
        publisher: data.publisher || undefined,
      };

      let response;
      if (isEdit && (book?.id || id)) {
        response = await bookService.updateBook(book?.id || parseInt(id!), bookData);
      } else {
        response = await bookService.createBook(bookData);
      }
      
      if (response.success) {
        setSuccess(true);
        if (!isEdit) {
          reset();
        }
        if (onSuccess) {
          onSuccess();
        }
        // 编辑成功后返回上一页
        if (isEdit) {
          setTimeout(() => {
            navigate(-1);
          }, 1500);
        }
      } else {
        setError(response.message || (isEdit ? '更新图书失败' : '添加图书失败'));
      }
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || '网络错误，请稍后重试');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="book-form-container">
      <h2>{isEdit ? '编辑图书' : '添加新图书'}</h2>
      
      {error && (
        <div className="alert alert-error">
          <span>{error}</span>
        </div>
      )}
      
      {success && (
        <div className="alert alert-success">
          <span>{isEdit ? '图书更新成功！' : '图书添加成功！'}</span>
        </div>
      )}

      <form onSubmit={handleSubmit(onSubmit)} className="book-form">
        <div className="form-group">
          <label htmlFor="isbn">ISBN *</label>
          <input
            type="text"
            id="isbn"
            {...register('isbn')}
            placeholder="请输入13位ISBN"
            maxLength={13}
          />
          {errors.isbn && <span className="error">{errors.isbn.message}</span>}
        </div>

        <div className="form-group">
          <label htmlFor="title">书名 *</label>
          <input
            type="text"
            id="title"
            {...register('title')}
            placeholder="请输入书名"
          />
          {errors.title && <span className="error">{errors.title.message}</span>}
        </div>

        <div className="form-group">
          <label htmlFor="author">作者 *</label>
          <input
            type="text"
            id="author"
            {...register('author')}
            placeholder="请输入作者"
          />
          {errors.author && <span className="error">{errors.author.message}</span>}
        </div>

        <div className="form-row">
          <div className="form-group">
            <label htmlFor="category">分类</label>
            <input
              type="text"
              id="category"
              {...register('category')}
              placeholder="请输入分类"
            />
            {errors.category && <span className="error">{errors.category.message}</span>}
          </div>

          <div className="form-group">
            <label htmlFor="publisher">出版社</label>
            <input
              type="text"
              id="publisher"
              {...register('publisher')}
              placeholder="请输入出版社"
            />
            {errors.publisher && <span className="error">{errors.publisher.message}</span>}
          </div>
        </div>

        <div className="form-row">
          <div className="form-group">
            <label htmlFor="total_copies">总数量 *</label>
            <input
              type="number"
              id="total_copies"
              {...register('total_copies', { valueAsNumber: true })}
              min="1"
              max="1000"
            />
            {errors.total_copies && <span className="error">{errors.total_copies.message}</span>}
          </div>

          <div className="form-group">
            <label htmlFor="available_copies">可用数量 *</label>
            <input
              type="number"
              id="available_copies"
              {...register('available_copies', { valueAsNumber: true })}
              min="0"
              max={totalCopies}
            />
            {errors.available_copies && <span className="error">{errors.available_copies.message}</span>}
          </div>
        </div>

        <div className="form-actions">
          <button type="submit" disabled={loading} className="btn btn-primary">
            {loading ? (isEdit ? '更新中...' : '添加中...') : (isEdit ? '更新图书' : '添加图书')}
          </button>
          {!isEdit && (
            <button type="button" onClick={() => reset()} className="btn btn-secondary">
              重置
            </button>
          )}
          <button type="button" onClick={() => navigate(-1)} className="btn btn-secondary">
            返回
          </button>
        </div>
      </form>
    </div>
  );
};

export default BookForm;