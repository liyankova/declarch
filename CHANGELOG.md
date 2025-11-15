# Changelog

All notable changes to declarch will be documented in this file.

## [0.2.0] - 2025-11-15

### Added
- Initial MVP release
- `declarch init` - Initialize setup with auto-detection
- `declarch host` - Host management (enable, status, list)
- `declarch edit` - Edit configuration files
- `declarch check` - Configuration validation
- `declarch info` - System status display
- `declarch clean` - Cache cleanup
- `declarch sync` - Package synchronization (dry-run mode)
- KDL-based configuration format
- State management via JSON
- Multi-backend support (pacman, AUR, flatpak)
- Interactive prompts and colored output
- Package resolution with overrides and exclusions

### Planned (Phase 2)
- Actual package installation execution
- Prune/orphan package removal
- Package pinning/versioning
- Conflict detection

### Planned (Phase 3)
- Backup and restore functionality
- System verification
- Service management
- Configuration file management

### Planned (Phase 4)
- Package repository sharing
- CI/CD integration
- GUI front-end
- Statistics and analytics

## [Unreleased]

### Upcoming
- Better error messages
- More comprehensive tests
- Performance optimizations
