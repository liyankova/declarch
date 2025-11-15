# declarch 🏗️

A **declarative Arch Linux package manager** inspired by NixOS. Define your system's packages once, synchronize everywhere.

```
╔════════════════════════════════════════════════════════════╗
║  Tired of managing packages across multiple machines?      ║
║  declarch makes your dotfiles include your entire setup.   ║
╚════════════════════════════════════════════════════════════╝
```

## ✨ Features

- **Declarative Configuration** - Define packages in simple `.decl` files
- **Multi-Backend Support** - Pacman, AUR, and Flatpak in one command
- **Module System** - Organize packages by purpose (base, dev, media, etc)
- **Host Profiles** - Different package sets for different machines
- **State Tracking** - Know exactly what's installed and where it came from
- **Dry-Run Mode** - Preview changes before applying them
- **Interactive Prompts** - User-friendly decision making
- **Beautiful Output** - Color-coded, compact information display

## 🚀 Quick Start

### 1. Initialize declarch

```bash
declarch init
```

Auto-detects your hostname and creates the configuration structure:
```
~/.config/declarch/
├── config.decl              # Global settings
├── hosts/
│   └── your-hostname.decl   # Host-specific packages
└── modules/
    └── base.decl            # Base system packages
```

### 2. Add Packages

Edit your base module:

```bash
declarch edit module base
```

Add packages (inline format):
```kdl
description "Essential system packages"
packages "zsh" "git" "neovim" "curl" "wget"
```

### 3. Create a Module

```bash
cat > ~/.config/declarch/modules/dev.decl << 'EOF'
description "Development tools"
packages "base-devel" "rustup" "nodejs" "python"
EOF
```

### 4. Update Your Host

```bash
declarch edit host <hostname>
```

Example:
```kdl
description "My laptop setup"

modules "base" "dev"

packages "firefox" "thunderbird"

exclude "nano"
```

### 5. Sync Packages

Preview what will happen:
```bash
declarch sync --dry-run
```

Apply changes:
```bash
declarch sync
```

That's it! 🎉

## 📖 Complete Usage

### Commands

#### `declarch init`
Initialize declarch on a new machine.

**Options:**
- `--host <NAME>` - Use custom hostname instead of auto-detect
- `--force` - Overwrite existing config without asking

```bash
declarch init                    # Auto-detect hostname
declarch init --host laptop     # Use custom name
declarch init --force           # Overwrite existing
```

#### `declarch host`
Manage host configurations.

**Subcommands:**
- `enable <NAME>` - Activate a host profile
- `status` - Show current host and active modules
- `list` - List all available hosts

```bash
declarch host enable desktop    # Switch to desktop profile
declarch host status            # Check current setup
declarch host list              # See all profiles
```

#### `declarch sync`
Synchronize system packages with declarations.

**Options:**
- `--dry-run` - Preview changes without applying
- `--prune` - Remove packages not in config
- `--host <NAME>` - Sync specific host (default: current)
- `--yes` - Skip confirmation prompt

```bash
declarch sync --dry-run         # Preview
declarch sync                   # Apply with confirmation
declarch sync --yes             # Apply automatically
declarch sync --host desktop   # Sync different host
```

#### `declarch edit`
Open configuration files in your `$EDITOR`.

**Targets:**
- `config` - Global configuration
- `host <NAME>` - Host-specific config
- `module <NAME>` - Module packages

```bash
declarch edit config            # Edit global settings
declarch edit host valiE       # Edit host
declarch edit module dev        # Edit module
```

#### `declarch check`
Validate all configuration files for syntax errors.

**Options:**
- `--verbose` - Show detailed validation results

```bash
declarch check                  # Quick validation
declarch check --verbose        # Detailed output
```

#### `declarch info`
Show current system status and installed packages.

**Options:**
- `--diff` - Show differences (future feature)

```bash
declarch info                   # Current status
declarch info --diff            # With differences
```

#### `declarch clean`
Clean package manager caches.

**Options:**
- `--full` - Aggressive cleanup (includes flatpak)
- `--orphans` - Remove packages not in any module

```bash
declarch clean                  # Basic cleanup
declarch clean --full           # Full cleanup
declarch clean --orphans        # Remove orphans
```

---

## 🎨 Configuration Format (KDL)

declarch uses **KDL** (KDL Document Language) for configuration files. It's human-friendly and easy to read.

### Syntax Examples

#### Simple Packages

```kdl
description "My packages"
packages "vim" "git" "zsh"
```

#### Multiple Fields

```kdl
description "Development setup"
packages "base-devel" "rustup" "nodejs"
exclude "gcc"
```

#### Host Configuration

```kdl
description "Desktop workstation"

# Which modules to load
modules "base" "dev" "media"

# Additional host-specific packages
packages "firefox" "obs-studio"

# Packages to never install
exclude "nano" "ed"

# Warn about conflicting packages
conflicts "sway" "openbox"
```

#### Package Backends

Specify which package manager to use:

```kdl
packages "zsh" "git" "aur:paru" "flatpak:discord" "flatpak:obsidian"
```

Format: `[backend:]package-name`

**Backends:**
- `pacman:` - Official repositories (default if omitted)
- `aur:` - Arch User Repository
- `flatpak:` - Flatpak applications

```kdl
description "Mixed setup"

packages 
  "vim"              # pacman (default)
  "aur:yay"          # AUR helper
  "flatpak:discord"  # Flatpak app
```

#### Comments

```kdl
// This is a comment
description "My setup"  // Inline comment also works

packages "zsh" "git"    // Simple packages
```

#### Multi-line Values

```kdl
description "A longer description
  that spans multiple lines"

packages "very-long-package-name" "another-package"
```

---

## 📁 Directory Structure

```
~/.config/declarch/
├── config.decl              # Global settings
├── .state.json              # Current state (auto-managed)
├── hosts/
│   ├── laptop.decl
│   ├── desktop.decl
│   └── server.decl
└── modules/
    ├── base.decl            # Base system
    ├── dev.decl             # Development
    ├── media.decl           # Media tools
    └── custom.decl          # Custom module
```

---

## 🔄 Workflow Example

### Setup a New Machine

```bash
# 1. Initialize
declarch init --host thinkpad

# 2. Edit base module
declarch edit module base
# Add: packages "zsh" "git" "neovim" "curl" "wget" "base-devel"

# 3. Create dev module
cat > ~/.config/declarch/modules/dev.decl << 'EOF'
description "Development tools"
packages "rustup" "nodejs" "python"
EOF

# 4. Configure host
declarch edit host thinkpad
# Add:
# modules "base" "dev"
# packages "firefox" "telegram-desktop"

# 5. Preview
declarch sync --dry-run

# 6. Install
declarch sync
```

### Switch Between Hosts

```bash
# List all hosts
declarch host list

# Switch to desktop
declarch host enable desktop

# Check what's active
declarch host status

# Sync
declarch sync
```

### Share Configuration

```bash
# Put declarch in your dotfiles repo
cp -r ~/.config/declarch ~/dotfiles/

# On another machine
cp -r ~/dotfiles/declarch ~/.config/
declarch host enable thinkpad
declarch sync
```

---

## ⚙️ Global Configuration

Edit `~/.config/declarch/config.decl`:

```kdl
// Which AUR helper to use
aur_helper "paru"

// Future options will go here
```

**Supported AUR Helpers:**
- `paru` (recommended, faster)
- `yay`

---

## 🔮 Future Functionality

### Phase 2: Enhanced Features

- [ ] **Actual Package Installation**
  - Real pacman/yay/flatpak execution
  - Progress tracking
  - Sudo prompt handling

- [ ] **Orphan Package Management**
  - `--prune` flag to remove packages not in config
  - Conflict detection
  - Manual review before removal

- [ ] **Package Pinning**
  ```kdl
  packages "linux=6.1.0" "gcc>=12.0"
  ```

- [ ] **Conditional Packages**
  ```kdl
  packages "nvidia-driver" if-feature="nvidia"
  ```

- [ ] **Package Groups**
  ```kdl
  packages "base-devel" "xorg"  // Already supported via pacman groups
  ```

### Phase 3: Advanced Features

- [ ] **Backup & Restore**
  - `declarch backup` - Save current state
  - `declarch restore <backup>` - Restore previous state

- [ ] **Conflict Detection**
  - Warn about incompatible packages
  - Suggest alternatives

- [ ] **Performance Mode**
  - `--parallel` for faster installation
  - Batch operations

- [ ] **System Verification**
  - `declarch verify` - Check consistency
  - Detect manual package installations
  - Report drift from declarations

- [ ] **Service Management**
  - Enable/disable systemd services
  - User services

- [ ] **Configuration Files**
  - Manage dotfiles alongside packages
  - Template expansion

### Phase 4: Distribution & Ecosystem

- [ ] **Package Repositories**
  - Share common modules
  - Community packages

- [ ] **CI/CD Integration**
  - GitHub Actions support
  - Auto-sync on commit

- [ ] **Desktop Integration**
  - GUI front-end
  - System tray indicator

- [ ] **Statistics & Analytics**
  - Track package usage
  - Identify unused packages
  - Trending packages

---

## 📊 Example Configurations

### Minimal Setup

```kdl
// ~/.config/declarch/modules/base.decl
description "Absolute minimum"
packages "zsh" "git" "vim"
```

```kdl
// ~/.config/declarch/hosts/minimal.decl
description "Minimal installation"
modules "base"
```

### Full Development Machine

```kdl
// ~/.config/declarch/modules/base.decl
description "Base system"
packages 
  "zsh" "git" "neovim"
  "curl" "wget" "ripgrep" "fd"
```

```kdl
// ~/.config/declarch/modules/dev.decl
description "Development tools"
packages 
  "base-devel" "rustup" "nodejs" "python"
  "docker" "podman" "aur:lazydocker"
```

```kdl
// ~/.config/declarch/modules/media.decl
description "Media tools"
packages 
  "ffmpeg" "imagemagick" "gimp"
  "flatpak:obs-studio"
```

```kdl
// ~/.config/declarch/hosts/workstation.decl
description "Development workstation"

modules "base" "dev" "media"

packages 
  "firefox" "thunderbird"
  "flatpak:discord" "flatpak:slack"
  "obsidian" "notion"

exclude "nano"
```

---

## 🐛 Troubleshooting

### "No state file found"
Run `declarch init` first to initialize your setup.

### "Module not found: xyz"
Create the module:
```bash
cat > ~/.config/declarch/modules/xyz.decl << 'EOF'
description "My module"
packages "package1" "package2"
EOF
```

### "Configuration error"
Validate your files:
```bash
declarch check --verbose
```

### Files opened in wrong editor
Set `$EDITOR` environment variable:
```bash
export EDITOR=vim  # or neovim, nano, etc
```

---

## 📝 Tips & Tricks

### Organize by Purpose

```
modules/
├── base.decl        # Essential
├── dev.decl         # Development
├── media.decl       # Media creation
├── desktop.decl     # Desktop environment
└── gaming.decl      # Games
```

Then mix-and-match in hosts:
```kdl
modules "base" "dev" "desktop"  // For workstation
modules "base" "server"          // For server
```

### Use Comments Effectively

```kdl
// Development tools
packages "rustup"      // Rust toolchain
packages "nodejs"      // Node.js runtime
packages "aur:lazygit" // Git UI tool

// Keep vim but not nano
exclude "nano"
```

### Keep Modules Small

```kdl
// DON'T do this
packages "zsh" "git" "vim" "neovim" "curl" "wget" "base-devel" "rustup" "nodejs" "python" "docker" ...

// DO this instead - split into modules
// base.decl: zsh git curl wget
// dev.decl: base-devel rustup nodejs python
// containers.decl: docker podman
```

---

## 🤝 Contributing

Found a bug? Have an idea?

```bash
# Report issues
git issue create "Bug: ..."

# Suggest features
git issue create "Feature: ..."
```

---

## 📄 License

MIT - Use freely!

---

## 🙏 Acknowledgments

Inspired by:
- **NixOS** - Declarative system configuration
- **Ansible** - Infrastructure as code
- **KDL** - Human-friendly configuration language

---

**Happy declaring! May your package list be forever synchronized.** 🎯

For more help:
```bash
declarch --help
declarch <command> --help
```
