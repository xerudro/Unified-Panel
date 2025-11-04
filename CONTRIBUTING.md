# Contributing to Unified Hosting Panel

Thank you for your interest in contributing to the Unified Hosting Panel! We welcome contributions from the community.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/unified-panel.git
   cd unified-panel
   ```
3. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

1. Install Rust (1.75+)
2. Install PostgreSQL (15+)
3. Set up the development environment:
   ```bash
   cd backend
   cp .env.example .env
   # Edit .env with your local configuration
   ```
4. Run migrations:
   ```bash
   cargo install sqlx-cli
   sqlx migrate run
   ```
5. Start the development server:
   ```bash
   cargo run
   ```

## Code Style

### Rust
- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Write tests for new functionality

### Templates
- Use semantic HTML
- Follow Tailwind CSS conventions
- Ensure accessibility (ARIA labels, keyboard navigation)
- Test on both light and dark themes

### Commit Messages
Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
feat: add user export functionality
fix: resolve login redirect issue
docs: update API documentation
refactor: simplify server creation logic
test: add tests for auth service
chore: update dependencies
```

## Pull Request Process

1. **Update documentation** if you're changing functionality
2. **Add tests** for new features
3. **Ensure all tests pass**: `cargo test`
4. **Run linting**: `cargo clippy`
5. **Format code**: `cargo fmt`
6. **Update CHANGELOG.md** with your changes
7. **Submit a pull request** with a clear description

### PR Description Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
How has this been tested?

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] No new warnings
- [ ] Tests added/updated
- [ ] All tests passing
```

## Code Review

All submissions require review before merging. Reviewers will look for:

- Code quality and style
- Test coverage
- Documentation completeness
- Performance implications
- Security considerations

## Areas for Contribution

We especially welcome contributions in these areas:

- **Features**: Implementing items from the roadmap
- **Bug Fixes**: Resolving open issues
- **Documentation**: Improving guides and API docs
- **Tests**: Increasing test coverage
- **Performance**: Optimizations and benchmarks
- **Security**: Security audits and improvements
- **UI/UX**: Design improvements and accessibility

## Reporting Bugs

Create an issue on GitHub with:

- **Title**: Clear, descriptive title
- **Description**: Detailed description of the bug
- **Steps to Reproduce**: Numbered steps to reproduce the issue
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: OS, Rust version, database version
- **Screenshots**: If applicable

## Feature Requests

Create an issue with:

- **Title**: Clear feature title
- **Problem**: What problem does this solve?
- **Proposed Solution**: Your suggested implementation
- **Alternatives**: Other approaches considered
- **Additional Context**: Any other relevant information

## Community

- Be respectful and inclusive
- Help others in discussions
- Provide constructive feedback
- Follow our [Code of Conduct](CODE_OF_CONDUCT.md)

## Questions?

If you have questions, feel free to:

- Open a discussion on GitHub
- Ask in the issue comments
- Contact the maintainers

Thank you for contributing! 🎉
