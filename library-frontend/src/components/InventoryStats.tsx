import React, { useEffect, useState } from 'react';
import { statisticsService } from '../services/api';
import type { InventoryStats as InventoryStatsType, CategoryStats } from '../services/api';
import './InventoryStats.css';

const InventoryStats: React.FC = () => {
  const [stats, setStats] = useState<InventoryStatsType | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchInventoryStats();
  }, []);

  const fetchInventoryStats = async () => {
    try {
      setLoading(true);
      const response = await statisticsService.getInventoryStats();
      if (response.success && response.data) {
        setStats(response.data);
      } else {
        setError(response.message || '获取库存统计失败');
      }
    } catch (err) {
      setError('获取库存统计失败，请稍后重试');
      console.error('Error fetching inventory stats:', err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="inventory-stats loading">
        <div className="spinner"></div>
        <p>正在加载库存统计...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="inventory-stats error">
        <div className="error-message">
          <i className="error-icon">⚠️</i>
          <p>{error}</p>
          <button onClick={fetchInventoryStats} className="retry-btn">
            重试
          </button>
        </div>
      </div>
    );
  }

  if (!stats) {
    return null;
  }

  return (
    <div className="inventory-stats">
      <div className="stats-header">
        <h2>📊 库存统计仪表板</h2>
        <p>实时查看图书馆藏书情况</p>
      </div>

      <div className="stats-overview">
        <div className="stat-card">
          <div className="stat-icon">📚</div>
          <div className="stat-content">
            <h3>{stats.total_books.toLocaleString()}</h3>
            <p>图书种类</p>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon">📖</div>
          <div className="stat-content">
            <h3>{stats.total_copies.toLocaleString()}</h3>
            <p>总藏书量</p>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon">✅</div>
          <div className="stat-content">
            <h3>{stats.available_copies.toLocaleString()}</h3>
            <p>可借图书</p>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-icon">📤</div>
          <div className="stat-content">
            <h3>{stats.borrowed_copies.toLocaleString()}</h3>
            <p>已借出</p>
          </div>
        </div>
      </div>

      <div className="category-stats">
        <h3>📂 按分类统计</h3>
        <div className="category-list">
          {stats.category_stats.map((category: CategoryStats, index: number) => (
            <div key={index} className="category-item">
              <div className="category-header">
                <h4>{category.category || '未分类'}</h4>
                <span className="book-count">{category.book_count} 种图书</span>
              </div>
              <div className="category-details">
                <div className="detail-item">
                  <span className="label">总藏书量:</span>
                  <span className="value">{category.total_copies.toLocaleString()}</span>
                </div>
                <div className="detail-item">
                  <span className="label">可借数量:</span>
                  <span className="value">{category.available_copies.toLocaleString()}</span>
                </div>
                <div className="detail-item">
                  <span className="label">借出数量:</span>
                  <span className="value">
                    {(category.total_copies - category.available_copies).toLocaleString()}
                  </span>
                </div>
                <div className="progress-bar">
                  <div 
                    className="progress-fill" 
                    style={{ 
                      width: `${category.total_copies > 0 ? (category.available_copies / category.total_copies) * 100 : 0}%` 
                    }}
                  ></div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      <div className="stats-summary">
        <div className="summary-item">
          <span>借阅率:</span>
          <strong>
            {stats.total_copies > 0 
              ? ((stats.borrowed_copies / stats.total_copies) * 100).toFixed(1) 
              : '0.0'}%
          </strong>
        </div>
        <div className="summary-item">
          <span>可借率:</span>
          <strong>
            {stats.total_copies > 0 
              ? ((stats.available_copies / stats.total_copies) * 100).toFixed(1) 
              : '0.0'}%
          </strong>
        </div>
      </div>
    </div>
  );
};

export default InventoryStats;