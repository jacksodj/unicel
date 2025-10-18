# Unicel Viewer Support

Welcome to Unicel Viewer support! This page provides help and resources for using the iOS app.

## About Unicel Viewer

Unicel Viewer is a read-only spreadsheet viewer for .usheet files created with the Unicel desktop application. It allows you to view unit-aware spreadsheets on your iPhone or iPad with full unit conversion and formula display capabilities.

## Getting Started

### Opening Files

1. Tap the "Open File" button on the home screen
2. Navigate to your .usheet file (from Files app, iCloud Drive, or other sources)
3. Select the file to view it

### Viewing Features

- **Multiple Sheets:** Swipe left/right or use the sheet tabs at the bottom to navigate between sheets
- **Unit Display:** Toggle between Metric and Imperial units using the display toggle button
- **Formula View:** Tap any cell to see its formula and calculated value
- **Zoom:** Pinch to zoom in/out on the spreadsheet grid
- **Scroll:** Swipe to scroll through large spreadsheets

## Frequently Asked Questions

### Can I edit files in Unicel Viewer?

No, the iOS version is a read-only viewer. To create and edit .usheet files, use the Unicel desktop application available at:
https://github.com/jacksodj/unicel

### What file formats are supported?

Unicel Viewer only supports .usheet and .usheet.json files created by the Unicel desktop application.

### Why can't I see my file?

Make sure:
- The file has a .usheet or .usheet.json extension
- The file is accessible from the iOS Files app
- The file was created with a compatible version of Unicel (v0.5.0 or later)

### How do unit conversions work?

Unicel treats units as first-class data types. When you toggle between Metric and Imperial display modes, values are automatically converted while preserving the original data. For example:
- 100 feet displays as 30.48 meters in Metric mode
- 50 mph displays as 80.47 km/h in Metric mode
- Temperature, mass, volume, and many other units are supported

### Does this app require internet?

No, Unicel Viewer works completely offline. All file processing happens locally on your device.

### Where is my data stored?

Unicel Viewer does not store any data. It only accesses files you explicitly open, and all processing happens in memory on your device.

## Supported Units

Unicel supports hundreds of units across many domains:

- **Length:** meters, feet, miles, kilometers, inches, yards, etc.
- **Mass:** kilograms, pounds, grams, ounces, tons, etc.
- **Time:** seconds, minutes, hours, days, weeks, months, years
- **Temperature:** Celsius, Fahrenheit, Kelvin
- **Currency:** USD, EUR, GBP, JPY, and many more
- **Digital Storage:** bytes, KB, MB, GB, TB, PB
- **Energy:** joules, calories, kWh, BTU
- **Power:** watts, horsepower, BTU/hour
- **Area:** square meters, square feet, acres, hectares
- **Volume:** liters, gallons, cubic meters, fluid ounces
- And many more...

## Reporting Issues

Found a bug or have a feature request? Please report it:

1. **GitHub Issues:** https://github.com/jacksodj/unicel/issues
2. **Email:** support@unicel.app

When reporting an issue, please include:
- iOS version
- Device model (iPhone/iPad)
- App version (found in Settings)
- Steps to reproduce the issue
- Screenshots if applicable

## Feature Requests

We'd love to hear your ideas for improving Unicel Viewer! Submit feature requests through:
- GitHub Issues: https://github.com/jacksodj/unicel/issues
- Email: feedback@unicel.app

## Version History

### Version 0.5.1 (Current)
- Initial iOS release
- Read-only .usheet file viewer
- Metric/Imperial display toggle
- Multi-sheet navigation
- Full unit conversion support
- Touch-optimized spreadsheet grid

## Get the Desktop Version

For full editing capabilities, download Unicel for macOS, Windows, or Linux:
https://github.com/jacksodj/unicel

## Contact

- **Email:** support@unicel.app
- **GitHub:** https://github.com/jacksodj/unicel
- **Privacy Policy:** https://github.com/jacksodj/unicel/blob/main/docs/app-store/PRIVACY_POLICY.md

---

**Thank you for using Unicel Viewer!**
