#!/bin/bash

# This script is used to rollout the new version of Getdeck Desktop via Tauri Updater.

VERSION=$1

if [ -z "$VERSION" ]
then
    echo "Please provide a version number."
    exit 1
fi

MACOS_SIG=$(wget -q -O- https://github.com/Getdeck/Getdeck-Desktop/releases/download/$VERSION/Getdeck-Desktop.app.tar.gz.sig)
LINUX_SIG=$(wget -q -O- https://github.com/Getdeck/Getdeck-Desktop/releases/download/$VERSION/getdeck-desktop_${VERSION}_amd64.AppImage.tar.gz.sig)
WINDOWS_SIG=$(wget -q -O- https://github.com/Getdeck/Getdeck-Desktop/releases/download/$VERSION/getdeck-desktop_${VERSION}_x64_en-US.msi.zip.sig)

MACOS_DOWNLOAD="https://github.com/Getdeck/Getdeck-Desktop/releases/download/$VERSION/Getdeck-Desktop.app.tar.gz"
LINUX_DOWNLOAD="https://github.com/Getdeck/Getdeck-Desktop/releases/download/$VERSION/getdeck-desktop_${VERSION}_amd64.AppImage.tar.gz"
WINDOWS_DOWNLOAD="https://github.com/Getdeck/Getdeck-Desktop/releases/download/$VERSION/getdeck-desktop_${VERSION}_x64_en-US.msi.zip"

NOTES="Getdeck Desktop $VERSION is now available."


jq --arg MACOS_SIG "$MACOS_SIG" --arg LINUX_SIG "$LINUX_SIG" --arg WINDOWS_SIG "$WINDOWS_SIG" '.platforms."darwin-x86_64".signature = $MACOS_SIG | 
    .platforms."linux-x86_64".signature = $LINUX_SIG |
    .platforms."windows-x86_64".signature = $WINDOWS_SIG' updater.json > updater.json.tmp && mv updater.json.tmp updater.json

jq --arg MACOS_DOWNLOAD "$MACOS_DOWNLOAD" --arg LINUX_DOWNLOAD "$LINUX_DOWNLOAD" --arg WINDOWS_DOWNLOAD "$WINDOWS_DOWNLOAD" '.platforms."darwin-x86_64".url = $MACOS_DOWNLOAD |
    .platforms."linux-x86_64".url = $LINUX_DOWNLOAD |
    .platforms."windows-x86_64".url = $WINDOWS_DOWNLOAD' updater.json > updater.json.tmp && mv updater.json.tmp updater.json

jq --arg VERSION "$VERSION" '.version = $VERSION' updater.json > updater.json.tmp && mv updater.json.tmp updater.json
jq --arg NOTES "$NOTES" '.notes = $NOTES' updater.json > updater.json.tmp && mv updater.json.tmp updater.json


