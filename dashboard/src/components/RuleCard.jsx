import { Trash2, RotateCcw, Clock, Zap } from "lucide-react";

export default function RuleCard({ rule, onDelete, onReset }) {
  return (
    <div style={styles.card}>
      <div style={styles.header}>
        <span style={styles.key}>{rule.key}</span>
        <span style={{
          ...styles.badge,
          background: rule.algorithm === "token_bucket" ? "#312e81" : "#164e63",
          color: rule.algorithm === "token_bucket" ? "#a5b4fc" : "#67e8f9",
        }}>
          {rule.algorithm === "token_bucket" ? "Token Bucket" : "Fixed Window"}
        </span>
      </div>

      <div style={styles.stats}>
        <div style={styles.stat}>
          <Zap size={14} color="#f59e0b" />
          <span>{rule.max_requests} req</span>
        </div>
        <div style={styles.stat}>
          <Clock size={14} color="#6366f1" />
          <span>{rule.window_secs}s window</span>
        </div>
        {rule.burst && (
          <div style={styles.stat}>
            <Zap size={14} color="#22c55e" />
            <span>burst: {rule.burst}</span>
          </div>
        )}
      </div>

      <div style={styles.id}>ID: {rule.id}</div>

      <div style={styles.actions}>
        <button style={styles.resetBtn} onClick={() => onReset(rule.key)}>
          <RotateCcw size={14} /> Reset
        </button>
        <button style={styles.deleteBtn} onClick={() => onDelete(rule.id)}>
          <Trash2 size={14} /> Delete
        </button>
      </div>
    </div>
  );
}

const styles = {
  card: {
    background: "#1e293b",
    border: "1px solid #334155",
    borderRadius: 12,
    padding: 20,
    display: "flex",
    flexDirection: "column",
    gap: 12,
  },
  header: { display: "flex", justifyContent: "space-between", alignItems: "center" },
  key: { color: "#f1f5f9", fontWeight: 600, fontSize: 15 },
  badge: { borderRadius: 999, padding: "2px 10px", fontSize: 11, fontWeight: 600 },
  stats: { display: "flex", gap: 16 },
  stat: { display: "flex", alignItems: "center", gap: 5, color: "#94a3b8", fontSize: 13 },
  id: { color: "#475569", fontSize: 11, fontFamily: "monospace" },
  actions: { display: "flex", gap: 8 },
  resetBtn: {
    display: "flex", alignItems: "center", gap: 5,
    padding: "6px 12px", borderRadius: 6, border: "1px solid #334155",
    background: "transparent", color: "#94a3b8", cursor: "pointer", fontSize: 13,
  },
  deleteBtn: {
    display: "flex", alignItems: "center", gap: 5,
    padding: "6px 12px", borderRadius: 6, border: "1px solid #7f1d1d",
    background: "transparent", color: "#ef4444", cursor: "pointer", fontSize: 13,
  },
};