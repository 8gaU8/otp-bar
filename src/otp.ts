import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { runOTP } from "./command.ts";
import { readToken } from "./config.ts";

export const clipOTP = async (clipID: string) => {
  const token = await readToken(clipID);
  const otp = await runOTP(token);
  console.log("OTP:", otp);

  await writeText(otp.trim());
};
