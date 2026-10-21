//! MIDI note editing (MIDI Editor, pencil on MIDI tracks).

use super::*;
use crate::Clipboard;
use crate::cmd;
use serde_json::json;
use soundcraft_midi::{Note, Sequence};
use soundcraft_model::{Clip, ClipContent, ClipId};
use soundcraft_time::TICKS_PER_QUARTER;

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "midi.notes", "List MIDI Notes", [], None, "{clip}", always, notes),
        cmd!(
            "midi.new_clip",
            "New MIDI Clip",
            ["Clip"],
            None,
            "{track, start?, end?|length?} (default: the edit selection, or 1 bar)",
            has_tracks,
            new_clip
        ),
        cmd!("midi.note_add", "Add Note", [], None, "{clip, pitch, start_ticks, length_ticks?: 480, velocity?: 100, channel?: 0}", always, note_add),
        cmd!("midi.note_edit", "Edit Note", [], None, "{clip, index, pitch?, start_ticks?, length_ticks?, velocity?}", always, note_edit),
        cmd!("midi.note_delete", "Delete Notes", [], None, "{clip, indices: [n]}", always, note_delete),
        cmd!("midi.notes_move", "Move Notes", [], None, "{clip, indices: [n], ticks?: 0, semitones?: 0}", always, notes_move),
        cmd!("midi.notes_resize", "Resize Notes", [], None, "{clip, indices: [n], ticks}", always, notes_resize),
        cmd!(noundo "midi.notes_copy", "Copy MIDI Notes", [], None, "{clip, indices: [n]}", always, notes_copy),
        cmd!("midi.notes_cut", "Cut MIDI Notes", [], None, "{clip, indices: [n]}", always, notes_cut),
        cmd!(
            "midi.notes_paste",
            "Paste MIDI Notes",
            [],
            None,
            "{clip, at_ticks?: 0, semitones?: 0} — extends the MIDI clip when the pasted notes need more room",
            has_midi_clipboard,
            notes_paste
        ),
        cmd!("midi.notes_duplicate", "Duplicate MIDI Notes", [], None, "{clip, indices: [n], ticks?: selection span}", always, notes_duplicate),
    ]
}

fn seq_mut<'a>(e: &'a mut Engine, id: ClipId, cmd: &str) -> Result<&'a mut Sequence> {
    match e.session_mut().find_clip_mut(id).map(|c| &mut c.content) {
        Some(ClipContent::Midi { sequence }) => Ok(sequence),
        Some(_) => Err(bad(cmd, "not a MIDI clip")),
        None => Err(bad(cmd, "no such clip")),
    }
}

fn notes(e: &mut Engine, p: &Value) -> Result<Value> {
    let id = clip_id(p, "midi.notes")?;
    match e.session().find_clip(id).map(|(_, c)| &c.content) {
        Some(ClipContent::Midi { sequence }) => Ok(json!({"notes": sequence.notes})),
        _ => Err(bad("midi.notes", "not a MIDI clip")),
    }
}

fn new_clip(e: &mut Engine, p: &Value) -> Result<Value> {
    let t = tracks_required(e, "midi.new_clip", p)?.first().copied().ok_or_else(|| bad("midi.new_clip", "no track"))?;
    if !e.session().track(t).is_some_and(|x| x.kind.is_midi()) {
        return Err(bad("midi.new_clip", "not a MIDI or instrument track"));
    }
    let mut r = range_param(e, "midi.new_clip", p)?;
    if r.is_empty() {
        let s = e.session();
        let bar = soundcraft_time::GridValue::Note { value: soundcraft_time::NoteValue::Bar, dotted: false, triplet: false };
        let start = bar.line_at_or_before(r.start, s.sample_rate, &s.tempo, s.frame_rate);
        r = soundcraft_time::Range::new(start, bar.next_line(start, s.sample_rate, &s.tempo, s.frame_rate));
    }
    let s = e.session_mut();
    let id = s.new_clip_id();
    let name = s.track(t).map(|x| x.name.clone()).unwrap_or_default();
    crate::edit::place_clip(s, t, Clip::midi(id, name, r.start, r.len(), Sequence::default()));
    Ok(json!({"clip": id, "start": r.start, "end": r.end}))
}

fn note_add(e: &mut Engine, p: &Value) -> Result<Value> {
    let id = clip_id(p, "midi.note_add")?;
    let n = Note {
        pitch: i64_or(p, "pitch", 60).clamp(0, 127) as u8,
        velocity: i64_or(p, "velocity", 100).clamp(1, 127) as u8,
        release_velocity: 64,
        channel: i64_or(p, "channel", 0).clamp(0, 15) as u8,
        start: i64_or(p, "start_ticks", 0).max(0),
        length: i64_or(p, "length_ticks", TICKS_PER_QUARTER / 2).max(1),
    };
    let seq = seq_mut(e, id, "midi.note_add")?;
    seq.notes.push(n);
    seq.sort();
    let index = seq.notes.iter().position(|x| *x == n).unwrap_or(0);
    Ok(json!({"index": index, "notes": seq.notes.len()}))
}

fn note_edit(e: &mut Engine, p: &Value) -> Result<Value> {
    let id = clip_id(p, "midi.note_edit")?;
    let idx = p.get("index").and_then(Value::as_u64).ok_or_else(|| bad("midi.note_edit", "`index` required"))? as usize;
    let seq = seq_mut(e, id, "midi.note_edit")?;
    let n = seq.notes.get_mut(idx).ok_or_else(|| bad("midi.note_edit", "index out of range"))?;
    if let Some(v) = p.get("pitch").and_then(Value::as_i64) {
        n.pitch = v.clamp(0, 127) as u8;
    }
    if let Some(v) = p.get("start_ticks").and_then(Value::as_i64) {
        n.start = v.max(0);
    }
    if let Some(v) = p.get("length_ticks").and_then(Value::as_i64) {
        n.length = v.max(1);
    }
    if let Some(v) = p.get("velocity").and_then(Value::as_i64) {
        n.velocity = v.clamp(1, 127) as u8;
    }
    let edited = *n;
    seq.sort();
    Ok(json!({"note": edited, "index": seq.notes.iter().position(|x| *x == edited)}))
}

fn indices(p: &Value, cmd: &str) -> Result<Vec<usize>> {
    let values = p.get("indices").and_then(Value::as_array).ok_or_else(|| bad(cmd, "`indices` array required"))?;
    if values.len() > soundcraft_midi::MAX_EVENTS {
        return Err(bad(cmd, "too many note indices"));
    }
    let mut out = Vec::new();
    out.try_reserve_exact(values.len()).map_err(|_| bad(cmd, "not enough memory for note indices"))?;
    for value in values {
        let raw = value.as_u64().ok_or_else(|| bad(cmd, "every note index must be a non-negative integer"))?;
        let index = usize::try_from(raw).map_err(|_| bad(cmd, "note index is too large for this platform"))?;
        out.push(index);
    }
    out.sort_unstable();
    out.dedup();
    Ok(out)
}

fn note_delete(e: &mut Engine, p: &Value) -> Result<Value> {
    let id = clip_id(p, "midi.note_delete")?;
    let idx = indices(p, "midi.note_delete")?;
    let seq = seq_mut(e, id, "midi.note_delete")?;
    let before = seq.notes.len();
    let mut i = 0;
    seq.notes.retain(|_| {
        let keep = idx.binary_search(&i).is_err();
        i += 1;
        keep
    });
    Ok(json!({"deleted": before - seq.notes.len()}))
}

fn notes_move(e: &mut Engine, p: &Value) -> Result<Value> {
    let id = clip_id(p, "midi.notes_move")?;
    let idx = indices(p, "midi.notes_move")?;
    let dt = i64_or(p, "ticks", 0);
    let ds = i64_or(p, "semitones", 0);
    let seq = seq_mut(e, id, "midi.notes_move")?;
    for (i, n) in seq.notes.iter_mut().enumerate() {
        if idx.binary_search(&i).is_ok() {
            n.start = (n.start + dt).max(0);
            n.pitch = (i64::from(n.pitch) + ds).clamp(0, 127) as u8;
        }
    }
    seq.sort();
    Ok(json!({"moved": idx.len()}))
}

fn notes_resize(e: &mut Engine, p: &Value) -> Result<Value> {
    let id = clip_id(p, "midi.notes_resize")?;
    let idx = indices(p, "midi.notes_resize")?;
    if idx.is_empty() {
        return Err(bad("midi.notes_resize", "select at least one MIDI note"));
    }
    let delta = p.get("ticks").and_then(Value::as_i64).ok_or_else(|| bad("midi.notes_resize", "`ticks` required"))?;
    let seq = seq_mut(e, id, "midi.notes_resize")?;
    if idx.iter().any(|&index| index >= seq.notes.len()) {
        return Err(bad("midi.notes_resize", "note index out of range"));
    }
    for &index in &idx {
        if let Some(note) = seq.notes.get_mut(index) {
            note.length = note.length.saturating_add(delta).max(1);
        }
    }
    Ok(json!({"resized": idx.len(), "ticks": delta}))
}

fn has_midi_clipboard(e: &Engine) -> std::result::Result<(), String> {
    if e.clipboard.midi_notes.is_empty() { Err("no MIDI notes have been copied".into()) } else { Ok(()) }
}

fn copied_notes(e: &Engine, p: &Value, cmd: &str) -> Result<(Vec<Note>, i64, i64)> {
    let id = clip_id(p, cmd)?;
    let idx = indices(p, cmd)?;
    if idx.is_empty() {
        return Err(bad(cmd, "select at least one MIDI note"));
    }
    let sequence = match e.session().find_clip(id).map(|(_, clip)| &clip.content) {
        Some(ClipContent::Midi { sequence }) => sequence,
        Some(_) => return Err(bad(cmd, "not a MIDI clip")),
        None => return Err(bad(cmd, "no such clip")),
    };
    let mut notes = Vec::new();
    notes.try_reserve_exact(idx.len()).map_err(|_| bad(cmd, "not enough memory for copied notes"))?;
    for index in idx {
        notes.push(*sequence.notes.get(index).ok_or_else(|| bad(cmd, "note index out of range"))?);
    }
    let anchor = notes.iter().map(|note| note.start).min().unwrap_or(0);
    let end = notes.iter().map(|note| note.start.saturating_add(note.length)).max().unwrap_or(anchor);
    for note in &mut notes {
        note.start = note.start.saturating_sub(anchor);
    }
    Ok((notes, end.saturating_sub(anchor).max(1), anchor))
}

fn notes_copy(e: &mut Engine, p: &Value) -> Result<Value> {
    let (notes, length, _) = copied_notes(e, p, "midi.notes_copy")?;
    let copied = notes.len();
    e.clipboard = Clipboard { midi_notes: notes, midi_length_ticks: length, ..Clipboard::default() };
    Ok(json!({"copied": copied, "length_ticks": length}))
}

fn notes_cut(e: &mut Engine, p: &Value) -> Result<Value> {
    let id = clip_id(p, "midi.notes_cut")?;
    let idx = indices(p, "midi.notes_cut")?;
    let (notes, length, _) = copied_notes(e, p, "midi.notes_cut")?;
    let cut = notes.len();
    let sequence = seq_mut(e, id, "midi.notes_cut")?;
    let mut i = 0usize;
    sequence.notes.retain(|_| {
        let keep = idx.binary_search(&i).is_err();
        i = i.saturating_add(1);
        keep
    });
    e.clipboard = Clipboard { midi_notes: notes, midi_length_ticks: length, ..Clipboard::default() };
    Ok(json!({"cut": cut, "length_ticks": length}))
}

fn notes_paste(e: &mut Engine, p: &Value) -> Result<Value> {
    let id = clip_id(p, "midi.notes_paste")?;
    let copied = e.clipboard.midi_notes.clone();
    if copied.is_empty() {
        return Err(bad("midi.notes_paste", "no MIDI notes have been copied"));
    }
    let pasted_count = copied.len();
    let at = i64_or(p, "at_ticks", 0).max(0);
    let semitones = i64_or(p, "semitones", 0);
    let copied_end = copied.iter().map(Note::end).max().unwrap_or(1);
    let required_length = {
        let s = e.session();
        let (_, clip) = s.find_clip(id).ok_or_else(|| bad("midi.notes_paste", "no such clip"))?;
        let ClipContent::Midi { sequence } = &clip.content else { return Err(bad("midi.notes_paste", "not a MIDI clip")) };
        if sequence.notes.len().saturating_add(copied.len()) > soundcraft_midi::MAX_EVENTS {
            return Err(bad("midi.notes_paste", "the MIDI event limit would be exceeded"));
        }
        let clip_tick = s.tempo.samples_to_ticks(clip.start, s.sample_rate);
        let required_tick = clip_tick.saturating_add(at).saturating_add(copied_end);
        s.tempo.tick_to_samples(required_tick, s.sample_rate).saturating_sub(clip.start).max(1)
    };
    let clip = e.session_mut().find_clip_mut(id).ok_or_else(|| bad("midi.notes_paste", "no such clip"))?;
    let ClipContent::Midi { sequence } = &mut clip.content else { return Err(bad("midi.notes_paste", "not a MIDI clip")) };
    sequence.notes.try_reserve(copied.len()).map_err(|_| bad("midi.notes_paste", "not enough memory for pasted notes"))?;
    for mut note in copied {
        note.start = at.saturating_add(note.start);
        note.pitch = (i64::from(note.pitch) + semitones).clamp(0, 127) as u8;
        sequence.notes.push(note);
    }
    sequence.sort();
    clip.length = clip.length.max(required_length);
    Ok(json!({"pasted": pasted_count, "at_ticks": at, "clip_length": clip.length}))
}

fn notes_duplicate(e: &mut Engine, p: &Value) -> Result<Value> {
    let id = clip_id(p, "midi.notes_duplicate")?;
    let (mut notes, span, anchor) = copied_notes(e, p, "midi.notes_duplicate")?;
    let offset = p.get("ticks").and_then(Value::as_i64).unwrap_or(span).max(1);
    let at = anchor.saturating_add(offset);
    let sequence = seq_mut(e, id, "midi.notes_duplicate")?;
    if sequence.notes.len().saturating_add(notes.len()) > soundcraft_midi::MAX_EVENTS {
        return Err(bad("midi.notes_duplicate", "the MIDI event limit would be exceeded"));
    }
    sequence.notes.try_reserve(notes.len()).map_err(|_| bad("midi.notes_duplicate", "not enough memory for duplicated notes"))?;
    for note in &mut notes {
        note.start = note.start.saturating_add(at);
    }
    let count = notes.len();
    sequence.notes.extend(notes);
    sequence.sort();
    Ok(json!({"duplicated": count, "ticks": offset}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use soundcraft_model::{ChannelFormat, TrackKind};

    #[test]
    fn create_clip_add_edit_move_delete() {
        let mut e = Engine::default();
        let t = e.session_mut().add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
        let r = e.execute("midi.new_clip", &json!({"track": t.0})).unwrap();
        let clip = r["clip"].as_u64().unwrap();
        e.execute("midi.note_add", &json!({"clip": clip, "pitch": 60, "start_ticks": 0})).unwrap();
        e.execute("midi.note_add", &json!({"clip": clip, "pitch": 64, "start_ticks": 480})).unwrap();
        e.execute("midi.note_edit", &json!({"clip": clip, "index": 0, "velocity": 30})).unwrap();
        e.execute("midi.notes_move", &json!({"clip": clip, "indices": [1], "semitones": 3})).unwrap();
        let n = e.execute("midi.notes", &json!({"clip": clip})).unwrap();
        assert_eq!(n["notes"][0]["velocity"], 30);
        assert_eq!(n["notes"][1]["pitch"], 67);
        e.execute("midi.note_delete", &json!({"clip": clip, "indices": [0]})).unwrap();
        assert_eq!(e.execute("midi.notes", &json!({"clip": clip})).unwrap()["notes"].as_array().map(Vec::len), Some(1));
        assert!(e.execute("midi.note_edit", &json!({"clip": clip, "index": 99})).is_err());
    }

    #[test]
    fn rejects_audio_tracks() {
        let mut e = Engine::default();
        let t = e.session_mut().add_track(TrackKind::Audio, ChannelFormat::Mono, None);
        assert!(e.execute("midi.new_clip", &json!({"track": t.0})).is_err());
    }

    #[test]
    fn note_clipboard_commands_are_relative_atomic_and_undoable() {
        let mut e = Engine::default();
        let track = e.session_mut().add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
        let clip = e.execute("midi.new_clip", &json!({"track": track.0})).unwrap()["clip"].as_u64().unwrap();
        e.execute("midi.note_add", &json!({"clip": clip, "pitch": 60, "start_ticks": 480, "length_ticks": 480})).unwrap();
        e.execute("midi.note_add", &json!({"clip": clip, "pitch": 64, "start_ticks": 960, "length_ticks": 480})).unwrap();

        let copied = e.execute("midi.notes_copy", &json!({"clip": clip, "indices": [0, 1]})).unwrap();
        assert_eq!(copied, json!({"copied": 2, "length_ticks": 960}));
        assert_eq!(e.clipboard.midi_notes.iter().map(|note| note.start).collect::<Vec<_>>(), vec![0, 480]);

        e.execute("midi.notes_paste", &json!({"clip": clip, "at_ticks": 1920, "semitones": -1})).unwrap();
        let pasted = e.execute("midi.notes", &json!({"clip": clip})).unwrap();
        assert_eq!(pasted["notes"].as_array().map(Vec::len), Some(4));
        assert_eq!(pasted["notes"][2]["start"], 1920);
        assert_eq!(pasted["notes"][2]["pitch"], 59);
        assert!(e.undo());
        assert_eq!(e.execute("midi.notes", &json!({"clip": clip})).unwrap()["notes"].as_array().map(Vec::len), Some(2));

        e.execute("midi.notes_duplicate", &json!({"clip": clip, "indices": [0, 1], "ticks": 960})).unwrap();
        let duplicated = e.execute("midi.notes", &json!({"clip": clip})).unwrap();
        assert_eq!(duplicated["notes"].as_array().map(Vec::len), Some(4));
        assert_eq!(duplicated["notes"][2]["start"], 1440);
        assert_eq!(duplicated["notes"][3]["start"], 1920);
        assert!(e.undo());

        e.execute("midi.notes_cut", &json!({"clip": clip, "indices": [0, 0, 1]})).unwrap();
        assert_eq!(e.execute("midi.notes", &json!({"clip": clip})).unwrap()["notes"].as_array().map(Vec::len), Some(0));
        assert!(e.undo());
        assert_eq!(e.execute("midi.notes", &json!({"clip": clip})).unwrap()["notes"].as_array().map(Vec::len), Some(2));
    }

    #[test]
    fn note_paste_extends_the_clip_and_undo_restores_its_length() {
        let mut e = Engine::default();
        let track = e.session_mut().add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
        let clip = e.execute("midi.new_clip", &json!({"track": track.0})).unwrap()["clip"].as_u64().unwrap();
        e.execute("midi.note_add", &json!({"clip": clip, "pitch": 60, "start_ticks": 0, "length_ticks": 480})).unwrap();
        e.execute("midi.notes_copy", &json!({"clip": clip, "indices": [0]})).unwrap();
        let original = e.session().find_clip(ClipId(clip)).map(|(_, clip)| clip.length).unwrap();

        e.execute("midi.notes_paste", &json!({"clip": clip, "at_ticks": 3_840})).unwrap();
        let extended = e.session().find_clip(ClipId(clip)).map(|(_, clip)| clip.length).unwrap();
        assert!(extended > original);
        assert!(e.undo());
        assert_eq!(e.session().find_clip(ClipId(clip)).map(|(_, clip)| clip.length), Some(original));
    }

    #[test]
    fn note_clipboard_rejects_empty_and_invalid_selections_and_clamps_pitch() {
        let mut e = Engine::default();
        let track = e.session_mut().add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
        let clip = e.execute("midi.new_clip", &json!({"track": track.0})).unwrap()["clip"].as_u64().unwrap();
        assert!(e.execute("midi.notes_paste", &json!({"clip": clip})).is_err());
        assert!(e.execute("midi.notes_copy", &json!({"clip": clip, "indices": []})).is_err());
        assert!(e.execute("midi.notes_copy", &json!({"clip": clip, "indices": [99]})).is_err());

        e.execute("midi.note_add", &json!({"clip": clip, "pitch": 127, "start_ticks": 0})).unwrap();
        e.execute("midi.notes_copy", &json!({"clip": clip, "indices": [0]})).unwrap();
        e.execute("midi.notes_paste", &json!({"clip": clip, "at_ticks": 480, "semitones": 1000})).unwrap();
        let notes = e.execute("midi.notes", &json!({"clip": clip})).unwrap();
        assert_eq!(notes["notes"][1]["pitch"], 127);
    }

    #[test]
    fn selected_notes_resize_together_and_undo_atomically() {
        let mut e = Engine::default();
        let track = e.session_mut().add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
        let clip = e.execute("midi.new_clip", &json!({"track": track.0})).unwrap()["clip"].as_u64().unwrap();
        e.execute("midi.note_add", &json!({"clip": clip, "pitch": 60, "start_ticks": 0, "length_ticks": 480})).unwrap();
        e.execute("midi.note_add", &json!({"clip": clip, "pitch": 64, "start_ticks": 960, "length_ticks": 960})).unwrap();

        let resized = e.execute("midi.notes_resize", &json!({"clip": clip, "indices": [0, 1], "ticks": -720})).unwrap();
        assert_eq!(resized, json!({"resized": 2, "ticks": -720}));
        let notes = e.execute("midi.notes", &json!({"clip": clip})).unwrap();
        assert_eq!(notes["notes"][0]["length"], 1);
        assert_eq!(notes["notes"][1]["length"], 240);

        assert!(e.undo());
        let restored = e.execute("midi.notes", &json!({"clip": clip})).unwrap();
        assert_eq!(restored["notes"][0]["length"], 480);
        assert_eq!(restored["notes"][1]["length"], 960);
    }
}
