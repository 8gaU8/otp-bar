# OTP Bar

A menu bar application for one time passwords

## Requirements
- OATH Toolkit
  - install with commands:
    ```bash
    brew install coreutils oath-toolkit
    ```

## Configurations

### Structure

```
$HOME/.config/otp-bar
├── config.json
├── token1
├── token2
├── token3
.
.
```

### `config.json`
- Create a config file at `$HOME/.config/otp-bar/config.json`
- Example:
    ```json
    {
        "oathtoolExecutablePath": "/opt/homebrew/bin/oathtool"
    }
    ```

### Token files
- Save token as a plain text file under `$HOME/.config/otp-bar/`
- Example:
```
$ cat token1
***YOURTOKEN1**
$ cat token2
***YOURTOKEN2**
```
  
## Installation

### Download from Release Page
1. Download the `.dmg` file from the [releases page](https://github.com/8gaU8/otp-bar/releases)
2. Open the downloaded `.dmg` file

### Handling "Damaged" Error on macOS
If you see an error message saying the app is "damaged" and needs to be moved to trash, this is due to macOS Gatekeeper security. You can fix this by removing the quarantine attribute:

1. Open Terminal
2. Navigate to the directory where you downloaded the DMG file (usually `~/Downloads`)
3. Run the following command (replace `VERSION` and `ARCH` with your actual file name):
   ```bash
   xattr -d com.apple.quarantine OTP.Bar_VERSION_ARCH.dmg
   ```
   For example:
   ```bash
   xattr -d com.apple.quarantine OTP.Bar_0.2.0_aarch64.dmg
   ```
4. Now open the DMG file again

Alternatively, after mounting the DMG, you can remove the quarantine from the app itself:
```bash
xattr -dr com.apple.quarantine /Volumes/OTP\ Bar/OTP\ Bar.app
```

**Note**: This is necessary because the app is not code-signed with an Apple Developer certificate. The app is safe to use - you can verify the source code in this repository.
