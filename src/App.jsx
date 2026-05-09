import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import FolderSelector from "./components/FolderSelector";
import "./styles/main.css";

function App() {
  const [folder, setFolder] = useState("");
  const [enabled, setEnabled] = useState(true);
  const [copyMode, setCopyMode] = useState("path");

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
        Automatically copies both the image and the file path to your clipboard.
      </p>
    </div>
  );
}

export default App;
