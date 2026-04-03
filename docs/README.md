# Documentation

Bilingual (English/Chinese) VitePress documentation for Obsidian Notes Karpathy.

## Development

### Install Dependencies

```bash
npm install
```

### Start Development Server

```bash
npm run dev
```

This starts a local server with hot reload at `http://localhost:5173`.

### Build for Production

```bash
npm run build
```

Output is generated in `.vitepress/dist/`.

### Preview Production Build

```bash
npm run preview
```

## Structure

```
docs/
├── .vitepress/
│   ├── config.ts          # VitePress configuration
│   └── theme/
│       ├── index.js       # Custom theme entry
│       └── custom.css     # Custom styles
├── public/                # Static assets (logos, images)
├── guide/                 # English guide pages
├── skills/                # English skills documentation
├── workflow/              # English workflow documentation
├── zh/                    # Chinese translations
│   ├── guide/
│   ├── skills/
│   └── workflow/
├── index.md               # English landing page
└── zh/index.md            # Chinese landing page
```

## Adding New Content

1. Create the English version in the appropriate directory (`guide/`, `skills/`, `workflow/`)
2. Create the Chinese translation in the corresponding `zh/` subdirectory
3. Update `.vitepress/config.ts` to add navigation and sidebar entries
4. Test with `npm run dev`
5. Build with `npm run build`

## Language Switching

The site supports automatic language switching:
- English: root paths (`/guide/introduction`)
- Chinese: `/zh/` prefix (`/zh/guide/introduction`)

Users can switch languages using the language selector in the navigation bar.
