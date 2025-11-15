# Quick Start Guide 🚀

Get declarch running in 5 minutes.

## Step 1: Initialize

```bash
declarch init
```

This creates:
- `~/.config/declarch/config.decl` - Global config
- `~/.config/declarch/hosts/{hostname}.decl` - Your host profile
- `~/.config/declarch/modules/base.decl` - Base packages

## Step 2: Add Base Packages

```bash
declarch edit module base
```

Add your essential packages:

```kdl
description "Essential packages"
packages "zsh" "git" "neovim" "curl"
```

## Step 3: Configure Your Host

```bash
declarch edit host <your-hostname>
```

Include the base module:

```kdl
description "My setup"
modules "base"
```

## Step 4: Preview Changes

```bash
declarch sync --dry-run
```

See what will be installed.

## Step 5: Install

```bash
declarch sync
```

Confirm when prompted. That's it! 🎉

---

## Common Next Steps

### Add Another Module

```bash
# Create dev module
cat > ~/.config/declarch/modules/dev.decl << 'EOF'
description "Development tools"
packages "base-devel" "rustup" "nodejs"
EOF

# Update host to include it
declarch edit host <your-hostname>
```

Then add to modules:
```kdl
modules "base" "dev"
```

### Exclude Packages

```bash
declarch edit host <your-hostname>
```

Add:
```kdl
exclude "nano"
```

### Use Different Backends

```kdl
packages "vim" "aur:paru" "flatpak:discord"
```

### Check Status

```bash
declarch info
declarch host status
```

---

## Getting Help

```bash
# See all commands
declarch --help

# Get help for specific command
declarch sync --help
declarch host --help

# Validate configuration
declarch check --verbose
```

You're ready! Start with the examples in README.md for more advanced usage.
