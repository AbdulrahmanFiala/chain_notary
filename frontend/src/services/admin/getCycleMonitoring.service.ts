import { getAuthClient } from "@/utils/authClient";
import type { CycleMonitoringData } from "declarations/backend/backend.did";

const getCycleMonitoringService = async (): Promise<CycleMonitoringData> => {
  const { actor } = await getAuthClient();

  const result = await actor.admin_get_cycle_monitoring();

  if ("Ok" in result) {
    return result.Ok;
  } else {
    throw new Error(result.Err);
  }
};

export default getCycleMonitoringService;
