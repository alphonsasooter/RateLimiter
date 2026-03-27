import { useState } from "react";
import { Plus } from "lucide-react";
import RuleCard from "../components/RuleCard";
import Modal from "../components/ui/Modal";
import { useRules, useCreateRule, useDeleteRule, useResetKey } from "../hooks/useRules";

const empty = { key: "", max_requests: 10, window_secs: 60, burst: "", algorithm: "token_bucket" };

export default function Rules() {
  const { data, isLoading } = useRules();
  const createRule = useCreateRule();
  const deleteRule = useDeleteRule();
  const resetKey = useResetKey();
  const [open, setOpen] = useState(false);
  const [form, setForm] = useState(empty);
  const [msg, setMsg] = useState("");

  const rules = data?.rules ?? [];

  function handleChange(e) {
    setForm((f) => ({ ...f, [e.target.name]: e.target.value }));
  }

  async function handleCreate() {
    if (!form.key) return setMsg("Key is required");
    try {
      await createRule.mutateAsync({
        ...form,
        max_requests: Number(form.max_requests),
        window_secs: Number(form.window_secs),
        burst: form.burst ? Number(form.burst) : undefined,
      });
      setOpen(false);
      setForm(empty);
      setMsg("");
    } catch {
      setMsg("Failed to create rule");
    }
  }

  async function handleDelete(id) {
    await deleteRule.mutateAsync(id);
  }

  async function handleReset(key) {
    await resetKey.mutateAsync(key);
  }

  return (
    <div style={styles.page}>
      <div style={styles.topbar}>
        <h2 style={styles.heading}>Rules</h2>
        <button style={styles.addBtn} onClick={() => setOpen(true)}>
          <Plus size={16} /> New Rule
        </button>
      </div>

      {isLoading && <p style={styles.muted}>Loading...</p>}

      {!isLoading && rules.length === 0 && (
        <p style={styles.muted}>No rules yet. Create one to get started.</p>
      )}

      <div style={styles.grid}>
        {rules.map((rule) => (
          <RuleCard
            key={rule.id}
            rule={rule}
            onDelete={handleDelete}
            onReset={handleReset}
          />
        ))}
      </div>

      <Modal open={open} onClose={() => setOpen(false)} title="Create Rule">
        {[
          { label: "Key (IP / user_id / api_key)", name: "key", type: "text" },
          { label: "Max Requests", name: "max_requests", type: "number" },
          { label: "Window (seconds)", name: "window_secs", type: "number" },
          { label: "Burst (optional)", name: "burst", type: "number" },
        ].map(({ label, name, type }) => (
          <div key={name} style={styles.field}>
            <label style={styles.label}>{label}</label>
            <input
              style={styles.input}
              type={type}
              name={name}
              value={form[name]}
              onChange={handleChange}
              placeholder={label}
            />
          </div>
        ))}

        <div style={styles.field}>
          <label style={styles.label}>Algorithm</label>
          <select
            style={styles.input}
            name="algorithm"
            value={form.algorithm}
            onChange={handleChange}
          >
            <option value="token_bucket">Token Bucket</option>
            <option value="fixed_window">Fixed Window</option>
          </select>
        </div>

        {msg && <p style={styles.error}>{msg}</p>}

        <button style={styles.createBtn} onClick={handleCreate}>
          Create Rule
        </button>
      </Modal>
    </div>
  );
}

const styles = {
  page: { padding: 28, display: "flex", flexDirection: "column", gap: 20 },
  topbar: { display: "flex", justifyContent: "space-between", alignItems: "center" },
  heading: { color: "#f1f5f9", fontSize: 20, fontWeight: 700 },
  addBtn: {
    display: "flex", alignItems: "center", gap: 6,
    padding: "8px 16px", borderRadius: 8,
    background: "#6366f1", color: "#fff",
    border: "none", cursor: "pointer", fontWeight: 600, fontSize: 14,
  },
  grid: { display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(300px, 1fr))", gap: 16 },
  muted: { color: "#64748b", fontSize: 14 },
  field: { marginBottom: 14 },
  label: { display: "block", color: "#94a3b8", fontSize: 13, marginBottom: 5 },
  input: {
    width: "100%", padding: "8px 12px", borderRadius: 8,
    background: "#0f172a", border: "1px solid #334155",
    color: "#f1f5f9", fontSize: 14, boxSizing: "border-box",
  },
  error: { color: "#ef4444", fontSize: 13 },
  createBtn: {
    width: "100%", padding: "10px", borderRadius: 8,
    background: "#6366f1", color: "#fff",
    border: "none", cursor: "pointer", fontWeight: 600, fontSize: 14, marginTop: 4,
  },
};