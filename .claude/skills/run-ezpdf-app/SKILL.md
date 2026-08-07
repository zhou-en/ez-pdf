---
name: run-ezpdf-app
description: Build, install, launch and drive the ezpdf Tauri desktop app on Linux/X11. Use when asked to run the desktop app, screenshot it, or verify a change works in the real GUI rather than in tests.
---

# Running the ezpdf desktop app (Linux / X11)

Verified end-to-end on Ubuntu-derived Linux with a real X11 session on 2026-08-07.
Every command below was actually run; the pitfalls are ones that actually bit.

## 0. Preconditions that are not obvious

**Check the right partition.** `/home/nathan/Projects` is its own mount, separate
from `/home/nathan`. `df -h /home/nathan` reports the root filesystem and will
happily show tens of gigabytes free while the build partition sits at 100%.

```bash
df -h /home/nathan/Projects   # this is the one that matters
```

A cold Tauri release build needs several GB and dies mid-compile with
`No space left on device (os error 28)` if it runs out.

**Put Rust output on the roomier filesystem:**

```bash
export CARGO_TARGET_DIR=/home/nathan/.cache/ezpdf-target
```

This is only half a fix. Vite writes its bundled config temp into `frontend/`,
and `tauri.conf.json` requires output at `frontend/dist` — both live on the
Projects partition regardless. Keep a few hundred MB free there or the frontend
build fails before Rust even starts.

**pnpm must be 9.x.** Corepack below 0.35 fails with `Cannot find matching keyid`
(its bundled npm signing keys are stale). But after upgrading corepack, the
default pnpm demands Node >= 22.13 while this box runs 22.12.

```bash
npm install -g corepack@latest --force
corepack prepare pnpm@9.15.9 --activate   # also matches lockfileVersion '9.0'
```

## 1. Build

```bash
cd ezpdf-app/frontend && pnpm install
cd .. && CARGO_TARGET_DIR=/home/nathan/.cache/ezpdf-target \
  ./frontend/node_modules/.bin/tauri build
```

Run `tauri` from `ezpdf-app`, never from `frontend` — the config's
`beforeBuildCommand` hooks resolve relative to the frontend directory already.

**Expect the AppImage step to fail.** `linuxdeploy` aborts with SIGABRT
(confirmed via `coredumpctl`). The `.deb` and `.rpm` are written *before* that
step and are unaffected, so a non-zero exit here does not mean you have no
artifacts. Check for them before treating the build as failed:

```bash
ls $CARGO_TARGET_DIR/release/bundle/deb/*.deb
```

## 2. Install

`sudo` has no TTY inside a Claude Code shell, and the `!` prefix does not help —
it runs in the same non-interactive shell. Use PolicyKit, which pops a graphical
prompt on the user's desktop:

```bash
pkexec dpkg -i "$CARGO_TARGET_DIR/release/bundle/deb/ezpdf_0.1.0_amd64.deb"
```

Installs `/usr/bin/ezpdf-app` plus `/usr/share/applications/ezpdf.desktop`.

## 3. Launch

Launch with `setsid` so the app is fully detached from the harness's process
group. An instance started with a plain `nohup ... &` died partway through a
session with no coredump and no stderr; the `setsid` instance survived a full
drive. Cause never proven, but the difference was reproducible.

Launch from a directory holding a test PDF — the GTK file chooser opens in the
process's cwd, which saves fighting its location bar later.

```bash
mkdir -p /tmp/pdftest && cp ezpdf-core/tests/fixtures/<some>.pdf /tmp/pdftest/sample.pdf
cd /tmp/pdftest && DISPLAY=:0 setsid nohup /usr/bin/ezpdf-app > /tmp/app.log 2>&1 < /dev/null &
```

## 4. Find the window (do not trust a name search)

`xdotool search --name ezpdf` matches unrelated windows — a terminal whose title
contains "ezpdf" will match first. Filter by PID *and* by the 900x600 geometry
from `tauri.conf.json`. Note X11 recycles window IDs across restarts, so always
re-resolve rather than reusing a previous ID.

```bash
export DISPLAY=:0
PID=$(pgrep -f '^/usr/bin/ezpdf-app$' | tail -1)
WID=$(xdotool search --pid $PID | while read w; do
  xdotool getwindowgeometry --shell $w | grep -q 'WIDTH=900' && echo $w; done | head -1)
xdotool getwindowpid $WID    # confirm it really is this process
```

## 5. Drive it

**Use XTEST, never `--window`.** `xdotool key --window` and
`xdotool type --window` use `XSendEvent`, which GTK ignores outright — the
keystrokes silently vanish and the dialog just sits there. Activate the window
first, then send input with no `--window` flag.

```bash
xdotool windowactivate --sync $WID; sleep 0.5
xdotool mousemove --window $WID 520 84; xdotool click 1     # drop zone -> Open File
```

Mouse clicks via `xdotool click` work fine on GTK; only synthetic key events are
the problem. Prefer clicking the file row and the **Open** button over typing a
path into the location bar.

Useful window-relative coordinates (900x600 window):

| Target | Coords |
|---|---|
| Drop zone | `520 84` |
| Sidebar items | `60 38` Merge, `60 72` Split … `60 353` Markdown (~35px apart) |
| Run button | `235 253` (with a file loaded) |

The `Open File` dialog is a separate window owned by the same PID; find it by
name and click **Open** at roughly `1031 799` in a 1080x822 dialog.

## 6. Verify — look at the screenshot

```bash
import -window $WID /tmp/shot.png
```

Then actually read the image. A blank frame is a failed launch, not a pass.

Verify output independently rather than trusting the UI banner:

```bash
pdfinfo out.pdf | grep -i pages          # cross-check the "N pages" the UI shows
file out.pdf                             # "PDF document, version 1.5, 2 page(s)"
pdftoppm -png -r 30 -f 1 -l 1 out.pdf /tmp/render   # proves it genuinely opens
```

A healthy end-to-end run: file loads with a correct page count, `Run <Op>`
produces a green `... -> /path/out.pdf` banner, and the output renders with its
content intact (the project's lossless guarantee).
