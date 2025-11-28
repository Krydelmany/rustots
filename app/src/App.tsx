import { useState, useCallback, useEffect } from 'react';
import Editor from '@monaco-editor/react';
import { TokensView, Token } from './components/TokensView';
import { ASTView } from './components/ASTView';
import { FolderOpen, Save, Play } from 'lucide-react';

interface AnalysisResult {
  tokens: Token[];
  ast: any;
}

type Tab = 'tokens' | 'ast';

function App() {
  const [code, setCode] = useState(`function hello(name: string): string {
  return "Hello, " + name;
}

const result = hello("TypeScript");
console.log(result);`);

  const [analysisResult, setAnalysisResult] = useState<AnalysisResult | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [currentFile, setCurrentFile] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<Tab>('tokens');
  const [panelWidth, setPanelWidth] = useState(750);
  const [isResizing, setIsResizing] = useState(false);

  const analyzeCode = useCallback(async () => {
    if (!window.api) {
      console.error('API not available');
      return;
    }

    setIsAnalyzing(true);
    try {
      const result = await window.api.analyze(code);
      setAnalysisResult(result);
    } catch (error) {
      console.error('Analysis failed:', error);
    } finally {
      setIsAnalyzing(false);
    }
  }, [code]);

  const openFile = useCallback(async () => {
    if (!window.api) return;

    try {
      const result = await window.api.openFile();
      if (result) {
        setCode(result.content);
        setCurrentFile(result.filePath);
      }
    } catch (error) {
      console.error('Failed to open file:', error);
    }
  }, []);

  const saveFile = useCallback(async () => {
    if (!window.api) return;

    try {
      const result = await window.api.saveFile(code, currentFile || undefined);
      if (result) {
        setCurrentFile(result);
      }
    } catch (error) {
      console.error('Failed to save file:', error);
    }
  }, [code, currentFile]);

  const handleMouseDown = useCallback(() => {
    setIsResizing(true);
  }, []);

  const handleMouseUp = useCallback(() => {
    setIsResizing(false);
  }, []);

  const handleMouseMove = useCallback((e: MouseEvent) => {
    if (isResizing) {
      const newWidth = window.innerWidth - e.clientX;
      if (newWidth >= 300 && newWidth <= 800) {
        setPanelWidth(newWidth);
      }
    }
  }, [isResizing]);

  useEffect(() => {
    if (isResizing) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
      };
    }
  }, [isResizing, handleMouseMove, handleMouseUp]);

  return (
    <div className="flex flex-col h-screen bg-black text-gray-100 overflow-hidden">
      <div className="border-b border-neutral-800 bg-neutral-950 px-6 py-3 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <h1 className="text-base font-semibold text-gray-200 tracking-tight">
            Rustots
          </h1>
          <div className="h-4 w-px bg-neutral-800"></div>
          <div className="flex gap-2">
            <button
              onClick={openFile}
              className="px-3 py-1.5 bg-neutral-900 hover:bg-neutral-800 border border-neutral-800 hover:border-neutral-700 rounded text-xs font-medium transition-all flex items-center gap-2 text-gray-400 hover:text-gray-200"
            >
              <FolderOpen className="w-3.5 h-3.5" />
              Abrir
            </button>
            <button
              onClick={saveFile}
              disabled={!code}
              className="px-3 py-1.5 bg-neutral-900 hover:bg-neutral-800 border border-neutral-800 hover:border-neutral-700 rounded text-xs font-medium transition-all flex items-center gap-2 disabled:opacity-40 disabled:cursor-not-allowed text-gray-400 hover:text-gray-200"
            >
              <Save className="w-3.5 h-3.5" />
              Salvar
            </button>
          </div>
        </div>
        <div className="flex items-center gap-4">
          {currentFile && (
            <span className="text-xs text-gray-500 font-mono max-w-[300px] truncate">
              {currentFile}
            </span>
          )}
          <button
            onClick={analyzeCode}
            disabled={isAnalyzing}
            className="px-4 py-1.5 bg-gray-200 hover:bg-white text-black disabled:bg-neutral-800 disabled:text-gray-500 disabled:cursor-not-allowed rounded text-xs font-semibold transition-all flex items-center gap-2"
          >
            {isAnalyzing ? (
              <>
                <div className="w-3 h-3 border-2 border-gray-600 border-t-transparent rounded-full animate-spin" />
                Analisando...
              </>
            ) : (
              <>
                <Play className="w-3.5 h-3.5" fill="currentColor" />
                Analisar
              </>
            )}
          </button>
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        <div className="flex-1 flex flex-col min-w-[400px]">
          <Editor
            height="100%"
            defaultLanguage="typescript"
            value={code}
            onChange={(value) => setCode(value || '')}
            theme="vs-dark"
            beforeMount={(monaco) => {
              monaco.editor.defineTheme('rustots-dark', {
                base: 'vs-dark',
                inherit: true,
                rules: [],
                colors: {
                  'editor.background': '#000000',
                  'editor.foreground': '#e5e5e5',
                  'editorLineNumber.foreground': '#525252',
                  'editorLineNumber.activeForeground': '#a3a3a3',
                  'editor.lineHighlightBackground': '#0a0a0a',
                  'editorCursor.foreground': '#ffffff',
                  'editor.selectionBackground': '#262626',
                  'editor.inactiveSelectionBackground': '#171717',
                }
              });
            }}
            onMount={(_editor, monaco) => {
              monaco.editor.setTheme('rustots-dark');
            }}
            options={{
              minimap: { enabled: false },
              fontSize: 13,
              lineNumbers: 'on',
              automaticLayout: true,
              tabSize: 2,
              insertSpaces: true,
              padding: { top: 16, bottom: 16 },
              fontFamily: "'JetBrains Mono', 'Fira Code', Consolas, monospace",
              fontLigatures: true,
            }}
          />
        </div>

        <div
          className="w-1 bg-neutral-900 hover:bg-neutral-700 cursor-col-resize transition-colors relative group"
          onMouseDown={handleMouseDown}
        >
          <div className="absolute inset-y-0 -left-1 -right-1 group-hover:bg-neutral-700/20"></div>
        </div>

        <div
          className="flex flex-col bg-neutral-950"
          style={{ width: `${panelWidth}px`, minWidth: '300px' }}
        >
          <div className="flex border-b border-neutral-900 bg-black">
            <button
              onClick={() => setActiveTab('tokens')}
              className={`flex-1 px-4 py-2.5 text-xs font-medium transition-all ${activeTab === 'tokens'
                ? 'text-gray-200 bg-neutral-950 border-b-2 border-gray-200'
                : 'text-gray-500 hover:text-gray-300 border-b-2 border-transparent'
                }`}
            >
              Tokens
            </button>
            <button
              onClick={() => setActiveTab('ast')}
              className={`flex-1 px-4 py-2.5 text-xs font-medium transition-all ${activeTab === 'ast'
                ? 'text-gray-200 bg-neutral-950 border-b-2 border-gray-200'
                : 'text-gray-500 hover:text-gray-300 border-b-2 border-transparent'
                }`}
            >
              AST
            </button>
          </div>

          <div className="flex-1 overflow-hidden bg-black">
            {activeTab === 'tokens' ? (
              <TokensView tokens={analysisResult?.tokens} />
            ) : (
              <ASTView ast={analysisResult?.ast} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
