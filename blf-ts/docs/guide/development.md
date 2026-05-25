# Development

From the `blf-ts/` directory:

```bash
npm install
npm run validate   # lint, test, typecheck, docs build
npm run build
```

## Scripts

| Script | Purpose |
|--------|---------|
| `npm run docs:gen` | Regenerate version chunk list for the guide |
| `npm run docs` | VitePress dev server |
| `npm run docs:build` | Production docs site (runs `docs:gen` first) |
| `npm run docs:preview` | Preview built docs |
| `npm run release` | Version bump and tag (npm publish on tag via CI) |

## Docs site

User guide: VitePress in `docs/` (same layout as [cstruct](https://github.com/craftycodie/cstruct)).

CI deploys the VitePress site to GitHub Pages on pushes to `main`.
