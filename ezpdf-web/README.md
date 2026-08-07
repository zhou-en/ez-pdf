# ezpdf-web

The web build of `ezpdf`. Same Rust core as the CLI and desktop app, compiled to
WebAssembly and run **in the browser** — Vercel only does authentication,
storage and metadata, never PDF work.

```
Browser                                    Vercel
─────────────────────────────────          ─────────────────────────
drop file → wasm → result (instant)
   │                                       Neon    — users, accounts, files
   ├─ [Download]  ......................   (never touches the server)
   │
   └─ [Save to library] ─── upload ──→     Blob    — private, user-scoped
                                           Neon    — one row per saved file
```

## Why in the browser

- **Privacy is the product.** Nothing is uploaded unless the user explicitly saves it.
- **No round-trip.** Conversion is ~80 ms locally; an upload/poll/download cycle can't compete.
- **No compute bill**, no cold starts, no function timeouts, no request-size ceiling.

The alternative — the Vercel Rust runtime — would have run `ezpdf-core` unchanged,
but it is public beta, wants `Cargo.toml` at the repo root (colliding with the
workspace), and puts every user document on someone else's disk to do work the
laptop can do instantly.

## Layout

| Path | What |
|---|---|
| `src/lib/wasm.ts` | Lazy loaders for the two wasm packages |
| `src/lib/operations.ts` | Operation catalogue + dispatch; the only place ops are enumerated |
| `src/lib/files.ts` | Server actions: list, record, delete (ownership-checked) |
| `src/lib/save.ts` | Client upload to Blob, then record the pair |
| `src/auth.ts` | Auth.js v5, Google, Drizzle adapter, JWT sessions |
| `src/db/schema.ts` | `users`, `accounts`, `files` |
| `src/components/` | UI, built on the Raycast primitives in `ui.tsx` |
| `src/wasm/` | Generated — rebuild with `../ezpdf-wasm/build.sh` |
| `DESIGN.md` | The design system. Its Do/Don't rules are binding. |

## The two wasm packages

Built by `../ezpdf-wasm/build.sh` and committed, because Vercel's build image
has no Rust toolchain.

| Package | Gzipped | Loaded |
|---|---|---|
| `ops` — merge/split/remove/rotate/info | ~0.14 MB | on first use |
| `markdown` — adds `pdf-inspector` | ~2.37 MB | only when Markdown is picked |

The 26× gap is why they are separate: `pdf-inspector` embeds glyph tables and
CMaps, and someone merging two PDFs should never download them.

## Local development

```bash
pnpm install
pnpm dev          # http://localhost:3000
pnpm test         # vitest
pnpm typecheck
pnpm build
```

`/styleguide` renders every primitive once — the cheapest way to catch a stray
drop shadow or a light surface.

### If your checkout is on a small partition

A Next.js `node_modules` needs a few hundred MB. If the disk holding this repo
is tight, create an **uncommitted** `.npmrc` here:

```ini
store-dir=~/.cache/ezpdf-pnpm-store
virtual-store-dir=~/.cache/ezpdf-web-pnpm
```

Only symlinks then land in this directory. **Both** settings are required:
given only `virtual-store-dir`, pnpm decides the home store is on another
device and silently creates `<drive>/.pnpm-store` next to the project, which
is exactly what you were trying to avoid.

`.npmrc` is gitignored deliberately — absolute local paths would break
`pnpm install` on Vercel.

## Deployment

1. Set **Root Directory** to `ezpdf-web` in the Vercel project settings.
2. `vercel integration add neon` → provisions `DATABASE_URL`.
3. Attach a Blob store → provisions `BLOB_READ_WRITE_TOKEN`.
4. Create a Google OAuth client (Web) with redirect URI
   `https://<domain>/api/auth/callback/google`; set `AUTH_GOOGLE_ID` /
   `AUTH_GOOGLE_SECRET`.
5. `npx auth secret` → `AUTH_SECRET`. Set `AUTH_TRUST_HOST=true` on **preview only**.
6. `pnpm db:push` to create the tables.

## Security model

The static pages are uninteresting; these are the parts that matter.

- Blobs are **private**. A public blob URL is a permanent unauthenticated handle
  to someone's document.
- Blob keys are user-scoped: `u/{userId}/{fileId}/{source|result}/{filename}`.
- `/api/blob/upload` authenticates **before** minting a token and pins the
  pathname prefix to the session user. An unscoped token endpoint is a
  write-anywhere hole.
- Delete and download re-check ownership server-side and never trust a
  client-supplied blob URL.
- Delete removes **blobs before the row**: a dangling row is recoverable, an
  orphaned private blob is invisible and bills forever.
- `getDb()` is a plain lazy function, never a `Proxy`. Auth.js inspects the
  adapter object, and a Proxy makes the request chain hang with no error.
