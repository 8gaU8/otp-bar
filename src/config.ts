import { BaseDirectory, readDir, readTextFile } from "@tauri-apps/plugin-fs";

const CONFIG_DIR = ".config/otp-bar";
const CONFIG_FILE = "config.json";

export async function getExecutablePath(): Promise<string> {
  // Read the oathtool executable path from config file
  const configText = await readTextFile(`${CONFIG_DIR}/${CONFIG_FILE}`, {
    baseDir: BaseDirectory.Home,
  });
  const config = JSON.parse(configText);
  return config.oathtoolExecutablePath || "oathtool";
}

export async function listTokenIDs(): Promise<Array<string>> {
  // ホームディレクトリの .config/otp-bar ディレクトリ内のファイル名を取得

  const entries = await readDir(CONFIG_DIR, {
    baseDir: BaseDirectory.Home,
  });

  const idList: Array<string> = [];
  for (const entry of entries) {
    if (entry.name && entry.name !== CONFIG_FILE) {
      idList.push(entry.name);
    }
  }
  console.log("Token IDs:", idList);
  return idList;
}

export async function readToken(id: string): Promise<string> {
  // 指定されたIDのトークンをファイルから読み込む
  const textContents = await readTextFile(`${CONFIG_DIR}/${id}`, {
    baseDir: BaseDirectory.Home,
  });
  const trimedContents = textContents.trim();
  console.log(`Token for ID ${id}:`, trimedContents);

  return trimedContents;
}
