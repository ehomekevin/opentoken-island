# OpenToken Island

A macOS menu bar companion for OpenToken.

It combines:

- A real macOS status bar item
- A compact Apple-style Dynamic Island event popup
- A popover extension panel with rank, XP, quests, and achievements
- Live data from the local `opentoken` CLI
- Manual upload through `opentoken upload`

## Install Locally

Build and install the app:

```bash
rm -rf build "/Applications/OpenToken Island.app"
mkdir -p "build/OpenToken Island.app/Contents/MacOS" \
  "build/OpenToken Island.app/Contents/Resources/assets/scys"

swiftc OpenTokenIsland.swift -framework Cocoa -framework WebKit \
  -o "build/OpenToken Island.app/Contents/MacOS/OpenToken Island"

cp popover.html island.html server.js "build/OpenToken Island.app/Contents/Resources/"
cp assets/scys/icon_topnav.png "build/OpenToken Island.app/Contents/Resources/assets/scys/icon_topnav.png"
```

Create `build/OpenToken Island.app/Contents/Info.plist` with `LSUIElement` enabled, then copy the app to `/Applications`.

This repository currently assumes:

```text
opentoken binary: /Users/yangguangxiaolaohu/.local/bin/opentoken
local API port: 4174
```

## Files

- `OpenTokenIsland.swift` - native AppKit menu bar shell
- `server.js` - local API bridge to the `opentoken` CLI
- `popover.html` - extension popover UI
- `island.html` - Dynamic Island notification UI
- `index.html` - original browser prototype kept for design review
