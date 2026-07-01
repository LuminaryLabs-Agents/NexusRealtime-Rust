const { app, BrowserWindow } = require("electron");
const fs = require("fs");
const path = require("path");

function readManifest() {
  const manifestPath = path.join(__dirname, "app", "nexus-package-manifest.json");
  try {
    return JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  } catch {
    return { appName: "Nexus Packaged App" };
  }
}

function createWindow() {
  const manifest = readManifest();
  const win = new BrowserWindow({
    width: 1280,
    height: 800,
    title: manifest.appName || "Nexus Packaged App",
    backgroundColor: "#101418",
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true
    }
  });
  win.loadFile(path.join(__dirname, "app", "index.html"));
}

app.whenReady().then(createWindow);

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});
