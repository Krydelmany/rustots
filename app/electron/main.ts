import { app, BrowserWindow, ipcMain, dialog } from 'electron';
import { spawn } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

const isDev = process.env.ELECTRON_IS_DEV === '1';

function createWindow() {
  const mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js'),
    },
  });

  if (isDev) {
    mainWindow.loadURL('http://localhost:5173');
    mainWindow.webContents.openDevTools();
  } else {
    mainWindow.loadFile(path.join(__dirname, '../build/index.html'));
  }
}

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

// IPC handlers
ipcMain.handle('analyze-code', async (event, code: string) => {
  return new Promise((resolve, reject) => {
    const rustotsBinary = path.join(__dirname, '../../core/target/release/rustots');
    
    // Check if binary exists, fallback to debug version
    const binaryPath = fs.existsSync(rustotsBinary) 
      ? rustotsBinary 
      : path.join(__dirname, '../../core/target/debug/rustots');
    
    const rustots = spawn(binaryPath, ['--lex', '--stdin'], {
      stdio: ['pipe', 'pipe', 'pipe']
    });

    let output = '';
    let error = '';

    rustots.stdout.on('data', (data) => {
      output += data.toString();
    });

    rustots.stderr.on('data', (data) => {
      error += data.toString();
    });

    rustots.on('close', (code) => {
      if (code === 0) {
        try {
          const result = JSON.parse(output);
          resolve(result);
        } catch (e) {
          reject(new Error(`Failed to parse output: ${e.message}`));
        }
      } else {
        reject(new Error(`Process exited with code ${code}: ${error}`));
      }
    });

    rustots.on('error', (err) => {
      reject(new Error(`Failed to spawn process: ${err.message}`));
    });

    // Send code to stdin
    rustots.stdin.write(code);
    rustots.stdin.end();
  });
});

ipcMain.handle('open-file', async () => {
  const result = await dialog.showOpenDialog({
    properties: ['openFile'],
    filters: [
      { name: 'TypeScript', extensions: ['ts', 'tsx'] },
      { name: 'JavaScript', extensions: ['js', 'jsx'] },
      { name: 'All Files', extensions: ['*'] }
    ]
  });

  if (!result.canceled && result.filePaths.length > 0) {
    const filePath = result.filePaths[0];
    const content = fs.readFileSync(filePath, 'utf-8');
    return { filePath, content };
  }

  return null;
});

ipcMain.handle('save-file', async (event, content: string, filePath?: string) => {
  let targetPath = filePath;

  if (!targetPath) {
    const result = await dialog.showSaveDialog({
      filters: [
        { name: 'TypeScript', extensions: ['ts'] },
        { name: 'JavaScript', extensions: ['js'] },
        { name: 'All Files', extensions: ['*'] }
      ]
    });

    if (result.canceled) {
      return null;
    }

    targetPath = result.filePath;
  }

  if (targetPath) {
    fs.writeFileSync(targetPath, content, 'utf-8');
    return targetPath;
  }

  return null;
});
