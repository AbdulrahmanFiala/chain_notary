import { getAuthClient } from "@/utils/authClient";
import type { Principal } from "@dfinity/principal";

const promoteToSuperAdminService = async (
  principalId: Principal,
): Promise<void> => {
  const { actor } = await getAuthClient();

  const result = await actor.admin_promote_to_super_admin(principalId);

  if ("Err" in result) {
    throw new Error(result.Err);
  }
};

export default promoteToSuperAdminService;
