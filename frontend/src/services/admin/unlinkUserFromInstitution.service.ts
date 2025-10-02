import { getAuthClient } from "@/utils/authClient";
import type { Principal } from "@dfinity/principal";

const unlinkUserFromInstitutionService = async (
  principalId: Principal,
): Promise<void> => {
  const { actor } = await getAuthClient();

  const result = await actor.admin_unlink_user_from_institution(principalId);

  if ("Err" in result) {
    throw new Error(result.Err);
  }
};

export default unlinkUserFromInstitutionService;
