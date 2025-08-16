import { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import BookForm from './components/BookForm';
import BookList from './components/BookList';
import Login from './components/Login';
import Register from './components/Register';
import Navigation from './components/Navigation';
import BorrowHistory from './components/BorrowHistory';
import PopularBooks from './components/PopularBooks';
import InventoryStats from './components/InventoryStats';
import './App.css';

interface User {
  id: number;
  username: string;
  role?: string;
  phone?: string;
}

function App() {
  const [user, setUser] = useState<User | null>(null);
  const [refreshKey, setRefreshKey] = useState(0);

  useEffect(() => {
    const token = localStorage.getItem('token');
    const userData = localStorage.getItem('user');
    if (token && userData) {
      setUser(JSON.parse(userData));
    }
  }, []);

  const handleLogin = (userData: User, token: string) => {
    localStorage.setItem('token', token);
    localStorage.setItem('user', JSON.stringify(userData));
    setUser(userData);
  };

  const handleLogout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    setUser(null);
  };

  const handleBookAdded = () => {
    setRefreshKey(prev => prev + 1);
  };

  return (
    <Router>
      <div className="app">
        <Navigation user={user} onLogout={handleLogout} />
        
        <main className="app-main">
          <Routes>
            <Route path="/" element={
              user ? (
                <>
                  {user.role === 'admin' ? (
                    <>
                      <section className="admin-dashboard">
                        <h2>管理员控制台</h2>
                        <p>欢迎使用管理员控制台，您可以管理图书、用户和借阅记录。</p>
                      </section>
                      <section className="books-section">
                        <BookList key={refreshKey} />
                      </section>
                    </>
                  ) : (
                    <>
                      <section className="books-section">
                        <BookList key={refreshKey} />
                      </section>
                    </>
                  )}
                </>
              ) : (
                <Navigate to="/login" replace />
              )
            } />
            <Route path="/books/:id" element={
              user ? <div>图书详情页面</div> : <Navigate to="/login" replace />
            } />
            <Route path="/login" element={
              user ? <Navigate to="/" replace /> : <Login onLogin={handleLogin} />
            } />
            <Route path="/register" element={
              user ? <Navigate to="/" replace /> : <Register />
            } />
            <Route path="/borrow-history" element={
              user ? <BorrowHistory /> : <Navigate to="/login" replace />
            } />
            <Route path="/popular-books" element={
              user ? <PopularBooks /> : <Navigate to="/login" replace />
            } />
            <Route path="/inventory-stats" element={
              user ? <InventoryStats /> : <Navigate to="/login" replace />
            } />
            <Route path="/admin/books/add" element={
              user?.role === 'admin' ? <BookForm onSuccess={handleBookAdded} /> : <Navigate to="/" replace />
            } />
            <Route path="/admin/users" element={
              user?.role === 'admin' ? <div>用户管理页面</div> : <Navigate to="/" replace />
            } />
            <Route path="/admin/borrow-records" element={
              user?.role === 'admin' ? <div>借阅记录管理页面</div> : <Navigate to="/" replace />
            } />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

export default App;
