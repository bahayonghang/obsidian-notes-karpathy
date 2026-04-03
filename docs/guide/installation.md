# Installation

Multiple options for installing Obsidian Notes Karpathy skills.

## Option 1: Install via npx (Recommended)

The easiest way to install is using the `npx skills add` command.

### Global Installation

Install globally to use the skills across all your projects:

```bash
npx skills add bahayonghang/obsidian-notes-karpathy -g
```

This installs the skills to your global agents directory, making them available everywhere.

### Project-Specific Installation

Install to your Obsidian vault or project directory for project-specific use:

```bash
# Navigate to your Obsidian vault root
cd /path/to/your/obsidian-vault

# Install the skills locally
npx skills add bahayonghang/obsidian-notes-karpathy
```

This creates a `skills/` directory in your project root with all the necessary skills.

## Option 2: Manual Installation

If you prefer manual installation or want to customize the skills:

```bash
# Copy skills to your Claude Code skills folder
cp -r skills/obsidian-notes-karpathy/* ~/.claude/skills/
```

On Windows (PowerShell):

```powershell
Copy-Item -Recurse skills/obsidian-notes-karpathy\* $env:USERPROFILE\.claude\skills\
```

## Option 3: Clone and Customize

For advanced users who want to modify the skills:

1. **Clone the repository**
   ```bash
   git clone https://github.com/bahayonghang/obsidian-notes-karpathy.git
   cd obsidian-notes-karpathy
   ```

2. **Modify** the SKILL.md files as needed for your workflow

3. **Install** using npx or copy to your skills folder

## Verify Installation

Check that the skills are installed by listing your skills:

```bash
ls ~/.claude/skills/obsidian-notes-karpathy/
```

You should see:

```
kb-init/
  └── SKILL.md
kb-compile/
  └── SKILL.md
kb-query/
  └── SKILL.md
```

## Required Dependencies

These skills build on [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills). Make sure you also have:

- **`obsidian-markdown`** — Obsidian Flavored Markdown syntax (wikilinks, callouts, frontmatter)
- **`obsidian-cli`** — Vault interaction via command line
- **`obsidian-canvas-creator`** — Canvas visualization generation

Install these from the [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills) repository:

```bash
# Clone the obsidian-skills repository
git clone https://github.com/kepano/obsidian-skills.git

# Copy the required skills
cp -r obsidian-skills/obsidian-markdown ~/.claude/skills/
cp -r obsidian-skills/obsidian-cli ~/.claude/skills/
cp -r obsidian-skills/obsidian-canvas-creator ~/.claude/skills/
```

## Optional: Web Clipper

For the best experience, install the **Obsidian Web Clipper** browser extension:

- [Obsidian Web Clipper](https://obsidian.md/clipper) — Available for Chrome, Firefox, Edge

This allows you to clip articles directly to your `raw/` directory.

## Next Steps

- [**Quick Start**](/guide/quick-start) — Get up and running
- [**Karpathy Workflow**](/guide/karpathy-workflow) — Understand the philosophy
- [**Skills Overview**](/skills/overview) — Learn about each skill
