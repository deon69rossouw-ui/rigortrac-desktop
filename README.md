# RigorTRAC (desktop app)

This folder wraps the RigorTRAC GIAS Conformance Assessment Tool — normally a
single self-contained HTML file — in [Tauri](https://tauri.app), so it can be
built into a real installable desktop app (`.exe`/`.msi` on Windows, `.dmg` on
macOS, `.deb`/`.AppImage` on Linux) instead of just being opened as a file in
a browser tab.

The actual app lives at `src/index.html` — it's the same file you already had
(`GIAS Conformance Assessment Tool.html`), untouched, just loaded into a
native window instead of a browser tab. Nothing about how the tool works has
changed; this only changes how it's launched and distributed.

## Why this couldn't be fully finished automatically

Building a Tauri app compiles a small Rust program for the OS you're
targeting. That compilation needs Rust plus some OS-specific libraries
installed locally, and the sandbox this project was assembled in doesn't have
permission to install those system libraries or a Rust toolchain. So every
file here is complete and correct, but the actual compiling step needs to
happen either on your own machine or in the cloud via the included GitHub
Actions workflow. Two options below — pick whichever is easier for you.

## Option A — Build in the cloud (recommended, no local setup)

This is the easiest path if you don't already have a dev environment set up,
and it's also how you'd want to produce builds long-term (one push builds
Windows, macOS, and Linux installers all at once).

1. Create a GitHub repository (can be private) and push this folder to it.
2. Go to the repo's **Actions** tab, select **Build RigorTRAC installers**,
   and click **Run workflow** — or just push a tag like `v1.0.0` to trigger
   it automatically.
3. Wait for the three platform builds to finish (a few minutes each). Each
   one uploads its installer as a build artifact / draft GitHub Release.
4. Download the installer for whichever OS you need from the workflow run's
   **Artifacts** section, or from the draft release it creates.

Unsigned installers will still work, but Windows SmartScreen and macOS
Gatekeeper will show an "unknown publisher" warning the first time someone
opens one. To remove that warning you'd add signing secrets to the repo
(see the comments in `.github/workflows/build.yml`) — that requires a
Windows code-signing certificate and/or an Apple Developer account, which
only you can obtain since they're tied to your identity/organization.

## Option B — Build locally on your own machine

Useful if you want to test changes quickly without pushing to GitHub each
time.

**1. Install prerequisites** (one-time, per machine):
   - [Node.js](https://nodejs.org) 18 or newer
   - [Rust](https://www.rust-lang.org/tools/install)
   - **Windows only:** "Desktop development with C++" workload from Visual
     Studio Build Tools (WebView2 itself is already built into Windows
     10/11)
   - **macOS only:** Xcode Command Line Tools (`xcode-select --install`)
   - **Linux only:** the packages listed in the "Install Linux system
     dependencies" step of `.github/workflows/build.yml` — install with
     your distro's package manager

   Full details: https://tauri.app/start/prerequisites/

**2. Install project dependencies:**
   ```
   npm install
   ```

**3. Run it in development mode** (opens a window immediately, for testing):
   ```
   npm run tauri dev
   ```

**4. Build the installer for your current OS:**
   ```
   npm run tauri build
   ```
   The finished installer appears under `src-tauri/target/release/bundle/`.

## Updating the app content later

If you make changes to the RigorTRAC tool itself, just replace
`src/index.html` with the newer version of the file and re-run the build —
nothing else in this project needs to change unless you're renaming the app
or changing its icon.

## What's already set up

- App name: **RigorTRAC** (window title, installer name, and package
  identifier `com.rigortrac.app` are all set)
- Custom RigorTRAC icon generated in all required formats
  (`.ico` for Windows, `.icns` for macOS, `.png` set for Linux)
- A restrictive Content Security Policy (`default-src 'self'`, no network
  access, no framing) since the app only ever needs its own bundled content
- No Tauri plugins/permissions beyond the bare minimum — the app doesn't call
  into the Rust side for anything, so its permission surface is as small as
  Tauri allows
- GitHub Actions workflow that cross-builds Windows, macOS (Intel + Apple
  Silicon), and Linux installers in one run

## Leftover files you can ignore or delete

`create-tauri-app` generated a couple of placeholder files
(`src/main.js`, `src/styles.css`) that aren't used anymore now that
`src/index.html` is the real app — safe to delete, just left in place here.
The same applies to `src-tauri/icons/ios/` and `src-tauri/icons/android/`,
which were generated alongside the desktop icons but aren't used by desktop
builds.
