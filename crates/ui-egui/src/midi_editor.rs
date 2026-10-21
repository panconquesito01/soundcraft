//! The MIDI Editor: a piano roll with a velocity lane, docked under the Edit window.

use crate::SoundApp;
use crate::theme::{Tokens, bold, regular};
use egui::{Align2, Color32, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};
use serde_json::json;
use soundcraft_model::{ClipContent, ClipId, Tool};
use soundcraft_time::TICKS_PER_QUARTER;

const KEY_W: f32 = 48.0;
const ROW_H: f32 = 9.0;
const VEL_H: f32 = 64.0;
const MIN_ROW_H: f32 = 7.5;
const MAX_ROW_H: f32 = 28.0;
const MIN_TICKS_PER_PIXEL: f64 = 0.25;
const NOTE_INDEX_BLOCK: usize = 256;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteLabelMode {
    Off,
    #[default]
    Roots,
    All,
}

#[derive(Debug, Clone)]
pub struct MidiViewport {
    /// First visible clip-relative tick.
    pub start_tick: f64,
    /// Horizontal scale. A smaller value is a closer zoom.
    pub ticks_per_pixel: f64,
    /// Height of one semitone row in logical points.
    pub row_height: f32,
}

impl Default for MidiViewport {
    fn default() -> Self {
        Self { start_tick: 0.0, ticks_per_pixel: 1.0, row_height: ROW_H }
    }
}

impl MidiViewport {
    fn fit(&mut self, len_ticks: i64, width: f32) {
        self.start_tick = 0.0;
        self.ticks_per_pixel = (len_ticks.max(1) as f64 / f64::from(width.max(1.0))).max(MIN_TICKS_PER_PIXEL);
    }

    fn max_ticks_per_pixel(len_ticks: i64) -> f64 {
        (len_ticks.max(1) as f64 / 40.0).max(1.0)
    }

    fn clamp_horizontal(&mut self, len_ticks: i64, width: f32) {
        self.ticks_per_pixel = self.ticks_per_pixel.clamp(MIN_TICKS_PER_PIXEL, Self::max_ticks_per_pixel(len_ticks));
        let visible = f64::from(width.max(1.0)) * self.ticks_per_pixel;
        self.start_tick = self.start_tick.clamp(0.0, (len_ticks as f64 - visible).max(0.0));
    }

    fn tick_at(&self, x: f32, left: f32) -> f64 {
        self.start_tick + f64::from(x - left) * self.ticks_per_pixel
    }

    fn x_of(&self, tick: i64, left: f32) -> f32 {
        left + ((tick as f64 - self.start_tick) / self.ticks_per_pixel) as f32
    }

    fn zoom_horizontal(&mut self, anchor_x: f32, left: f32, factor: f64, len_ticks: i64, width: f32) {
        let anchor_tick = self.tick_at(anchor_x, left);
        self.ticks_per_pixel = (self.ticks_per_pixel / factor.max(0.01)).clamp(MIN_TICKS_PER_PIXEL, Self::max_ticks_per_pixel(len_ticks));
        self.start_tick = anchor_tick - f64::from(anchor_x - left) * self.ticks_per_pixel;
        self.clamp_horizontal(len_ticks, width);
    }
}

#[derive(Debug, Clone, Default)]
pub struct MidiEditorState {
    pub selected: Vec<usize>,
    /// Keyboard clipboard ownership. A click inside the Piano Roll claims it; a click elsewhere
    /// returns Ctrl/Cmd+C/X/V/D to timeline clips even if note selection remains visible.
    pub keyboard_focus: bool,
    pub drag: Option<NoteDrag>,
    /// A held secondary-button pass. Each crossed note is removed immediately and all deletes in
    /// one pass share an undo step.
    pub erase: Option<NoteErase>,
    pub scroll: f32,
    pub clip: Option<ClipId>,
    /// Rubber-band selection start (screen position).
    pub band: Option<egui::Pos2>,
    pub viewport: MidiViewport,
    /// Middle-button pan origin: pointer, horizontal tick and vertical pixel offset.
    pub pan: Option<(egui::Pos2, f64, f32)>,
    /// The pointer was over the note or velocity area in the last rendered frame.
    pub hovered: bool,
    /// Last rendered panel height and a one-frame request from the Size menu. The dock owns the
    /// actual split, so the request is applied after the dock frame completes.
    pub panel_height: f32,
    pub panel_height_request: Option<f32>,
    note_label_cache: Vec<String>,
    note_label_style: Option<(soundcraft_midi::NoteLanguage, soundcraft_midi::AccidentalStyle)>,
    note_index: MidiNoteIndex,
    visible_note_scratch: Vec<usize>,
}

/// A small interval index over the sequence's existing start-sorted note vector. It skips whole
/// blocks whose latest note end is left of the viewport, while preserving original note indices
/// for selection and command parameters.
#[derive(Debug, Clone, Default)]
struct MidiNoteIndex {
    clip: Option<ClipId>,
    revision: u64,
    note_count: usize,
    block_max_end: Vec<i64>,
}

impl MidiNoteIndex {
    fn rebuild(&mut self, clip: ClipId, revision: u64, notes: &[soundcraft_midi::Note]) {
        self.clip = Some(clip);
        self.revision = revision;
        self.note_count = notes.len();
        self.block_max_end.clear();
        let blocks = notes.len().div_ceil(NOTE_INDEX_BLOCK);
        if self.block_max_end.try_reserve(blocks).is_err() {
            return;
        }
        for block in notes.chunks(NOTE_INDEX_BLOCK) {
            self.block_max_end.push(block.iter().map(soundcraft_midi::Note::end).max().unwrap_or(i64::MIN));
        }
    }

    fn collect_visible(&mut self, clip: ClipId, revision: u64, notes: &[soundcraft_midi::Note], start: i64, end: i64, output: &mut Vec<usize>) {
        if self.clip != Some(clip) || self.revision != revision || self.note_count != notes.len() {
            self.rebuild(clip, revision, notes);
        }
        output.clear();
        let upper = notes.partition_point(|note| note.start < end);
        if self.block_max_end.len() != notes.len().div_ceil(NOTE_INDEX_BLOCK) {
            output.extend((0..upper).filter(|&index| notes.get(index).is_some_and(|note| note.end() > start)));
            return;
        }
        for (block_index, &max_end) in self.block_max_end.iter().enumerate().take(upper.div_ceil(NOTE_INDEX_BLOCK)) {
            if max_end <= start {
                continue;
            }
            let first = block_index * NOTE_INDEX_BLOCK;
            let last = (first + NOTE_INDEX_BLOCK).min(upper);
            output.extend((first..last).filter(|&index| notes.get(index).is_some_and(|note| note.end() > start)));
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoteDrag {
    pub index: usize,
    pub resize: bool,
    pub start: egui::Pos2,
    pub dx_ticks: i64,
    pub dy_semi: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct NoteErase {
    pub clip: ClipId,
    pub last: egui::Pos2,
}

fn segment_intersects_rect(from: egui::Pos2, to: egui::Pos2, rect: Rect) -> bool {
    if rect.contains(from) || rect.contains(to) {
        return true;
    }
    let delta = to - from;
    let mut enter = 0.0_f32;
    let mut leave = 1.0_f32;
    for (p, q) in [(-delta.x, from.x - rect.min.x), (delta.x, rect.max.x - from.x), (-delta.y, from.y - rect.min.y), (delta.y, rect.max.y - from.y)] {
        if p.abs() <= f32::EPSILON {
            if q < 0.0 {
                return false;
            }
            continue;
        }
        let ratio = q / p;
        if p < 0.0 {
            enter = enter.max(ratio);
        } else {
            leave = leave.min(ratio);
        }
        if enter > leave {
            return false;
        }
    }
    true
}

fn remap_selection_after_delete(selected: &mut Vec<usize>, removed: &[usize]) {
    selected.retain(|index| removed.binary_search(index).is_err());
    for index in selected {
        *index = index.saturating_sub(removed.partition_point(|removed_index| removed_index < index));
    }
}

fn delete_notes(app: &mut SoundApp, clip: ClipId, mut indices: Vec<usize>, merged: bool) {
    indices.sort_unstable();
    indices.dedup();
    if indices.is_empty() {
        return;
    }
    let params = json!({"clip": clip.0, "indices": indices});
    let changed = if merged {
        match app.engine.execute_merged("midi.note_delete", &params, &format!("midi_erase:{}", clip.0)) {
            Ok(_) => true,
            Err(error) => {
                app.ui.status = error.to_string();
                false
            }
        }
    } else {
        app.run("midi.note_delete", params).is_ok()
    };
    if changed {
        remap_selection_after_delete(&mut app.midi.selected, &indices);
    }
}

/// The MIDI clip being edited: the first selected MIDI clip, else a MIDI clip under the insertion
/// point on a selected track.
pub fn target_clip(app: &SoundApp) -> Option<ClipId> {
    let s = app.engine.session();
    for id in &s.edit.selected_clips {
        if let Some((_, c)) = s.find_clip(*id)
            && matches!(c.content, ClipContent::Midi { .. })
        {
            return Some(*id);
        }
    }
    let at = s.edit.selection.start;
    s.tracks
        .iter()
        .filter(|t| s.edit.selected_tracks.contains(&t.id) && t.kind.is_midi())
        .flat_map(|t| t.clips().iter())
        .find(|c| c.range().contains(at) || c.range().overlaps(&s.edit.selection))
        .or_else(|| s.tracks.iter().filter(|t| s.edit.selected_tracks.contains(&t.id) && t.kind.is_midi()).flat_map(|t| t.clips().iter()).next())
        .map(|c| c.id)
}

fn bar_start_at_or_after(map: &soundcraft_time::TempoMap, tick: i64) -> i64 {
    let at = tick.max(0);
    let bar = map.bar_beat_at_tick(at).bar;
    let start = map.bar_start_tick(bar);
    if start < at { map.bar_start_tick(bar.saturating_add(1)) } else { start }
}

/// First bar-aligned position after the selected notes whose pasted span does not overlap any
/// existing note. Notes are sorted, but this bounded scan deliberately remains simple because it
/// runs only on Paste, never in the audio callback or a paint frame.
fn next_free_bar_for_notes(app: &SoundApp, clip_id: ClipId, selected: &[usize]) -> Option<i64> {
    let s = app.engine.session();
    let (_, clip) = s.find_clip(clip_id)?;
    let ClipContent::Midi { sequence } = &clip.content else { return None };
    let selected_end = selected.iter().filter_map(|&index| sequence.notes.get(index)).map(soundcraft_midi::Note::end).max()?;
    let clip_tick = s.tempo.samples_to_ticks(clip.start, s.sample_rate);
    let span = app.engine.clipboard.midi_length_ticks.max(1);
    let mut candidate = bar_start_at_or_after(&s.tempo, clip_tick.saturating_add(selected_end));

    for _ in 0..=sequence.notes.len() {
        let relative_start = candidate.saturating_sub(clip_tick).max(0);
        let relative_end = relative_start.saturating_add(span);
        let blocker_end =
            sequence.notes.iter().filter(|note| note.start < relative_end && note.end() > relative_start).map(soundcraft_midi::Note::end).max();
        let Some(blocker_end) = blocker_end else { return Some(relative_start) };
        let next = bar_start_at_or_after(&s.tempo, clip_tick.saturating_add(blocker_end));
        if next <= candidate {
            return None;
        }
        candidate = next;
    }
    None
}

/// Route standard clipboard shortcuts to the Piano Roll while it owns pointer context.
pub fn clipboard_shortcut(app: &mut SoundApp, key: egui::Key) {
    let Some(clip_id) = target_clip(app) else {
        app.ui.status = "Select a MIDI clip before using note clipboard commands".into();
        return;
    };
    let selected = app.midi.selected.clone();
    let result = match key {
        egui::Key::C => app.run("midi.notes_copy", json!({"clip": clip_id.0, "indices": selected})),
        egui::Key::X => app.run("midi.notes_cut", json!({"clip": clip_id.0, "indices": selected})),
        egui::Key::D => app.run("midi.notes_duplicate", json!({"clip": clip_id.0, "indices": selected})),
        egui::Key::V => {
            let at = next_free_bar_for_notes(app, clip_id, &selected).or_else(|| {
                let session = app.engine.session();
                session.find_clip(clip_id).map(|(_, clip)| {
                    if clip.range().contains(app.position()) {
                        let base = session.tempo.samples_to_ticks(clip.start, session.sample_rate);
                        session.tempo.samples_to_ticks(app.position(), session.sample_rate).saturating_sub(base).max(0)
                    } else {
                        app.midi.viewport.start_tick.round().clamp(0.0, i64::MAX as f64) as i64
                    }
                })
            });
            let Some(at) = at else {
                app.ui.status = "The MIDI clip is no longer available".into();
                return;
            };
            app.run("midi.notes_paste", json!({"clip": clip_id.0, "at_ticks": at}))
        }
        _ => return,
    };
    if result.is_ok() && matches!(key, egui::Key::X | egui::Key::D) {
        app.midi.selected.clear();
    }
}

fn is_black(p: i32) -> bool {
    matches!(p.rem_euclid(12), 1 | 3 | 6 | 8 | 10)
}

pub fn show(app: &mut SoundApp, ui: &mut Ui) {
    // Painter-only contents do not advance the layout. Claim the resized
    // space, including the empty state, so the dock keeps its chosen height.
    ui.take_available_space();
    let t = Tokens::current();
    let language = app.ui.language;
    let tr = |source| crate::i18n::text(language, source);
    let full = ui.max_rect();
    if ui.input(|input| input.pointer.any_pressed())
        && let Some(origin) = ui.input(|input| input.pointer.press_origin())
    {
        app.midi.keyboard_focus = full.contains(origin);
    }
    app.midi.panel_height = full.height();
    ui.painter().rect_filled(full, 0.0, t.panel_bg);
    let header = Rect::from_min_size(full.min, vec2(full.width(), 22.0));
    ui.painter().rect_filled(header, 0.0, t.panel_bg2);
    ui.painter().text(pos2(header.min.x + 8.0, header.center().y), Align2::LEFT_CENTER, tr("MIDI EDITOR"), bold(11.5), t.header_text);
    let target = target_clip(app);
    if app.midi.erase.is_some_and(|erase| Some(erase.clip) != target) {
        app.midi.erase = None;
        app.engine.end_merge();
    }
    let Some(cid) = target else {
        let track = {
            let s = app.engine.session();
            s.tracks.iter().find(|t| t.kind.is_midi() && s.edit.selected_tracks.contains(&t.id)).map(|t| (t.id, t.name.clone()))
        };
        let msg = match &track {
            Some((_, name)) => format!("Click to add a MIDI clip to {name} at the insertion point."),
            None => "Select a MIDI clip (or a MIDI track) to edit its notes.".into(),
        };
        ui.painter().text(full.center(), Align2::CENTER_CENTER, msg, regular(13.0), t.text_dim);
        let body = Rect::from_min_max(pos2(full.min.x, header.max.y), full.max);
        if let Some((id, _)) = track
            && ui.interact(body, ui.id().with("new_clip"), Sense::click()).clicked()
        {
            let sel = app.engine.session().edit.selection;
            if let Ok(v) = app.run("midi.new_clip", json!({"track": id.0, "start": sel.start, "end": sel.end})) {
                let _ = app.run("edit.select", json!({"clips": [v["clip"]]}));
            }
        }
        return;
    };
    let Some((track, clip)) = app.engine.session().find_clip(cid).map(|(t, c)| (t, c.clone())) else { return };
    let ClipContent::Midi { sequence } = &clip.content else { return };
    let track_color = app.engine.session().track(track).map_or([120, 120, 200], |t| t.color);
    ui.painter().text(
        pos2(header.min.x + 110.0, header.center().y),
        Align2::LEFT_CENTER,
        format!(
            "{} · {} notes · {} selected · Pencil: add · drag: move/select · right edge: resize (Alt: free) · right-drag: erase · Ctrl+C/V: copy/paste",
            clip.name,
            sequence.notes.len(),
            app.midi.selected.len()
        ),
        regular(11.0),
        t.text_dim,
    );
    let grid_ticks = match app.engine.session().edit.grid {
        soundcraft_time::GridValue::Note { value, dotted, triplet } => {
            let mut g = value.ticks();
            if dotted {
                g = g * 3 / 2;
            }
            if triplet {
                g = g * 2 / 3;
            }
            g.max(1)
        }
        _ => TICKS_PER_QUARTER / 4,
    };
    // Header toolbar: whole-clip MIDI operations.
    let bar =
        Rect::from_min_max(pos2((header.max.x - 490.0).max(header.min.x + 100.0), header.min.y + 1.0), pos2(header.max.x - 6.0, header.max.y - 1.0));
    let mut tb = ui.new_child(egui::UiBuilder::new().max_rect(bar).layout(egui::Layout::right_to_left(egui::Align::Center)));
    tb.spacing_mut().item_spacing.x = 4.0;
    let mut op: Option<(&str, serde_json::Value)> = None;
    let mut zoom_action = 0i8;
    if tb.small_button("+").on_hover_text("Zoom in horizontally").clicked() {
        zoom_action = 1;
    }
    if tb.small_button(tr("Fit")).on_hover_text("Fit the entire MIDI clip").clicked() {
        zoom_action = 2;
    }
    if tb.small_button("−").on_hover_text("Zoom out horizontally").clicked() {
        zoom_action = -1;
    }
    tb.menu_button(tr("Size"), |ui| {
        ui.label(tr("Piano Roll height"));
        let maximum = (ui.ctx().content_rect().height() - 96.0).max(260.0);
        let mut height = app.midi.panel_height.clamp(260.0, maximum);
        if ui.add(egui::Slider::new(&mut height, 260.0..=maximum).suffix(" pt")).changed() {
            app.midi.panel_height_request = Some(height);
        }
        ui.separator();
        for (label, requested) in [("Compact", 260.0), ("Comfortable", 420.0), ("Large", 600.0), ("Maximum", maximum)] {
            if ui.button(tr(label)).clicked() {
                app.midi.panel_height_request = Some(requested.min(maximum));
                ui.close();
            }
        }
        ui.label(egui::RichText::new(tr("You can also drag the divider above the editor.")).small());
    })
    .response
    .on_hover_text(tr("Resize the Piano Roll or drag its upper divider"));
    tb.menu_button(tr("Labels"), |ui| {
        ui.label(tr("Keyboard labels"));
        ui.selectable_value(&mut app.ui.midi_note_labels, NoteLabelMode::Off, tr("Off"));
        ui.selectable_value(&mut app.ui.midi_note_labels, NoteLabelMode::Roots, tr("C / Do only"));
        ui.selectable_value(&mut app.ui.midi_note_labels, NoteLabelMode::All, tr("Every key"));
        ui.separator();
        ui.label(tr("Note names"));
        ui.selectable_value(&mut app.ui.midi_note_language, soundcraft_midi::NoteLanguage::English, "English (C, D, E)");
        ui.selectable_value(&mut app.ui.midi_note_language, soundcraft_midi::NoteLanguage::Spanish, "Español (Do, Re, Mi)");
        ui.separator();
        ui.label(tr("Accidentals"));
        ui.selectable_value(&mut app.ui.midi_accidentals, soundcraft_midi::AccidentalStyle::Sharps, tr("Sharps (♯)"));
        ui.selectable_value(&mut app.ui.midi_accidentals, soundcraft_midi::AccidentalStyle::Flats, tr("Flats (♭)"));
        ui.selectable_value(&mut app.ui.midi_accidentals, soundcraft_midi::AccidentalStyle::Auto, tr("Auto (key-aware later)"));
    })
    .response
    .on_hover_text("Choose which piano keys are labelled and how note names are written");
    if tb.small_button(tr("Legato")).clicked() {
        op = Some(("event.change_duration", json!({"legato": 0})));
    }
    if tb.small_button("Vel −10").clicked() {
        op = Some(("event.change_velocity", json!({"add": -10})));
    }
    if tb.small_button("Vel +10").clicked() {
        op = Some(("event.change_velocity", json!({"add": 10})));
    }
    if tb.small_button(tr("Quantize")).on_hover_text("Quantize to the grid").clicked() {
        op = Some(("event.quantize", json!({"grid": grid_ticks, "strength": 100})));
    }
    if let Some((id, mut p)) = op {
        p["clips"] = json!([cid.0]);
        let _ = app.run(id, p);
    }
    let s = app.engine.session_arc();
    let sr = s.sample_rate;
    let base_tick = s.tempo.samples_to_ticks(clip.start, sr);
    let len_ticks = (s.tempo.samples_to_ticks(clip.end(), sr) - base_tick).max(1);
    let roll = Rect::from_min_max(pos2(full.min.x + KEY_W, header.max.y), pos2(full.max.x - 4.0, full.max.y - VEL_H - 4.0));
    let keys = Rect::from_min_max(pos2(full.min.x, roll.min.y), pos2(roll.min.x, roll.max.y));
    let vel = Rect::from_min_max(pos2(roll.min.x, roll.max.y + 4.0), pos2(roll.max.x, full.max.y - 2.0));
    app.midi.hovered = ui.rect_contains_pointer(roll) || ui.rect_contains_pointer(vel);
    // Vertical range: centre on the notes the first time.
    if app.midi.clip != Some(cid) {
        app.midi.selected.clear();
        app.midi.viewport.fit(len_ticks, roll.width());
        app.midi.viewport.row_height = ROW_H;
        let rows = (roll.height() / app.midi.viewport.row_height).floor() as i32;
        let hi = sequence.notes.iter().map(|n| i32::from(n.pitch)).max().unwrap_or(72);
        let lo = sequence.notes.iter().map(|n| i32::from(n.pitch)).min().unwrap_or(48);
        app.midi.scroll = (127 - (hi + lo) / 2 - rows / 2) as f32 * app.midi.viewport.row_height;
        app.midi.clip = Some(cid);
    }
    if zoom_action == 2 {
        app.midi.viewport.fit(len_ticks, roll.width());
    } else if zoom_action != 0 {
        let factor = if zoom_action > 0 { 1.5 } else { 1.0 / 1.5 };
        app.midi.viewport.zoom_horizontal(roll.center().x, roll.min.x, factor, len_ticks, roll.width());
    }
    if ui.rect_contains_pointer(roll) {
        let (delta, zoom, vertical_zoom, sideways, pointer) =
            ui.input(|i| (i.smooth_scroll_delta, i.zoom_delta(), i.modifiers.alt, i.modifiers.shift, i.pointer.hover_pos()));
        if zoom != 1.0 {
            app.midi.viewport.zoom_horizontal(pointer.map_or(roll.center().x, |p| p.x), roll.min.x, f64::from(zoom), len_ticks, roll.width());
        } else if vertical_zoom && delta.y != 0.0 {
            let anchor_y = pointer.map_or(roll.center().y, |p| p.y);
            let old_height = app.midi.viewport.row_height;
            let row_at_anchor = (anchor_y - roll.min.y + app.midi.scroll) / old_height;
            app.midi.viewport.row_height = (old_height * (delta.y * 0.005).exp()).clamp(MIN_ROW_H, MAX_ROW_H);
            app.midi.scroll = row_at_anchor * app.midi.viewport.row_height - (anchor_y - roll.min.y);
        } else if sideways || delta.x != 0.0 {
            let dx = if delta.x != 0.0 { delta.x } else { delta.y };
            app.midi.viewport.start_tick -= f64::from(dx) * app.midi.viewport.ticks_per_pixel;
        } else {
            app.midi.scroll -= delta.y;
        }
    }
    app.midi.viewport.clamp_horizontal(len_ticks, roll.width());
    let row_height = app.midi.viewport.row_height;
    let rows = (roll.height() / row_height).floor() as i32;
    app.midi.scroll = app.midi.scroll.clamp(0.0, (128.0 * row_height - roll.height()).max(0.0));
    let scroll = app.midi.scroll;
    let label_style = (app.ui.midi_note_language, app.ui.midi_accidentals);
    if app.midi.note_label_style != Some(label_style) || app.midi.note_label_cache.len() != 128 {
        app.midi.note_label_cache = (0..=127).map(|pitch| soundcraft_midi::note_name_with(pitch, label_style.0, label_style.1)).collect();
        app.midi.note_label_style = Some(label_style);
    }
    let viewport = app.midi.viewport.clone();
    let x_of = |tick: i64| viewport.x_of(tick, roll.min.x);
    let y_of = |pitch: i32| roll.min.y + (127 - pitch) as f32 * row_height - scroll;
    let pitch_at = |y: f32| 127 - ((y - roll.min.y + scroll) / row_height).floor() as i32;
    let painter = ui.painter().with_clip_rect(roll.union(keys));
    // Background rows and keys.
    let top = pitch_at(roll.min.y);
    for r in 0..=rows + 1 {
        let pitch = top - r;
        let y = y_of(pitch);
        let row = Rect::from_min_size(pos2(roll.min.x, y), vec2(roll.width(), row_height));
        painter.rect_filled(row, 0.0, if is_black(pitch) { Color32::from_rgb(30, 30, 32) } else { Color32::from_rgb(40, 40, 42) });
        let key = Rect::from_min_size(pos2(keys.min.x, y), vec2(KEY_W - 2.0, row_height - 1.0));
        painter.rect_filled(key, 1.0, if is_black(pitch) { Color32::from_rgb(20, 20, 20) } else { Color32::from_rgb(225, 225, 225) });
        let show_label = match app.ui.midi_note_labels {
            NoteLabelMode::Off => false,
            NoteLabelMode::Roots => pitch.rem_euclid(12) == 0,
            NoteLabelMode::All => true,
        };
        if show_label && (0..=127).contains(&pitch) {
            let black = is_black(pitch);
            painter.text(
                pos2(key.max.x - 3.0, key.center().y),
                Align2::RIGHT_CENTER,
                app.midi.note_label_cache.get(pitch as usize).map_or("?", String::as_str),
                regular(if app.ui.midi_note_labels == NoteLabelMode::All { (row_height - 1.5).clamp(7.0, 9.0) } else { 8.0 }),
                if black { Color32::from_rgb(235, 235, 235) } else { Color32::from_rgb(40, 40, 40) },
            );
        }
        if pitch.rem_euclid(12) == 0 && (0..=127).contains(&pitch) {
            painter
                .line_segment([pos2(roll.min.x, y + row_height), pos2(roll.max.x, y + row_height)], Stroke::new(1.0, Color32::from_rgb(64, 64, 68)));
        }
    }
    // Grid.
    let mut tick = ((viewport.start_tick.floor() as i64).max(0) / grid_ticks) * grid_ticks;
    let visible_start = viewport.start_tick.floor().max(0.0) as i64;
    let visible_end = (viewport.start_tick + f64::from(roll.width()) * viewport.ticks_per_pixel).ceil() as i64;
    while tick <= len_ticks.min(visible_end.saturating_add(grid_ticks)) {
        let bar = tick % (TICKS_PER_QUARTER * 4) == 0;
        let x = x_of(tick);
        if grid_ticks as f64 / viewport.ticks_per_pixel > 4.0 || bar {
            painter.line_segment(
                [pos2(x, roll.min.y), pos2(x, roll.max.y)],
                Stroke::new(1.0, if bar { Color32::from_rgb(80, 80, 86) } else { Color32::from_rgb(50, 50, 54) }),
            );
        }
        tick += grid_ticks;
    }
    // Notes.
    let base = egui::Color32::from_rgb(track_color[0], track_color[1], track_color[2]);
    let drag = app.midi.drag.clone();
    let mut visible_notes = std::mem::take(&mut app.midi.visible_note_scratch);
    app.midi.note_index.collect_visible(cid, app.engine.revision, &sequence.notes, visible_start, visible_end, &mut visible_notes);
    if drag.is_some() {
        visible_notes.extend(app.midi.selected.iter().copied());
        visible_notes.sort_unstable();
        visible_notes.dedup();
    }
    let lowest_visible_pitch = pitch_at(roll.max.y).saturating_sub(1);
    let highest_visible_pitch = pitch_at(roll.min.y).saturating_add(1);
    for &i in &visible_notes {
        let Some(n) = sequence.notes.get(i) else { continue };
        let (mut st, mut ln, mut p) = (n.start, n.length, i32::from(n.pitch));
        if let Some(d) = &drag
            && (d.index == i || (app.midi.selected.contains(&i) && app.midi.selected.contains(&d.index)))
        {
            if d.resize {
                ln = ln.saturating_add(d.dx_ticks).max(1);
            } else {
                st = (st + d.dx_ticks).max(0);
                p = (p + d.dy_semi as i32).clamp(0, 127);
            }
        }
        if p < lowest_visible_pitch || p > highest_visible_pitch {
            continue;
        }
        let r = Rect::from_min_max(pos2(x_of(st), y_of(p) + 1.0), pos2(x_of(st + ln).max(x_of(st) + 3.0), y_of(p) + row_height - 1.0));
        let sel = app.midi.selected.contains(&i);
        let k = 0.45 + 0.55 * f32::from(n.velocity) / 127.0;
        let fill = if sel {
            Color32::WHITE
        } else {
            Color32::from_rgb((f32::from(base.r()) * k) as u8, (f32::from(base.g()) * k) as u8, (f32::from(base.b()) * k) as u8)
        };
        painter.rect(r, 2.0, fill, Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
    }
    // Interaction.
    let resp = ui.interact(roll, ui.id().with(("roll", cid.0)), Sense::click_and_drag());
    let note_rect = |i: usize| -> Option<Rect> {
        let n = sequence.notes.get(i)?;
        Some(Rect::from_min_max(
            pos2(x_of(n.start), y_of(i32::from(n.pitch))),
            pos2(x_of(n.start + n.length).max(x_of(n.start) + 3.0), y_of(i32::from(n.pitch)) + row_height),
        ))
    };
    let hit = |pos: egui::Pos2| -> Option<(usize, bool)> {
        visible_notes.iter().rev().find_map(|&i| {
            let r = note_rect(i)?;
            let resize_handle = Rect::from_min_max(pos2(r.max.x - 7.0, r.min.y), pos2(r.max.x + 4.0, r.max.y));
            if resize_handle.contains(pos) { Some((i, true)) } else { r.contains(pos).then_some((i, false)) }
        })
    };
    let crossed = |from: egui::Pos2, to: egui::Pos2| -> Vec<usize> {
        visible_notes.iter().copied().filter(|&i| note_rect(i).is_some_and(|rect| segment_intersects_rect(from, to, rect))).collect()
    };
    let snap = |tk: f64| -> i64 { ((tk / grid_ticks as f64).round() as i64) * grid_ticks };
    if resp.hovered()
        && let Some(position) = resp.hover_pos()
        && hit(position).is_some_and(|(_, edge)| edge)
    {
        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
    }
    if resp.clicked_by(egui::PointerButton::Secondary)
        && let Some(position) = resp.interact_pointer_pos()
        && let Some((index, _)) = hit(position)
    {
        delete_notes(app, cid, vec![index], false);
    }
    let erase_started = resp.drag_started_by(egui::PointerButton::Secondary);
    if erase_started && let Some(position) = resp.interact_pointer_pos() {
        let origin = ui.input(|input| input.pointer.press_origin()).unwrap_or(position);
        app.midi.erase = Some(NoteErase { clip: cid, last: position });
        delete_notes(app, cid, crossed(origin, position), true);
        ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
    }
    if !erase_started
        && resp.dragged_by(egui::PointerButton::Secondary)
        && let Some(position) = resp.interact_pointer_pos()
        && let Some(from) = app.midi.erase.as_mut().filter(|erase| erase.clip == cid).map(|erase| {
            let from = erase.last;
            erase.last = position;
            from
        })
    {
        delete_notes(app, cid, crossed(from, position), true);
        ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
    }
    if resp.drag_stopped_by(egui::PointerButton::Secondary) {
        app.midi.erase = None;
        app.engine.end_merge();
    }
    if app.midi.erase.is_some() && !ui.input(|input| input.pointer.button_down(egui::PointerButton::Secondary)) {
        app.midi.erase = None;
        app.engine.end_merge();
    }
    if resp.drag_started_by(egui::PointerButton::Middle)
        && let Some(p) = resp.interact_pointer_pos()
    {
        app.midi.pan = Some((p, app.midi.viewport.start_tick, app.midi.scroll));
        app.midi.band = None;
        app.midi.drag = None;
    }
    if resp.dragged_by(egui::PointerButton::Middle)
        && let (Some((origin, start_tick, start_scroll)), Some(p)) = (app.midi.pan, resp.interact_pointer_pos())
    {
        app.midi.viewport.start_tick = start_tick - f64::from(p.x - origin.x) * app.midi.viewport.ticks_per_pixel;
        app.midi.scroll = start_scroll - (p.y - origin.y);
        app.midi.viewport.clamp_horizontal(len_ticks, roll.width());
        app.midi.scroll = app.midi.scroll.clamp(0.0, (128.0 * row_height - roll.height()).max(0.0));
    }
    if resp.drag_stopped_by(egui::PointerButton::Middle) {
        app.midi.pan = None;
    }
    if resp.drag_started_by(egui::PointerButton::Primary)
        && let Some(p) = resp.interact_pointer_pos()
    {
        let origin = ui.input(|input| input.pointer.press_origin()).unwrap_or(p);
        if let Some((i, edge)) = hit(origin) {
            if !app.midi.selected.contains(&i) {
                app.midi.selected = vec![i];
            }
            app.midi.drag = Some(NoteDrag { index: i, resize: edge, start: origin, dx_ticks: 0, dy_semi: 0 });
        } else {
            app.midi.band = Some(origin);
        }
    }
    // Rubber-band selection on empty space.
    if let (Some(a), Some(b)) = (app.midi.band, ui.ctx().pointer_latest_pos()) {
        let band = Rect::from_two_pos(a, b);
        ui.painter().rect(
            band,
            0.0,
            Color32::from_rgba_unmultiplied(120, 170, 255, 40),
            Stroke::new(1.0, Color32::from_rgb(120, 170, 255)),
            StrokeKind::Inside,
        );
        if resp.drag_stopped_by(egui::PointerButton::Primary) {
            app.midi.selected = visible_notes
                .iter()
                .filter_map(|&i| sequence.notes.get(i).map(|note| (i, note)))
                .filter(|(_, n)| {
                    let r = Rect::from_min_max(
                        pos2(x_of(n.start), y_of(i32::from(n.pitch))),
                        pos2(x_of(n.start + n.length), y_of(i32::from(n.pitch)) + row_height),
                    );
                    r.intersects(band)
                })
                .map(|(i, _)| i)
                .collect();
            app.midi.band = None;
        }
    }
    if resp.dragged_by(egui::PointerButton::Primary)
        && let (Some(d), Some(p)) = (&mut app.midi.drag, resp.interact_pointer_pos())
    {
        let raw_ticks = (f64::from(p.x - d.start.x) * viewport.ticks_per_pixel).round() as i64;
        d.dx_ticks = if d.resize {
            if ui.input(|input| input.modifiers.alt) {
                raw_ticks
            } else {
                sequence.notes.get(d.index).map_or(0, |note| snap(note.end().saturating_add(raw_ticks) as f64).saturating_sub(note.end()))
            }
        } else {
            snap(raw_ticks as f64)
        };
        d.dy_semi = -((p.y - d.start.y) / row_height).round() as i64;
    }
    if resp.drag_stopped_by(egui::PointerButton::Primary)
        && let Some(d) = app.midi.drag.take()
    {
        if d.resize {
            if d.dx_ticks != 0 {
                let indices = if app.midi.selected.contains(&d.index) { app.midi.selected.clone() } else { vec![d.index] };
                let _ = app.run("midi.notes_resize", json!({"clip": cid.0, "indices": indices, "ticks": d.dx_ticks}));
            }
        } else if d.dx_ticks != 0 || d.dy_semi != 0 {
            let _ = app.run("midi.notes_move", json!({"clip": cid.0, "indices": app.midi.selected, "ticks": d.dx_ticks, "semitones": d.dy_semi}));
            app.midi.selected.clear();
        }
    }
    if resp.clicked_by(egui::PointerButton::Primary)
        && let Some(p) = resp.interact_pointer_pos()
    {
        match hit(p) {
            Some((i, _)) => {
                if ui.input(|x| x.modifiers.shift) {
                    if !app.midi.selected.contains(&i) {
                        app.midi.selected.push(i);
                    }
                } else {
                    app.midi.selected = vec![i];
                }
                // Audition on the instrument would go here (needs a live MIDI input path).
            }
            None if s.edit.tool == Tool::Pencil => {
                let pitch = pitch_at(p.y).clamp(0, 127);
                let start = (viewport.tick_at(p.x, roll.min.x) as i64 / grid_ticks) * grid_ticks;
                let _ = app.run(
                    "midi.note_add",
                    json!({"clip": cid.0, "pitch": pitch, "start_ticks": start.max(0), "length_ticks": grid_ticks, "velocity": 100}),
                );
                app.midi.selected.clear();
            }
            None => app.midi.selected.clear(),
        }
    }
    // Keyboard: Cmd+A select all; arrows transpose (Shift = octave) and move by the grid.
    if ui.rect_contains_pointer(roll) && !ui.ctx().egui_wants_keyboard_input() {
        let (all, up, down, left, right, shift) = ui.input(|i| {
            let arrows = !i.modifiers.alt && !i.modifiers.command && !i.modifiers.ctrl;
            (
                i.modifiers.command && i.key_pressed(egui::Key::A),
                arrows && i.key_pressed(egui::Key::ArrowUp),
                arrows && i.key_pressed(egui::Key::ArrowDown),
                arrows && !i.modifiers.shift && i.key_pressed(egui::Key::ArrowLeft),
                arrows && !i.modifiers.shift && i.key_pressed(egui::Key::ArrowRight),
                i.modifiers.shift,
            )
        });
        if all {
            app.midi.selected = (0..sequence.notes.len()).collect();
        }
        let semi = if up {
            1
        } else if down {
            -1
        } else {
            0
        } * if shift { 12 } else { 1 };
        let ticks = if right {
            grid_ticks
        } else if left {
            -grid_ticks
        } else {
            0
        };
        if (semi != 0 || ticks != 0) && !app.midi.selected.is_empty() {
            let _ = app.run("midi.notes_move", json!({"clip": cid.0, "indices": app.midi.selected, "ticks": ticks, "semitones": semi}));
        }
    }
    if !app.midi.selected.is_empty()
        && !ui.ctx().egui_wants_keyboard_input()
        && ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace))
    {
        let _ = app.run("midi.note_delete", json!({"clip": cid.0, "indices": app.midi.selected}));
        app.midi.selected.clear();
    }
    // Velocity lane.
    ui.painter().rect_filled(vel, 0.0, Color32::from_rgb(28, 28, 30));
    ui.painter().text(pos2(full.min.x + 6.0, vel.center().y), Align2::LEFT_CENTER, "Velocity", regular(10.0), t.text_dim);
    let vresp = ui.interact(vel, ui.id().with(("vel", cid.0)), Sense::click_and_drag());
    for &i in &visible_notes {
        let Some(n) = sequence.notes.get(i) else { continue };
        let x = x_of(n.start);
        let h = (vel.height() - 4.0) * f32::from(n.velocity) / 127.0;
        let col = if app.midi.selected.contains(&i) { Color32::WHITE } else { base };
        ui.painter().line_segment([pos2(x + 1.0, vel.max.y - 2.0), pos2(x + 1.0, vel.max.y - 2.0 - h)], Stroke::new(3.0, col));
        ui.painter().circle_filled(pos2(x + 1.0, vel.max.y - 2.0 - h), 2.5, col);
    }
    if (vresp.clicked() || vresp.dragged())
        && let Some(p) = vresp.interact_pointer_pos()
        && let Some((i, n)) = visible_notes
            .iter()
            .filter_map(|&i| sequence.notes.get(i).map(|note| (i, note)))
            .min_by_key(|(_, n)| ((x_of(n.start) - p.x).abs() * 10.0) as i64)
        && (x_of(n.start) - p.x).abs() < 8.0
    {
        let v = (((vel.max.y - 2.0 - p.y) / (vel.height() - 4.0)) * 127.0).round().clamp(1.0, 127.0) as i64;
        let _ = app.engine.execute_merged("midi.note_edit", &json!({"clip": cid.0, "index": i, "velocity": v}), &format!("vel:{}", cid.0));
    }
    app.midi.visible_note_scratch = visible_notes;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Services;
    use soundcraft_model::{ChannelFormat, TrackKind};

    #[test]
    fn viewport_coordinate_conversion_round_trips() {
        let viewport = MidiViewport { start_tick: 1234.5, ticks_per_pixel: 2.5, row_height: 12.0 };
        for tick in [0, 1, 960, 9_600, 1_000_000] {
            let x = viewport.x_of(tick, 48.0);
            let restored = viewport.tick_at(x, 48.0);
            assert!((restored - tick as f64).abs() < 0.25, "tick {tick}: {restored}");
        }
    }

    #[test]
    fn erase_segment_hits_crossed_notes_without_deleting_nearby_notes() {
        let crossed = Rect::from_min_max(pos2(10.0, 10.0), pos2(30.0, 20.0));
        let nearby = Rect::from_min_max(pos2(10.0, 30.0), pos2(30.0, 40.0));
        assert!(segment_intersects_rect(pos2(0.0, 15.0), pos2(50.0, 15.0), crossed));
        assert!(!segment_intersects_rect(pos2(0.0, 15.0), pos2(50.0, 15.0), nearby));
        assert!(segment_intersects_rect(pos2(0.0, 0.0), pos2(50.0, 50.0), crossed));
    }

    #[test]
    fn note_selection_indices_follow_deletions() {
        let mut selected = vec![1, 3, 6];
        remap_selection_after_delete(&mut selected, &[0, 3, 5]);
        assert_eq!(selected, vec![0, 3]);
    }

    #[test]
    fn horizontal_zoom_keeps_cursor_tick_stable() {
        let mut viewport = MidiViewport { start_tick: 2_000.0, ticks_per_pixel: 4.0, row_height: ROW_H };
        let before = viewport.tick_at(420.0, 48.0);
        viewport.zoom_horizontal(420.0, 48.0, 2.0, 100_000, 1_200.0);
        let after = viewport.tick_at(420.0, 48.0);
        assert!((before - after).abs() < f64::EPSILON);
        assert_eq!(viewport.ticks_per_pixel, 2.0);
    }

    #[test]
    fn viewport_clamps_to_clip_bounds() {
        let mut viewport = MidiViewport { start_tick: -100.0, ticks_per_pixel: 0.01, row_height: ROW_H };
        viewport.clamp_horizontal(9_600, 1_000.0);
        assert_eq!(viewport.start_tick, 0.0);
        assert_eq!(viewport.ticks_per_pixel, MIN_TICKS_PER_PIXEL);
        viewport.start_tick = 100_000.0;
        viewport.clamp_horizontal(9_600, 1_000.0);
        assert_eq!(viewport.start_tick, 9_350.0);
    }

    #[test]
    fn note_index_returns_only_horizontal_overlaps_and_keeps_long_notes() {
        let mut notes: Vec<soundcraft_midi::Note> = (0..2_000)
            .map(|index| soundcraft_midi::Note { pitch: 60, velocity: 100, release_velocity: 64, channel: 0, start: index * 10, length: 5 })
            .collect();
        notes[0].length = 20_000;
        let mut index = MidiNoteIndex::default();
        let mut visible = Vec::new();
        index.collect_visible(ClipId(7), 1, &notes, 10_000, 10_021, &mut visible);

        assert!(visible.contains(&0), "a sustained note crossing the left edge must remain visible");
        assert_eq!(visible, vec![0, 1_000, 1_001, 1_002]);
        assert!(visible.iter().all(|&i| notes.get(i).is_some_and(|note| note.start < 10_021 && note.end() > 10_000)));
    }

    #[test]
    fn note_index_matches_linear_search_across_large_varied_sequences() {
        let mut seed = 0x5eed_u64;
        let mut notes: Vec<soundcraft_midi::Note> = (0..100_000)
            .map(|_| {
                seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
                let start = (seed % 2_000_000) as i64;
                seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
                let length = 1 + (seed % 20_000) as i64;
                soundcraft_midi::Note { pitch: (seed % 128) as u8, velocity: 100, release_velocity: 64, channel: 0, start, length }
            })
            .collect();
        notes.sort_by_key(|note| (note.start, note.pitch, note.channel, note.length, note.velocity));
        let mut index = MidiNoteIndex::default();
        let mut actual = Vec::new();

        for query in 0..64 {
            let start = query * 31_337;
            let end = start + 4_801;
            index.collect_visible(ClipId(9), 1, &notes, start, end, &mut actual);
            let expected: Vec<_> = notes.iter().enumerate().filter_map(|(i, note)| (note.start < end && note.end() > start).then_some(i)).collect();
            assert_eq!(actual, expected, "visible range {start}..{end}");
        }
    }

    #[test]
    fn piano_roll_clipboard_shortcuts_use_note_commands() {
        let mut engine = soundcraft_engine::Engine::default();
        let track = engine.session_mut().add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
        let clip = engine.execute("midi.new_clip", &json!({"track": track.0})).unwrap()["clip"].as_u64().unwrap();
        engine.execute("midi.note_add", &json!({"clip": clip, "pitch": 60, "start_ticks": 0})).unwrap();
        engine.session_mut().edit.selected_clips = vec![ClipId(clip)];
        let mut app = SoundApp::new(engine, None, Services::default());
        app.midi.selected = vec![0];

        clipboard_shortcut(&mut app, egui::Key::C);
        assert_eq!(app.engine.clipboard.midi_notes.len(), 1);
        clipboard_shortcut(&mut app, egui::Key::X);
        assert_eq!(app.engine.execute("midi.notes", &json!({"clip": clip})).unwrap()["notes"].as_array().map(Vec::len), Some(0));
        clipboard_shortcut(&mut app, egui::Key::V);
        assert_eq!(app.engine.execute("midi.notes", &json!({"clip": clip})).unwrap()["notes"].as_array().map(Vec::len), Some(1));
        app.midi.selected = vec![0];
        clipboard_shortcut(&mut app, egui::Key::D);
        assert_eq!(app.engine.execute("midi.notes", &json!({"clip": clip})).unwrap()["notes"].as_array().map(Vec::len), Some(2));
    }

    #[test]
    fn selected_notes_paste_as_a_group_into_each_next_free_bar() {
        let mut engine = soundcraft_engine::Engine::default();
        let track = engine.session_mut().add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
        let clip = engine.execute("midi.new_clip", &json!({"track": track.0})).unwrap()["clip"].as_u64().unwrap();
        for (pitch, start) in [(60, 0), (64, 480), (72, 3_840)] {
            engine.execute("midi.note_add", &json!({"clip": clip, "pitch": pitch, "start_ticks": start, "length_ticks": 480})).unwrap();
        }
        engine.session_mut().edit.selected_clips = vec![ClipId(clip)];
        let mut app = SoundApp::new(engine, None, Services::default());
        app.midi.selected = vec![0, 1];

        clipboard_shortcut(&mut app, egui::Key::C);
        clipboard_shortcut(&mut app, egui::Key::V);
        clipboard_shortcut(&mut app, egui::Key::V);

        let notes = app.engine.execute("midi.notes", &json!({"clip": clip})).unwrap();
        let starts = notes["notes"].as_array().unwrap().iter().filter_map(|note| note["start"].as_i64()).collect::<Vec<_>>();
        assert_eq!(starts, vec![0, 480, 3_840, 7_680, 8_160, 11_520, 12_000]);
        assert_eq!(app.midi.selected, vec![0, 1], "repeated paste keeps the source group selected and advances to another free bar");
    }
}
