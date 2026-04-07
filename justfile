# Justfile for Obsidian Notes Karpathy
# Usage: just <command>

python_cmd := if os_family() == "windows" { "py -3" } else { "python3" }

# Default recipe
default:
  @just --list

# ==================== Documentation ====================

# Install docs dependencies
[working-directory: "docs"]
docs-install:
  npm install

# Start docs dev server with hot reload (top-level shortcut)
[working-directory: "docs"]
docs:
  npm run dev

# Start docs dev server with hot reload
[working-directory: "docs"]
docs-dev:
  npm run dev

# Build docs for production
[working-directory: "docs"]
docs-build:
  npm run build

# Preview production docs build
[working-directory: "docs"]
docs-preview:
  npm run preview

# ==================== CI / Quality ====================

# Run all CI checks locally
ci:
  just test
  just lint
  just docs-build

# Lint / validate documentation
lint:
  @echo "Checking docs for issues..."
  @just --dry-run ci
  @echo "All checks passed!"

# Run deterministic skill-bundle regression tests
test:
  {{python_cmd}} -m unittest tests/test_skill_bundle.py

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
  {{python_cmd}} -c "from pathlib import Path; import shutil; [shutil.rmtree(Path(p), ignore_errors=True) for p in ['docs/.vitepress/dist', 'docs/.vitepress/cache', 'docs/node_modules']]"
  @echo "Cleaned build artifacts"

# Install all dependencies (docs + skills)
[working-directory: "docs"]
install:
  npm install
  @echo "All dependencies installed"
