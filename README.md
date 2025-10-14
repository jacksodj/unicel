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
git clone https://github.com/jacksodj/unicel.git
cd unicel

# Install dependencies
npm install

# Run development server
npm run tauri:dev

# Build for production
npm run tauri:build
```

## Development

### Task Tracking

All tasks are tracked in `docs/TASKS.md` - this is the single source of truth for project progress.

- Mark tasks complete by changing `[ ]` to `[x]` in TASKS.md
- Add future ideas to the "Future Backlog" section
- Do not create TODO files in other locations

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

**Current Phase**: Phase 6 - Tauri Integration (Week 19 of 24)

**Phase 0 - Foundation:** ✅ Complete (5/5 tasks)
**Phase 1 - Core Unit System:** ✅ Complete (14/14 tasks)
**Phase 2 - Cell & Formula Engine:** ✅ Complete (22/22 tasks)
**Phase 3 - Basic Workbook & Sheet:** ✅ Complete (15/15 tasks)
**Phase 4 - File Format:** ✅ Complete (10/10 tasks)
**Phase 5 - Basic UI:** ✅ Complete (20/20 tasks)
- [x] Grid component with cell rendering and selection
- [x] Cell editing with inline input and double-click support
- [x] Input parsing for "value unit" format (e.g., "100 m")
- [x] Formula bar with editing capabilities
- [x] Keyboard shortcuts (Enter to commit, Escape to cancel)
- [x] Ribbon with display toggle (As Entered/Metric/Imperial)
- [x] File menu with New/Open/Save/Save As
- [x] Status bar with cell count and unit indicator
- [x] Toast notifications for user feedback
- [x] Loading overlay for async operations
- [x] Tooltip component for warnings

**Overall Progress:** 86/126 tasks (68.3%)

See detailed tracking in:
- [PROJECT_PLAN.md](./docs/PROJECT_PLAN.md) - Complete implementation plan with timeline
- [TASKS.md](./docs/TASKS.md) - Detailed task checklist (mark tasks as you complete them)
- [Design Document](./requirements/Unit%20Aware%20Spreadsheet%20Design.md) - Full technical specification

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

This project is dual-licensed under MIT OR Apache-2.0. See [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) for details.

## Documentation

**Planning & Tracking:**
- [PROJECT_PLAN.md](./docs/PROJECT_PLAN.md) - 24-week implementation plan with phases and deliverables
- [TASKS.md](./docs/TASKS.md) - Detailed task checklist for tracking progress

**Architecture & Design:**
- [CLAUDE.md](./CLAUDE.md) - Development guidance for Claude Code
- [Design Document](./requirements/Unit%20Aware%20Spreadsheet%20Design.md) - Complete technical specification
- [MLP Requirements](./requirements/MLP%20Requirements%20-%20Unit%20Conversion.pdf) - Original requirements

**Use Cases:**
- [AWS Pricing](./requirements/use%20cases/AWS%20Pricing%20Use%20Case.md)
- [Construction Estimator](./requirements/use%20cases/Construction%20Estimator%20Use%20Case.md)
- [Investment Portfolio](./requirements/use%20cases/Investment%20Portfolio%20Use%20Case.md)

## Community

- GitHub Issues: [Report bugs or request features](https://github.com/jacksodj/unicel/issues)
- Discussions: [Ask questions and share ideas](https://github.com/jacksodj/unicel/discussions)

## Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [React](https://react.dev/) - UI framework
- [SQLite](https://www.sqlite.org/) - Database engine
