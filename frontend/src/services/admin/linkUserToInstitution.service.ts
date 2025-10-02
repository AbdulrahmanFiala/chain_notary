import { getAuthClient } from "@/utils/authClient";
import type { Principal } from "@dfinity/principal";

const linkUserToInstitutionService = async (
  principalId: Principal,
  institutionId: string,
): Promise<void> => {
  const { actor } = await getAuthClient();
  const result = await actor.admin_link_user_to_institution(
    principalId,
    institutionId,
  );

  if ("Err" in result) {
    throw new Error(result.Err);
  }
};

export default linkUserToInstitutionService;
