import { useQuery } from "@tanstack/react-query";
import { checkHealth } from "../api/stats";

export function useHealth() {
  return useQuery({
    queryKey: ["health"],
    queryFn: checkHealth,
    refetchInterval: 3000,
  });
}