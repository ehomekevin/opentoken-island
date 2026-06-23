# Windows Tauri GUI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a Windows Tauri tray app that wraps the existing OpenToken Island local proxy and UI.

**Architecture:** Keep `server.js` as the local API/proxy. Add a Tauri Rust shell that starts the server when needed, creates tray actions, and opens WebView windows backed by `http://127.0.0.1:4174`. Keep the browser fallback intact.

**Tech Stack:** Tauri 2, Rust 1.94, Node 24, npm, existing HTML/CSS/JS UI.

## Global Constraints

- Do not install or depend on .NET SDK.
- Do not port `server.js` to Rust in this iteration.
- Do not expose the full SCYS webhook URL in logs or docs.
- Preserve the existing browser UI fallback at `http://127.0.0.1:4174/popover.html`.
- Work on branch `codex/windows-tauri-gui`.
- Do not push or publish.

---

## File Structure

- `package.json`: npm scripts for tests, Tauri dev, and Tauri build.
- `src-tauri/Cargo.toml`: Rust package metadata and dependencies.
- `src-tauri/build.rs`: Tauri build integration.
- `src-tauri/tauri.conf.json`: app metadata, bundle settings, and WebView permissions.
- `src-tauri/src/main.rs`: Tauri setup, tray menu, windows, and server process lifecycle.
- `src-tauri/src/windows_support.rs`: pure helper functions that can be unit tested.
- `src-tauri/icons/icon.png`: tray/app icon copied from existing SCYS asset.
- `tests/windows_support_contract.test.cjs`: Node contract test for scaffolded files and config values.
- `docs/windows-gui.md`: user-facing run/build notes.

## Task 1: Add Tauri Scaffold Contract

**Files:**
- Create: `package.json`
- Create: `tests/windows_support_contract.test.cjs`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/build.rs`
- Create: `src-tauri/icons/icon.png`

**Interfaces:**
- Produces npm scripts: `test`, `tauri`, `tauri:dev`, `tauri:build`.
- Produces Tauri app identifier: `com.opentoken.island.windows`.

- [ ] **Step 1: Write failing contract test**

Create `tests/windows_support_contract.test.cjs`:

```js
const assert = require("assert");
const fs = require("fs");
const path = require("path");

const root = path.resolve(__dirname, "..");
const readJson = (file) => JSON.parse(fs.readFileSync(path.join(root, file), "utf8"));

const pkg = readJson("package.json");
assert.equal(pkg.scripts.test, "node tests/windows_support_contract.test.cjs");
assert.equal(pkg.scripts["tauri:dev"], "tauri dev");
assert.equal(pkg.scripts["tauri:build"], "tauri build");
assert.equal(pkg.devDependencies["@tauri-apps/cli"], "^2.11.3");

const config = readJson("src-tauri/tauri.conf.json");
assert.equal(config.identifier, "com.opentoken.island.windows");
assert.equal(config.productName, "OpenToken Island");
assert.equal(config.app.withGlobalTauri, false);
assert.deepEqual(config.bundle.targets, ["nsis"]);
assert.ok(config.bundle.icon.includes("icons/icon.png"));

const cargoToml = fs.readFileSync(path.join(root, "src-tauri/Cargo.toml"), "utf8");
assert.match(cargoToml, /tauri = \\{ version = "2"/);
assert.match(cargoToml, /features = \\["tray-icon", "image-png"\\]/);

console.log("windows scaffold contract ok");
```

- [ ] **Step 2: Run test and verify it fails**

Run: `node tests/windows_support_contract.test.cjs`

Expected: FAIL because `package.json` and `src-tauri/tauri.conf.json` do not exist.

- [ ] **Step 3: Add minimal scaffold**

Create `package.json`:

```json
{
  "name": "opentoken-island",
  "version": "0.1.0",
  "private": true,
  "type": "commonjs",
  "scripts": {
    "test": "node tests/windows_support_contract.test.cjs",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.11.3"
  }
}
```

Create `src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "OpenToken Island",
  "version": "0.1.0",
  "identifier": "com.opentoken.island.windows",
  "build": {
    "frontendDist": "../."
  },
  "app": {
    "withGlobalTauri": false,
    "windows": []
  },
  "bundle": {
    "active": true,
    "targets": ["nsis"],
    "icon": ["icons/icon.png"],
    "resources": ["../server.js", "../assets"]
  }
}
```

Create `src-tauri/Cargo.toml`:

```toml
[package]
name = "opentoken-island"
version = "0.1.0"
description = "Windows tray companion for OpenToken Island"
authors = ["OpenToken Island"]
edition = "2021"

[build-dependencies]
tauri-build = { version = "2" }

[dependencies]
tauri = { version = "2", features = ["tray-icon", "image-png"] }
```

Create `src-tauri/build.rs`:

```rust
fn main() {
    tauri_build::build()
}
```

Copy `assets/scys/icon_symbol.png` to `src-tauri/icons/icon.png`.

- [ ] **Step 4: Run test and verify it passes**

Run: `node tests/windows_support_contract.test.cjs`

Expected: PASS and prints `windows scaffold contract ok`.

- [ ] **Step 5: Commit**

```bash
git add package.json tests/windows_support_contract.test.cjs src-tauri/tauri.conf.json src-tauri/Cargo.toml src-tauri/build.rs src-tauri/icons/icon.png
git commit -m "chore: add tauri windows scaffold"
```

## Task 2: Add Testable Windows Support Helpers

**Files:**
- Create: `src-tauri/src/windows_support.rs`
- Create: `src-tauri/src/main.rs`

**Interfaces:**
- Produces `pub const DEFAULT_PORT: u16 = 4174`.
- Produces `pub fn opentoken_bin(home: &Path) -> PathBuf`.
- Produces `pub fn server_resource_path(resource_dir: &Path) -> PathBuf`.
- Produces `pub fn local_url(path: &str) -> String`.

- [ ] **Step 1: Write failing Rust tests**

Create `src-tauri/src/windows_support.rs`:

```rust
use std::path::{Path, PathBuf};

pub const DEFAULT_PORT: u16 = 4174;

pub fn opentoken_bin(home: &Path) -> PathBuf {
    home.join(".opentoken").join("bin").join("opentoken.exe")
}

pub fn server_resource_path(resource_dir: &Path) -> PathBuf {
    resource_dir.join("server.js")
}

pub fn local_url(path: &str) -> String {
    let clean = path.trim_start_matches('/');
    format!("http://127.0.0.1:{}/{}", DEFAULT_PORT, clean)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_default_opentoken_path() {
        let path = opentoken_bin(Path::new(r"C:\\Users\\ty"));
        assert_eq!(path, PathBuf::from(r"C:\\Users\\ty\\.opentoken\\bin\\opentoken.exe"));
    }

    #[test]
    fn builds_server_resource_path() {
        let path = server_resource_path(Path::new(r"C:\\App\\resources"));
        assert_eq!(path, PathBuf::from(r"C:\\App\\resources\\server.js"));
    }

    #[test]
    fn builds_local_urls() {
        assert_eq!(local_url("popover.html"), "http://127.0.0.1:4174/popover.html");
        assert_eq!(local_url("/island.html"), "http://127.0.0.1:4174/island.html");
    }
}
```

Create `src-tauri/src/main.rs`:

```rust
mod windows_support;

fn main() {
    println!("{}", windows_support::local_url("popover.html"));
}
```

- [ ] **Step 2: Run tests and verify failure or compile gaps**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

Expected: tests compile and pass after module exists. If Cargo dependencies are missing, run `npm install` later and keep this task focused on Rust tests.

- [ ] **Step 3: Keep helpers minimal**

No extra helper should be added until `main.rs` needs it. Do not add process management here yet.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/windows_support.rs src-tauri/src/main.rs
git commit -m "test: add windows support helper contract"
```

## Task 3: Implement Tauri Tray Shell

**Files:**
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/src/windows_support.rs`

**Interfaces:**
- Consumes `windows_support::local_url`.
- Produces tray menu item IDs: `open-panel`, `show-island`, `open-browser`, `open-logs`, `quit`.
- Produces WebView window labels: `panel`, `island`.

- [ ] **Step 1: Add process and port helper tests**

Extend `src-tauri/src/windows_support.rs` tests with:

```rust
#[test]
fn detects_closed_local_port() {
    assert!(!is_port_open(9));
}
```

Add the function signature before implementation:

```rust
pub fn is_port_open(_port: u16) -> bool {
    todo!("port check not implemented")
}
```

- [ ] **Step 2: Run test and verify it fails**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

Expected: FAIL because `is_port_open` is not implemented.

- [ ] **Step 3: Implement minimal port check**

Replace `is_port_open` with:

```rust
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub fn is_port_open(port: u16) -> bool {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    TcpStream::connect_timeout(&addr, Duration::from_millis(250)).is_ok()
}
```

- [ ] **Step 4: Run Rust tests**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

Expected: PASS.

- [ ] **Step 5: Implement Tauri runtime shell**

Replace `src-tauri/src/main.rs` with a Tauri setup that:

- Starts `server.js` only if port `4174` is closed.
- Creates a tray icon with menu actions.
- Opens a `panel` WebView window at `local_url("popover.html")`.
- Opens an `island` WebView window at `local_url("island.html")`.
- Opens `C:\Users\ty\.opentoken\island-events.log` through `cmd /C start`.
- Quits through `app.exit(0)`.

- [ ] **Step 6: Run Rust tests**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/main.rs src-tauri/src/windows_support.rs
git commit -m "feat: add windows tray shell"
```

## Task 4: Add Windows User Documentation

**Files:**
- Create: `docs/windows-gui.md`
- Modify: `README.md`

**Interfaces:**
- Documents `npm install`, `npm run tauri:dev`, and `npm run tauri:build`.
- Documents that .NET SDK is not required.

- [ ] **Step 1: Write docs**

Create `docs/windows-gui.md` with exact local commands and expected behavior.

- [ ] **Step 2: Link docs from README**

Add a `Windows GUI` section to `README.md` that links to `docs/windows-gui.md`.

- [ ] **Step 3: Commit**

```bash
git add docs/windows-gui.md README.md
git commit -m "docs: add windows gui usage"
```

## Task 5: Install Dependencies And Verify

**Files:**
- Create or modify: `package-lock.json`

**Interfaces:**
- Consumes all previous tasks.
- Produces a local build or exact prerequisite failure.

- [ ] **Step 1: Install npm dependencies**

Run: `npm install`

Expected: installs `@tauri-apps/cli` and creates `package-lock.json`.

- [ ] **Step 2: Run Node contract test**

Run: `npm test`

Expected: PASS.

- [ ] **Step 3: Run Rust tests**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

Expected: PASS.

- [ ] **Step 4: Run Tauri build**

Run: `npm run tauri:build`

Expected: build succeeds and creates Windows bundle output, or fails with a concrete missing prerequisite such as WebView2/C++ Build Tools.

- [ ] **Step 5: Manual launch check**

If build succeeds, run the generated exe from `src-tauri/target/release/`.

Expected:

- Tray icon appears.
- Open Panel displays OpenToken stats.
- Show Island displays compact overlay.
- Existing `http://127.0.0.1:4174/popover.html` still works.

- [ ] **Step 6: Final commit**

```bash
git add package-lock.json
git commit -m "build: verify windows tauri gui"
```

## Self-Review

- Spec coverage: The plan covers Tauri scaffold, tray shell, Node server reuse, docs, tests, and build validation.
- Incomplete-marker scan: No unfinished markers remain in the plan text.
- Type consistency: Helper names are consistent across tasks: `DEFAULT_PORT`, `opentoken_bin`, `server_resource_path`, `local_url`, and `is_port_open`.
