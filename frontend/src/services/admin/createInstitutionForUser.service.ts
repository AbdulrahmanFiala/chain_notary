import { getAuthClient } from "@/utils/authClient";
import type { Principal } from "@dfinity/principal";

const createInstitutionForUserService = async (
  principalId: Principal,
  institutionName: string,
  institutionEmail: string,
): Promise<void> => {
  const { actor } = await getAuthClient();
  const result = await actor.admin_create_institution_for_user(
    principalId,
    institutionName,
    institutionEmail,
  );

  if ("Err" in result) {
    throw new Error(result.Err);
  }
};

export default createInstitutionForUserService;
