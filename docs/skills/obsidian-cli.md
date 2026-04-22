# Obsidian CLI

> Companion skill reference. This page documents an adjacent ecosystem skill, not one of the six core skills shipped by this repository.

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
obsidian search path="wiki/live/concepts/"
```

### Vault Management

- Open vault in Obsidian app
- List available vaults
- Get vault configuration

## Usage Around This Bundle

This repository's shipped `kb-*` skills do not depend on `obsidian-cli` as their primary runtime baseline.

Treat it as an adjacent ecosystem tool for people who also want direct app-level Obsidian automation alongside the review-gated `onkb` lifecycle.

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
