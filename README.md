# OTP Bar

A menu bar application for one time passwords, implemented in Rust.

## Installation

Simply, run the following command

```bash
curl -sSL https://raw.githubusercontent.com/8gaU8/otp-bar/refs/heads/main/install.sh | /bin/bash
```

## Manual Installation
1. Download the latest `*.dmg` file from a release page

2. Run the following command to sign.
    ```bash
    xattr -d com.apple.quarantine /path/to/download/OTP.Bar_${version}_aarch64.dmg
    ```

3. Copy `.app` into `/Applications` directory



## Configurations

### Structure

```
$HOME/.config/otp-bar
├── token1
├── token2
├── token3
.
.
```

### Token files

- Upload an image file (JPEG or PNG) of an exported QR code from Google Authenticator App using the "Configure" menu option.
- Or save token as a plain text file under `$HOME/.config/otp-bar/`
- Example:
```
$ cat token1
***YOURTOKEN1**
$ cat token2
***YOURTOKEN2**
```
  
