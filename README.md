# Unicel - Unit-Aware Spreadsheet

A next-generation open-source spreadsheet application that treats units as first-class data types, enabling dimensional analysis, automatic unit conversion, and type-safe calculations.

## Core Innovation

Units are data, not formatting. Values stored as `(number, unit)` tuples enable operations like `$100/hr × 720hr/month → $72,000/month` to work naturally with automatic unit cancellation.

## Key Features

- **Units as First-Class Data**: Values stored with their units, preserved through all operations
- **Non-Destructive Display Conversion**: Toggle between Metric ↔ Imperial without changing stored data
- **Automatic Unit Cancellation**: Intelligent formula operations with dimensional analysis
- **SQL-Queryable Tables**: Entity-aware structured data with unit-aware filtering
- **AI-Native via MCP**: Integration with AI tools through Model Context Protocol
- **Open Source & LLM-Friendly**: JSON file format, comprehensive APIs

## Technology Stack

- **Backend**: Rust with Tauri for native performance
- **Frontend**: React + TypeScript + Tailwind CSS
- **Database**: SQLite (in-memory for runtime)
- **File Format**: JSON (MVP), SQLite hybrid (future)
- **Testing**: cargo test, proptest, vitest

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) 18+ and npm
- Platform-specific build tools (see [Tauri prerequisites](https://tauri.app/v2/guides/prerequisites))

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/unicel.git
cd unicel

# Install dependencies
npm install

# Run development server
npm run tauri:dev

# Build for production
npm run tauri:build
```

## Development

### Project Structure

```
unicel/
├── src/                    # Frontend React/TypeScript
│   ├── components/         # UI components
│   ├── hooks/              # React hooks
│   ├── store/              # Zustand state management
│   └── utils/              # Utility functions
├── src-tauri/              # Rust backend
│   └── src/
│       ├── core/           # Calculation engine
│       │   ├── units/      # Unit system
│       │   ├── formula/    # Formula parser/evaluator
│       │   ├── table/      # Table system
│       │   └── workbook/   # Workbook management
│       ├── formats/        # File I/O (JSON, Excel)
│       └── mcp/            # MCP integration
├── tests/                  # Rust tests
├── docs/                   # Documentation
└── examples/               # Example workbooks
```

### Running Tests

```bash
# Rust tests
cargo test

# Frontend tests
npm run test

# Specific test file
cargo test --test unit_conversions
```

### Code Style

```bash
# Format code
cargo fmt
npm run format

# Lint code
cargo clippy
npm run lint
```

## Implementation Status

**Current Phase**: Phase 0 - Foundation (Weeks 1-2)

- [x] Project setup
- [x] Core data structures defined
- [x] Technology stack selected
- [ ] Unit system implementation
- [ ] Formula engine
- [ ] Table system
- [ ] UI implementation
- [ ] MCP integration

See [Unit Aware Spreadsheet Design.md](./Unit%20Aware%20Spreadsheet%20Design.md) for full roadmap.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

This project is dual-licensed under MIT OR Apache-2.0. See [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) for details.

## Documentation

- [CLAUDE.md](./CLAUDE.md) - Guidance for Claude Code
- [Design Document](./Unit%20Aware%20Spreadsheet%20Design.md) - Comprehensive design specification
- [MLP Requirements](./MLP%20Requirements%20-%20Unit%20Conversion.pdf) - Original requirements

## Community

- GitHub Issues: [Report bugs or request features](https://github.com/yourusername/unicel/issues)
- Discussions: [Ask questions and share ideas](https://github.com/yourusername/unicel/discussions)

## Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [React](https://react.dev/) - UI framework
- [SQLite](https://www.sqlite.org/) - Database engine
