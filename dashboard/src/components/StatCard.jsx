export default function StatCard({ label, value, icon: Icon, color = "#6366f1" }) {
  return (
    <div style={styles.card}>
      <div style={styles.top}>
        <span style={styles.label}>{label}</span>

        {Icon && <Icon size={18} color={color} />}

      </div>

      <div style={{ ...styles.value, color }}>
        {value}
      </div>
    </div>
  );
}

const styles = {
  card: {
    background: "#1e293b",
    borderRadius: 12,
    padding: "20px 24px",
    flex: 1,
    minWidth: 160,
    border: "1px solid #334155",
  },

  top: {
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    marginBottom: 12,
  },

  label: {
    color: "#94a3b8",
    fontSize: 13,
  },

  value: {
    fontSize: 28,
    fontWeight: 700,
  },
};