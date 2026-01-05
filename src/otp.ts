import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { runOTP } from "./command.ts";
import { getExecutablePath, readToken } from "./config.ts";

export const clipOTP = async (clipID: string) => {
  const token = await readToken(clipID);
  const executablePath = await getExecutablePath();
  const otp = await runOTP(executablePath, token);
  console.log("OTP:", otp);

  await writeText(otp.trim());
};
