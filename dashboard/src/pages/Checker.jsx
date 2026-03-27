import { useState } from "react";
import { checkRateLimit } from "../api/stats";
import { CheckCircle, XCircle } from "lucide-react";

const empty = { key: "", max_requests: 10, window_secs: 60 };

export default function Checker() {
  const [form, setForm] = useState(empty);
  const [result, setResult] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  function handleChange(e) {
    setForm((f) => ({ ...f, [e.target.name]: e.target.value }));
  }

  async function handleCheck() {
    if (!form.key) return setError("Key is required");
    setLoading(true);
    setError("");
    try {
      const data = await checkRateLimit({
        key: form.key,
        max_requests: Number(form.max_requests),
        window_secs: Number(form.window_secs),
      });
      setResult(data);
    } catch {
      setError("Failed to reach server. Is it running?");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div style={styles.page}>
      <h2 style={styles.heading}>Rate Limit Checker</h2>
      <p style={styles.sub}>Manually test if a key is allowed or blocked.</p>

      <div style={styles.form}>
        {[
          { label: "Key", name: "key", type: "text", placeholder: "e.g. user_123" },
          { label: "Max Requests", name: "max_requests", type: "number", placeholder: "10" },
          { label: "Window (seconds)", name: "window_secs", type: "number", placeholder: "60" },
        ].map(({ label, name, type, placeholder }) => (
          <div key={name} style={styles.field}>
            <label style={styles.label}>{label}</label>
            <input
              style={styles.input}
              type={type}
              name={name}
              value={form[name]}
              onChange={handleChange}
              placeholder={placeholder}
            />
          </div>
        ))}

        {error && <p style={styles.error}>{error}</p>}

        <button style={styles.btn} onClick={handleCheck} disabled={loading}>
          {loading ? "Checking..." : "Check"}
        </button>
      </div>

      {result && (
        <div style={{
          ...styles.result,
          borderColor: result.allowed ? "#22c55e" : "#ef4444",
        }}>
          <div style={styles.resultHeader}>
            {result.allowed
              ? <CheckCircle size={28} color="#22c55e" />
              : <XCircle size={28} color="#ef4444" />}
            <span style={{
              fontSize: 20,
              fontWeight: 700,
              color: result.allowed ? "#22c55e" : "#ef4444",
            }}>
              {result.allowed ? "Allowed" : "Blocked"}
            </span>
          </div>
          <div style={styles.resultStats}>
            <div style={styles.stat}>
              <span style={styles.statLabel}>Remaining</span>
              <span style={styles.statVal}>{result.remaining}</span>
            </div>
            {result.retry_after_secs && (
              <div style={styles.stat}>
                <span style={styles.statLabel}>Retry After</span>
                <span style={styles.statVal}>{result.retry_after_secs}s</span>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

const styles = {
  page: { padding: 28, display: "flex", flexDirection: "column", gap: 24, maxWidth: 520 },
  heading: { color: "#f1f5f9", fontSize: 20, fontWeight: 700 },
  sub: { color: "#64748b", fontSize: 14, marginTop: -16 },
  form: {
    background: "#1e293b", borderRadius: 12,
    padding: 24, border: "1px solid #334155",
    display: "flex", flexDirection: "column", gap: 4,
  },
  field: { marginBottom: 12 },
  label: { display: "block", color: "#94a3b8", fontSize: 13, marginBottom: 5 },
  input: {
    width: "100%", padding: "8px 12px", borderRadius: 8,
    background: "#0f172a", border: "1px solid #334155",
    color: "#f1f5f9", fontSize: 14, boxSizing: "border-box",
  },
  error: { color: "#ef4444", fontSize: 13 },
  btn: {
    padding: "10px", borderRadius: 8,
    background: "#6366f1", color: "#fff",
    border: "none", cursor: "pointer",
    fontWeight: 600, fontSize: 14, marginTop: 4,
  },
  result: {
    background: "#1e293b", borderRadius: 12,
    padding: 24, border: "2px solid",
    display: "flex", flexDirection: "column", gap: 16,
  },
  resultHeader: { display: "flex", alignItems: "center", gap: 12 },
  resultStats: { display: "flex", gap: 32 },
  stat: { display: "flex", flexDirection: "column", gap: 4 },
  statLabel: { color: "#64748b", fontSize: 12 },
  statVal: { color: "#f1f5f9", fontSize: 22, fontWeight: 700 },
};