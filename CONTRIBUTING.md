# Contributing to Notion Killer

First off, thank you for considering contributing to Notion Killer! It's people like you that make Notion Killer such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples to demonstrate the steps**
- **Describe the behavior you observed after following the steps**
- **Explain which behavior you expected to see instead and why**
- **Include screenshots and animated GIFs if possible**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- **Use a clear and descriptive title**
- **Provide a step-by-step description of the suggested enhancement**
- **Provide specific examples to demonstrate the steps**
- **Describe the current behavior and explain which behavior you expected to see instead**
- **Explain why this enhancement would be useful**

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code lints
6. Issue that pull request!

## Development Setup

### Prerequisites

- Node.js 20+
- Rust 1.75+
- Docker & Docker Compose
- pnpm 9+

### Setting Up Your Development Environment

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/notion-killer.git
cd notion-killer

# Add upstream remote
git remote add upstream https://github.com/notion-killer/notion-killer.git

# Install dependencies
pnpm install

# Start infrastructure
docker compose -f docker/docker-compose.yml up -d

# Setup backend
cd packages/backend
cp .env.example .env
cargo run

# Setup frontend (new terminal)
cd packages/web
pnpm dev
```

### Project Structure

```
packages/
├── backend/        # Rust API (Axum)
├── web/            # SvelteKit frontend
├── mobile/         # Flutter app
└── shared/         # Shared code (CRDT)
```

## Style Guides

### Git Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools

**Examples:**
```
feat(editor): add toggle block support
fix(sync): resolve conflict when offline
docs: update installation instructions
refactor(api): simplify auth middleware
```

### Rust Style Guide

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Write documentation for public APIs

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Run tests
cargo test
```

### TypeScript/Svelte Style Guide

- Use TypeScript for all new code
- Follow the existing code style
- Use ESLint and Prettier

```bash
# Lint code
pnpm lint

# Format code
pnpm format

# Type check
pnpm check
```

### CSS Style Guide

- Use TailwindCSS utilities when possible
- Follow BEM naming for custom CSS
- Keep styles co-located with components

## Testing

### Backend Tests

```bash
cd packages/backend

# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with logs
cargo test -- --nocapture
```

### Frontend Tests

```bash
cd packages/web

# Run unit tests
pnpm test

# Run with coverage
pnpm test:coverage

# Run E2E tests
pnpm test:e2e
```

## Documentation

- Update README.md if you change functionality
- Add JSDoc comments for TypeScript functions
- Add rustdoc comments for Rust functions
- Update API documentation for endpoint changes

## Review Process

1. A maintainer will review your PR
2. They may request changes or ask questions
3. Once approved, a maintainer will merge your PR
4. Your contribution will be part of the next release!

## Recognition

Contributors are recognized in:
- The README.md file
- Release notes
- Our website's contributors page

Thank you for contributing! 🎉
