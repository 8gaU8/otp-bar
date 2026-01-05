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
Download from a release page
