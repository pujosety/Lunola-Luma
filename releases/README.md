# Lunola Luma release materials

## Status

No Lunola Luma release has been published, tagged, signed, or packaged. The workspace currently inherits version `0.7.0` from the PhotoCraft source; that value alone does not make this a Lunola Luma release. `0.7.0-draft.md` is a planning record, not release notes for distribution.

Do not rename or redistribute upstream PhotoCraft artifacts as Lunola Luma. Upstream download references in the [project README](../README.md#downloads) are labelled and remain the original project's builds.

## Before any release

- [ ] Approve the version/tag, release owner, distribution channels, domains, and support route.
- [ ] Confirm permission and usage terms for the supplied Lunola Luma marks; keep upstream attribution and third-party notices intact.
- [ ] Run format, full workspace tests, platform builds, corpus checks required by changed code, and an editing/open-save-export smoke test. Record known baseline failures.
- [ ] Build and inspect packages for Windows, macOS, Linux, FreeBSD, and web as intended; verify icons, installer wizard text, file associations, and stable upgrade keys.
- [ ] Configure and verify trusted signing/notarization credentials through the project's approved CI secrets; never ship unsigned artifacts as signed.
- [ ] Generate checksums, SBOM/license materials if required, versioned notes, and rollback/incident instructions.
- [ ] Verify every download URL and screenshot points to Lunola Luma artifacts, not the PhotoCraft upstream release page.
- [ ] Review and publish only after the owner explicitly approves the release candidate.

The inherited [`docs/releasing.md`](../docs/releasing.md) documents the upstream PhotoCraft release workflow. It is useful implementation reference, not authorization to publish Lunola Luma builds.
