import { open } from "@tauri-apps/plugin-dialog";

function FolderSelector({ folder, onSelect }) {
  const handleSelect = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Screenshot Folder",
      });
      if (selected) {
        onSelect(selected);
      }
    } catch (err) {
      console.error("Failed to open dialog:", err);
    }
  };

  return (
    <div className="folder-selector">
      <div className="folder-display">
        {folder || "No folder selected"}
      </div>
      <button onClick={handleSelect} className="select-btn">
        Browse Folder
      </button>
    </div>
  );
}

export default FolderSelector;
