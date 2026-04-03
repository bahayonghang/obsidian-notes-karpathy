# Justfile for Obsidian Notes Karpathy
# Usage: just <command>

# Default recipe
default:
  @just --list

# ==================== Documentation ====================

# Install docs dependencies
docs-install:
  cd docs && npm install

# Start docs dev server with hot reload
docs-dev:
  cd docs && npm run dev

# Build docs for production
docs-build:
  cd docs && npm run build

# Preview production docs build
docs-preview:
  cd docs && npm run preview

# ==================== CI / Quality ====================

# Run all CI checks locally
ci:
  just lint
  just docs-build

# Lint / validate documentation
lint:
  @echo "Checking docs for issues..."
  @just --dry-run ci
  @echo "All checks passed!"

# ==================== Git / Workflow ====================

# Quick status check
status:
  @git status --short
  @echo ""
  @echo "=== Recent Commits ==="
  @git log --oneline -5

# Commit with conventional commit message
commit msg type="feat":
  git commit -m "{{type}}: {{msg}}"

# ==================== Utility ====================

# Clean build artifacts
clean:
  rm -rf docs/.vitepress/dist
  rm -rf docs/node_modules
  @echo "Cleaned build artifacts"

# Install all dependencies (docs + skills)
install:
  cd docs && npm install
  @echo "All dependencies installed"
