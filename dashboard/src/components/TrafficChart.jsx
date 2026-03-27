import { useEffect, useState } from "react";
import {
  LineChart, Line, XAxis, YAxis,
  CartesianGrid, Tooltip, ResponsiveContainer,
} from "recharts";

function generatePoint(index) {
  return {
    time: new Date(Date.now() - (19 - index) * 3000)
      .toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" }),
    allowed: Math.floor(Math.random() * 40) + 10,
    blocked: Math.floor(Math.random() * 10),
  };
}

export default function TrafficChart() {
  const [data, setData] = useState(() =>
    Array.from({ length: 20 }, (_, i) => generatePoint(i))
  );

  useEffect(() => {
    const interval = setInterval(() => {
      setData((prev) => [
        ...prev.slice(1),
        generatePoint(19),
      ]);
    }, 3000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div style={styles.container}>
      <h3 style={styles.title}>Live Traffic</h3>
      <ResponsiveContainer width="100%" height={220}>
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" />
          <XAxis dataKey="time" tick={{ fill: "#64748b", fontSize: 11 }} />
          <YAxis tick={{ fill: "#64748b", fontSize: 11 }} />
          <Tooltip
            contentStyle={{ background: "#0f172a", border: "1px solid #334155", borderRadius: 8 }}
            labelStyle={{ color: "#94a3b8" }}
          />
          <Line type="monotone" dataKey="allowed" stroke="#22c55e" strokeWidth={2} dot={false} />
          <Line type="monotone" dataKey="blocked" stroke="#ef4444" strokeWidth={2} dot={false} />
        </LineChart>
      </ResponsiveContainer>
      <div style={styles.legend}>
        <span style={{ color: "#22c55e" }}>● Allowed</span>
        <span style={{ color: "#ef4444" }}>● Blocked</span>
      </div>
    </div>
  );
}

const styles = {
  container: {
    background: "#1e293b",
    borderRadius: 12,
    padding: 24,
    border: "1px solid #334155",
  },
  title: { color: "#f1f5f9", marginBottom: 16, fontSize: 15, fontWeight: 600 },
  legend: { display: "flex", gap: 20, marginTop: 12, fontSize: 13 },
};