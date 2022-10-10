import { app, BrowserWindow, session, ipcMain } from 'electron';
import axios from 'axios';
import { API, API_BASE, TOKEN } from './config/index'
import { ApiMessage } from './core/types'
import { existsSync } from 'fs'
const filters = {
  urls: []
}

const createWindow = () => {
  const isDev = process.env.ENV_NAME === 'dev';
  console.log(`isDev ${isDev}`);
  const win = new BrowserWindow({
    width: 1200,
    height: 800,
    titleBarStyle: 'hiddenInset',
    fullscreen: true,
    webPreferences: {
      nodeIntegration: true,
      webviewTag: true,
      contextIsolation: false
    }
  });
  if (isDev) {
    win.loadURL('http://localhost:1234')
    // win.webContents.openDevTools();
  } else {
    win.loadFile('dist/index.html');
  }
  return win;
};

app.whenReady().then(() => {
  let win = createWindow();
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      win = createWindow();
    }
  });
  const ses = session.fromPartition('webview');
  ses.webRequest.onBeforeRequest(filters, (details, callback) => {
    const res = details.url.match(/https(\S)*\.m3u8/);
    if (res) {
      win.webContents.send('capture-url', details.url)
    }
    callback({})
  })

  ipcMain.handle('m3u8', async (e, message) => {
    console.log('message', message);
    const { storagePath } = message;
    const exists = existsSync(storagePath)
    if (exists) {
      console.log(`file exists skip: ${message.title}`);
      win.webContents.send('m3u8-result', { ms: 1000 });
    } else {
      const response = await axios.post(API, message, { headers: { Authorization: TOKEN } });
      const body = response.data;
      win.webContents.send('m3u8-result', body)
      console.log(`file not exists handle : ${message.title}`);
    }
  });
  // 获取配置
  ipcMain.handle('fetch-api', async (e, message: ApiMessage) => {
    console.log('message', message);
    const { route, callback } = message;
    const response = await axios.get(`${API_BASE}/${route}`, { headers: { Authorization: TOKEN } });
    const body = response.data;
    win.webContents.send(callback, body)
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

