import React, { useState, useEffect } from 'react';
import { HashRouter as Router, Routes, Route, Link, useLocation, useParams, Navigate } from 'react-router-dom';
import { Database } from 'lucide-react';
import Editor from './components/Editor';
import TopMenu from './components/TopMenu';
import ResearchView from './components/ResearchView';
import Hierarchy from './components/tools/Hierarchy';
import Codex from './components/tools/Codex';
import Mindmap from './components/tools/Mindmap';
import Brainstorm from './components/tools/Brainstorm';
import Plot from './components/tools/Plot';
import Notes from './components/tools/Notes';
import { db, log } from './api/ipc';
import './styles/main.css';

const MainWindow = () => {
  const [dbStatus, setDbStatus] = useState('Connecting...')

  useEffect(() => {
    async function checkDb() {
      try {
        log('Checking database connection...')
        // Ensure documents table exists
        await db.execute(`
          CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY,
            content TEXT,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
          )
        `);

        const result = await db.query("SELECT count(*) as count FROM sqlite_master")
        log(`Database check result: ${JSON.stringify(result)}`)
        if (result.data && result.data.length > 0) {
          setDbStatus(`Connected! Tables: ${result.data[0].count}`)
        } else {
          setDbStatus('Connected but no data returned')
        }
      } catch (err) {
        console.error(err)
        setDbStatus(`Error: ${err.message}`)
        log(`Database error: ${err.message}`)
      }
    }
    checkDb()
  }, [])

  return (
    <div className="app-container flex-col h-screen overflow-hidden">
      <TopMenu />
      <div className="flex-1 flex flex-col overflow-hidden">
        <div className="p-2 bg-gray-800 border-b border-gray-700 flex justify-between items-center">
          <div className="flex items-center gap-2 text-sm text-gray-300">
            <Database size={14} className="text-blue-400" />
            <span>{dbStatus}</span>
          </div>
        </div>
        <div className="flex-1 overflow-auto bg-gray-900">
          <Routes>
            <Route path="/" element={<Navigate to="/manuscript" replace />} />
            <Route path="/manuscript" element={<Editor />} />
            <Route path="/tools/hierarchy" element={<Hierarchy />} />
            <Route path="/tools/codex" element={<Codex />} />
            <Route path="/tools/plot" element={<Plot />} />
            <Route path="/tools/notes" element={<Notes />} />
            <Route path="/tools/research" element={<ResearchView />} />
            <Route path="/tools/mindmap" element={<Mindmap />} />
            <Route path="/tools/brainstorm" element={<Brainstorm />} />
          </Routes>
        </div>
      </div>
    </div>
  );
};

const ToolWindow = () => {
  const { toolId } = useParams();
  return (
    <div className="app-container flex-col">
      <div className="titlebar">
        <span className="font-medium">Tool: {toolId}</span>
      </div>
      <div className="main-content p-4">
        <h2 className="text-xl font-bold mb-4 capitalize">{toolId} Tool</h2>
        <Editor placeholder={`Start writing in ${toolId}...`} />
      </div>
    </div>
  );
};

function App() {
  return (
    <Routes>
      <Route path="/*" element={<MainWindow />} />
      <Route path="/tool/:toolId" element={<ToolWindow />} />
    </Routes>
  );
}

export default App;
