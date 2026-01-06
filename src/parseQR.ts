import { readFile } from "@tauri-apps/plugin-fs";
import jsQR from "jsqr";
import { parseMigURL } from "./parseUrl";


async function readFileAsPNGBlob(path: string): Promise<Blob> {
  // select format based on file extension if needed
  let type = "image/png";
  if (path.endsWith(".png")) {
    type = "image/png";
  } else if (path.endsWith(".jpg") || path.endsWith(".jpeg")) {
    type = "image/jpeg";
  } else {
    throw new Error("Unsupported image format");
  }
  const rawData = await readFile(path);
  return new Blob([rawData], { type: type });
}

async function pngBlobToImageData(pngBlob: Blob): Promise<ImageData> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => {
      const canvas = document.createElement("canvas");
      const ctx = canvas.getContext("2d");
      if (!ctx) {
        reject(new Error("Canvas context is not available."));
        return;
      }

      canvas.width = img.width;
      canvas.height = img.height;
      ctx.drawImage(img, 0, 0);

      // ImageDataを取得
      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      resolve(imageData);
    };
    img.onerror = (error) => {
      reject(error);
    };
    img.src = URL.createObjectURL(pngBlob); // BlobからデータURLを生成して読み込む
  });
}

async function loadImageDataFromFile(path: string): Promise<ImageData> {
  const pngBlob = await readFileAsPNGBlob(path);
  const imageData = await pngBlobToImageData(pngBlob);
  return imageData;
}

export async function readQR(imagePath: string) {
  // Load image and convert to ImageData
  const imageData = await loadImageDataFromFile(imagePath);

  // Use jsQR to read the QR code from ImageData
  const code = jsQR(imageData.data, imageData.width, imageData.height);
  if (!code) {
    throw new Error("QR code not found");
  }
  console.log(code.data);
  console.log(parseMigURL(code.data));
  console.log(parseMigURL(code.data).params[0].secret?.base32);
  return code;
}

class UserToken {
  public name: string;
  public secret: string;
  constructor(name: string, secret: string) {
    this.name = name;
    this.secret = secret;
  }
}

export async function generateConfiguration(imagePath: string) {
  const code = await readQR(imagePath);
  const otpSecrets = parseMigURL(code.data);
  const configurationData: Array<UserToken> = [];
  otpSecrets.params.forEach((param, index) => {
    if (param.secret?.base32) {
      const accountName = param.name || `account_${index + 1}`;
      configurationData.push(new UserToken(accountName, param.secret.base32));
    }
  });
  return configurationData;
}
