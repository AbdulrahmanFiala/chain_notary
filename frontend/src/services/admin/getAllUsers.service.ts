import { getAuthClient } from "@/utils/authClient";
import type { UserProfile } from "declarations/backend/backend.did";

const getAllUsersService = async (): Promise<UserProfile[]> => {
  const { actor } = await getAuthClient();
  const response = await actor.admin_get_all_users();
  if ("Ok" in response) {
    return response.Ok;
  }
  throw new Error("Failed to fetch users");
};

export default getAllUsersService;
