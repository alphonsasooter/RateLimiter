import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { getRules, createRule, deleteRule, resetKey } from "../api/rules";

export function useRules() {
  return useQuery({
    queryKey: ["rules"],
    queryFn: getRules,
    refetchInterval: 5000,
  });
}

export function useCreateRule() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: createRule,
    onSuccess: () => qc.invalidateQueries(["rules"]),
  });
}

export function useDeleteRule() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: deleteRule,
    onSuccess: () => qc.invalidateQueries(["rules"]),
  });
}

export function useResetKey() {
  return useMutation({ mutationFn: resetKey });
}