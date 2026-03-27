import client from "./client";

export const getRules = () => client.get("/rules").then((r) => r.data);

export const createRule = (rule) =>
  client.post("/rules", rule).then((r) => r.data);

export const deleteRule = (id) =>
  client.delete(`/rules/${id}`).then((r) => r.data);

export const resetKey = (key) =>
  client.post(`/reset/${key}`).then((r) => r.data);