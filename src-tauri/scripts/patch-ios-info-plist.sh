#!/bin/bash
# Patch the iOS Info.plist to ensure proper document type handling
# This fixes the issue where the file picker shows photos instead of documents

PLIST_PATH="$1"

if [ -z "$PLIST_PATH" ]; then
    echo "Usage: $0 <path-to-Info.plist>"
    exit 1
fi

if [ ! -f "$PLIST_PATH" ]; then
    echo "Error: Info.plist not found at $PLIST_PATH"
    exit 1
fi

echo "Patching Info.plist at: $PLIST_PATH"

# Use PlistBuddy to modify the Info.plist
/usr/libexec/PlistBuddy -c "Delete :UTExportedTypeDeclarations:0:UTTypeConformsTo" "$PLIST_PATH" 2>/dev/null || true
/usr/libexec/PlistBuddy -c "Add :UTExportedTypeDeclarations:0:UTTypeConformsTo array" "$PLIST_PATH" 2>/dev/null || true
/usr/libexec/PlistBuddy -c "Add :UTExportedTypeDeclarations:0:UTTypeConformsTo:0 string public.json" "$PLIST_PATH" 2>/dev/null || true
/usr/libexec/PlistBuddy -c "Add :UTExportedTypeDeclarations:0:UTTypeConformsTo:1 string public.data" "$PLIST_PATH" 2>/dev/null || true
/usr/libexec/PlistBuddy -c "Add :UTExportedTypeDeclarations:0:UTTypeConformsTo:2 string public.content" "$PLIST_PATH" 2>/dev/null || true

# Add MIME type to UTTypeTagSpecification
/usr/libexec/PlistBuddy -c "Add :UTExportedTypeDeclarations:0:UTTypeTagSpecification:public.mime-type array" "$PLIST_PATH" 2>/dev/null || true
/usr/libexec/PlistBuddy -c "Add :UTExportedTypeDeclarations:0:UTTypeTagSpecification:public.mime-type:0 string application/json" "$PLIST_PATH" 2>/dev/null || true

echo "Info.plist patched successfully"
