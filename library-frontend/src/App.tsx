import { useState } from 'react';
import BookForm from './components/BookForm';
import BookList from './components/BookList';
import './App.css';

function App() {
  const [refreshKey, setRefreshKey] = useState(0);

  const handleBookAdded = () => {
    setRefreshKey(prev => prev + 1);
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>📚 图书馆管理系统</h1>
        <p>欢迎使用图书管理系统，您可以在这里添加和管理图书</p>
      </header>

      <main className="app-main">
        <section className="add-book-section">
          <BookForm onSuccess={handleBookAdded} />
        </section>

        <section className="books-section">
          <BookList key={refreshKey} />
        </section>
      </main>
    </div>
  );
}

export default App;
