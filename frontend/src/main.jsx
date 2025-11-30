import React from 'react';
import ReactDOM from 'react-dom/client';
import { HashRouter } from 'react-router-dom';
import App from './App';
import './styles/main.css';

import { log } from './api/ipc';

console.log('Main JSX executing');
if (window.ipc) {
  window.ipc.postMessage(JSON.stringify({ type: 'log', payload: { message: 'Main JSX executing' }, id: 'debug-main' }));
}

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <HashRouter>
      <App />
    </HashRouter>
  </React.StrictMode>,
);
