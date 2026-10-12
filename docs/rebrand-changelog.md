# Lunola Luma rebrand change log

**Scope:** display identity and release preparation for the Lunola Luma fork, based on PhotoCraft commit `c20b81b25168057e9cbffa2f45c36ec02dacf756`. This is an implementation record, not a published release changelog.

## Identity and assets

- Set the visible product name to **Lunola Luma**, parent brand to **LUNOLA**, product to **LUMA**, and category to **Professional Photo Editor**.
- Used the supplied app icon and lockup as the authoritative references. Platform icon files and the macOS DMG art are generated from those references; no new logo artwork was drawn.
- Updated desktop/window/About branding and app-store/package descriptions. Windows resources display ProductName `Lunola Luma`, CompanyName `LUNOLA`, while retaining the original executable filenames and upstream copyright line.
- No separate launch splash was added: the current start screen already displays the approved lockup, and a transient duplicate would delay the editor without adding approved artwork.
- Removed old PhotoCraft UI captures from the Lunola Luma README feature gallery. The archival images remain under `docs/images/` with attribution and are explicitly identified as upstream references, not Lunola Luma screenshots.
- Kept upstream author credits, source-code licenses, third-party notices, and the historical ArtCraft trademark restrictions. Removed ArtCraft artwork from the rebrand while retaining its legal notice.

## Menus and community controls

- Removed the Help top-level menu from the custom menu bar and macOS native menu setup.
- Moved About Lunola Luma to File. System Info remains searchable.
- About displays **By Pujo Setyo** as the Lunola Luma project byline; the Contributors and Models tabs are removed. Original PhotoCraft contributor data, author credits, and legal notices remain intact.
- Replaced visible product wording in About's engine attribution, GUI/CLI version output, startup/error labels, and CLI help with Lunola Luma. Compatibility-oriented executable names, settings/log identifiers, and upstream/legal attribution remain unchanged.
- Removed Discord/community UI links and controls without removing required plain-text attribution to PhotoCraft and its authors.

## Distribution and compatibility

- Updated macOS bundle/DMG display assets, Linux desktop/AppStream/package descriptions, and Windows executable/installer display metadata.
- Preserved compatibility identifiers: Cargo packages, executable/bundle/desktop IDs, registry upgrade keys, MIME types, document formats and signatures, extensions, environment variables, portable-data folders, settings paths, and command IDs.
- Release directory contains a draft checklist only. No Lunola Luma package, signing operation, release tag, or GitHub release is asserted.

## Verification

- Windows `cargo fmt --all --check` and `CARGO_INCREMENTAL=0 cargo build -j 1 -p photocraft -p photocraft-cli`: passed. GUI and CLI report `0.7.0 (dev build)` under Lunola Luma names.
- Full UI library tests: 2,184 passed, 1 failed, 10 ignored. The sole failure is the Czech duplicate `"Tool"` catalog key, also present in the recorded baseline; details are in [`windows-smoke-test.md`](windows-smoke-test.md).
- The four targeted Wayland-guidance tests passed; the live About dialog shows the byline, the Lunola Luma Engine attribution, and no Contributors or Models tabs.
- Fresh Windows GUI run confirmed no Help menu, About under File, and opening/saving a synthetic PNG. CLI conversion PNG → `.pcraft` → WebP was reopened and inspected successfully.
- Windows shortcut icon-reference check and XML well-formedness checks passed. WiX/MSI build and signing were not verified because `dotnet`/`wix` were unavailable. See the [smoke-test report](windows-smoke-test.md).
