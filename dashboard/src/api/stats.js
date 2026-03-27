import client from "./client";

export const checkHealth = () =>
  client.get("/health").then((r) => r.data);

export const checkRateLimit = (payload) =>
  client.post("/check", payload).then((r) => r.data);