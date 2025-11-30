import React, { useState, useEffect } from 'react';
import { HashRouter as Router, Routes, Route, Link, useLocation, useParams, Navigate, useNavigate } from 'react-router-dom';
import { Database, X } from 'lucide-react';
import Editor from './components/Editor';
import TopMenu from './components/TopMenu';
import ResearchView from './components/ResearchView';
import Hierarchy from './components/tools/Hierarchy';
import Codex from './components/tools/Codex';
import Mindmap from './components/tools/Mindmap';
import Brainstorm from './components/tools/Brainstorm';
import Plot from './components/tools/Plot';
import Notes from './components/tools/Notes';
import ConceptMap from './components/tools/ConceptMap';
import FlowChart from './components/tools/FlowChart';
import { db, log } from './api/ipc';
import TitleBar from './components/TitleBar';
import ResizeHandles from './components/ResizeHandles';
import { ThemeProvider } from './contexts/ThemeContext';
import { StatsProvider } from './contexts/StatsContext';
import './styles/main.css';

const MainWindow = () => {
  const [dbStatus, setDbStatus] = useState('Connecting...')
  const navigate = useNavigate(); // Need to move Router up or use specific hook if MainWindow is inside Router.
  // MainWindow is inside Router in App component. Wait, App renders Routes. MainWindow is a component rendered by Route.
  // So MainWindow is inside Router context.

  // So MainWindow is inside Router context.

  useEffect(() => {
    const handleOpenDocument = (e) => {
      const docId = e.detail;
      log(`[MainWindow] Navigating to document: ${docId}`);
      navigate(`/manuscript?docId=${docId}`);
    };

    window.addEventListener('open-document', handleOpenDocument);
    return () => window.removeEventListener('open-document', handleOpenDocument);
  }, [navigate]);

  useEffect(() => {
    async function checkDb() {
      try {
        log('Checking database connection...')
        // Check connection by querying sqlite_master

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
      <ResizeHandles />
      <TopMenu />
      <div className="flex-1 flex flex-col overflow-hidden" style={{ WebkitAppRegion: 'no-drag' }}>
        <div className="flex-1 overflow-auto" style={{ backgroundColor: 'var(--bg-primary)' }}>
          <Routes>
            <Route path="/" element={<Navigate to="/manuscript" replace />} />
            <Route path="/manuscript" element={<Editor documentId="manuscript" trackGlobalStats={true} />} />
            <Route path="/tools/hierarchy" element={<Hierarchy />} />
            <Route path="/tools/codex" element={<Codex />} />
            <Route path="/tools/plot" element={<Plot />} />
            <Route path="/tools/notes" element={<Notes />} />
            <Route path="/tools/research" element={<ResearchView />} />
            <Route path="/tools/mindmap" element={<Mindmap />} />
            <Route path="/tools/brainstorm" element={<Brainstorm />} />
            <Route path="/tools/concept-map" element={<ConceptMap />} />
            <Route path="/tools/flow-chart" element={<FlowChart />} />
          </Routes>
        </div>
        <StatusBar dbStatus={dbStatus} />
      </div>
    </div>
  );
};


const ToolWindow = ({ toolId: propToolId }) => {
  const { toolId: paramsToolId } = useParams();
  const toolId = propToolId || paramsToolId;

  const tools = {
    hierarchy: Hierarchy,
    codex: Codex,
    plot: Plot,
    notes: Notes,
    research: ResearchView,
    mindmap: Mindmap,
    brainstorm: Brainstorm,
    'concept-map': ConceptMap,
    'flow-chart': FlowChart
  };

  const ToolComponent = tools[toolId];

  return (
    <div className="app-container flex-col">
      <ResizeHandles />
      <TitleBar title={`Tool: ${toolId}`} />
      <div className="main-content flex-1 overflow-hidden relative" style={{ WebkitAppRegion: 'no-drag' }}>
        {ToolComponent ? (
          <ToolComponent />
        ) : (
          <div className="p-4">Tool not found: {toolId}</div>
        )}
      </div>
    </div>
  );
};

const StatusBar = ({ dbStatus, wordCount }) => { // wordCount is now passed as prop
  // const { wordCount } = useStats(); // No longer needed here, passed as prop

  return (
    <div className="p-2 border-t flex items-center select-none" style={{
      backgroundColor: 'var(--bg-secondary)',
      borderColor: 'var(--border-color)',
      paddingLeft: '50px',
      gap: '30px',
      WebkitAppRegion: 'no-drag'
    }}>
      <div className="flex items-center gap-2 text-sm" style={{ color: 'var(--text-secondary)' }}>
        <Database size={14} className="text-blue-400" />
        <span>{dbStatus},</span>
      </div>
      <div className="flex items-center gap-2 text-sm" style={{ color: 'var(--text-secondary)' }}>
        <span>Manuscript Words: {wordCount}</span>
      </div>
    </div>
  );
};

class ErrorBoundary extends React.Component {
  constructor(props) {
    super(props);
    this.state = { hasError: false, error: null, errorInfo: null };
  }

  static getDerivedStateFromError(error) {
    return { hasError: true, error };
  }

  componentDidCatch(error, errorInfo) {
    console.error("Uncaught error:", error, errorInfo);
    log(`[Frontend Error] ${error.toString()} ${JSON.stringify(errorInfo)}`);
    this.setState({ error, errorInfo });
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="p-8 text-red-500">
          <h1 className="text-2xl font-bold mb-4">Something went wrong.</h1>
          <pre className="bg-gray-100 p-4 rounded overflow-auto">
            {this.state.error && this.state.error.toString()}
            <br />
            {this.state.errorInfo && this.state.errorInfo.componentStack}
          </pre>
        </div>
      );
    }

    return this.props.children;
  }
}

function App() {
  const location = useLocation();
  // Check window.location.search (browser URL) first, then hash search
  const browserSearchParams = new URLSearchParams(window.location.search);
  const hashSearchParams = new URLSearchParams(location.search);
  const toolId = browserSearchParams.get('tool') || hashSearchParams.get('tool') || (location.pathname.startsWith('/tool/') ? location.pathname.split('/')[2] : null);

  if (toolId && !location.pathname.startsWith('/tool/')) {
    return (
      <ErrorBoundary>
        <ThemeProvider>
          <StatsProvider>
            <Routes>
              <Route path="*" element={<Navigate to={`/tool/${toolId}`} replace />} />
            </Routes>
          </StatsProvider>
        </ThemeProvider>
      </ErrorBoundary>
    );
  }

  // Force ToolWindow if toolId is present
  if (toolId) {
    return (
      <ErrorBoundary>
        <ThemeProvider>
          <StatsProvider>
            <ToolWindow toolId={toolId} />
          </StatsProvider>
        </ThemeProvider>
      </ErrorBoundary>
    );
  }

  return (
    <ErrorBoundary>
      <ThemeProvider>
        <StatsProvider>
          <Routes>
            <Route path="/*" element={<MainWindow />} />
          </Routes>
        </StatsProvider>
      </ThemeProvider>
    </ErrorBoundary>
  );
}

export default App;
