#!/bin/bash

# This script is used to bump the version of Getdeck Desktop in all relevant files.
# This script requires toml-cli (https://github.com/gnprice/toml-cli) to work correctly.

VERSION=$1

if [ -z "$VERSION" ]
then
    echo "Please provide a version number."
    exit 1
fi

jq --arg VERSION "$VERSION" '.version = $VERSION' package.json > package.json.tmp && mv package.json.tmp package.json
jq --arg VERSION "$VERSION" '.version = $VERSION' src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp && mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json
toml set src-tauri/Cargo.toml package.version "$VERSION" > src-tauri/Cargo.toml.tmp && mv src-tauri/Cargo.toml.tmp src-tauri/Cargo.toml
