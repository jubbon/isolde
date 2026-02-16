# Backlog

This file tracks planned features, bug fixes, and improvements for the Claude Code devcontainer template system.

## Completed ✅

- [x] **Fix symlinks issue** - Copy features instead of creating symlinks (Docker cannot follow symlinks outside build context)
- [x] **Duplicate proxy settings** - Add proxy settings to both `proxy` and `claude-code` features
- [x] **Add Node.js to all templates** - Node.js and npx are required by Claude Code tools
- [x] **Match host paths inside container** - Use full host path as `workspaceFolder`

## In Progress 🚧

_Currently working on..._

## Planned Features 📋

### High Priority
- [ ] **Remove proxy duplication** - Find a way to share proxy settings between features without duplication
- [ ] **Better error messages** - Improve error reporting when feature resolution fails

### Medium Priority
- [ ] **Auto-update templates** - Mechanism to update existing projects with new template changes
- [ ] **Template validation** - Verify template syntax before applying
- [ ] **Custom template directory** - Allow users to specify custom template locations

### Low Priority
- [ ] **Interactive preset editor** - GUI or TUI for creating/editing presets
- [ ] **Template versioning** - Support multiple versions of templates
- [ ] **Export/import configurations** - Share project configurations across machines

## Bug Reports 🐛

_Recently reported issues_

- [ ] **Proxy feature execution order** - Currently requires duplication because feature execution order is not guaranteed

## Technical Debt 💸

- [ ] **Refactor templates.sh** - Split into smaller, more focused functions
- [ ] **Add unit tests** - Test template substitution logic
- [ ] **Improve logging** - Better debug output for troubleshooting

## Ideas 💡

_Potential future features_

- [ ] **clog** - Single binary CLI (Go/Rust) replacing shell scripts
  - Name: **clog** (alternatives: clodck, clodock)
  - Read TOML configuration file
  - Deploy devcontainer project structure
  - Single executable, no shell dependencies
- [ ] **Multi-container support** - Docker Compose based templates
- [ ] **GPU support templates** - Preconfigured for CUDA/ROCm
- [ ] **Service integration** - Built-in database, Redis, etc.
- [ ] **CI/CD templates** - GitHub Actions, GitLab CI configs

---

**Legend:**
- ✅ Completed
- 🚧 In Progress
- 📋 Planned
- 🐛 Bug Report
- 💸 Technical Debt
- 💡 Ideas
