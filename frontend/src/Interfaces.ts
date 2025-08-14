import dayjs from "dayjs";

export interface FormData {
  nftName: string;
  nftDescription: string;
  birthDate: dayjs.ConfigType | null;
  issuerAddress: string;
  nationalId: string;
  rewarderName: string;
  birthplace: string;
  nationality: string;
  dateOfRewarding: dayjs.ConfigType | null;
  issuerName: string;
  fileData: any
}