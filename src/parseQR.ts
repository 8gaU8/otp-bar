import { readFile } from "@tauri-apps/plugin-fs";
import jsQR from "jsqr";
import { parseMigURL } from "./parseUrl";

function resolveImageURL(imgPath: string): string {
  try {
    const resolvedImagePath = new URL(imgPath, import.meta.url).href;
    console.log("RESLOVED", resolvedImagePath);
    return resolvedImagePath;
  } catch (_error) {
    console.log("NOT RESOLVED", imgPath);
    return imgPath;
  }
}

async function loadImageElement(src: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = () => resolve(image);
    image.onerror = () => {
      throw new Error(`Failed to load image from path: ${src}`);
    };
    image.src = src;
  });
}

async function createImageData(imgPath: string): Promise<ImageData> {
  // const imageSrc = resolveImageURL(imgPath);
  const imageSrc = imgPath;
  console.log(imageSrc);
  console.log("Loading image from:", await readFile(imgPath, {}));
  const image = await loadImageElement(imageSrc);
  const canvas = document.createElement("canvas");
  canvas.width = image.naturalWidth || image.width;
  canvas.height = image.naturalHeight || image.height;
  const context = canvas.getContext("2d");
  if (!context) {
    throw new Error("Unable to acquire 2D canvas context");
  }
  context.drawImage(image, 0, 0);
  return context.getImageData(0, 0, canvas.width, canvas.height);
}

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

export async function readQR(imagePath: string) {
  // const imageData = await createImageData(imagePath);
  // const width = imageData.width;
  // const height = imageData.height;
  const pngBlob = await readFileAsPNGBlob(imagePath);
  const imageData = await pngBlobToImageData(pngBlob);
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
