import { useState } from 'react';
import { BrowserRouter as Router } from 'react-router-dom';
import Navigation from './components/Navigation';
import './App.css';
import RoutesConfig from './routes/RoutesConfig';
import useAuth from './hooks/useAuth';

function App() {
  const { user, handleLogin, handleLogout } = useAuth();
  const [refreshKey, setRefreshKey] = useState(0);

  const handleBookAdded = () => {
    setRefreshKey(prev => prev + 1);
  };

  return (
    <Router>
      <div className="app">
        <Navigation user={user} onLogout={handleLogout} />
        
        <main className="app-main">
          <RoutesConfig 
            user={user} 
            refreshKey={refreshKey} 
            handleBookAdded={handleBookAdded} 
            onLogin={handleLogin} 
          />
        </main>
      </div>
    </Router>
  );
}

export default App;
