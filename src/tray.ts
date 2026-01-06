import { defaultWindowIcon } from "@tauri-apps/api/app";
import {
  Menu,
  MenuItemOptions,
  PredefinedMenuItem,
} from "@tauri-apps/api/menu";
import { TrayIcon } from "@tauri-apps/api/tray";
import { open } from "@tauri-apps/plugin-dialog";
import { listTokenIDs, writeTokenFile } from "./config";
import { clipOTP } from "./otp";
import { generateConfiguration } from "./parseQR";

async function createTray(): Promise<TrayIcon> {
  const tray = await TrayIcon.new({
    id: "js_tray_icon",
    icon: (await defaultWindowIcon()) || undefined,
  });

  return tray;
}

function createMenuItemOptions(id: string): MenuItemOptions {
  return {
    id: `otp-bar-${id}`,
    text: `OTP: ${id}`,
    action: async () => {
      clipOTP(id);
    },
  };
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
      await writeTokenFile(`1_${user.name}`, user.secret);
    }
    console.log(data);
  }
}

async function defaultMenu(): Promise<Menu> {
  const seperator = await PredefinedMenuItem.new({
    item: "Separator",
  });
  const quit = await PredefinedMenuItem.new({
    item: "Quit",
  });
  const configureMenuItem: MenuItemOptions = {
    id: "otp-bar-configure",
    text: "Configure...",
    action: async () => {
      handleConfigure();
    },
  };
  const menu = await Menu.new({
    items: [configureMenuItem, quit, seperator],
  });
  return menu;
}

async function createMenu(idList: Array<string>): Promise<Menu> {
  // const items: MenuItemOptions[] = [
  //   {
  //     id: "otp-bar-configure",
  //     text: "Configure...",
  //     action: async () => {
  //       handleConfigure();
  //     },
  //   },
  // ];
  // for (const id of idList) {
  //   const option = createMenuItemOptions(id);
  //   items.push(option);
  // }
  const menu = await defaultMenu();

  return menu;
}

export async function setup() {
  // トレイアイコンを作成
  const tray = await createTray();
  // メニューを作成
  const tokenIdList = await listTokenIDs();
  const menu = await createMenu(tokenIdList);
  await tray.setMenu(menu);
}
