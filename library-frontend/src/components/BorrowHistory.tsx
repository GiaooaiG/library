import React, { useState, useEffect } from 'react';
import { borrowService } from '../services/api';
import './BorrowHistory.css';

interface BorrowRecord {
  id: number;
  book_id: number;
  book_title: string;
  borrow_date: string;
  due_date: string;
  status: string;
}

const BorrowHistory: React.FC = () => {
  const [records, setRecords] = useState<BorrowRecord[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchBorrowHistory = async () => {
    try {
      const response = await borrowService.getBorrowHistory();
      
      if (response.success) {
        setRecords(response.data || []);
      } else {
        console.error('获取借阅历史失败:', response.message);
      }
    } catch (error) {
      console.error('获取借阅历史失败:', error);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit'
    });
  };

  const getStatusText = (status: string) => {
    const statusMap: { [key: string]: string } = {
      'borrowed': '借阅中',
      'returned': '已归还',
      'overdue': '已逾期'
    };
    return statusMap[status] || status;
  };

  const getStatusClass = (status: string) => {
    return `status ${status}`;
  };

  useEffect(() => {
    fetchBorrowHistory();
  }, []);

  if (loading) {
    return <div className="loading">加载中...</div>;
  }

  if (records.length === 0) {
    return <div className="empty">暂无借阅记录</div>;
  }

  return (
    <div className="borrow-history">
      <h2>我的借阅历史</h2>
      <div className="records">
        {records.map((record) => (
          <div key={record.id} className="record-card">
            <div className="record-info">
              <h4>{record.book_title}</h4>
              <p className="date">借阅日期: {formatDate(record.borrow_date)}</p>
              <p className="date">应还日期: {formatDate(record.due_date)}</p>
              <p className={getStatusClass(record.status)}>
                状态: {getStatusText(record.status)}
              </p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default BorrowHistory;