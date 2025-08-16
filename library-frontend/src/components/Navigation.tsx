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
              <span className="nav-user">欢迎, {user.username}</span>
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