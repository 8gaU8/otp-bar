import { setup } from "./tray";
import { getCurrentWindow } from "@tauri-apps/api/window";

async function main() {
  await setup();
  
  // Hide the window but keep it alive to ensure timers continue running
  const window = getCurrentWindow();
  await window.hide();
}

main();