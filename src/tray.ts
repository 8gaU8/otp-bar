import { defaultWindowIcon } from "@tauri-apps/api/app";
import {
  Menu,
  MenuItem,
  MenuItemOptions,
  PredefinedMenuItem,
} from "@tauri-apps/api/menu";
import { TrayIcon } from "@tauri-apps/api/tray";
import { open } from "@tauri-apps/plugin-dialog";
import { listTokenIDs, writeTokenFile } from "./config";
import { clipOTP } from "./otp";
import { generateConfiguration } from "./parseQR";
import { relaunch } from "@tauri-apps/plugin-process";
import { getOTPRemainingTime, isOTPInWarningPeriod } from "./otpTimer";

// Global reference to timer menu item for updating
let timerMenuItem: MenuItem | null = null;

async function createTray(): Promise<TrayIcon> {
  const tray = await TrayIcon.new({
    id: "js_tray_icon",
    icon: (await defaultWindowIcon()) || undefined,
  });

  return tray;
}

function getTimerDisplayText(): string {
  const remainingTime = getOTPRemainingTime();
  const isWarning = isOTPInWarningPeriod();
  return isWarning 
    ? `⚠️ Time: ${remainingTime}s` 
    : `⏱️ Time: ${remainingTime}s`;
}

async function createTimerMenuItem(): Promise<MenuItem> {
  const options: MenuItemOptions = {
    id: "otp-bar-timer",
    text: getTimerDisplayText(),
    enabled: false, // Make it non-clickable as it's just for display
  };
  const item = await MenuItem.new(options);
  timerMenuItem = item; // Store reference for later updates
  return item;
}

async function createMenuItem(id: string): Promise<MenuItem> {
  const options = {
    id: `otp-bar-${id}`,
    text: `OTP: ${id}`,
    action: async () => {
      console.log("Menu item clicked for ID:", id);
      await clipOTP(id);
    },
  };
  return await MenuItem.new(options);
}

async function handleConfigure() {
  const file = await open({
    multiple: false,
    directory: false,
    extensions: ["png"],
  });
  console.log(file);
  if (typeof file === "string") {
    console.log(file);
    const data = await generateConfiguration(file);
    for (const user of data) {
      await writeTokenFile(user.name, user.secret);
    }
    console.log(data);
  }
}

async function defaultMenu(): Promise<Menu> {
  const separatorItem = await PredefinedMenuItem.new({
    item: "Separator",
  });
  const quitItem = await PredefinedMenuItem.new({
    item: "Quit",
  });
  const configureMenuItepOptions: MenuItemOptions = {
    id: "otp-bar-configure",
    text: "Configure (restart automatically)",
    action: async () => {
      await handleConfigure();
      await relaunch();
    },
  };
  const configMenuItem = await MenuItem.new(configureMenuItepOptions);
  const items  = [configMenuItem, quitItem, separatorItem];
  const menu = await Menu.new();
  for (const item of items) {
    menu.append(item);
  }

  return menu;
}

async function createMenu(idList: Array<string>): Promise<Menu> {
  const menu = await defaultMenu();

  // Add timer display at the top
  const timerItem = await createTimerMenuItem();
  menu.append(timerItem);

  // Add separator
  const separatorItem = await PredefinedMenuItem.new({
    item: "Separator",
  });
  menu.append(separatorItem);

  for (const id of idList) {
    const option = await createMenuItem(id);
    menu.append(option);
  }
  return menu;
}

async function updateTimerDisplay() {
  // Update only the timer menu item text instead of recreating the entire menu
  if (timerMenuItem) {
    const newText = getTimerDisplayText();
    await timerMenuItem.setText(newText);
  }
}

export async function setup() {
  // トレイアイコンを作成
  const tray = await createTray();
  // メニューを作成
  const tokenIdList = await listTokenIDs();
  const menu = await createMenu(tokenIdList);
  await tray.setMenu(menu);

  // Update timer display every second
  setInterval(async () => {
    await updateTimerDisplay();
  }, 1000);
}
