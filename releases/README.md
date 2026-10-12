# Lunola Luma by Pujo Setyo — release materials

## Status

The GitHub draft **Lunola Luma v0.7.0** is created and remains unpublished. Maintainers can review the [draft and its assets](https://github.com/pujosety/Lunola-Luma/releases/tag/untagged-e224a33c9c5c21e772fe); the public [Releases page](https://github.com/pujosety/Lunola-Luma/releases) will show downloads after publication. The draft contains 19 verified CI package files plus `SHA256SUMS.txt`; it does not include macOS, Linux riscv64, or WebAssembly builds. Windows MSI and portable executables are unsigned. The current evidence and limitations are in [`0.7.0-draft.md`](0.7.0-draft.md).

Only artifacts built from the Lunola Luma candidate and verified for the stated platform may be attached. Do not relabel unrelated downloads as Lunola Luma builds.

## Before any release

- [ ] Confirm version/tag `0.7.0`, release owner, distribution channels, domains, and support route.
- [ ] Confirm permission and usage terms for the supplied Lunola Luma marks; keep upstream attribution and third-party notices intact.
- [ ] Run formatting, tests, platform builds, required corpus checks, and an editing/open-save-export smoke test. Record known baseline failures.
- [ ] Build and inspect packages for Windows, macOS, Linux, FreeBSD, and web as intended; verify icons, installer wizard text, file associations, and stable upgrade keys.
- [ ] Configure and verify trusted signing/notarization credentials through the project's approved CI secrets; never ship unsigned artifacts as signed.
- [ ] Generate checksums, SBOM/license materials if required, versioned notes, and rollback/incident instructions.
- [ ] Verify every download URL and screenshot points to the intended Lunola Luma candidate.
- [ ] Review and publish only after the owner explicitly approves the release candidate.

[`docs/releasing.md`](../docs/releasing.md) documents the configured release workflow. It normally creates or updates a draft after all required jobs pass; for 0.7.0, the partial draft was assembled from successful CI outputs after the automated release job skipped. Publishing still requires explicit owner approval.
