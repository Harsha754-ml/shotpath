import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./styles/main.css";

function App() {
  const [folder, setFolder] = useState("");
  const [enabled, setEnabled] = useState(true);

  useEffect(() => {
    invoke("get_config").then((config) => {
      setFolder(config.folder_path);
      setEnabled(config.enabled);
    });
  }, []);

  const saveConfig = (newFolder, newEnabled) => {
    invoke("update_config", { config: { folder_path: newFolder, enabled: newEnabled, copy_mode: "path" } });
  };

  const handleFolderSelect = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Screenshot Folder",
      });
      if (selected) {
        setFolder(selected);
        saveConfig(selected, enabled);
      }
    } catch (err) {
      console.error("Failed to open dialog:", err);
    }
  };

  return (
    <div className="container">
      <header className="header">
        <div className="header-icon">
          <svg viewBox="0 0 24 24"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zm-5-7l-3 3.5L9 13l-3 4h12z"/></svg>
        </div>
        <div className="header-text">
          <h1>ShotPath</h1>
          <p>Smart clipboard for screenshots</p>
        </div>
      </header>

      <div className="status-card">
        <div className="status-info">
          <div className={`status-dot ${enabled ? "active" : "inactive"}`}></div>
          <div>
            <div className="status-label">Status</div>
            <div className="status-value">{enabled ? "Monitoring" : "Paused"}</div>
          </div>
        </div>
        <div className="status-toggle">
          <label className="toggle-switch">
            <input type="checkbox" checked={enabled} onChange={(e) => {
              setEnabled(e.target.checked);
              saveConfig(folder, e.target.checked);
            }}/>
            <span className="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div className="folder-section">
        <div className="section-label">Screenshot Folder</div>
        <div className="folder-card">
          <div className="folder-icon">
            <svg viewBox="0 0 24 24" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
          </div>
          <div className="folder-info">
            <div className={`folder-path ${!folder ? "folder-placeholder" : ""}`}>
              {folder || "No folder selected"}
            </div>
          </div>
          <button className="browse-btn" onClick={handleFolderSelect}>
            Browse
          </button>
        </div>
      </div>

      <div className="features">
        <div className="feature">
          <div className="feature-icon">
            <svg viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
            </svg>
          </div>
          <span>Real-time</span>
        </div>
        <div className="feature">
          <div className="feature-icon">
            <svg viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round">
              <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
              <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
            </svg>
          </div>
          <span>Multi-format</span>
        </div>
        <div className="feature">
          <div className="feature-icon">
            <svg viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round">
              <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
            </svg>
          </div>
          <span>Lightweight</span>
        </div>
      </div>

      <footer className="footer">
        <p>Automatically copies images to clipboard<span className="version">v0.1.0</span></p>
      </footer>
    </div>
  );
}

export default App;
