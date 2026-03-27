import { Activity, Shield, CheckCircle, XCircle } from "lucide-react";
import StatCard from "../components/StatCard";
import TrafficChart from "../components/TrafficChart";
import { useHealth } from "../hooks/useStats";
import { useRules } from "../hooks/useRules";

export default function Dashboard() {
  const { data: health } = useHealth();
  const { data: rulesData } = useRules();

  const ruleCount = rulesData?.count ?? 0;
  const status = health?.status ?? "...";

  return (
    <div style={styles.page}>
      <h2 style={styles.heading}>Dashboard</h2>

      <div style={styles.cards}>
        <StatCard
          label="Server Status"
          value={status === "ok" ? "Online" : "Offline"}
          icon={Activity}
          color={status === "ok" ? "#22c55e" : "#ef4444"}
        />
        <StatCard label="Active Rules" value={ruleCount} icon={Shield} color="#6366f1" />
        <StatCard label="Allowed" value="—" icon={CheckCircle} color="#22c55e" />
        <StatCard label="Blocked" value="—" icon={XCircle} color="#ef4444" />
      </div>

      <TrafficChart />

      <div style={styles.info}>
        <span style={styles.version}>
          Version: {health?.version ?? "—"}
        </span>
      </div>
    </div>
  );
}

const styles = {
  page: { padding: 28, display: "flex", flexDirection: "column", gap: 24 },
  heading: { color: "#f1f5f9", fontSize: 20, fontWeight: 700 },
  cards: { display: "flex", gap: 16, flexWrap: "wrap" },
  info: { color: "#475569", fontSize: 13 },
  version: { fontFamily: "monospace" },
};