import { contextBridge, ipcRenderer } from 'electron';

interface API {
  analyze: (code: string) => Promise<any>;
  openFile: () => Promise<{ filePath: string; content: string } | null>;
  saveFile: (content: string, filePath?: string) => Promise<string | null>;
}

const api: API = {
  analyze: (code: string) => ipcRenderer.invoke('analyze-code', code),
  openFile: () => ipcRenderer.invoke('open-file'),
  saveFile: (content: string, filePath?: string) => ipcRenderer.invoke('save-file', content, filePath),
};

contextBridge.exposeInMainWorld('api', api);

// Type declaration for the renderer process
declare global {
  interface Window {
    api: API;
  }
}
