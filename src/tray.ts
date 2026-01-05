import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Menu, MenuItemOptions } from "@tauri-apps/api/menu";
import { TrayIcon } from "@tauri-apps/api/tray";
import { listTokenIDs } from "./config";
import { clipOTP } from "./otp";

async function createTray(): Promise<TrayIcon> {
  const tray = await TrayIcon.new({
    id: "js_tray_icon",
    icon: (await defaultWindowIcon()) || undefined,
  });

  return tray;
}

async function createMenuItemOptions(id: string): Promise<MenuItemOptions> {
  return {
    id: `otp-bar-${id}`,
    text: `OTP: ${id}`,
    action: async () => {
      clipOTP(id);
    },
  };
}

async function createMenu(idList: Array<string>): Promise<Menu> {
  const items: MenuItemOptions[] = [];
  for (const id of idList) {
    const option = await createMenuItemOptions(id);
    items.push(option);
  }
  const menu = await Menu.new({ items });

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
