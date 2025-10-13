# Contributing to Unicel

Thank you for your interest in contributing to Unicel! This document provides guidelines and information for contributors.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## How to Contribute

### Reporting Bugs

- Check existing issues first
- Provide a clear description and steps to reproduce
- Include system information (OS, Rust version, Node version)
- Add relevant logs or error messages

### Suggesting Features

- Check existing issues and discussions
- Explain the use case and benefits
- Consider how it fits with the unit-aware design philosophy

### Contributing Code

1. **Fork and clone** the repository
2. **Create a branch** for your changes (`git checkout -b feature/amazing-feature`)
3. **Make your changes** following our code style
4. **Write tests** for new functionality
5. **Run tests** to ensure nothing breaks
6. **Commit your changes** (`git commit -m 'Add amazing feature'`)
7. **Push to your branch** (`git push origin feature/amazing-feature`)
8. **Open a Pull Request**

## Development Setup

See [README.md](./README.md) for installation instructions.

## Code Style

### Rust

- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Write doc comments for public APIs using `///`
- Prefer `Result<T>` over panics
- Use descriptive variable names

### TypeScript/React

- Use `prettier` for formatting: `npm run format`
- Use `eslint` for linting: `npm run lint`
- Prefer functional components with hooks
- Use TypeScript types, avoid `any`
- Use descriptive variable names

## Testing

- All new features must include tests
- Unit tests for core functionality
- Integration tests for cross-component features
- Property-based tests for mathematical operations

```bash
# Run all tests
cargo test
npm run test

# Run specific test
cargo test --test unit_conversions
```

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb (Add, Fix, Update, Remove)
- Reference issue numbers when applicable

Examples:
- `Add temperature unit conversions`
- `Fix unit cancellation in compound formulas`
- `Update conversion graph pathfinding algorithm`

## Pull Request Guidelines

- Link to related issues
- Describe what changes were made and why
- Include screenshots for UI changes
- Ensure all tests pass
- Update documentation if needed

## Areas for Contribution

We especially welcome contributions in these areas:

- **Unit definitions**: Adding new units and conversion factors
- **Formula functions**: Implementing new spreadsheet functions
- **UI improvements**: Better user experience and accessibility
- **Documentation**: Examples, guides, and API documentation
- **Testing**: Expanding test coverage
- **MCP servers**: Integration with external data sources
- **Performance**: Optimization and benchmarking

## Questions?

Feel free to open a discussion or issue if you have questions!

## License

By contributing, you agree that your contributions will be licensed under the same MIT OR Apache-2.0 dual license as the project.
