import { Command } from "@tauri-apps/plugin-shell";

export async function runOTP(executablePath: string, token: string) {
  console.log("Using executable path:", executablePath);
  console.log("Running OTP command with token:", token);
  // refer /src-tauri/capabilities/default.json
  const command = Command.create("bash-exec", [
    "-c",
    `${executablePath} --totp -b ${token}`,
  ]);
  console.log("Constructed command:", command);
  const result = await command.execute();
  console.log("Command execution result:", result);
  return result.stdout;
}
