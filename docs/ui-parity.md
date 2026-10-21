# UI parity

> **Last reviewed:** 2026-10-20 · **Last updated:** 2026-10-20 · **Change:** minor (Languages & Themes and deterministic MIDI clipboard focus) · **Target:** Avid Pro Tools Ultimate 2026.4.1

How SoundCraft feels in the hands of a Pro Tools user: tools, modes, modifiers, shortcuts,
windows, meters. Measured from `crates/model/src/session.rs` (tools, modes), the command
registry (shortcuts), `crates/ui-egui/src` (windows, gestures) and user issues; compared with
reference screenshots kept locally under `plan/protools/screenshots/` (16, never committed).

**UI/UX fidelity: ~55 % ready** (estimated). Remaining ≈ 60–100 h.

Legend: ✅ matches · 🟡 partial · ❌ missing.

## Tools and modes

| Feature | Status | Notes |
|---|---|---|
| Tools: Zoom, Trim, Selector, Grabber, Scrubber, Pencil, Smart (7) | ✅ present | Trim variants (TCE, loop, scrub trim) and Grabber variants (separation, object) partial; Multitool fade/trim handles don't adjust audio (#5) |
| Smart-tool Fade In / Fade Out drag bounds and preview | ✅ | One thin curve replaces the dragged side's previous curve and matches its appearance after release; bounded to the clip, including full-clip fades. Hovering the curve's upper endpoint shows the fade cursor and lets the existing fade be dragged |
| Fade midpoint curvature | ✅ | Visible fades retain a small midpoint handle in Smart mode, including after release and extreme bends. Hover shows the vertical resize cursor; repeated drags change curvature without changing length. Curves sample at physical-pixel resolution; preview, playback and bounce share the envelope. Length drags retain curvature |
| Live waveform fade preview | ✅ | Waveform amplitude updates during fade-length and midpoint-curvature drags, sharing the curve's temporary envelope. Mono/stereo, waveform styles, zoom and clip gain remain effective; release preserves the displayed waveform with one undo step and cached peaks |
| Edit modes: Shuffle, Slip, Spot, Grid (absolute/relative) | ✅ | Spot dialog present |
| Grid and nudge values, all timebases | ✅ | Bars/Beats, Min:Secs, Timecode, Feet+Frames, Samples |
| Keyboard focus modes (Commands, Clip List, Groups) | 🟡 | Commands focus single-key editing ✅; list focus partial |
| Link timeline/edit, link track/edit, Tab to transient, insertion follows playback | ✅ | |
| Mirrored MIDI, MIDI merge, layered editing | 🟡 | Stored and partly honoured |

## Keyboard and mouse

| Feature | Status | Notes |
|---|---|---|
| Default shortcuts | 🟡 146 bound (measured: distinct `Some("…")` shortcuts in the registry) | Pro Tools' default set is several hundred incl. numeric keypad and focus keys. Collisions: Mix/Edit toggle vs zoom (#144), Heal (#140), Export (#7) |
| Ctrl as Cmd on Windows/Linux | ✅ since #15, #41 still open in some builds | #62 shows Ctrl in labels |
| Numeric keypad transport and memory-location recall | 🟡 | Classic / Transport / Shuttle modes not all present |
| Modifier gestures (Alt for all tracks, Shift-Alt for selected, Cmd-click fine faders, Ctrl-click defaults) | 🟡 | Main ones on faders and pan; not across every control |
| Scroll/zoom: wheel, trackpad pinch | 🟡 | Anchored trackpad zoom in open PRs #112, #137 |
| Drag and drop: Clip List → tracks, files → tracks | ✅ macOS/Windows; 🟡 Wayland (PR #150) | |
| Context menus (clip, track name, ruler, Clip List) | 🟡 | Clip menu includes exact-target cut/copy/duplicate and Paste Here at the clicked track/time; depth remains lower than Pro Tools'; Clip List delete (#4) |
| Drag across checkboxes to check or uncheck several | ✅ | #399; every checkbox, as in the other Craft apps |

## Windows and panels

| Window | Status | Notes |
|---|---|---|
| Edit window: rulers, track headers, view columns, Clip List, Groups, Tracks list | ✅ | Track Pin (new in 2026.4) ✅ (`track.pin`) |
| Mix window: strips, views (inserts, sends, I/O, comments, EQ curve, preamps, instruments, object) | ✅ | Narrow Mix ✅; Mix and Edit on screen at once (Window › Mix in Separate Window, its own native window) ✅; Delay Compensation view and HEAT missing |
| MIDI Editor, Score Editor, Event List | 🟡 | MIDI Editor has cursor-centred horizontal zoom, vertical zoom, pan, sideways scroll, English/Spanish key labels, explicitly focused note copy/cut/paste/duplicate, group right-edge duration resizing, continuous right-drag erase and a cached visible-note interval index verified against 100k-note linear queries; stale note selection no longer steals timeline MIDI clip shortcuts; audition and notation depth remain |
| Transport, Big Counter, Memory Locations, Automation, Video, Universe, Renderer, Languages & Themes | ✅ present | Languages & Themes keeps a stable window identity across translations; old automation paths remain aliases |
| Workspace / sound browser with previews | ❌ | G-16 |
| I/O Setup | 🟡 | Busses and paths; no hardware channel matrix (G-01) |
| Preferences | 🟡 | Many settings stored, fewer honoured |
| Window configurations, Tile/Cascade | 🟡 | Configurations ✅; Tile/Cascade in PR #151 |
| Plugin windows | 🟡 | Generic parameter sliders; Pro Tools plugins have bespoke GUIs (#138 asks for skeuomorphic design); multi-page plugin views (#6) |
| Command palette (Search) | ✅ | Over every command |
| Narrow-window toolbar and main menu | 🟡 | Stop/Play/Record and position remain fixed; secondary controls move to `More`; the main menu collapses before overlapping the title. Adaptive side-panel policy remains |

## Look and meters

| Feature | Status | Notes |
|---|---|---|
| Theme system | ✅ | Studio Dark default, Studio Light, live system appearance (#85), Nord Studio, Dracula Studio, Midnight Cyber and bounded/contrast-validated custom JSON themes |
| Interface scaling | 🟡 | Slider in PR #147 |
| Meter types and ballistics (peak, RMS, VU, K-scales, gain reduction) | 🟡 | Peak/RMS; meter type choice and GR meters partial |
| Clip indicators, track colours, waveform views (peak/power/rectified/outlines) | ✅ | |
| Fader resolution, fine adjust, numeric entry | 🟡 | |
| macOS system menu bar | ✅ | #10 |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-20 | minor | Made immediate clip paste visibly target the first free interval and selected-note paste target successive free musical bars, extending MIDI clips atomically when required |
| 2026-10-20 | minor | Renamed and stabilized Languages & Themes, added three low-fatigue palettes plus validated JSON import, and made MIDI note/timeline clipboard focus deterministic |
| 2026-10-18 | minor | Added press-origin group right-edge note resizing with end-to-grid snapping/selection retention, continuous secondary-drag erase, and keyboard/context-menu clip copy/paste |
| 2026-10-17 | minor | Piano Roll height presets/slider persist through docking, right-click deletes one note with undo, and supported requested tool/track/Commands Focus shortcuts were completed |
| 2026-10-17 | minor | Piano Roll render, hit-test, rubber-band and velocity work share a revision-cached visible-note query; exact-query regression covers 100k notes; profiling thresholds remain |
| 2026-10-17 | minor | Adaptive toolbar and main menu verified offscreen at 960×600, 480×300 logical points (high-DPI equivalent) and 1600×900; side-panel adaptation remains |
| 2026-10-17 | minor | MIDI Editor viewport/navigation, persistent English/Spanish sharp/flat key labels and note clipboard/duplicate commands; readiness estimate unchanged |
| 2026-10-13 | minor | Drag across checkboxes (#399) |
| 2026-10-13 | minor | Waveforms and fade curves share temporary envelopes during drag, without rebuilding media or editing the document before release. Readiness estimate unchanged |
| 2026-10-13 | minor | Fade curves use bounded physical-pixel sampling; midpoint handles stay visible and inside the clip, with repeat-drag coverage after extreme bends. Readiness estimate unchanged |
| 2026-10-13 | minor | Continuous fade curvature from a midpoint drag, shared UI/audio envelope, one-step undo and saved session data. Readiness estimate unchanged |
| 2026-10-13 | minor | Existing fades have draggable upper endpoints with matching hover/drag cursors, including at adjacent clip boundaries. Readiness estimate unchanged |
| 2026-10-13 | minor | Fade drags use the same thin S-curve renderer as committed fades, replacing the dragged side's old curve. Readiness estimate unchanged |
| 2026-10-13 | minor | Bounded Smart-tool fade previews and commits; full-clip fades target only the dragged clip and side. Readiness estimate unchanged |
| 2026-10-12 | minor | Mix window: Mix in Separate Window (Edit and Mix on screen at once) |
| 2026-10-10 | major | Created: tools, modes, shortcuts, windows and meters checklist against Pro Tools 2026.4.1 |
