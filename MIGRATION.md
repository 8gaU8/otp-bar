# Migration Guide: Individual Files to TOML Configuration

If you were using OTP Bar before the TOML configuration update, you may have individual token files in `$HOME/.config/otp-bar/`. This guide will help you migrate to the new TOML format.

## Migration Steps

### Option 1: Manual Migration (Recommended)

If you still have your old token files, you can manually create the TOML config:

1. Create a new file at `$HOME/.config/otp-bar/config.toml`
2. Add your tokens in the following format:

```toml
[tokens.token1]
secret = "YOUR_TOKEN_SECRET_1"

[tokens.token2]
secret = "YOUR_TOKEN_SECRET_2"
priority = 1
```

For example, if you had:
```bash
$ cat ~/.config/otp-bar/GitHub
JBSWY3DPEHPK3PXP

$ cat ~/.config/otp-bar/Google
HXDMVJECJJWSRB3HWIZR4IFUGFTMXBOZ
```

Create `config.toml`:
```toml
[tokens.GitHub]
secret = "JBSWY3DPEHPK3PXP"

[tokens.Google]
secret = "HXDMVJECJJWSRB3HWIZR4IFUGFTMXBOZ"
```

You can optionally add a `priority` field to control the menu order:
```toml
[tokens.GitHub]
secret = "JBSWY3DPEHPK3PXP"
priority = 1

[tokens.Google]
secret = "HXDMVJECJJWSRB3HWIZR4IFUGFTMXBOZ"
priority = 2
```

Tokens with priority will appear first in the menu (sorted by priority value), followed by tokens without priority (sorted alphabetically).

### Option 2: Re-import from QR Codes

If you have access to your QR codes:

1. Delete all old token files in `$HOME/.config/otp-bar/`
2. Use the "Configure" menu option in OTP Bar
3. Upload your QR code images
4. The tokens will be automatically added to the new `config.toml` file

### Option 3: Manual Script Migration

Run this script to automatically convert your old token files to TOML format:

```bash
#!/bin/bash

CONFIG_DIR="$HOME/.config/otp-bar"
TOML_FILE="$CONFIG_DIR/config.toml"

# Remove the old config file if it exists
rm -f "$TOML_FILE"

# Convert each file to a TOML entry
for file in "$CONFIG_DIR"/*; do
    # Skip if it's already the config.toml or doesn't exist
    if [[ ! -f "$file" ]] || [[ "$file" == "$TOML_FILE" ]]; then
        continue
    fi
    
    filename=$(basename "$file")
    content=$(cat "$file" | tr -d '\n')
    
    # Add to TOML
    cat >> "$TOML_FILE" << EOF
[tokens."$filename"]
secret = "$content"

EOF
    
    # Optionally, backup the old file
    # mv "$file" "$file.bak"
done

echo "Migration complete! Check $TOML_FILE"
```

## Cleanup

After verifying that your tokens work in the new format:

1. You can safely delete the old individual token files
2. Keep only the `config.toml` file in `$HOME/.config/otp-bar/`

## Verification

To verify the migration was successful:

1. Open OTP Bar from the menu bar
2. Check that all your tokens appear in the menu
3. Test copying an OTP code to ensure it works correctly
