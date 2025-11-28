"use strict";
const electron = require("electron");
const api = {
  analyze: (code) => electron.ipcRenderer.invoke("analyze-code", code),
  openFile: () => electron.ipcRenderer.invoke("open-file"),
  saveFile: (content, filePath) => electron.ipcRenderer.invoke("save-file", content, filePath)
};
electron.contextBridge.exposeInMainWorld("api", api);
