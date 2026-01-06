// ref: https://qiita.com/McbeEringi/items/9441709e031cfe203028
const depb = (w) => [
  ...{
    [Symbol.iterator]: (
      p = 0,
      v = (_) =>
        [
          ...{
            [Symbol.iterator]: (d) => ({
              next: (_) => ({
                done: d,
                value: d || ((d = !(w[p] & 0x80)), w[p++] & 0x7f),
              }),
            }),
          },
        ].reduce((a, x, i) => (x << (7 * i)) | a)
    ) => ({
      next: (_) =>
        w.length <= p
          ? { done: 1 }
          : {
              value: ((x) => ({
                i: x >>> 3,
                type: (x = x & 7),
                value: [
                  (x = v()) =>
                    Object.assign(x, { s: ((x + 1) / (x & 1 ? -2 : 2)) | 0 }),
                  (_) => w.slice(p, (p += 8)),
                  (l = v()) => w.slice(p, (p += l)),
                  ,
                  ,
                  (_) => w.slice(p, (p += 4)),
                ][x](),
              }))(v()),
            },
    }),
  },
];

export const parseMigURL = (w) =>
  ((
    td = new TextDecoder(),
    b32en = (w) =>
      ((x) =>
        [...Array(Math.ceil(x.length / 5))]
          .map(
            (_, i) =>
              "abcdefghijklmnopqrstuvwxyz234567"[
                +`0b${x.slice(5 * i++, 5 * i).padEnd(5, 0)}`
              ]
          )
          .join("")
          .padEnd(Math.ceil(x.length / 40) * 8, "="))(
        w.reduce((a, x) => a + x.toString(2).padStart(8, 0), "")
      )
  ) => (
    (w = new URL(w)),
    w.href.match(/^[^?]+/) == "otpauth-migration://offline" &&
      (w = w.searchParams.get("data")) &&
      depb(
        new Uint8Array(
          [...atob(decodeURIComponent(w))].map((x) => x.charCodeAt())
        )
      ).reduce(
        (a, x) => (
          x.i == 1
            ? a.params.push(
                depb(x.value).reduce(
                  (a, x) => (
                    (
                      [
                        ,
                        (x) => (a.secret = { raw: x, base32: b32en(x) }),
                        (x) => (a.name = td.decode(x)),
                        (x) => (a.issuer = td.decode(x)),
                        (x) =>
                          (a.algorithm = [
                            ,
                            "SHA-1",
                            "SHA-256",
                            "SHA-512",
                            "MD5",
                          ][x]),
                        (x) => (a.digits = [, 6, 8][x]),
                        (x) => (a.type = [, "HOTP", "TOTP"][x]),
                        (x) => (a.conter = x),
                      ][x.i] || ((y) => (a[x.i] = y))
                    )(x.value),
                    a
                  ),
                  {}
                )
              )
            : (a[
                [, , "version", "batch_size", "batch_index", "batch_id"][x.i] ||
                  x.i
              ] = x.value),
          a
        ),
        { params: [] }
      )
  ))();