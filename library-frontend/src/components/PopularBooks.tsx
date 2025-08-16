import React, { useState, useEffect } from 'react';
import { statisticsService } from '../services/api';
import './PopularBooks.css';

interface PopularBook {
    book_id: number;
    title: string;
    author: string;
    category?: string;
    publisher?: string;
    borrow_count: number;
    total_copies: number;
    available_copies: number;
}

interface PopularBooksParams {
    start_date?: string;
    end_date?: string;
    limit?: number;
}

const PopularBooks: React.FC = () => {
    const [popularBooks, setPopularBooks] = useState<PopularBook[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [timeRange, setTimeRange] = useState<string>('all');
    const [customStartDate, setCustomStartDate] = useState<string>('');
    const [customEndDate, setCustomEndDate] = useState<string>('');

    useEffect(() => {
        fetchPopularBooks();
    }, []);

    const fetchPopularBooks = async (params?: PopularBooksParams) => {
        try {
            setLoading(true);
            setError(null);
            
            const queryParams = new URLSearchParams();
            if (params?.start_date) queryParams.append('start_date', params.start_date);
            if (params?.end_date) queryParams.append('end_date', params.end_date);
            if (params?.limit) queryParams.append('limit', params.limit.toString());
            
            const response = await statisticsService.getPopularBooks(params);
            setPopularBooks(response.data || []);
        } catch (err) {
            setError('获取借阅排行榜失败，请稍后重试');
            console.error('Error fetching popular books:', err);
        } finally {
            setLoading(false);
        }
    };

    const handleTimeRangeChange = (range: string) => {
        setTimeRange(range);
        
        const params: PopularBooksParams = { limit: 10 };
        
        switch (range) {
            case 'week':
                const weekAgo = new Date();
                weekAgo.setDate(weekAgo.getDate() - 7);
                params.start_date = weekAgo.toISOString().split('T')[0];
                break;
            case 'month':
                const monthAgo = new Date();
                monthAgo.setMonth(monthAgo.getMonth() - 1);
                params.start_date = monthAgo.toISOString().split('T')[0];
                break;
            case 'year':
                const yearAgo = new Date();
                yearAgo.setFullYear(yearAgo.getFullYear() - 1);
                params.start_date = yearAgo.toISOString().split('T')[0];
                break;
            case 'custom':
                if (customStartDate) params.start_date = customStartDate;
                if (customEndDate) params.end_date = customEndDate;
                break;
            default:
                // 'all' - 不添加时间限制
                break;
        }
        
        fetchPopularBooks(params);
    };

    const handleCustomDateChange = () => {
        if (timeRange === 'custom') {
            handleTimeRangeChange('custom');
        }
    };

    const getRankClass = (rank: number) => {
        if (rank === 1) return 'rank-1';
        if (rank === 2) return 'rank-2';
        if (rank === 3) return 'rank-3';
        return '';
    };

    const formatDate = (dateString: string) => {
        return new Date(dateString).toLocaleDateString('zh-CN');
    };

    if (loading) {
        return (
            <div className="popular-books-container">
                <div className="loading">加载中...</div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="popular-books-container">
                <div className="error">{error}</div>
            </div>
        );
    }

    return (
        <div className="popular-books-container">
            <div className="popular-books-header">
                <h1 className="popular-books-title">📚 借阅排行榜</h1>
                
                <div className="time-filter-section">
                    <label>时间段：</label>
                    <select 
                        value={timeRange} 
                        onChange={(e) => handleTimeRangeChange(e.target.value)}
                    >
                        <option value="all">全部时间</option>
                        <option value="week">最近一周</option>
                        <option value="month">最近一月</option>
                        <option value="year">最近一年</option>
                        <option value="custom">自定义</option>
                    </select>
                    
                    {timeRange === 'custom' && (
                        <>
                            <input
                                type="date"
                                value={customStartDate}
                                onChange={(e) => {
                                    setCustomStartDate(e.target.value);
                                    handleCustomDateChange();
                                }}
                                placeholder="开始日期"
                            />
                            <input
                                type="date"
                                value={customEndDate}
                                onChange={(e) => {
                                    setCustomEndDate(e.target.value);
                                    handleCustomDateChange();
                                }}
                                placeholder="结束日期"
                            />
                        </>
                    )}
                </div>
            </div>

            {popularBooks.length === 0 ? (
                <div className="empty-state">
                    <h3>暂无借阅数据</h3>
                    <p>当前时间段内没有图书借阅记录</p>
                </div>
            ) : (
                <table className="popular-books-table">
                    <thead>
                        <tr>
                            <th>排名</th>
                            <th>图书信息</th>
                            <th>借阅次数</th>
                            <th>库存信息</th>
                        </tr>
                    </thead>
                    <tbody>
                        {popularBooks.map((book, index) => (
                            <tr key={book.book_id}>
                                <td className={`rank-cell ${getRankClass(index + 1)}`}>
                                    {index + 1}
                                </td>
                                <td>
                                    <div className="book-info">
                                        <div className="book-title">{book.title}</div>
                                        <div className="book-author">作者：{book.author}</div>
                                        {book.category && (
                                            <div className="book-category">分类：{book.category}</div>
                                        )}
                                        {book.publisher && (
                                            <div className="book-category">出版社：{book.publisher}</div>
                                        )}
                                    </div>
                                </td>
                                <td className="borrow-count">{book.borrow_count}</td>
                                <td>
                                    <div className="stock-info">
                                        <div className="total-copies">总库存：{book.total_copies}</div>
                                        <div className="available-copies">可借：{book.available_copies}</div>
                                    </div>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            )}
        </div>
    );
};

export default PopularBooks;