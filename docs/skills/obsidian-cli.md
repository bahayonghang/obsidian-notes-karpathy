# Obsidian CLI

Command-line interface for interacting with Obsidian vaults.

## Overview

The `obsidian-cli` skill provides command-line access to Obsidian vault operations, enabling programmatic interaction with your knowledge base.

## Capabilities

### File Operations

- Create, read, update, and delete notes within the vault
- Search vault content by query text
- List directory contents
- Move and rename files

### Search

```bash
obsidian search query="transformer architecture"
obsidian search tag="concept"
obsidian search path="wiki/concepts/"
```

### Vault Management

- Open vault in Obsidian app
- List available vaults
- Get vault configuration

## Usage in Knowledge Base

The Karpathy skills use `obsidian-cli` for:

1. **kb-init**: Creating initial directory structure and files
2. **kb-compile**: Searching and reading vault content during compilation
3. **kb-query**: Full-text search across the wiki

## When CLI is Not Available

If `obsidian-cli` is not installed, the skills fall back to:
- **Write tool**: Directly create/edit files
- **Grep tool**: Search file contents
- **Read tool**: Read file contents

## Installation

Install from [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills):

```bash
cp -r obsidian-skills/obsidian-cli ~/.claude/skills/
```

## Reference

- [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills)
- [Obsidian CLI Documentation](https://help.obsidian.md/Advanced+topics/Using+Obsidian+CLI)
