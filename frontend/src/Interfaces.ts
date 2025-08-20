import type { UploadFile } from "antd";
import dayjs from "dayjs";

export interface FormData {
  fileData: UploadFile,
  nftName: string;
  nftDescription: string;
  rewarderName: string;
  dateOfRewarding: dayjs.ConfigType | null;
  email: string;
  nationalId: string;
}