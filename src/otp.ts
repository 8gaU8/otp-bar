import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { runOTP } from "./command.ts";
import { getExecutablePath, readToken } from "./config.ts";

export const generateOTP = async (id: string): Promise<string> => {
  const token = await readToken(id);
  const executablePath = await getExecutablePath();
  const otp = await runOTP(executablePath, token);
  return otp.trim();
};

export const clipOTP = async (clipID: string) => {
  const otp = await generateOTP(clipID);
  console.log("OTP:", otp);

  await writeText(otp);
};
