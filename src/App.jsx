import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import FolderSelector from "./components/FolderSelector";
import "./styles/main.css";

function App() {
  const [folder, setFolder] = useState("");
  const [enabled, setEnabled] = useState(true);
  const [copyMode, setCopyMode] = useState("path"); // "path" or "image"

  useEffect(() => {
    invoke("get_config").then((config) => {
      setFolder(config.folder_path);
      setEnabled(config.enabled);
      setCopyMode(config.copy_mode || "path");
    });
  }, []);

  const saveConfig = (newFolder, newEnabled, newMode) => {
    invoke("update_config", { config: { folder_path: newFolder, enabled: newEnabled, copy_mode: newMode } });
  };

  const handleFolderChange = (newFolder) => {
    setFolder(newFolder);
    saveConfig(newFolder, enabled, copyMode);
  };

  return (
    <div className="container">
      <h1>shotpath</h1>
      <div className="status">
        Status: <span className={enabled ? "active" : "inactive"}>{enabled ? "Monitoring" : "Paused"}</span>
      </div>
      
      <div className="field">
        <label>Screenshot Folder:</label>
        <FolderSelector folder={folder} onSelect={handleFolderChange} />
      </div>

      <div className="field">
        <label>Copy Mode:</label>
        <div className="toggle-group">
          <button 
            className={copyMode === "path" ? "selected" : ""} 
            onClick={() => { setCopyMode("path"); saveConfig(folder, enabled, "path"); }}
          >Path</button>
          <button 
            className={copyMode === "image" ? "selected" : ""} 
            onClick={() => { setCopyMode("image"); saveConfig(folder, enabled, "image"); }}
          >Image</button>
        </div>
      </div>

      <div className="field">
        <label className="switch">
          <input 
            type="checkbox" 
            checked={enabled} 
            onChange={(e) => {
              setEnabled(e.target.checked);
              saveConfig(folder, e.target.checked, copyMode);
            }}
          />
          Enable Monitoring
        </label>
      </div>

      <p className="hint">
        {copyMode === "path" 
          ? "Copies file path (best for CLI/Programming)." 
          : "Copies actual image (best for Discord/Slack/Word)."}
      </p>
    </div>
  );
}

export default App;
