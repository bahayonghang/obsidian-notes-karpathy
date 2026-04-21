# Justfile for Obsidian Notes Karpathy
# Usage: just <command>

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
  just lint
  just test
  just docs-build

# Lint / validate bundle contracts and docs
lint:
  cargo fmt --check
  cargo clippy --all-targets --all-features -- -D warnings
  cargo run -- --json dev contract-validate

# Run deterministic skill-bundle regression tests
test:
  cargo test

# Run non-blocking runtime skill comparisons
runtime-eval:
  cargo run -- --json dev eval-runtime --dry-run

# Audit shipped skill quality and evaluation coverage
skill-audit:
  cargo run -- --json dev audit-skills

# Install the Rust CLI locally
install-local: cli-install-local

# Install the Rust CLI locally
cli-install-local:
  cargo install --path . --locked --force

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
  node -e "const fs=require('fs'); ['docs/.vitepress/dist','docs/.vitepress/cache','docs/node_modules'].forEach((p)=>fs.rmSync(p,{recursive:true,force:true}));"
  @echo "Cleaned build artifacts"

# Install all dependencies (docs + skills)
[working-directory: "docs"]
install:
  npm install
  @echo "All dependencies installed"
