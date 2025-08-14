import dayjs from "dayjs";

export interface FormData {
  fileData: any,
  nftName: string;
  nftDescription: string;
  rewarderName: string;
  dateOfRewarding: dayjs.ConfigType | null;
  email: string;
  nationalId: string;
}