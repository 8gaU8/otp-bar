import { useCallback, useEffect, useState } from "react";
import { TrayIcon } from "@tauri-apps/api/tray";
import { Menu, MenuItemOptions } from "@tauri-apps/api/menu";

async function createTray(): Promise<TrayIcon> {
  const tray = await TrayIcon.new({
    id: "js_tray_icon",
    // `src-tauri`を起点にしたパス
    icon: "icons/icon.png",
  });

  return tray;
}

async function createMenu(): Promise<Menu> {
  // メニューを作成
  const menu = await Menu.new();

  // メニューアイテムを追加
  const menuItems: MenuItemOptions[] = [
    {
      id: "menuID1",
      text: "メニュー1",
      action: async (id) => {
        console.log(`menu "${id}" clicked`);
      },
    },
    {
      id: "menuID2",
      text: "メニュー2",
      action: async (id) => {
        console.log(`menu "${id}" clicked`);
      },
    },
  ];
  await menu.append(menuItems.map((menu) => menu));

  return menu;
}

export function useTray() {
  const [menu, setMenu] = useState<Menu | null>(null);
  const [tray, setTray] = useState<TrayIcon | null>(null);

  // メニューをリセット
  const resetMenu = useCallback(async () => {
    if (!tray) return;
    await tray.setMenu(menu);
  }, [tray, menu]);

  async function addMenu(text: string) {
    if (!menu) return;

    // メニューアイテムを追加
    await menu.append({
      id: crypto.randomUUID(),
      text: text,
    });
    setMenu(menu);
  }

  useEffect(() => {
    async function setup() {
      // トレイアイコンを作成
      const tray = await createTray();
      // メニューを作成
      const menu = await createMenu();
      await tray.setMenu(menu);

      setTray(tray);
      setMenu(menu);
    }
    setup();
  }, []);

  useEffect(() => {
    resetMenu();
  }, [resetMenu]);

  return { addMenu };
}
