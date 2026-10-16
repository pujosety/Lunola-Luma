# Lunola Luma Stability Audit

- **Audit date:** 2026-10-15
- **Repository:** `pujosety/Lunola-Luma`
- **Work branch:** `fix/luma-stability-overhaul`
- **Base:** `67f3154ff3fc44d6c961f4eefbfb0285a8a0f086` (`v0.8.3`)
**Audited HEAD:** `57774eb93c53c6512b6ba5c23a008ee1a0540448`

**Historical scope:** This report describes the `fix/luma-stability-overhaul` v0.8.3 snapshot and pre-merge WIP evidence. It is not post-merge verification of the v0.9.0 integration; the v0.9 port still needs CI and UI validation.

## 1. Executive summary

The audit confirmed two important architectural/platform gaps and fixed three independently testable defects:

1. **BUG-002, empty-canvas deselection:** a Move Tool zero-motion release previously ended the marquee without clearing the old layer selection. A regression test failed before the fix and passed after it. The fix preserves Shift-click selection semantics. Commit `162a63eddfabc4c21e0fa6b20f215510d38c981b`.
2. **BUG-024, Wayland clipboard non-PNG images:** the app previously relied on arboard's image getter, which requests PNG and returns no image when a Wayland source offers only another raster MIME type. Ported the relevant change from merged upstream PR #4752: bounded Wayland image reads, `image/*` MIME filtering, decoding through existing codecs, and tests for BMP/JPEG, malformed offers, and TGA MIME validation. Commit `1219b307c5749049ec9b0527383af8dc832e901f`.
3. **BUG-055, maximized window overlapping top/side taskbars:** the earlier custom-frame port only corrected the bottom taskbar. The current v0.9 WIP extends the geometry correction to all four work-area edges; its new regression tests have not yet run on the integrated tree. The old 109/109 desktop-suite result does not verify this change. Baseline commit `57774eb93c53c6512b6ba5c23a008ee1a0540448`.

**Confirmed gaps:** Free Transform has an uncommitted local multi-layer implementation: the Move controls use normalized selection bounds, preview targets preserve stack order for supported plain layers, and commit sends all targets through one engine transaction. Focused WIP tests passed on the pre-merge v0.8.3 branch, but have not been rerun after the upstream v0.9 integration. PR CI and native UI validation are pending. Native Wayland file-drop events remain unavailable in the repository's winit 0.30.13 stack; switching winit alone would be an incompatible eframe/egui-winit override. Existing XWayland fallback is available, but native Wayland DnD remains blocked pending a compatible upstream integration.

This is an evidence report, not a claim that every native interaction has been reproduced. Most BUG-IDs below have source and checked-in-test evidence only; those tests are explicitly not claimed as run unless listed in §4. Native Linux/Wayland, macOS, real Windows multi-monitor taskbar, external Photoshop round-trip, and requested performance targets remain unverified.

## 2. Bug matrix

Status names follow the master prompt. **Already Fixed** means the code/test coverage or a merged upstream fix is present; it does not mean the referenced test was executed during this audit. “Blocked” means the behavior could not be established with the available local platform/runtime or the necessary measurement was not made.

| ID | Description | Severity | Reproducible / evidence | Fix status | Tests / evidence and limits | Commit / upstream |
|---|---|---:|---|---|---|---|
| BUG-001 | Shift multi-selection leaves transform controls on one layer | P0 | **Prior gap now addressed locally:** controls use combined bounds for normalized multi-selection. | Implemented locally; uncommitted | `canvas.rs` computes normalized target union; `transform_tool::tests::selected_layers_share_one_box_and_commit_as_one_undo_step` verifies combined bounds. Native canvas interaction still needs manual try. | Local WIP; not pushed. |
| BUG-002 | Empty-canvas click does not deselect | P0 | **Reproduced RED → GREEN.** Pre-fix test left selected IDs; post-fix clears selection and active layer. | Fixed & Tested | `move_box.rs:53-68,148-179`; `move_box::tests` 3/3 and related `move_` tests 84/84 passed. Plain click clears; Shift-click preserves. | `162a63eddfabc4c21e0fa6b20f215510d38c981b` |
| BUG-003 | Transform bounds stale after edits, history or document switch | P0–P1 | Not independently reproduced; current controls recompute bounds from the active layer each frame. | Blocked | `canvas.rs:4280-4295`; the full requested move/resize/rotate/duplicate/delete/group/undo/redo/document-switch matrix was not executed. | — |
| BUG-004 | Multi-layer scale/rotate/combined bounds/one-step undo | P0 | **Pre-merge WIP implementation and focused tests passed:** normalized multi-target quad transform, per-layer stack-order previews, one-step engine transaction. Complex blend/clipping overlays are explicitly refused until preview is reliable. | Implemented locally; v0.9 CI/manual validation pending | `transform_tool` tests verify union bounds, both layers changing in one undo step, selection changes, intervening-stack preservation and Multiply refusal; `transform_cmds` selected-layer tests passed 2/2 on the pre-merge branch and have not been rerun on v0.9. | Local WIP; not pushed. |
| BUG-005 | Transform handles at 25/50/100/200% and high DPI | P1 | Visible handles and hit targets are larger and use screen-space dimensions. Native multi-DPI matrix is still untested. | Improved locally; v0.9 CI/manual verification pending | `HANDLE_SIZE_PX` is 11 pt and hit radius 14 pt; the zoom-scaling test passed on the pre-merge WIP branch and has not been rerun after the v0.9 integration. | Local WIP; not pushed. |
| BUG-006 | Blurry Transform preview | P1 | Upstream issue #91 is closed; full-resolution preview path and regression are present. | Already Fixed | `transform_tool.rs:749-792,4162-4172`; `transform_tex.rs:139-196`. Oversized content still downsamples to device texture limits. Test not rerun here. | Upstream PR #183, merge `f857b4a31f1dfd9768d6508e66274df60d488653` |
| BUG-007 | Enter/Escape/undo/redo transform behavior | P1 | No new failing report reproduced in this audit. | Blocked | Transform tool documents Enter commit/Escape cancel (`transform_tool.rs:1-8`); full native keyboard/focus/undo matrix not run. | — |
| BUG-008 | Modifier consistency (Shift, Alt/Option, snapping while dragging) | P1 | Existing modifier paths are present; no new defect reproduced. | Blocked | Transform gesture semantics in `transform_tool.rs:4-8`; modifier matrix not rerun on a native window. | — |
| BUG-009 | Pivot/anchor and zoom/pan coordinates | P1 | Not independently reproduced. Session carries a document-space pivot. | Blocked | `state.rs:883-919`; transform session initializes a center pivot in `transform_tool.rs:251-255`; group/pan/zoom scenarios not fully exercised here. | — |
| BUG-010 | Move Auto-Select hit accuracy, transparency, locks and stack order | P1 | Existing unit coverage found; not rerun in this audit. | Already Fixed | `move_auto_select_tests.rs:53-…`; locked-background/marquee tests in `move_box.rs:128-146`. | — |
| BUG-011 | Move drag freezes or layer disappears | P0–P1 | Upstream issue #128 remains open; no current frame-time reproduction. | Blocked | Live preview/local damage code is present in `move_ui.rs:99-154,259-273`. Existing perf budget is 25 ms and not enforced at `perf/budgets.toml:74-81`; the issue's ≤16 ms target is not verified. | Upstream #128; PRs #226 and #893 improve paths but issue remains open. |
| BUG-012 | Pan gestures conflict with moving layers | P1 | Space-pan is tested; middle-button behavior not separately reproduced. | Blocked | `input_tests.rs:174-189`; middle-button pan code in `canvas.rs:2999,3072-3097`; no explicit middle-drag regression in inspected tests. Trackpad hardware not tested. | — |
| BUG-013 | Wheel/trackpad zoom smoothness | P1 | Existing interaction tests; no current latency measurement. | Already Fixed | `input_tests.rs:393-435,1009-1025`; native wheel/pinch timing not measured. | — |
| BUG-014 | Cursor-centered zoom | P1 | Existing anchoring tests; not rerun here. | Already Fixed | `input_tests.rs:393-435`; actual monitor/device behavior not tested. | — |
| BUG-015 | Canvas flicker / texture invalidation | P1 | No visual flicker reproduction. | Blocked | Cache and incremental update tests exist (`canvas.rs:5678-5815`, `gpu_canvas_mip_tests.rs:52-103`) but do not constitute a native visual flicker test. | — |
| BUG-016 | Fit/Fill/100%/pixel-grid accuracy | P2 | Existing geometry/device-pixel tests; not rerun here. | Already Fixed | `canvas.rs:7109-7152`; native fractional scaling remains untested. | — |
| BUG-017 | Layers Panel and canvas selection synchronization | P0–P1 | Selection is derived from the document's selected-layer set. | Already Fixed | `panels.rs:2088-2090,2267,2652-2657`; BUG-002 adds an executed empty-click regression. | — |
| BUG-018 | Reveal selected layer in Layers Panel | P2 | Existing active-row reveal test. | Already Fixed | `layer_reveal_tests.rs:66-115`; test not rerun here. | — |
| BUG-019 | Layer reorder insertion, hierarchy and undo | P1 | UI reorder/group-boundary coverage exists; no data-loss defect reproduced. | Blocked | `layer_row_ui/tests.rs:348-360`; `engine/src/clip_group.rs:217-235`. Reorder test does not establish undo/redo restoration. | — |
| BUG-020 | Undo/redo for duplicate/delete/group/ungroup/reorder/rename/masks | P1 | Partial history coverage only. | Blocked | Duplicate/delete/group/mask tests: `layer_multi_cmds.rs:1662-1727,2083-2104,2444-2468`; `extra_cmds.rs:1540-1560`. Ungroup/reorder tests do not establish history restoration (`extra_cmds.rs:1647-1656`). | — |
| BUG-021 | Hidden/locked content modified through unintended paths | P0 | Guard paths and tests are present; not rerun here. | Already Fixed | `hidden_layer_tests.rs:48-83`; locked transform refusal in `canvas.rs:7256-7294`. | — |
| BUG-022 | Smart Objects disappear after layer/document operations | P1 | Persistence and duplicate-document tests are present; not rerun here. | Already Fixed | `engine/tests/smart_duplicates.rs:84-137`; `smart_psd.rs:78-144`. | — |
| BUG-023 | Native Linux Wayland file-drop failure | P0 | **Dependency-stack blocker confirmed:** `Cargo.lock` resolves eframe/egui-winit 0.36.2 and winit 0.30.13; winit 0.30.13 emits native drop events in its X11 processor ([source](https://github.com/rust-windowing/winit/blob/v0.30.13/src/platform_impl/linux/x11/event_processor.rs#L544-L550)), with no corresponding Wayland event path. | Blocked | App requests DnD in `apps/photocraft/src/main.rs:76-95`; UI consumes `raw.dropped_files` in `crates/ui-egui/src/lib.rs:1667-1676`. Existing XWayland helper is `services.rs:388-400`. No native compositor run. Do not override winit without an eframe/egui-winit compatibility port. | Upstream #386 open/reopened; #1893 is an issue, not a PR. |
| BUG-024 | Linux clipboard image import, including non-PNG Wayland offers | P0 | Pre-fix local test returned `None`; post-fix decode/MIME helper tests pass. Linux native data-control path not exercised. | Fixed but Untested | Fallback and 1 GiB bounded read: `apps/photocraft/src/services.rs:351-385,485-495`; tests cover BMP/JPEG, malformed offers and TGA. `cargo tree --locked --target x86_64-unknown-linux-gnu` resolves wl-clipboard-rs 0.9.4, but Linux-specific code was not compiled/run here. Copied-file-first behavior remains. | Upstream PR #4752 (merge `6f85054288a564f9fbbccb99268ffdb34e581584`); local `1219b307c5749049ec9b0527383af8dc832e901f` |
| BUG-025 | Multi-file import/import-as-layers and partial failure | P1 | Open-many and sequential dropped-file placement are covered; batch import-as-layers is not established. | Blocked | `file_open/tests.rs:126-140,164-174,405-455`; tests inject events and do not test native compositor drops. | — |
| BUG-026 | Save/Save As/export correctness and overwrite behavior | P0 | Core save/export paths and failure cases have tests. | Already Fixed | `file_open/tests.rs:176-243`; atomic write failures: `crates/format/src/atomic/tests.rs:116-177`. Native overwrite dialog was not exercised. | — |
| BUG-027 | PNG alpha, dimensions and color profiles | P1 | Codec support and synthetic tests are present. | Already Fixed | `crates/codecs/src/format.rs:232-234`; `codecs/src/codecs/png.rs:258-279`; `codecs/tests/png_jpeg.rs:68-94,166-174`; ICC tests `codecs/tests/metadata.rs:117-145`. | — |
| BUG-028 | PSD round-trip hierarchy/masks/blends/text/artboards/Smart Objects | P0 | Broad synthetic round-trip coverage; no external Photoshop open-save session in this audit. | Already Fixed | `io/tests/roundtrip.rs:30-50`; `psd_structure.rs:43-89,329-350,421-550`; `comps_artboards.rs:168-195`; `text_corpus.rs:175-216`; `smart_psd.rs:78-144`; `adjust_warnings.rs:17-40`. | PSD legacy `Txt2` fix is already merged in prior PR #4; Photoshop manual validation remains outstanding. |
| BUG-029 | Corrupt/oversized documents and allocation failures | P0 | Corruption/dimension limits are tested; 29,000² PSB stress test is ignored. | Blocked | `format/tests/corruption.rs:41-95`; `codecs/tests/limits.rs:20-52,83-110`; `format/src/read.rs:46-53,105-114`; ignored stress case `io/tests/large_psb.rs:26-49` needs about 12 GB. | — |
| BUG-030 | Marquee Shift/Alt and zoom/pan behavior | P1 | Existing geometry/modifier tests. | Already Fixed | `marquee_tests.rs:136-237`; not rerun in this audit. | — |
| BUG-031 | Brush/Eraser/Gradient Shift angle snapping | P1 | Upstream issue #178 closed; implementation and tests are present. | Already Fixed | `stroke_constraint.rs:3-9,64-84,183-210,294-311`; wiring in `canvas.rs:4577-4580,4702-4709,4931-4944`. Separate Ctrl straight-line request from #178 is not evidenced as implemented. | Upstream PR #188 (`4b53f62`). |
| BUG-032 | Shape Tool ignores selected stroke color | P1 | Upstream issue #172 closed; selected stroke and visible preview/commit path present. | Already Fixed | `vector_ui.rs:148-175`; `tool_feedback.rs:155-185,213-217`; preview/commit parity test `vector_paint_tests.rs:150-179`. No native visual run. | Upstream PR #188 (`4b53f62`), later tests in #274 (`1520d1c`). |
| BUG-033 | Crop/Pen/Path Selection hit testing, handles and commit/cancel | P1 | Existing interaction/history tests. | Already Fixed | `crop_ui.rs:1075-1095,1128-1173,1245-1272`; `path_selection_tests.rs:53-103`. | — |
| BUG-034 | Font fallback, CJK glyphs and positioning | P2 | CJK/script fallback and metrics tests exist. | Already Fixed | `cjk_fonts.rs:683-755,778-827`; installed host fonts/native rendering not verified. | — |
| BUG-035 | Keyboard shortcuts and text-field focus | P1 | Context/focus tests exist. | Already Fixed | `shortcut_dispatch/tests.rs:182-226`; not rerun here. | — |
| BUG-036 | Filter preview differs from committed output | P1 | Preview/command comparisons exist. | Already Fixed | `adjust_preview_tests.rs:111-144`; not rerun here. | — |
| BUG-037 | Slow layer selection/switching and ≤16 ms/frame performance | P1 | Bench harness exists; no comparable measurements collected. | Blocked | `examples/layout_bench.rs:240-274`; synthetic document definition `tests/support/layout_doc.rs:1-8,29-37`. Budget/zoom caveat in `perf/budgets.toml:209-214`. No performance claim is made. | Upstream #125 closed via #226, but target performance is not verified locally. |
| BUG-038 | Thumbnail/tile/transform/composite cache correctness | P1 | Unit/integration cache parity and lifecycle coverage present. | Already Fixed | `canvas.rs:5678-5815`; `gpu_canvas_mip_tests.rs:52-103`; `engine/tests/smart_filter_stages.rs:23-52`. Not rerun here. | — |
| BUG-039 | Long menus clip on short displays | P2 | Scrollable menu path and short-window tests exist. | Already Fixed | `menu_nav.rs:423-452`; `menu_nav_tests.rs:154-203`; `menus.rs:1763-1790`. No native 1080p reproduction. | Upstream #166 closed via #160/#168; later refinement #272. |
| BUG-040 | Dock tab overflow / inaccessible tabs | P2 | Elide/chevron behavior and 1×/2× dock tests exist. | Already Fixed | `tab_strip.rs:1-23,82-124`; `dock_tests.rs:810-910`. Native visual run not performed. | Upstream #151 closed via #181 (`b9809ca`). |
| BUG-041 | Closed panels cannot be reopened | P2 | Close/reopen panel tests exist. | Already Fixed | `dock_tests.rs:625-672`; not rerun here. | — |
| BUG-042 | Workspace panel positions/visibility after restart | P1 | Workspace state persists, but exact dock geometry/visibility after a fresh process restart is not established by the inspected test. | Blocked | Workspace switch test `panel_docking_tests.rs:99-113`; preference round-trip `prefs_ui.rs:2772-2791`; JSON name check `workspace_ui.rs:604-610`. | — |
| BUG-043 | Visible preferences that do nothing | P1 | A guard checks preference consumption/hiding; a complete manual audit of every visible control was not performed. | Blocked | `engine/tests/prefs_usage.rs:92-151`; no specific dead control reproduced in this audit. | — |
| BUG-044 | Linux GNOME/KDE/fractional HiDPI and native resize/input | P1 | UI scale and physical-pixel geometry tests exist; compositor/hardware behavior not tested. | Blocked | `prefs_ui.rs:2500-2547`; `canvas.rs:7136-7152`. No GNOME/KDE/Wayland session available. | — |
| BUG-045 | Move Tool resize controls (#4005) | P1 | The Move controls now use the normalized selection union with larger screen-space handles. | Implemented locally; v0.9 CI and native try pending | `canvas.rs` computes combined bounds; 11 pt visual handles and 14 pt hit radius were tested for zoom scaling on the pre-merge WIP branch; v0.9 rerun is pending. | Local WIP; not pushed. |
| BUG-046 | Atomic save can damage existing files on failure | P0 | Same-directory temp+rename strategy and write/rename failure tests are present. | Already Fixed | `engine/src/file_cmds.rs:89-97`; `format/src/atomic.rs:173-205`; failure injection `format/src/atomic/tests.rs:119-177`. Separate durability caveat: directory sync result after publication is ignored at `atomic.rs:202-205`; no routine data loss inferred. | Upstream #203 closed by PR #230 (`82270711`); follow-up #4968/#5420 remains open. |
| BUG-047 | Preferences/recent files lost after restart | P1 | Recents persist to preferences; persistence tests and unused-preference guard are present. | Already Fixed | `crates/ui-egui/src/lib.rs:1117-1158`; test `canvas.rs:6221-6226`; `prefs_usage.rs:92-151`. | Upstream #204 closed by PR #245 (`34de1a5`). |
| BUG-048 | Opening artboard PSD crashes | P0 | Upstream investigation attributes sample failure to GPU OOM/device loss; CPU fallback and preservation test exist. | Already Fixed | `canvas.rs:1445-1456,1480-1494`; `tests/gpu_device_loss.rs:129-147`. GPU test may skip without adapter; no native sample replay. | Upstream #332 closed; fixes referenced in #187/#252. |
| BUG-049 | Eyedropper stops sampling while dragging | P1 | Move sampling and red-to-green drag test are present. | Already Fixed | `canvas.rs:4701-4709,6353-6366`; test not rerun here. | Upstream #164 closed via #176 (`d8cf255`). |
| BUG-050 | Liquify Undo does not refresh preview | P1 | Undo rebuild/preview invalidation path and comparison test exist. | Already Fixed | `liquify_ui.rs:286-305,325-329,1189-1235`; not rerun here. | Upstream #158 closed via #175 (`107302f`). |
| BUG-051 | Curves endpoint handles clipped | P1 | Graph inset and endpoint-fit test exist. | Already Fixed | `adjust_editors.rs:343-349,1694-1706`; DPI variants not explicitly covered. | Upstream #156 closed via #177 (`043e050`). |
| BUG-052 | Select → Color Range fails on Linux | P1 | No source-level gap found; dialog open/preview/apply and CPU/no-GPU tests are present. | Already Fixed | `menus.rs:464-465`; `dialogs.rs:564-573`; `color_range_ui.rs:1256-1300`. No native Linux/XWayland run. | Upstream #145 closed; dialog work #80 and no-GPU coverage #168. |
| BUG-053 | Character panel hides Properties | P2 | Character/Paragraph panels share a group; Properties-retention tests exist. | Already Fixed | `dock.rs:101-110`; `dock_tests.rs:767-805`; not checked in a native window. | Upstream #150 closed via #181 (`b9809ca`). |
| BUG-054 | Document tab overflow hides open documents | P1 | Overflow menu/tab elision and active-tab tests exist. | Already Fixed | `canvas.rs:1917-2017,2249-2330`; `canvas/tabs_tests.rs:38-88`. | Upstream #1276 closed via #1293/#1367 (`c84ec1b`). |
| BUG-055 | Maximized custom frame overlaps non-bottom taskbars | P1 | The earlier `57774` port trimmed the bottom work area. The current v0.9 WIP uses a four-edge rectangle correction; its tests have not run on this integrated tree. | WIP; v0.9 unverified | New `clamped_rect` tests cover bottom/top/left/right, negative monitor origins, border sizes and dragged/snapped windows. The earlier 109/109 desktop suite predates this enhancement and is not evidence for the new code; actual Windows multi-monitor GUI remains untested. | Upstream PR #5237 baseline (`c73bb0ed`); local all-edge WIP. |

## 3. Other required phases and upstream/security findings

### Wayland and clipboard

- `Cargo.lock` resolves eframe/egui/egui-winit 0.36.2 and winit 0.30.13. The desktop requests native drag-and-drop, but the current winit 0.30 Wayland implementation lacks the event path; the UI's `raw.dropped_files` handler cannot compensate for events never delivered.
- XWayland restart/selection guidance and `services::xwayland_command` exist. This is a fallback, **not a verified native Wayland fix**. The GNOME 51/XWayland reporter path in upstream #386 was not reproduced.
- The non-PNG clipboard patch adds `wl-clipboard-rs = 0.9.4` only under Linux target dependencies and limits transferred bytes to 1 GiB. Unit tests validate image selection/decoding, not the Wayland compositor/data-control call itself.
- Multi-file open and per-file canvas placement work through existing paths; “import all as a layer stack” is not established by inspected tests.

### Upstream issue status sampled for this audit

| Issue | Upstream state and verified fix | Local conclusion |
|---|---|---|
| #91 | Closed; PR #183 merged. | Full-resolution preview path is present. |
| #128 | Still open; PR #226 and #893 improve device-loss/live-preview/effect-overhang paths. | No current ≤16 ms measurement; not declared resolved. |
| #4005 | Open; no linked fix found. | Local implementation adds combined multi-layer controls and larger handles; PR/CI and manual desktop validation pending. |
| #203 | Closed by atomic-save PR #230. | Temp+rename fix present; post-publication directory-sync result remains a durability caveat. |
| #204 | Closed by preferences/recents PR #245. | Recents/prefs persistence present. |
| #332 | Closed; comments identify large-document GPU OOM/device loss rather than artboards themselves. | CPU fallback test present. |
| #1276 | Closed by #1293/#1367. | Document overflow menu and active-tab tests present. |
| #3864 | Issue remains open; PR #5237 merged for top-taskbar work-area correction. | Ported locally and geometry-tested; reporter's exact Windows setup unverified. |
| #386 | Open/reopened; #384/#1073/#1686 merged for related clipboard/X11 guidance and preference. | Native Wayland DnD remains dependency-blocked. |
| #338 | Open; merged #4752 fixes non-PNG Wayland clipboard images. | Ported locally; Linux-specific integration uncompiled here. |
| #145 | Closed/completed; #80/#168 contain dialog/no-GPU work. | No code-level gap found; native Linux not run. |
| #178/#172/#164/#158/#156/#125/#166/#151/#150 | Closed with the corresponding merged fixes/tests listed in BUG-031/032/039/040/049/050/051/053. | Source/test parity found; most test files were not rerun during this audit. |
| #50 | Open broad RAW-support tracking issue; some RAW PRs merged; CR3 PR #5795 closed unmerged. | CR3 is identified but explicitly “not decoded yet” in `crates/raw/src/lib.rs:88-130,189-240`; do not claim full CR3 processing. |
| #913 | Closed, not planned; it requests a private security-report channel, not a demonstrated exploit. | `SECURITY.md:5-11` has no dedicated private channel and directs reporters to contact maintainers privately. This is a repository-process limitation, not a confirmed vulnerability. |

### Security scope and limits

- `SECURITY.md:3,37-46,50-52` documents hostile documents/automation as attack surfaces and existing control-server/workspace limits; it also lists missing JSON-depth, render, memory, command-duration, client-identity, encryption and audit controls.
- The code search found the macOS tablet module as the isolated workspace unsafe boundary (`crates/tablet/src/lib.rs:8-16`; `crates/tablet/src/macos.rs:43-69`), plus unsafe code in the sample plugin. This is not a line-by-line unsafe-code audit.
- `cargo audit`, `cargo deny` and `cargo vet` are not installed in this environment. No advisory database scan or fuzz run was performed; the baseline CI fuzz job was skipped. No vulnerability is claimed either way.

## 4. Testing evidence

### Executed on the current work branch

- **BUG-002 RED:** `cargo test -p photocraft-ui-egui --lib move_box::tests::clicking_an_empty_spot_clears_layer_selection -- --exact --nocapture` failed before the production fix because the previous selected layers remained.
- **BUG-002 GREEN:** `cargo test -p photocraft-ui-egui --lib move_box::tests -- --nocapture` — 3 passed; includes the no-motion plain click and Shift-click preservation.
- `cargo test -p photocraft-ui-egui --lib move_ -- --nocapture` — 84 passed.
- **BUG-024 RED/GREEN:** the exact non-PNG helper test failed against the empty helper stub before implementation. After the port, `cargo test -p photocraft --bin photocraft clipboard -- --nocapture` — 3 passed, 0 failed.
- **BUG-055:** `cargo test -p photocraft --bin photocraft window_frame::platform::tests -- --nocapture` — 5 passed, 0 failed.
- Full desktop app unit suite: `cargo test -p photocraft --bin photocraft -- --nocapture` — **109 passed, 0 failed**.
- `cargo clippy -p photocraft --bin photocraft -- -D warnings` — passed.
- `cargo fmt --all -- --check` — passed; `git diff --check` — passed.
- `cargo check --target wasm32-unknown-unknown -p photocraft-ui-egui` — passed (the UI crate did not change afterward).
- `cargo tree --locked --target x86_64-unknown-linux-gnu -p photocraft -e normal` — passed and resolved direct `wl-clipboard-rs 0.9.4`; this is a dependency graph check, **not** a Linux compile.

### Baseline before edits

At base commit `67f3154ff3fc44d6c961f4eefbfb0285a8a0f086`, `cargo fmt --all -- --check`, `cargo check --workspace`, and `cargo clippy --workspace --all-targets` passed. `cargo test --workspace` timed out at 420 seconds without a final result; it is neither recorded as pass nor fail. GitHub CI run `38555468221` on that base passed Windows/Linux tests, corpus tests, wasm32 check, and web-size gate; its fuzz job was skipped. These baseline CI results do **not** validate the three local commits.

### Not verified

- Linux build/test of the new `#[cfg(target_os = "linux")]` clipboard call and a real Wayland/XWayland clipboard round-trip.
- Native Wayland DnD, GNOME/KDE/fractional HiDPI, trackpad and compositor-specific interactions.
- macOS build/runtime; current machine is Windows.
- Full workspace tests after the changes; previous baseline run timed out.
- Photoshop manual PSD open-save round-trip, external CR3 decoding, full 29,000² PSB stress test, or the ≤16 ms/frame layer-selection performance target.
- Real Windows stock/ExplorerPatcher multi-monitor taskbar appearance; BUG-055 coverage here is geometry/unit and app-suite coverage only.
- Full dependency advisory scan/fuzz/security assessment.

## 5. Technical changes and remaining debt

### Changes committed locally

1. `162a63eddfabc4c21e0fa6b20f215510d38c981b` — clear layer selection on a plain zero-motion empty Move-tool click; preserve Shift-click and one-axis marquee no-op behavior.
2. `1219b307c5749049ec9b0527383af8dc832e901f` — Linux-only `wl-clipboard-rs` 0.9.4 fallback for clipboard image MIME offers arboard cannot read as PNG; MIME filtering, image codec decode, TGA type check, and bounded reads.
3. `57774eb93c53c6512b6ba5c23a008ee1a0540448` — correct custom maximized window frame against all four work-area edges, with idempotence and dragged/snapped-window guards.

### Verified local draft (not yet committed or pushed)

- Multi-layer Free Transform uses normalized selected targets, combined content bounds and a single engine transform/undo step. The preview draws targets in stack order, excludes other targets from per-layer overlays, caps aggregate preview allocation, and refuses non-Normal/blended/clipped arrangements it cannot reproduce faithfully.
- Selection changes end the current transform snapshot. Move-tool controls use the same combined bounds. Visible handles are 11 pt; hit targets use a 14 pt screen-space radius.
- Post-preview-cap focused UI regressions, selected-layer engine tests, strict UI Clippy (`-D warnings`) and formatting pass. Desktop build, CI, and manual UI interaction are still pending.

### Remaining technical debt / next steps

- Build and manually try the local desktop app, then finish PR/CI verification for multi-layer Free Transform. Complex preview cases remain explicitly unsupported until they can be shown correctly.
- Integrate native Wayland DnD only with a compatible eframe/egui-winit/winit update or upstream integration. Keep XWayland as a documented fallback; do not force an incompatible dependency override.
- Run Linux CI/native clipboard integration and platform GUI tests for Wayland, X11, Windows taskbars and macOS before declaring cross-platform completion.
- Measure BUG-011/037 against the actual performance target on a comparable layered document; keep the current budget honest.
- Resolve/clarify RAW/CR3 capability messaging and add a licensed fixture or explicit unsupported-format test as appropriate.
- Run `cargo audit`/advisory scanning and the configured fuzz suite when those tools/runners are available; establish the private disclosure route as repository policy if maintainers choose to provide one.
- Expand structural undo/reorder/ungroup tests, multi-file-as-layers tests, fresh-process workspace layout tests, and native overwrite-dialog tests.

**Audit snapshot at initial report commit:** the three code fixes above plus report commit `12fd7ae757040f53d10adc6558c8675361babbc6` were local (four commits ahead of `lunola/main`); no push had been performed.