import { Routes, Route, Navigate } from 'react-router-dom';
import BookList from '../components/BookList';
import Login from '../components/Login';
import Register from '../components/Register';
import BorrowHistory from '../components/BorrowHistory';
import PopularBooks from '../components/PopularBooks';
import InventoryStats from '../components/InventoryStats';
import BookForm from '../components/BookForm';
import UserManagement from '../components/UserManagement';
import BorrowRecordsManagement from '../components/BorrowRecordsManagement';

interface User {
  id: number;
  username: string;
  role?: string;
  phone?: string;
}

interface RoutesConfigProps {
  user: User | null;
  refreshKey: number;
  handleBookAdded: () => void;
  onLogin: (userData: User, token: string) => void;
}

function RoutesConfig({ user, refreshKey, handleBookAdded, onLogin }: RoutesConfigProps) {
  return (
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
        user ? <Navigate to="/" replace /> : <Login onLogin={onLogin} />
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
        user?.role === 'admin' ? <UserManagement /> : <Navigate to="/" replace />
      } />
      <Route path="/admin/borrow-records" element={
        user?.role === 'admin' ? <BorrowRecordsManagement /> : <Navigate to="/" replace />
      } />
    </Routes>
  );
}

export default RoutesConfig;