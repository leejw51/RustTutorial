const { app, BrowserWindow } = require("electron");
const { exec } = require("child_process");
const execa = require("execa");

if (require("electron-squirrel-startup")) {
  app.quit();
}

function run_program() {
  (async () => {
    const { stdout } = await execa("/usr/local/bin/client-rpc", [
      "--network-id",
      "ab"
    ]);
  })();
}

let mainWindow;

const createWindow = () => {
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600
  });

  run_program();

  mainWindow.loadURL(`file://${__dirname}/index.html`);

  mainWindow.webContents.openDevTools();

  mainWindow.on("closed", () => {
    mainWindow = null;
  });
};

app.on("ready", createWindow);

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (mainWindow === null) {
    createWindow();
  }
});
