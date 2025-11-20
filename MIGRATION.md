# Migration Guide

## 🔄 **Upgrading to Herding Cats Rust 0.50.00**

This guide helps you migrate from previous versions of Herding Cats Rust to version 0.50.00, which represents a complete architectural rewrite and the first release under the RKTakami organization.

## 📋 **Version Overview**

### **What's Changed in 0.50.00**

Version 0.50.00 represents a **major architectural overhaul** with:

- **Complete Rewrite**: New enterprise-grade architecture
- **New Database System**: SQLx with SQLite WAL mode, FTS5, and vector embeddings
- **Modern UI Framework**: Slint-based interface with professional appearance
- **Service Architecture**: Advanced dependency injection with ServiceFactory pattern
- **Enhanced AI Integration**: Multi-provider support with security and cost tracking
- **Performance Improvements**: <100ms database queries with comprehensive monitoring
- **Security Enhancements**: Encrypted storage, rate limiting, and audit logging

### **Breaking Changes**

⚠️ **Important**: This is a **breaking change** release. Previous versions are not compatible with 0.50.00.

## 🚀 **Migration Process**

### **Step 1: Backup Your Data**

Before starting the migration, ensure you have a complete backup of your existing data.

#### **Automatic Backup (Recommended)**

```bash
# Navigate to your existing installation
cd /path/to/old/herding-cats-rust

# Create a complete backup
cargo run -- --backup --format=json --include-all

# This creates a backup file in the storage/backups/ directory
```

#### **Manual Backup**

If automatic backup is not available in your version:

1. **Locate Data Files**:
   - Database: `data/database.db` or `data/*.db`
   - Storage: `storage/context.json`, `storage/project.json`, `storage/tasks.json`
   - Backups: `storage/backups/` directory

2. **Copy Data**:
```bash
# Create backup directory
mkdir herding-cats-backup-$(date +%Y%m%d)

# Copy data files
cp -r data/ herding-cats-backup-$(date +%Y%m%d)/
cp -r storage/ herding-cats-backup-$(date +%Y%m%d)/
```

### **Step 2: Install Version 0.50.00**

#### **Download and Install**

```bash
# Clone the new version
git clone https://github.com/RKTakami/herding-cats-rust.git
cd herding-cats-rust

# Build the application
cargo build --release

# Verify installation
cargo run -- --version
# Should display: herding-cats-rust 0.50.00
```

### **Step 3: Data Migration**

#### **Automatic Migration (For Recent Versions)**

If you're migrating from a version that supports the new backup format:

```bash
# Copy your backup file to the new installation
cp /path/to/backup/file.json herding-cats-rust/storage/migration/

# Start the application - it will detect and migrate the backup
cargo run --release
```

#### **Manual Data Migration**

For older versions or custom data formats:

1. **Prepare Migration Directory**:
```bash
# Create migration directory
mkdir -p storage/migration

# Copy your old data files
cp /path/to/backup/data/*.db storage/migration/
cp /path/to/backup/storage/*.json storage/migration/
```

2. **Start Migration Process**:
```bash
# Run the application with migration mode
cargo run --release -- --migration-mode

# Follow the on-screen instructions
```

### **Step 4: Configuration Migration**

#### **Configuration File Update**

Your old configuration file needs to be updated for the new version:

```toml
# Old configuration format (pre-0.50.00)
[database]
path = "data/database.db"
backup_interval = 300

[ai]
provider = "openai"
api_key = "your-key-here"

# New configuration format (0.50.00+)
[database]
path = "data/database.db"
wal_mode = true
backup_interval = 300
full_text_search = true
vector_embeddings = true

[ai]
default_provider = "openai"
usage_limit = 1000
rate_limit = 60

[security]
enable_encryption = true
enable_audit_logging = true
enable_rate_limiting = true

[ui]
theme = "default"
language = "en"
auto_save = true
```

#### **Environment Variables**

Update any environment variables:

```bash
# Old format
export HERDING_CATS_DB_PATH=data/database.db
export OPENAI_API_KEY=your-key-here

# New format
export HERDING_CATS_DB_PATH=data/database.db
export HERDING_CATS_ENCRYPTION_KEY=your-encryption-key
export HERDING_CATS_OPENAI_API_KEY=your-key-here
export HERDING_CATS_SECURITY_MODE=true
```

### **Step 5: Verify Migration**

#### **Data Integrity Check**

```bash
# Run integrity check
cargo run --release -- --check-integrity

# Verify all projects are accessible
cargo run --release -- --list-projects

# Test AI integration
cargo run --release -- --test-ai
```

#### **Performance Validation**

```bash
# Run performance benchmarks
cargo run --release -- --benchmark

# Check database performance
cargo run --release -- --db-stats

# Monitor memory usage
cargo run --release -- --memory-stats
```

## 🔄 **Migration Scenarios**

### **Scenario 1: Direct Upgrade (v0.1.x → v0.50.00)**

**For users upgrading from early prototype versions:**

1. **Backup**: Create complete backup using old application
2. **Export**: Export projects to JSON format if available
3. **Install**: Install new version
4. **Import**: Use migration tools to import data
5. **Validate**: Verify all data is accessible

### **Scenario 2: Custom Installation Migration**

**For users with custom configurations or modifications:**

1. **Document**: Document all custom settings and modifications
2. **Backup**: Create complete backup including custom files
3. **Review**: Review [CHANGELOG.md](CHANGELOG.md) for breaking changes
4. **Migrate**: Use manual migration process
5. **Reconfigure**: Re-apply custom settings using new configuration format

### **Scenario 3: Enterprise Deployment Migration**

**For organizations with multiple users or custom deployments:**

1. **Plan**: Create detailed migration plan with rollback procedures
2. **Test**: Test migration on staging environment first
3. **Backup**: Create multiple backup copies
4. **Migrate**: Perform migration during maintenance window
5. **Validate**: Comprehensive testing of all features
6. **Train**: Train users on new interface and features

## 🛠️ **Troubleshooting Migration Issues**

### **Common Migration Problems**

#### **Issue: Backup Import Fails**

**Symptoms**: Migration process fails with import errors

**Solutions**:
1. Verify backup file integrity
2. Check file permissions
3. Try manual data copying
4. Contact support with error logs

```bash
# Check backup integrity
cargo run --release -- --verify-backup /path/to/backup/file

# Manual data copy as fallback
cp storage/migration/database.db data/database.db
cp storage/migration/context.json storage/context.json
```

#### **Issue: Configuration Errors**

**Symptoms**: Application fails to start with configuration errors

**Solutions**:
1. Use configuration validation tool
2. Compare with example configuration
3. Reset to defaults and reconfigure

```bash
# Validate configuration
cargo run --release -- --validate-config

# Reset to defaults
mv config.toml config.toml.backup
cargo run --release  # Creates new default config
```

#### **Issue: Missing Dependencies**

**Symptoms**: Build or runtime errors related to missing dependencies

**Solutions**:
1. Update Rust and Cargo
2. Install platform-specific dependencies
3. Clean and rebuild

```bash
# Update Rust
rustup update

# Clean build
cargo clean

# Rebuild
cargo build --release
```

#### **Issue: Performance Problems**

**Symptoms**: Application runs slowly after migration

**Solutions**:
1. Run database optimization
2. Check system requirements
3. Update configuration for performance

```bash
# Optimize database
cargo run --release -- --optimize-db

# Check performance
cargo run --release -- --performance-check

# Update configuration for better performance
cargo run --release -- --tune-performance
```

### **Data Recovery Options**

#### **If Migration Fails Completely**

1. **Restore Original Installation**: Return to original version
2. **Manual Data Extraction**: Extract data manually from database files
3. **Professional Assistance**: Contact support for complex migrations

#### **Partial Data Recovery**

```bash
# Export specific projects
cargo run --release -- --export-project <project-name> --format=json

# Export specific data types
cargo run --release -- --export-data --type=hierarchy
cargo run --release -- --export-data --type=codex
cargo run --release -- --export-data --type=notes
```

## 📊 **Post-Migration Validation**

### **Functional Testing**

1. **Core Features**:
   - [ ] Create new project
   - [ ] Add writing tools content
   - [ ] Test AI integration
   - [ ] Verify search functionality
   - [ ] Test backup and restore

2. **Data Integrity**:
   - [ ] All projects accessible
   - [ ] All data preserved
   - [ ] Cross-references working
   - [ ] Search results accurate
   - [ ] AI features functional

3. **Performance**:
   - [ ] Application startup < 5 seconds
   - [ ] Database queries < 100ms
   - [ ] UI responsiveness smooth
   - [ ] Memory usage reasonable
   - [ ] No performance regressions

### **Security Validation**

1. **Encryption**: Verify sensitive data is encrypted
2. **Access Control**: Test user permissions
3. **Audit Logging**: Verify security events are logged
4. **API Security**: Test AI provider security

### **Backup Verification**

1. **Create Test Backup**: Verify backup system works
2. **Restore Test**: Test backup restoration
3. **Integrity Check**: Verify backup integrity
4. **Schedule Test**: Test automatic backups

## 📞 **Migration Support**

### **Getting Help**

If you encounter issues during migration:

1. **Documentation**: Review [SUPPORT.md](SUPPORT.md) for troubleshooting
2. **Issues**: Create [GitHub Issue](https://github.com/RKTakami/herding-cats-rust/issues) with `migration` label
3. **Email**: Contact migration@herdingcats.dev for complex migrations
4. **Community**: Ask for help in [GitHub Discussions](https://github.com/RKTakami/herding-cats-rust/discussions)

### **Professional Migration Services**

For enterprise users or complex migrations:

- **Data Migration**: Professional data migration services
- **Custom Integration**: Custom migration tool development
- **Training**: User training and documentation
- **Support**: Priority support during migration period

Contact enterprise@herdingcats.dev for professional services.

## 🎯 **Next Steps After Migration**

### **Configuration Optimization**

1. **Review Settings**: Optimize configuration for your use case
2. **AI Providers**: Configure preferred AI providers
3. **Performance Tuning**: Adjust performance settings
4. **Security Settings**: Review and enhance security configuration

### **Feature Exploration**

1. **New Tools**: Explore the 8 specialized writing tools
2. **AI Features**: Configure and test AI integration
3. **Database Features**: Utilize advanced database capabilities
4. **Performance Monitoring**: Set up monitoring and alerts

### **Integration Setup**

1. **Backup Systems**: Configure automatic backups
2. **External Tools**: Integrate with external writing tools
3. **Cloud Services**: Set up cloud backup (when available)
4. **Team Features**: Configure team collaboration (future releases)

## 📚 **Additional Resources**

- [CHANGELOG.md](CHANGELOG.md) - Complete list of changes in 0.50.00
- [README.md](README.md) - Updated documentation for new features
- [docs/](docs/) - Technical documentation and guides
- [examples/](examples/) - Configuration examples and templates
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines

## 🙏 **Migration Success**

Congratulations on successfully migrating to Herding Cats Rust 0.50.00! You now have access to:

- **Enterprise-Grade Architecture**: Robust, scalable, and secure
- **Advanced Writing Tools**: 8 specialized tools for comprehensive writing support
- **AI Integration**: Multi-provider AI assistance with security and cost control
- **Performance Monitoring**: Real-time performance tracking and optimization
- **Modern UI**: Professional interface with excellent user experience

If you need any assistance during or after your migration, please don't hesitate to reach out through our support channels.

**Welcome to the future of writing assistance! 🚀**
