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
  renewal_count?: number;
}

const BorrowHistory: React.FC = () => {
  const [records, setRecords] = useState<BorrowRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);
  const [showRenewConfirmDialog, setShowRenewConfirmDialog] = useState(false);
  const [selectedRecord, setSelectedRecord] = useState<BorrowRecord | null>(null);
  const [returning, setReturning] = useState(false);
  const [renewing, setRenewing] = useState(false);

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

  const handleReturnBook = (record: BorrowRecord) => {
    setSelectedRecord(record);
    setShowConfirmDialog(true);
  };

  const handleRenewBook = (record: BorrowRecord) => {
    setSelectedRecord(record);
    setShowRenewConfirmDialog(true);
  };

  const confirmReturnBook = async () => {
    if (!selectedRecord) return;

    setReturning(true);
    try {
      const response = await borrowService.returnBook(selectedRecord.id);
      
      if (response.success) {
        // 更新本地记录
        setRecords(records.map(record =>
          record.id === selectedRecord.id
            ? { ...record, status: 'returned' }
            : record
        ));
        
        // 显示成功消息
        alert(response.message || '图书归还成功！');
      } else {
        alert(response.message || '归还失败，请重试');
      }
    } catch (error) {
      console.error('归还图书失败:', error);
      alert('归还失败，请稍后重试');
    } finally {
      setReturning(false);
      setShowConfirmDialog(false);
      setSelectedRecord(null);
    }
  };

  const confirmRenewBook = async () => {
    if (!selectedRecord) return;

    setRenewing(true);
    try {
      const response = await borrowService.renewBook(selectedRecord.id);
      
      if (response.success && response.data) {
        // 更新本地记录
        setRecords(records.map(record =>
          record.id === selectedRecord.id
            ? {
                ...record,
                due_date: response.data!.due_date,
                renewal_count: response.data!.renewal_count
              }
            : record
        ));
        
        // 显示成功消息
        alert(response.message || '图书续借成功！');
      } else {
        alert(response.message || '续借失败，请重试');
      }
    } catch (error: any) {
      console.error('续借图书失败:', error);
      alert(error.response?.data?.message || '续借失败，请稍后重试');
    } finally {
      setRenewing(false);
      setShowRenewConfirmDialog(false);
      setSelectedRecord(null);
    }
  };

  const cancelReturnBook = () => {
    setShowConfirmDialog(false);
    setSelectedRecord(null);
  };

  const cancelRenewBook = () => {
    setShowRenewConfirmDialog(false);
    setSelectedRecord(null);
  };

  const isOverdue = (dueDate: string) => {
    return new Date(dueDate) < new Date();
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
              {record.status === 'borrowed' && (
                <div className="record-actions">
                  <button
                    className="renew-btn"
                    onClick={() => handleRenewBook(record)}
                    disabled={(record.renewal_count || 0) >= 1}
                  >
                    续借图书
                  </button>
                  <button
                    className="return-btn"
                    onClick={() => handleReturnBook(record)}
                  >
                    归还图书
                  </button>
                  {isOverdue(record.due_date) && (
                    <span className="overdue-warning">已逾期</span>
                  )}
                  <div className="renewal-info">
                    已续借: {record.renewal_count || 0}/1次
                  </div>
                </div>
              )}
            </div>
          </div>
        ))}
      </div>

      {/* 归还确认对话框 */}
      {showConfirmDialog && (
        <div className="confirm-dialog-overlay">
          <div className="confirm-dialog">
            <h3>确认归还</h3>
            <p>确定要归还《{selectedRecord?.book_title}》吗？</p>
            {selectedRecord && isOverdue(selectedRecord.due_date) && (
              <p className="overdue-notice">
                注意：该图书已逾期，请及时归还。
              </p>
            )}
            <div className="dialog-actions">
              <button
                className="cancel-btn"
                onClick={cancelReturnBook}
                disabled={returning}
              >
                取消
              </button>
              <button
                className="confirm-btn"
                onClick={confirmReturnBook}
                disabled={returning}
              >
                {returning ? '归还中...' : '确认归还'}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* 续借确认对话框 */}
      {showRenewConfirmDialog && (
        <div className="confirm-dialog-overlay">
          <div className="confirm-dialog">
            <h3>确认续借</h3>
            <p>确定要续借《{selectedRecord?.book_title}》吗？</p>
            <p>续借后将延长7天借阅期限。</p>
            {selectedRecord && isOverdue(selectedRecord.due_date) && (
              <p className="overdue-notice">
                注意：该图书已逾期，续借后请及时归还。
              </p>
            )}
            <div className="dialog-actions">
              <button
                className="cancel-btn"
                onClick={cancelRenewBook}
                disabled={renewing}
              >
                取消
              </button>
              <button
                className="confirm-btn"
                onClick={confirmRenewBook}
                disabled={renewing}
              >
                {renewing ? '续借中...' : '确认续借'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default BorrowHistory;