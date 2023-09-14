#!/bin/bash

REPO_OWNER="tnickel-web"
REPO_NAME="weather-wand"
INSTALL_DIR="/usr/local/bin"
EXECUTABLE_NAME="weather-wand"

API_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"

DOWNLOAD_URL=$(curl -s "$API_URL" | grep -Eo '"browser_download_url": "[^"]+"' | grep -v ".exe\"" | cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Failed to retrieve the latest release information."
    exit 1
fi

echo "Downloading the latest release..."
curl -L -o "$INSTALL_DIR/$EXECUTABLE_NAME" "$DOWNLOAD_URL"
chmod +x "$INSTALL_DIR/$EXECUTABLE_NAME"

if [ $? -eq 0 ]; then
    echo "Installation completed successfully."
else
    echo "Installation failed."
    exit 1
fi

exit 0