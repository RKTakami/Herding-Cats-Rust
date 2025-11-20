# Contributing to Herding Cats Rust

Thank you for your interest in contributing to Herding Cats Rust! This document provides guidelines and instructions for contributing to the project.

## 🤝 **Code of Conduct**

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## 🚀 **Getting Started**

### **Prerequisites**

- **Rust 1.70+**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo**: Rust's package manager (included with Rust)
- **Git**: Version control system
- **Slint Dependencies**: Platform-specific dependencies for Slint

### **Platform-Specific Setup**

#### **Windows**
```bash
# Install Windows SDK and Visual Studio Build Tools
# Then install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### **macOS**
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Slint dependencies (if needed)
brew install cmake
```

#### **Linux**
```bash
# Install build essentials
# Ubuntu/Debian:
sudo apt-get update
sudo apt-get install build-essential cmake pkg-config

# Fedora/RHEL:
sudo dnf install gcc-c++ cmake pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### **Clone and Build**

```bash
# Clone the repository
git clone https://github.com/RKTakami/herding-cats-rust.git
cd herding-cats-rust

# Build the project
cargo build --release

# Run tests
cargo test

# Run the application
cargo run --release
```

## 📝 **Development Guidelines**

### **Code Style**

- **Rust Style**: Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **Formatting**: Use `cargo fmt` for automatic formatting
- **Linting**: Use `cargo clippy` for additional linting

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy

# Run all quality checks
cargo fmt && cargo clippy && cargo test
```

### **Branch Naming**

- `feature/` - New features
- `bugfix/` - Bug fixes
- `docs/` - Documentation changes
- `chore/` - Maintenance tasks
- `hotfix/` - Critical fixes

Examples:
- `feature/ai-integration`
- `bugfix/database-connection`
- `docs/api-documentation`

### **Commit Messages**

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Examples:
- `feat(database): add SQLx integration`
- `fix(ui): resolve window positioning bug`
- `docs: update installation instructions`
- `test: add integration tests for backup system`

### **Pull Request Guidelines**

1. **Create Feature Branch**: `git checkout -b feature/your-feature`
2. **Write Tests**: Include appropriate tests for new functionality
3. **Update Documentation**: Update relevant documentation
4. **Squash Commits**: Clean up commit history before submitting
5. **Fill PR Template**: Provide detailed description and context

### **Testing**

#### **Test Structure**
- **Unit Tests**: `src/*/mod.rs` - Test individual components
- **Integration Tests**: `tests/` - Test component interactions
- **Performance Tests**: `benches/` - Benchmark critical operations

#### **Running Tests**
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_function_name

# Run tests with output
cargo test -- --nocapture

# Run performance tests
cargo bench

# Run tests in parallel
cargo test -- --test-threads=4
```

### **Documentation**

#### **Code Documentation**
- Use `///` for public API documentation
- Use `//` for implementation comments
- Include examples in documentation

```rust
/// Adds two numbers and returns the result
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

#### **Documentation Standards**
- **README.md**: Project overview and quick start
- **CHANGELOG.md**: Version history and changes
- **docs/**: Technical documentation and guides
- **examples/**: Code examples and configurations

## 🐛 **Issue Reporting**

### **Before Reporting**

1. **Check Existing Issues**: Search for similar issues
2. **Update to Latest Version**: Ensure you're using the latest release
3. **Check Documentation**: Review relevant documentation

### **Bug Reports**

Use the bug report template:

```markdown
**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment (please complete the following information):**
 - OS: [e.g. Windows 10, macOS 12, Ubuntu 20.04]
 - Rust Version: [e.g. 1.70.0]
 - Herding Cats Version: [e.g. 0.50.00]
 - Slint Version: [e.g. 1.14.1]

**Additional context**
Add any other context about the problem here.
```

### **Feature Requests**

Use the feature request template:

```markdown
**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is.

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```

## 🔧 **Development Workflow**

### **Setting Up Development Environment**

1. **Fork the Repository**
2. **Clone Your Fork**
3. **Set Upstream Remote**
```bash
git remote add upstream https://github.com/RKTakami/herding-cats-rust.git
```

4. **Create Development Branch**
```bash
git checkout -b feature/your-feature
```

### **Development Process**

1. **Sync with Upstream**
```bash
git fetch upstream
git rebase upstream/main
```

2. **Make Changes**
3. **Write Tests**
4. **Update Documentation**
5. **Run Quality Checks**
```bash
cargo fmt
cargo clippy
cargo test
```

6. **Commit Changes**
```bash
git add .
git commit -m "feat: add new feature"
```

7. **Push and Create PR**
```bash
git push origin feature/your-feature
# Create PR on GitHub
```

### **Code Review Process**

1. **Automated Checks**: CI will run tests and quality checks
2. **Peer Review**: Maintainers will review the code
3. **Feedback**: Address review feedback
4. **Merge**: Approved PRs will be merged

## 🏗️ **Architecture Guidelines**

### **Service Architecture**

- **ServiceFactory Pattern**: Use dependency injection for service management
- **Separation of Concerns**: Keep business logic separate from UI
- **Error Handling**: Implement comprehensive error handling
- **Testing**: Write tests for all services

### **UI Guidelines**

- **Slint Best Practices**: Follow Slint documentation and examples
- **Responsive Design**: Ensure UI works on different screen sizes
- **Accessibility**: Follow WCAG 2.1 AA guidelines
- **Performance**: Optimize for smooth 60 FPS experience

### **Database Guidelines**

- **SQLx Patterns**: Use typed queries and proper error handling
- **Migration Support**: Include database migrations for schema changes
- **Performance**: Optimize queries and use appropriate indexes
- **Security**: Validate all inputs and use parameterized queries

### **AI Integration Guidelines**

- **Multi-Provider Support**: Design for multiple AI providers
- **Error Handling**: Handle AI service failures gracefully
- **Rate Limiting**: Implement appropriate rate limiting
- **Security**: Never log sensitive information like API keys

## 📊 **Performance Guidelines**

### **Database Performance**
- Use connection pooling
- Implement proper indexing
- Optimize query patterns
- Monitor query performance

### **UI Performance**
- Minimize Slint component updates
- Use efficient data structures
- Implement lazy loading where appropriate
- Monitor frame rates

### **Memory Management**
- Avoid unnecessary allocations
- Use appropriate data structures
- Implement proper cleanup
- Monitor memory usage

## 🧪 **Testing Strategy**

### **Test Coverage**
- Aim for 80%+ code coverage
- Test all public APIs
- Include edge cases and error conditions
- Test performance-critical paths

### **Test Types**
- **Unit Tests**: Individual component testing
- **Integration Tests**: Component interaction testing
- **Performance Tests**: Benchmark critical operations
- **UI Tests**: Slint component testing
- **End-to-End Tests**: Full workflow testing

### **Mocking Strategy**
- Use proper mocking for external dependencies
- Mock AI services for consistent testing
- Mock database operations where appropriate
- Use real databases for integration tests

## 🚀 **Release Process**

### **Versioning**
- Follow Semantic Versioning (MAJOR.MINOR.PATCH)
- Update version in `Cargo.toml`
- Update `CHANGELOG.md`
- Create release notes

### **Release Checklist**
- [ ] All tests pass
- [ ] Code coverage maintained
- [ ] Documentation updated
- [ ] Examples tested
- [ ] Performance benchmarks run
- [ ] Security review completed
- [ ] Release notes prepared

## 📞 **Getting Help**

### **Community Support**
- **GitHub Discussions**: General questions and community help
- **Issues**: Bug reports and feature requests
- **Documentation**: Comprehensive guides and references

### **For Contributors**
- **Code Questions**: Open an issue with `question` label
- **Design Decisions**: Discuss in GitHub discussions
- **Architecture**: Review existing architecture documentation

## 🎯 **Contribution Areas**

### **Core Development**
- Database architecture improvements
- Service layer enhancements
- Performance optimizations
- Security enhancements

### **UI/UX Improvements**
- Slint component improvements
- Accessibility enhancements
- Performance optimizations
- New interface features

### **AI Integration**
- New provider integrations
- Smart routing improvements
- Usage optimization
- Feature enhancements

### **Documentation**
- API documentation
- User guides
- Tutorial creation
- Example development

### **Testing**
- Test coverage improvements
- Performance test development
- Integration test enhancement
- Test infrastructure improvements

## 🙏 **Recognition**

Contributors will be recognized in:
- **Release Notes**: Major contributions acknowledged
- **Contributors File**: List of all contributors
- **Special Recognition**: For significant contributions

Thank you for contributing to Herding Cats Rust! Your efforts help make this project better for everyone.
