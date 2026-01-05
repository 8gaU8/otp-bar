import { Command } from "@tauri-apps/plugin-shell";

export async function runOTP(token: string) {
  // refer /src-tauri/capabilities/default.json
  let result = await Command.create("run-otp", [
    "--totp",
    "-b",
    token,
  ]).execute();
  return result.stdout;
}
