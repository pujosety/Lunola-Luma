# Lunola Luma by Pujo Setyo — release materials

## Status

The public **Lunola Luma v0.8.2** prerelease is live at [GitHub Releases](https://github.com/pujosety/Lunola-Luma/releases/tag/v0.8.2), tagged at main commit `9da31de69a84f160f7f4634c4929387c5e8cf0d6`. It has 15 uploaded assets: 14 package files and `SHA256SUMS.txt`; every package checksum matches GitHub's reported SHA-256 digest. It includes unsigned Windows x64 MSI/portable ZIP, Linux x86_64/aarch64 packages, and Flatpak x86_64/aarch64. It omits macOS, Linux riscv64, WebAssembly, Windows x86/ARM64, and FreeBSD. The CI/test caveats are recorded in [`0.8.2.md`](0.8.2.md). The earlier [v0.7.0 release](https://github.com/pujosety/Lunola-Luma/releases/tag/v0.7.0) remains available.

Only artifacts built from the Lunola Luma candidate and verified for the stated platform may be attached. Do not relabel unrelated downloads as Lunola Luma builds.

## Before any release

- [ ] Confirm the next version/tag, release owner, distribution channels, domains, and support route.
- [ ] Confirm permission and usage terms for the supplied Lunola Luma marks; keep upstream attribution and third-party notices intact.
- [ ] Run formatting, tests, platform builds, required corpus checks, and an editing/open-save-export smoke test. Record known baseline failures.
- [ ] Build and inspect packages for Windows, macOS, Linux, FreeBSD, and web as intended; verify icons, installer wizard text, file associations, and stable upgrade keys.
- [ ] Configure and verify trusted signing/notarization credentials through the project's approved CI secrets; never ship unsigned artifacts as signed.
- [ ] Generate checksums, SBOM/license materials if required, versioned notes, and rollback/incident instructions.
- [ ] Verify every download URL and screenshot points to the intended Lunola Luma candidate.
- [ ] Review and publish only after the owner explicitly approves the release candidate.

[`docs/releasing.md`](../docs/releasing.md) documents the configured release workflow. For 0.7.0, the automated release job skipped after some platform jobs failed; a maintainer assembled the verified successful CI outputs and published the preview with the missing platforms, unsigned Windows packages, and known test failure disclosed. Future releases still require explicit owner approval.
