import React, { useState, useEffect } from 'react';
import { adminService } from '../services/api';
import type { User } from '../services/api';
import styles from './UserManagement.module.css';

interface UserManagementProps {
  refreshKey?: number;
}

const UserManagement: React.FC<UserManagementProps> = ({ refreshKey = 0 }) => {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');
  const [currentPage] = useState(1);

  useEffect(() => {
    fetchUsers();
  }, [currentPage, searchTerm, refreshKey]);

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const response = await adminService.getAllUsers({
        page: currentPage,
        per_page: 10,
        search: searchTerm || undefined,
      });
      
      if (response.success && response.data) {
        setUsers(response.data.data);
      }
    } catch {
      setLoading(false);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (date?: string) => date ? new Date(date).toLocaleDateString('zh-CN') : 'N/A';

  return (
    <div className={styles.container}>
      <h2>用户管理</h2>
      
      <div className={styles.search}>
        <input
          placeholder="搜索用户名..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
      </div>

      {loading ? (
        <div>加载中...</div>
      ) : (
        <table className={styles.table}>
          <thead>
            <tr>
              <th>ID</th>
              <th>用户名</th>
              <th>电话</th>
              <th>角色</th>
              <th>注册时间</th>
            </tr>
          </thead>
          <tbody>
            {users.map(user => (
              <tr key={user.id}>
                <td>{user.id}</td>
                <td>{user.username}</td>
                <td>{user.phone || 'N/A'}</td>
                <td>
                  <span className={styles[user.role || 'unknown']}>
                    {user.role === 'admin' ? '管理员' : '用户'}
                  </span>
                </td>
                <td>{formatDate(user.created_at)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
};

export default UserManagement;