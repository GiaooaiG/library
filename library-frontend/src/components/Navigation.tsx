import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import './Navigation.css';

interface User {
  id: number;
  username: string;
  role?: string;
  phone?: string;
}

interface NavigationProps {
  user: User | null;
  onLogout: () => void;
}

const Navigation: React.FC<NavigationProps> = ({ user, onLogout }) => {
  const navigate = useNavigate();

  const handleLogout = () => {
    onLogout();
    navigate('/');
  };

  return (
    <nav className="navigation">
      <div className="nav-container">
        <Link to="/" className="nav-brand">
          📚 图书馆管理系统
        </Link>
        
        <div className="nav-menu">
          {user ? (
            <>
              <Link to="/popular-books" className="nav-link">
                借阅排行榜
              </Link>
              <Link to="/inventory-stats" className="nav-link">
                库存统计
              </Link>
              
              {user.role === 'admin' ? (
                <>
                  <Link to="/admin/books/add" className="nav-link">
                    添加图书
                  </Link>
                  <Link to="/admin/users" className="nav-link">
                    用户管理
                  </Link>
                  <Link to="/admin/borrow-records" className="nav-link">
                    借阅记录
                  </Link>
                </>
              ) : (
                <Link to="/borrow-history" className="nav-link">
                  借阅历史
                </Link>
              )}
              
              <span className="nav-user">
                欢迎, {user.username}
                {user.role === 'admin' && <span className="role-badge">管理员</span>}
              </span>
              <button onClick={handleLogout} className="nav-button">
                退出登录
              </button>
            </>
          ) : (
            <>
              <Link to="/login" className="nav-link">
                登录
              </Link>
              <Link to="/register" className="nav-link">
                注册
              </Link>
            </>
          )}
        </div>
      </div>
    </nav>
  );
};

export default Navigation;