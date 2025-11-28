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

  if (process.env.VITE_DEV_SERVER_URL) {
    mainWindow.loadURL(process.env.VITE_DEV_SERVER_URL);
    mainWindow.webContents.openDevTools();
  } else {
    mainWindow.loadFile(path.join(__dirname, '../build/index.html'));
  }
}

app.disableHardwareAcceleration();
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
    const isWindows = process.platform === 'win32';
    const extension = isWindows ? '.exe' : '';

    const rustotsBinary = path.join(__dirname, `../../core/target/release/rustots${extension}`);
    const debugBinary = path.join(__dirname, `../../core/target/debug/rustots${extension}`);

    console.log('__dirname:', __dirname);
    console.log('Checking release binary:', rustotsBinary);
    console.log('Checking debug binary:', debugBinary);

    // Check if binary exists, fallback to debug version
    let binaryPath = '';
    if (fs.existsSync(rustotsBinary)) {
      binaryPath = rustotsBinary;
      console.log('Found release binary');
    } else if (fs.existsSync(debugBinary)) {
      binaryPath = debugBinary;
      console.log('Found debug binary');
    } else {
      console.error('Binary not found in release or debug paths');
      reject(new Error(`Rust binary not found. Checked:\n${rustotsBinary}\n${debugBinary}`));
      return;
    }

    // Write code to a temporary file to avoid stdin pipe issues on Windows
    const tempFilePath = path.join(app.getPath('userData'), 'temp_analysis.ts');
    try {
      fs.writeFileSync(tempFilePath, code);
    } catch (e) {
      reject(new Error(`Failed to create temp file: ${e.message}`));
      return;
    }

    // Pass the file path as an argument instead of using --stdin
    const rustots = spawn(binaryPath, [tempFilePath], {
      stdio: ['ignore', 'pipe', 'pipe']
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
      // Clean up temp file
      try {
        if (fs.existsSync(tempFilePath)) {
          fs.unlinkSync(tempFilePath);
        }
      } catch (e) {
        console.error('Failed to delete temp file:', e);
      }

      if (code === 0) {
        try {
          const result = JSON.parse(output);
          resolve(result);
        } catch (e) {
          console.error('Parse error:', e);
          reject(new Error(`Failed to parse output: ${e.message}`));
        }
      } else {
        console.error('Process failed with code', code, 'Error:', error);
        reject(new Error(`Process exited with code ${code}: ${error}`));
      }
    });

    rustots.on('error', (err) => {
      console.error('Spawn error:', err);
      reject(new Error(`Failed to spawn process: ${err.message}`));
    });

    // No need to write to stdin anymore

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
