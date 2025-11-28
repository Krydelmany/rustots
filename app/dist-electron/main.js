"use strict";
const electron = require("electron");
const child_process = require("child_process");
const path = require("path");
const fs = require("fs");
function _interopNamespaceDefault(e) {
  const n = Object.create(null, { [Symbol.toStringTag]: { value: "Module" } });
  if (e) {
    for (const k in e) {
      if (k !== "default") {
        const d = Object.getOwnPropertyDescriptor(e, k);
        Object.defineProperty(n, k, d.get ? d : {
          enumerable: true,
          get: () => e[k]
        });
      }
    }
  }
  n.default = e;
  return Object.freeze(n);
}
const path__namespace = /* @__PURE__ */ _interopNamespaceDefault(path);
const fs__namespace = /* @__PURE__ */ _interopNamespaceDefault(fs);
process.env.ELECTRON_IS_DEV === "1";
function createWindow() {
  const mainWindow = new electron.BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path__namespace.join(__dirname, "preload.js")
    }
  });
  if (process.env.VITE_DEV_SERVER_URL) {
    mainWindow.loadURL(process.env.VITE_DEV_SERVER_URL);
    mainWindow.webContents.openDevTools();
  } else {
    mainWindow.loadFile(path__namespace.join(__dirname, "../build/index.html"));
  }
}
electron.app.disableHardwareAcceleration();
electron.app.whenReady().then(createWindow);
electron.app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    electron.app.quit();
  }
});
electron.app.on("activate", () => {
  if (electron.BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});
electron.ipcMain.handle("analyze-code", async (event, code) => {
  return new Promise((resolve, reject) => {
    const isWindows = process.platform === "win32";
    const extension = isWindows ? ".exe" : "";
    const rustotsBinary = path__namespace.join(__dirname, `../../core/target/release/rustots${extension}`);
    const debugBinary = path__namespace.join(__dirname, `../../core/target/debug/rustots${extension}`);
    console.log("__dirname:", __dirname);
    console.log("Checking release binary:", rustotsBinary);
    console.log("Checking debug binary:", debugBinary);
    let binaryPath = "";
    if (fs__namespace.existsSync(rustotsBinary)) {
      binaryPath = rustotsBinary;
      console.log("Found release binary");
    } else if (fs__namespace.existsSync(debugBinary)) {
      binaryPath = debugBinary;
      console.log("Found debug binary");
    } else {
      console.error("Binary not found in release or debug paths");
      reject(new Error(`Rust binary not found. Checked:
${rustotsBinary}
${debugBinary}`));
      return;
    }
    const tempFilePath = path__namespace.join(electron.app.getPath("userData"), "temp_analysis.ts");
    try {
      fs__namespace.writeFileSync(tempFilePath, code);
    } catch (e) {
      reject(new Error(`Failed to create temp file: ${e.message}`));
      return;
    }
    const rustots = child_process.spawn(binaryPath, [tempFilePath], {
      stdio: ["ignore", "pipe", "pipe"]
    });
    let output = "";
    let error = "";
    rustots.stdout.on("data", (data) => {
      output += data.toString();
    });
    rustots.stderr.on("data", (data) => {
      error += data.toString();
    });
    rustots.on("close", (code2) => {
      try {
        if (fs__namespace.existsSync(tempFilePath)) {
          fs__namespace.unlinkSync(tempFilePath);
        }
      } catch (e) {
        console.error("Failed to delete temp file:", e);
      }
      if (code2 === 0) {
        try {
          const result = JSON.parse(output);
          resolve(result);
        } catch (e) {
          console.error("Parse error:", e);
          reject(new Error(`Failed to parse output: ${e.message}`));
        }
      } else {
        console.error("Process failed with code", code2, "Error:", error);
        reject(new Error(`Process exited with code ${code2}: ${error}`));
      }
    });
    rustots.on("error", (err) => {
      console.error("Spawn error:", err);
      reject(new Error(`Failed to spawn process: ${err.message}`));
    });
  });
});
electron.ipcMain.handle("open-file", async () => {
  const result = await electron.dialog.showOpenDialog({
    properties: ["openFile"],
    filters: [
      { name: "TypeScript", extensions: ["ts", "tsx"] },
      { name: "JavaScript", extensions: ["js", "jsx"] },
      { name: "All Files", extensions: ["*"] }
    ]
  });
  if (!result.canceled && result.filePaths.length > 0) {
    const filePath = result.filePaths[0];
    const content = fs__namespace.readFileSync(filePath, "utf-8");
    return { filePath, content };
  }
  return null;
});
electron.ipcMain.handle("save-file", async (event, content, filePath) => {
  let targetPath = filePath;
  if (!targetPath) {
    const result = await electron.dialog.showSaveDialog({
      filters: [
        { name: "TypeScript", extensions: ["ts"] },
        { name: "JavaScript", extensions: ["js"] },
        { name: "All Files", extensions: ["*"] }
      ]
    });
    if (result.canceled) {
      return null;
    }
    targetPath = result.filePath;
  }
  if (targetPath) {
    fs__namespace.writeFileSync(targetPath, content, "utf-8");
    return targetPath;
  }
  return null;
});
