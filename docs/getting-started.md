# Getting started with Lunola Luma

## Release status

Lunola Luma v0.8.3 is published as a partial preview; it is not a stable release. Review the [release notes](../releases/0.8.3.md) and [GitHub release](https://github.com/pujosety/Lunola-Luma/releases/tag/v0.8.3). Downloads cover Windows x86/x64/ARM64, Linux x86_64/aarch64/riscv64, FreeBSD 14 x86_64, Flatpak x86_64/aarch64, and WebAssembly. macOS is unavailable and Windows installers/executables are unsigned; verify packages against `SHA256SUMS.txt`. Do not use unrelated upstream downloads as Lunola Luma packages. For development, build this repository from source.

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

For automation or batch conversion, see the [CLI guide](development.md#headless-cli). Internal package, crate, and command identifiers remain unchanged for compatibility.

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

The app keeps legacy settings and data paths to avoid breaking existing profiles. On Windows, logs are under `%APPDATA%\Photocraft\logs\`; see the [development guide](development.md#logs) for rotation and diagnostics.

## More help

- [Documentation index](README.md)
- [Troubleshooting and GPU startup](development.md#graphics-startup-and-device-loss)
- [Security reporting](../SECURITY.md)
- [Brand and screenshot evidence](../brand/lunola-luma/README.md)
