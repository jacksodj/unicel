#!/bin/bash

echo "=== Fixing Xcode Setup for iOS Development ==="
echo ""

# Check current xcode-select path
echo "Current xcode-select path:"
xcode-select -p
echo ""

# Fix xcode-select path (requires sudo)
echo "Switching xcode-select to Xcode.app..."
echo "This requires sudo access. Please enter your password when prompted."
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer

if [ $? -ne 0 ]; then
    echo "ERROR: Failed to switch xcode-select path"
    exit 1
fi

echo "✓ xcode-select path updated"
echo ""

# Accept Xcode license if needed
echo "Accepting Xcode license..."
sudo xcodebuild -license accept 2>/dev/null || echo "License already accepted or not needed"
echo ""

# Verify the fix
echo "=== Verification ==="
echo "New xcode-select path:"
xcode-select -p
echo ""

echo "Testing xcrun simctl (this should now work):"
xcrun simctl list devices available | head -20
echo ""

echo "=== Setup Complete ==="
echo "You can now run: npm run tauri:ios:dev"
