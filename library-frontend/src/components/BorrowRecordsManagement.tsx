import React, { useState, useEffect } from 'react';
import { adminService } from '../services/api';
import type { BorrowRecord } from '../services/api';
import styles from './BorrowRecordsManagement.module.css';

interface BorrowRecordsManagementProps {
  refreshKey?: number;
}

const BorrowRecordsManagement: React.FC<BorrowRecordsManagementProps> = ({ refreshKey = 0 }) => {
  const [records, setRecords] = useState<BorrowRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const [filterStatus, setFilterStatus] = useState<string>('all');

  useEffect(() => {
    fetchRecords();
  }, [currentPage, searchTerm, filterStatus, refreshKey]);

  const fetchRecords = async () => {
    try {
      setLoading(true);
      const response = await adminService.getAllBorrowRecords({
        page: currentPage,
        per_page: 10,
        search: searchTerm || undefined,
      });
      
      if (response.success && response.data) {
        let filtered = response.data.data;
        if (filterStatus !== 'all') {
          filtered = filtered.filter(r => r.status === filterStatus);
        }
        setRecords(filtered);
      }
    } catch (err) {
      setError('获取借阅记录失败');
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (date?: string) => date ? new Date(date).toLocaleDateString('zh-CN') : 'N/A';

  const getStatusDisplay = (status: string) => {
    switch (status) {
      case 'borrowed': return '借阅中';
      case 'returned': return '已归还';
      case 'overdue': return '已逾期';
      default: return status;
    }
  };

  const getStatusClass = (status: string) => {
    switch (status) {
      case 'borrowed': return styles.statusBorrowed;
      case 'returned': return styles.statusReturned;
      case 'overdue': return styles.statusOverdue;
      default: return '';
    }
  };

  if (loading) return <div className={styles.loading}>加载中...</div>;

  return (
    <div className={styles.container}>
      <h2>借阅记录管理</h2>
      
      <div className={styles.filters}>
        <input
          placeholder="搜索图书或用户..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
        <select value={filterStatus} onChange={(e) => setFilterStatus(e.target.value)}>
          <option value="all">全部状态</option>
          <option value="borrowed">借阅中</option>
          <option value="returned">已归还</option>
          <option value="overdue">已逾期</option>
        </select>
      </div>

      <table className={styles.table}>
        <thead>
          <tr>
            <th>ID</th>
            <th>图书</th>
            <th>借阅时间</th>
            <th>应还时间</th>
            <th>状态</th>
            <th>续借次数</th>
          </tr>
        </thead>
        <tbody>
          {records.map(record => (
            <tr key={record.id}>
              <td>{record.id}</td>
              <td>{record.book_title}</td>
              <td>{formatDate(record.borrow_date)}</td>
              <td>{formatDate(record.due_date)}</td>
              <td>
                <span className={`${styles.status} ${getStatusClass(record.status)}`}>
                  {getStatusDisplay(record.status)}
                </span>
              </td>
              <td>{record.renewal_count || 0}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default BorrowRecordsManagement;