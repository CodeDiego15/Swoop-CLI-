#!/bin/bash

APP_NAME="My Terminal App"
BINARY_NAME="my_terminal_app"
APP_DIR="$APP_NAME.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"

# Create directory structure
mkdir -p "$MACOS_DIR"

# Copy binary and Info.plist
cp "../target/release/$BINARY_NAME" "$MACOS_DIR/"
cp "Info.plist" "$CONTENTS_DIR/"

# Set executable permissions
chmod +x "$MACOS_DIR/$BINARY_NAME"

echo "Created $APP_DIR"
