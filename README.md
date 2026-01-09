# OTP Bar

A menu bar application for one time passwords, implemented in Rust.

## Installation

Simply, run the following command

```bash
curl -sSL https://raw.githubusercontent.com/8gaU8/otp-bar/refs/heads/main/install.sh | /bin/bash
```

## Manual Installation
1. Download the latest `*.dmg` file from a release page and copy `.app` into `/Applications` directory

2. Run the following command to sign.
    ```bash
    xattr -d com.apple.quarantine /path/to/download/OTP.Bar_${version}_aarch64.dmg
    ```

3. Create a configuration directory (optional - will be created automatically).
    ```bash
    mkdir -p $HOME/.config/otp-bar
    ```


## Configurations

### Structure

```
$HOME/.config/otp-bar
└── config.toml
```

### Configuration File

The application now uses a TOML configuration file at `$HOME/.config/otp-bar/config.toml`.

#### Adding tokens via QR code

- Upload an image file (JPEG or PNG) of an exported QR code from Google Authenticator App using the "Configure" menu option.
- The tokens will be automatically added to the `config.toml` file.

#### Manual configuration

You can also manually edit the `config.toml` file:

```toml
[tokens."Google Account"]
secret = "JBSWY3DPEHPK3PXP"
priority = 1

[tokens.GitHub]
secret = "HXDMVJECJJWSRB3HWIZR4IFUGFTMXBOZ"
priority = 2

[tokens.AWS]
secret = "MFRGGZDFMZTWQ2LK"
```

Each token entry consists of:
- **name**: The token identifier (shown in the menu)
- **secret**: The base32-encoded secret
- **priority** (optional): Determines the order in the menu. Tokens with priority are shown first (sorted by priority value), followed by tokens without priority (sorted alphabetically).

See [example.config.toml](example.config.toml) for a template.

### Migrating from Old Configuration

If you were using an older version of OTP Bar with individual token files, see [MIGRATION.md](MIGRATION.md) for instructions on migrating to the new TOML format.
  
