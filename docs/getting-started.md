# Getting started with Lunola Luma

## Release status

There is no published Lunola Luma installer yet. The download links in the [project README](../README.md#downloads) are explicitly labelled as upstream PhotoCraft releases; they retain PhotoCraft branding and are not Lunola Luma packages. For now, build this source branch yourself or wait for a separately reviewed Lunola Luma release.

## Build from source on Windows

1. Install Rust stable (the workspace declares Rust 1.95+) and Visual Studio or Microsoft C++ Build Tools with **Desktop development with C++**. See the [development guide](development.md#prerequisites) for platform prerequisites and Windows application-control notes.
2. Open a terminal in the repository root and build the desktop app:

   ```powershell
   cargo build -p photocraft
   ```

3. Launch it, optionally opening an image by path:

   ```powershell
   .\target\debug\photocraft.exe
   .\target\debug\photocraft.exe "C:\path\to\image.psd"
   ```

   For an optimized build, use `cargo build --release -p photocraft` and launch `target\release\photocraft.exe`.

The internal package and executable remain named `photocraft` for compatibility; the window, visible product identity, and Windows product metadata display **Lunola Luma**.

## Open, edit, save, and export

- **Open:** use **File → Open** (`Ctrl+O`) or pass a supported image path when launching the app. The app also accepts PSD/PSB and the formats listed in the [format overview](../README.md#everything-in-the-box).
- **Edit:** use the left toolbar, options bar, and panels. Adjustment layers and masks keep supported edits non-destructive; the README and codec matrix describe feature and format limits.
- **Save:** use **File → Save** (`Ctrl+S`) for an existing writable document, or **File → Save As** (`Ctrl+Shift+S`) to choose a native `.pcraft` or supported interchange format. Keep a backup when testing on valuable work.
- **Export:** choose **File → Export → Export As** to select a delivery format and its available settings. Reopen the exported file to confirm it matches the intended output.

For automation or batch conversion, see the [CLI guide](development.md#headless-cli). Package/crate and command names remain PhotoCraft-compatible.

## Menus and support information

The top-level **Help** menu has been removed. **About Lunola Luma** is available under **File**. **System Info** can be found through command search (`Ctrl+F` on Windows/Linux; the app shows the platform shortcut). This change affects navigation only; it does not remove the editing engine or supported document operations.

## Graphics troubleshooting

If GPU startup fails on your machine, first try the one-launch safe option:

```powershell
.\target\debug\photocraft.exe --safe-gpu
```

For a forced CPU canvas during diagnosis:

```powershell
$env:PHOTOCRAFT_CPU_CANVAS = "1"
.\target\debug\photocraft.exe
Remove-Item Env:PHOTOCRAFT_CPU_CANVAS
```

The app keeps PhotoCraft-compatible settings and data paths to avoid breaking existing profiles. On Windows, logs are under `%APPDATA%\Photocraft\logs\`; see the [development guide](development.md#logs) for rotation and diagnostics.

## More help

- [Documentation index](README.md)
- [Troubleshooting and GPU startup](development.md#graphics-startup-and-device-loss)
- [Security reporting](../SECURITY.md)
- [Brand and screenshot evidence](../brand/lunola-luma/README.md)
