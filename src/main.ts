import { relaunch } from "@tauri-apps/plugin-process";

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

function getOTPRemainingTime(): number {
  const now = Math.floor(Date.now() / 1000); // Current Unix time in seconds
  const period = 30; // OTP period in seconds
  const timeInPeriod = now % period;
  const remainingTime = period - timeInPeriod;
  return remainingTime;
}

async function getOtp(id: string) {
  const tokenIds: string = await invoke("generate_otp_command", {
    id: id,
  });
  await writeText(tokenIds);
  console.log("Token IDs:", tokenIds);
}

async function handleConfigure() {
  const file = await open({
    multiple: false,
    directory: false,
    extensions: ["png"],
  });
  if (typeof file === "string") {
    console.log(file);
    await invoke("handle_configure_command", { filePathStr: file });
    await relaunch();
  }
}

async function listTokenIds(): Promise<string[]> {
  let tokenIds: string[] = ["No Tokens are configured"];
  try {
    tokenIds = await invoke("list_token_ids_command");
  } catch (e) {
    console.log("Error fetching token IDs:", e);
  }
  return tokenIds;
}

async function init() {
  console.log("App initialized");
  // generate HTML elements corresponding to token IDs
  const tokenIds: string[] = await listTokenIds();
  const container = document.getElementById("token-ids-container");
  if (container) {
    tokenIds.forEach((id) => {
      const buttonContainer = document.createElement("div");
      buttonContainer.className = "token-button-container";
      const button = document.createElement("button");
      button.textContent = id + " - Copy OTP";
      button.className = "token-button";
      button.addEventListener("click", async () => {
        await getOtp(id);
      });
      buttonContainer.appendChild(button);
      container.appendChild(buttonContainer);
    });
    // update button texts every second to show remaining time
  }
}

async function main() {
  await init();
  const remainingTimeContainer = document.getElementById("remaining-time");
  if (!remainingTimeContainer) {
    console.log("Container for remaining time not found");
    return;
  }
  setInterval(() => {
    const remainingTime: number = getOTPRemainingTime();
    const inWarning: boolean = remainingTime <= 5;
    remainingTimeContainer.textContent = `Time left: ${remainingTime} seconds ${
      inWarning ? "!" : ""
    }`;
  }, 500);

  const configureButton = document
    .querySelector("#configure-form")
    ?.querySelector("#Configure");
  configureButton?.addEventListener("click", async (e) => {
    e.preventDefault();
    console.log("Configure button clicked");
    await handleConfigure();
  });
}

window.addEventListener("DOMContentLoaded", async () => {
  main();
});
