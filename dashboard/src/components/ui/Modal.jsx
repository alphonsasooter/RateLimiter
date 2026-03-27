export default function Modal({ open, onClose, title, children }) {
  if (!open) return null;
  return (
    <div style={styles.overlay} onClick={onClose}>
      <div style={styles.box} onClick={(e) => e.stopPropagation()}>
        <div style={styles.header}>
          <h3 style={styles.title}>{title}</h3>
          <button style={styles.close} onClick={onClose}>✕</button>
        </div>
        {children}
      </div>
    </div>
  );
}

const styles = {
  overlay: {
    position: "fixed", inset: 0,
    background: "rgba(0,0,0,0.6)",
    display: "flex", alignItems: "center", justifyContent: "center",
    zIndex: 200,
  },
  box: {
    background: "#1e293b",
    border: "1px solid #334155",
    borderRadius: 14,
    padding: 28,
    width: "100%",
    maxWidth: 460,
  },
  header: {
    display: "flex", justifyContent: "space-between",
    alignItems: "center", marginBottom: 20,
  },
  title: { color: "#f1f5f9", fontSize: 16, fontWeight: 600 },
  close: {
    background: "none", border: "none",
    color: "#64748b", fontSize: 18, cursor: "pointer",
  },
};