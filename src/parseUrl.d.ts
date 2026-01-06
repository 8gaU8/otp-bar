export function parseMigURL(data: string): {
  params: Array<{
    secret?: {
      raw: Uint8Array;
      base32: string;
    };
    name?: string;
    issuer?: string;
    algorithm?: string;
    digits?: number;
    type?: string;
    conter?: number;
    [key: string | number]: any;
  }>;
  version?: any;
  batch_size?: any;
  batch_index?: any;
  batch_id?: any;
  [key: string | number]: any;
};