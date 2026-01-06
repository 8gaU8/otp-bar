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
1. Install dependencies if you don't have it.
    ```bash
    brew install coreutils oath-toolkit
    ```
2. Download `*.dmg` from a release page.

3. Run the following command.
    ```bash
    $ xattr -d com.apple.quarantine /path/to/download/OTP.Bar_${version}_aarch64.dmg
    ```

4. Create a configuration file.

    ```bash
    $ mkdir -p $HOME/.config/otp-bar
    $ touch $HOME/.config/otp-bar/config.json
    ```
