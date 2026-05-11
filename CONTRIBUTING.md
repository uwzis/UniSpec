# Contributing to UniSpec

Thank you for your interest in contributing to UniSpec! This document outlines how you can help.

## Ways to Contribute

### 1. Create a Mode

Modes are the heart of UniSpec. Create your own:

```bash
# Structure
.agent/modes/my-mode/
├── mode.toml          # Required: Mode metadata
├── skill.md          # Required: Agent persona
├── workflows/        # Optional: Custom workflows
│   ├── my:workflow.md
├── areas/            # Optional: Area templates
│   ├── staging/
│   └── production/
└── templates/        # Optional: Topic templates
    ├── specs.md
    └── tasks.md
```

See [Simple Mode documentation](../docs/simple-mode/) for details.

### 2. Improve Documentation

- Fix typos
- Add examples
- Write tutorials
- Translate to other languages

### 3. Report Bugs

When reporting bugs, include:
- OS and version
- Steps to reproduce
- Expected vs actual behavior
- UniSpec version (`unispec --version`)

### 4. Feature Requests

We love feature requests! Please describe:
- The problem you're solving
- How you envision it working
- Any existing workarounds

### 5. Code Contributions

1. Fork the repository
2. Create a branch: `git checkout -b feature/my-feature`
3. Write specs for your changes first
4. Implement with tests
5. Submit a PR

## Development Setup

```bash
# Clone the repo
git clone https://github.com/uwzis/UniSpec.git
cd UniSpec

# Build
cargo build

# Run tests
cargo test

# Run with your changes
cargo run -- --help
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Add comments for non-obvious code
- Write specs for new features

## Commit Messages

Format:
```
type: short description

longer explanation if needed

Fixes #123
```

Types:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `refactor:` Code refactoring
- `test:` Adding tests
- `chore:` Maintenance

## Pull Request Process

1. Update README if needed
2. Add tests for new features
3. Ensure all tests pass
4. Update documentation
5. Request review

## Patty's Contribution Tips

> "Every contribution starts with a spec. Even contributions to the spec tool should have specs."

- Write a spec for your contribution
- Keep PRs focused and small
- Respond to feedback constructively
- Test your changes manually

## Code of Conduct

Be respectful. Be helpful. Be kind to Patty.

## Questions?

- Open an issue
- Join the discussion
- Email the maintainers

Thank you for making OpenSDD better! 🦫
