import { getAuthClient } from "@/utils/authClient";

const getStorageInfoService = async (): Promise<string[]> => {
  const { actor } = await getAuthClient();

  const result = await actor.admin_get_storage_info();

  if ("Ok" in result) {
    return result.Ok;
  } else {
    throw new Error(result.Err);
  }
};

export default getStorageInfoService;
