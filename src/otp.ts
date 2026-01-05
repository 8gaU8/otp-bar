import { readToken } from "./config.ts";
import { runOTP } from "./command.ts";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

export const clipOTP = async (clipID: string) => {
  const token = await readToken(clipID);
  const otp = await runOTP(token);
  console.log("OTP:", otp);

  await writeText(otp.trim());
};
