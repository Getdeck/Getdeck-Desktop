#!/bin/env bash

download_url=$(curl https://api.github.com/repos/Getdeck/beiboot/releases/latest | jq -r ".assets[0].browser_download_url")
curl -o download.zip -L $download_url
unzip -d /tmp/ -o download.zip
mkdir -p src-tauri/bin
mv -f /tmp/beibootctl src-tauri/bin/beibootctl-x86_64-unknown-linux-gnu
chmod +x src-tauri/bin/beibootctl-x86_64-known-linux-gnu
rm -rf download.zip
