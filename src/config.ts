import { BaseDirectory, readDir, readTextFile } from "@tauri-apps/plugin-fs";

const CONFIG_DIR = ".config/otp-bar";

export async function listTokenIDs(): Promise<Array<string>> {
  // ホームディレクトリの .config/otp-bar ディレクトリ内のファイル名を取得

  const entries = await readDir(CONFIG_DIR, {
    baseDir: BaseDirectory.Home,
  });

  const idList: Array<string> = [];
  for (const entry of entries) {
    if (entry.name) {
      idList.push(entry.name);
    }
  }
  return idList;
}

export async function readToken(id: string): Promise<string> {
  // 指定されたIDのトークンをファイルから読み込む
  const textContents = await readTextFile(`${CONFIG_DIR}/${id}`, {
    baseDir: BaseDirectory.Home,
  });
  const trimedContents = textContents.trim();

  return trimedContents;
}
