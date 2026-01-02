#!/bin/bash
set -e

echo "--- Protonhaxora Universal Installer ---"

# 1. Create local bin and application folders if they don't exist
mkdir -p ~/.local/bin
mkdir -p ~/.local/share/applications
mkdir -p ~/.local/share/icons/hicolor/256x256/apps

# 2. Build the project
echo "Building Protonhaxora..."
cargo build --release

# 3. Install Protonhaxora Binary
cp target/release/protonhaxora ~/.local/bin/

# 4. Install Icon and Desktop Entry
# This ensures the icon appears in the App Menu and File Manager
cp ui/icon.png ~/.local/share/icons/hicolor/256x256/apps/protonhaxora.png

cat <<EOF > ~/.local/share/applications/protonhaxora.desktop
[Desktop Entry]
Name=Protonhaxora
Exec=$HOME/.local/bin/protonhaxora
Icon=protonhaxora
Type=Application
Categories=Game;Utility;
Comment=Launch Aurora via Protonhax
Terminal=false
EOF

# 5. Fetch and Install Protonhax from your fork
echo "Fetching latest protonhax from LunaBaloona fork..."
# This finds the latest release zip URL automatically
LATEST_URL=$(curl -s https://api.github.com/repos/LunaBaloona/protonhax/releases/latest | grep "browser_download_url.*zip" | cut -d '"' -f 4)

curl -L $LATEST_URL -o protonhax_latest.zip
unzip -o protonhax_latest.zip -d /tmp/protonhax_temp
cp /tmp/protonhax_temp/protonhax ~/.local/bin/
chmod +x ~/.local/bin/protonhax

echo "--- Installation Complete! ---"
echo "If you haven't yet, you should install Aurora before running Prootonhaxora."
