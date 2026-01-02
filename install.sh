#!/bin/bash
set -e

echo "--- Protonhaxora Universal Installer ---"

# 1. Define Paths (User-Agnostic)
BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"

# Create directories if they don't exist
mkdir -p "$BIN_DIR"
mkdir -p "$APP_DIR"
mkdir -p "$ICON_DIR"

# 2. Install Protonhaxora Binary
if [ -f "./protonhaxora" ]; then
    echo "Installing protonhaxora binary..."
    cp ./protonhaxora "$BIN_DIR/"
    chmod +x "$BIN_DIR/protonhaxora"
else
    echo "Error: protonhaxora binary not found in current directory."
    echo "Please run this script from the folder containing the release assets."
    exit 1
fi

# 3. Install Icon
if [ -f "./icon.png" ]; then
    echo "Installing icon..."
    cp ./icon.png "$ICON_DIR/protonhaxora.png"
else
    # Fallback check if icon is in ui/
    if [ -f "./ui/icon.png" ]; then
        cp ./ui/icon.png "$ICON_DIR/protonhaxora.png"
    fi
fi

# 4. Create Desktop Entry
echo "Creating Desktop Entry..."
cat <<EOF > "$APP_DIR/protonhaxora.desktop"
[Desktop Entry]
Name=Protonhaxora
Exec=$BIN_DIR/protonhaxora
Icon=protonhaxora
Type=Application
Categories=Game;Utility;
Comment=Launch Aurora via Protonhax
Terminal=false
EOF

# 5. Fetch and Install Protonhax Dependency
echo "Fetching latest protonhax from LunaBaloona fork..."
# This uses the GitHub API to find the download link for the Linux zip
LATEST_URL=$(curl -s https://api.github.com/repos/LunaBaloona/protonhax/releases/latest | grep "browser_download_url.*zip" | cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
    echo "Could not find protonhax release. Please check https://github.com/LunaBaloona/protonhax/releases"
else
    curl -L "$LATEST_URL" -o /tmp/protonhax.zip
    unzip -o /tmp/protonhax.zip -d /tmp/protonhax_extracted
    # Moves the binary specifically (handling potential subfolders in zip)
    find /tmp/protonhax_extracted -type f -name "protonhax" -exec cp {} "$BIN_DIR/" \;
    chmod +x "$BIN_DIR/protonhax"
    rm -rf /tmp/protonhax.zip /tmp/protonhax_extracted
    echo "Protonhax dependency installed to $BIN_DIR"
fi

echo ""
echo "--- Installation Complete! ---"
echo "You can now launch Protonhaxora from your Application Menu."

