# OTP Bar

A menu bar application for one time passwords

## Installation

Simply, run the following command

```bash
curl -sSL https://raw.githubusercontent.com/8gaU8/otp-bar/refs/heads/main/install.sh | /bin/bash
```

## Manual Installation
1. Install dependencies if you don't have it.
    ```bash
    brew install coreutils oath-toolkit
    ```
2. Download the latest `*.dmg` file from a release page and copy `.app` into `/Applications` directory

3. Run the following command to sign.
    ```bash
    xattr -d com.apple.quarantine /path/to/download/OTP.Bar_${version}_aarch64.dmg
    ```

4. Create a configuration file.

    ```bash
    mkdir -p $HOME/.config/otp-bar; 
    echo '{ "oathtoolExecutablePath": "/opt/homebrew/bin/oathtool" }'> "${HOME}/.config/otp-bar/config.json"
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

- Upload an image file (JPEG or PNG) of an exported QR code from Google Authenticator App.
- Or save token as a plain text file under `$HOME/.config/otp-bar/`
- Example:
```
$ cat token1
***YOURTOKEN1**
$ cat token2
***YOURTOKEN2**
```
  
