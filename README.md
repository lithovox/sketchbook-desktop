# Sketchbook desktop

Sketchbook is an app designed by LithoVox to sketch the subsoil along a dike or
road: referencelines, CPTs and boreholes from the BRO and soil profiles from
GeoScanner on a map, a geotechnical profile and a first voxel model. Log in with
your LithoVox account, or start the free version. An internet connection is
needed (the LithoVox API, the BRO and the map).

## Download

The latest version:

| System | Installer |
| --- | --- |
| Windows (64-bit) | [Sketchbook-windows-x64-setup.exe](https://github.com/lithovox/sketchbook/releases/latest/download/Sketchbook-windows-x64-setup.exe) or [Sketchbook-windows-x64.msi](https://github.com/lithovox/sketchbook/releases/latest/download/Sketchbook-windows-x64.msi) |
| macOS (Apple Silicon and Intel) | [Sketchbook-macos-universal.dmg](https://github.com/lithovox/sketchbook/releases/latest/download/Sketchbook-macos-universal.dmg) |
| Linux (Debian/Ubuntu, 64-bit) | [Sketchbook-linux-amd64.deb](https://github.com/lithovox/sketchbook/releases/latest/download/Sketchbook-linux-amd64.deb) |

Older versions: [all releases](https://github.com/lithovox/sketchbook/releases).

The installers aren't code-signed yet. On Windows, SmartScreen may warn about
an unknown publisher ("More info" → "Run anyway"). On macOS the first start is
blocked; allow it in System Settings → Privacy & Security ("Open Anyway"). On
Linux, install with `sudo apt install ./Sketchbook-linux-amd64.deb`.

## Building

This folder holds only the Tauri 2 shell (`src-tauri/`); the app itself is
`../sketchbook`, built with `vite build --mode desktop` into `dist/`
(`.env.desktop` there sets the API to `https://api.lithovox.nl`). The version is
`../sketchbook/package.json`'s.

```
npm install
npm run dev     # the app in a window, with ../sketchbook's dev server (port 1421)
npm run build   # installers in src-tauri/target/release/bundle/
```

Releases are built by `../sketchbook/.github/workflows/desktop.yml` on a push to
`master` with a new version in `package.json`.
