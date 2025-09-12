import React, { useState, useCallback } from 'react';
import Editor from '@monaco-editor/react';

interface Token {
  type: string;
  value: string;
  position: {
    start: number;
    end: number;
    line: number;
    column: number;
  };
}

interface Diagnostic {
  level: string;
  message: string;
  location: {
    line: number;
    column: number;
    length: number;
  };
  code?: string;
}

interface AnalysisResult {
  tokens: Token[];
  diagnostics: Diagnostic[];
}

function App() {
  const [code, setCode] = useState(`function hello(name: string): string {
  return "Hello, " + name;
}

const result = hello("TypeScript");
console.log(result);`);
  
  const [analysisResult, setAnalysisResult] = useState<AnalysisResult | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [currentFile, setCurrentFile] = useState<string | null>(null);

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
      // Show error in UI
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

  return (
    <div className="flex h-screen bg-gray-900 text-white">
      {/* Sidebar */}
      <div className="w-80 bg-gray-800 flex flex-col">
        {/* Header */}
        <div className="p-4 border-b border-gray-700">
          <h1 className="text-xl font-bold">TypeScript Analyzer</h1>
          <div className="flex gap-2 mt-2">
            <button
              onClick={openFile}
              className="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm"
            >
              Open
            </button>
            <button
              onClick={saveFile}
              className="px-3 py-1 bg-green-600 hover:bg-green-700 rounded text-sm"
            >
              Save
            </button>
            <button
              onClick={analyzeCode}
              disabled={isAnalyzing}
              className="px-3 py-1 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 rounded text-sm"
            >
              {isAnalyzing ? 'Analyzing...' : 'Analyze'}
            </button>
          </div>
          {currentFile && (
            <div className="mt-2 text-xs text-gray-400 truncate">
              {currentFile}
            </div>
          )}
        </div>

        {/* Tokens */}
        <div className="flex-1 overflow-auto">
          <div className="p-4">
            <h3 className="font-semibold mb-2">Tokens</h3>
            {analysisResult?.tokens ? (
              <div className="space-y-1">
                {analysisResult.tokens
                  .filter(token => token.type !== 'whitespace' && token.type !== 'newline')
                  .map((token, index) => (
                  <div
                    key={index}
                    className="p-2 bg-gray-700 rounded text-xs"
                  >
                    <div className="flex justify-between">
                      <span className={`px-1 rounded text-xs ${
                        token.type === 'keyword' ? 'bg-purple-600' :
                        token.type === 'identifier' ? 'bg-blue-600' :
                        token.type === 'literal' ? 'bg-green-600' :
                        token.type === 'operator' ? 'bg-yellow-600' :
                        token.type === 'punctuation' ? 'bg-gray-600' :
                        'bg-gray-500'
                      }`}>
                        {token.type}
                      </span>
                      <span className="text-gray-300">
                        {token.position.line}:{token.position.column}
                      </span>
                    </div>
                    <div className="mt-1 font-mono text-white">
                      {token.value}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-gray-400 text-sm">
                Click "Analyze" to see tokens
              </div>
            )}
          </div>

          {/* Diagnostics */}
          {analysisResult?.diagnostics && analysisResult.diagnostics.length > 0 && (
            <div className="p-4 border-t border-gray-700">
              <h3 className="font-semibold mb-2">Diagnostics</h3>
              <div className="space-y-1">
                {analysisResult.diagnostics.map((diagnostic, index) => (
                  <div
                    key={index}
                    className={`p-2 rounded text-xs ${
                      diagnostic.level === 'error' ? 'bg-red-900 border border-red-600' :
                      diagnostic.level === 'warning' ? 'bg-yellow-900 border border-yellow-600' :
                      'bg-blue-900 border border-blue-600'
                    }`}
                  >
                    <div className="flex justify-between">
                      <span className="capitalize font-semibold">
                        {diagnostic.level}
                      </span>
                      <span className="text-gray-300">
                        {diagnostic.location.line}:{diagnostic.location.column}
                      </span>
                    </div>
                    <div className="mt-1">
                      {diagnostic.message}
                    </div>
                    {diagnostic.code && (
                      <div className="mt-1 text-gray-400">
                        Code: {diagnostic.code}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Editor */}
      <div className="flex-1 flex flex-col">
        <div className="flex-1">
          <Editor
            height="100%"
            defaultLanguage="typescript"
            value={code}
            onChange={(value) => setCode(value || '')}
            theme="vs-dark"
            options={{
              minimap: { enabled: false },
              fontSize: 14,
              lineNumbers: 'on',
              automaticLayout: true,
              tabSize: 2,
              insertSpaces: true,
            }}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
