use egui::os::OperatingSystem;
use egui::{Event, Id, Key, Modifiers, PointerButton, Pos2, RawInput, Rect, pos2, vec2};
use serde_json::json;
use soundcraft_ui_egui::{Services, SoundApp, midi_editor};

fn frame(ctx: &egui::Context, app: &mut SoundApp, events: Vec<Event>) {
    let raw = RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1600.0, 1000.0))), events, ..Default::default() };
    ctx.run_ui(raw, |ui| {
        app.logic(ui.ctx());
        app.ui(ui);
    })
    .textures_delta
    .clear();
}

fn panel(ctx: &egui::Context) -> Rect {
    ctx.data(|data| data.get_temp::<Rect>(Id::new("soundcraft-midi-body"))).unwrap()
}

fn button(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton { pos, button: PointerButton::Primary, pressed, modifiers: Default::default() }
}

fn secondary_button(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton { pos, button: PointerButton::Secondary, pressed, modifiers: Default::default() }
}

fn key(key: Key, pressed: bool, modifiers: Modifiers) -> Event {
    Event::Key { key, physical_key: None, pressed, repeat: false, modifiers }
}

fn check_resize(with_clip: bool) {
    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    app.ui.show_midi_editor = true;
    if with_clip {
        app.run("edit.select", json!({"tracks": ["Keys"]})).unwrap();
    }
    assert_eq!(midi_editor::target_clip(&app).is_some(), with_clip);
    let ctx = egui::Context::default();
    for _ in 0..6 {
        frame(&ctx, &mut app, vec![]);
    }
    let initial = panel(&ctx);
    assert!((initial.height() - 320.0).abs() < 1.0, "initial panel: {initial:?}");
    let notes = app.engine.session().clone();

    // Drag the shared boundary up and down, then allow several idle frames
    // to catch a panel that looks resized but snaps back after release.
    for height in [440.0, 280.0, 200.0] {
        let before = panel(&ctx);
        let start = pos2(before.center().x, before.top() - 26.0 - 2.0);
        let end = pos2(start.x, before.bottom() - height - 26.0 - 2.0);
        frame(&ctx, &mut app, vec![Event::PointerMoved(start)]);
        frame(&ctx, &mut app, vec![button(start, true)]);
        for step in 1..=6 {
            frame(&ctx, &mut app, vec![Event::PointerMoved(start.lerp(end, step as f32 / 6.0))]);
        }
        frame(&ctx, &mut app, vec![button(end, false)]);
        for _ in 0..4 {
            frame(&ctx, &mut app, vec![]);
        }
        let after = panel(&ctx);
        assert!((after.height() - height.max(260.0)).abs() < 1.0, "requested {height}, panel: {after:?}");
        assert!((after.bottom() - initial.bottom()).abs() < 1.0);
        assert!(app.edit_layout.timeline[3] <= after.top());
    }

    // Closing and reopening the dock keeps the chosen height.
    let resized = panel(&ctx);
    app.run("window.midi_editor", json!({})).unwrap();
    frame(&ctx, &mut app, vec![]);
    app.run("window.midi_editor", json!({})).unwrap();
    for _ in 0..4 {
        frame(&ctx, &mut app, vec![]);
    }
    assert_eq!(panel(&ctx), resized);
    let saved = serde_json::to_string(&app.ui).unwrap();
    app.ui = serde_json::from_str(&saved).unwrap();
    for _ in 0..4 {
        frame(&ctx, &mut app, vec![]);
    }
    assert_eq!(panel(&ctx), resized);
    assert_eq!(*app.engine.session(), notes);
}

#[test]
fn midi_panel_resizes_with_a_selected_clip() {
    check_resize(true);
}

#[test]
fn midi_panel_resizes_without_a_selected_clip() {
    check_resize(false);
}

#[test]
fn midi_size_request_resizes_the_dock_and_persists() {
    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    app.ui.show_midi_editor = true;
    app.run("edit.select", json!({"tracks": ["Keys"]})).unwrap();
    let ctx = egui::Context::default();
    for _ in 0..4 {
        frame(&ctx, &mut app, vec![]);
    }
    let initial = panel(&ctx);
    app.midi.panel_height_request = Some(600.0);
    for _ in 0..4 {
        frame(&ctx, &mut app, vec![]);
    }
    let resized = panel(&ctx);
    assert!(resized.height() > initial.height() + 100.0, "requested 600, panel: {resized:?}");
    assert!(resized.height() <= 600.0, "the canvas minimum may clamp the requested height: {resized:?}");
    let saved = serde_json::to_string(&app.ui).unwrap();
    app.ui = serde_json::from_str(&saved).unwrap();
    for _ in 0..4 {
        frame(&ctx, &mut app, vec![]);
    }
    let restored = panel(&ctx);
    assert_eq!(restored, resized, "restored panel: {restored:?}");
}

#[test]
fn layout_reset_cancels_a_held_midi_divider_without_editing_the_session() {
    check_held_replacement("reset");
}

#[test]
fn arrange_cancels_a_held_midi_divider_without_editing_the_session() {
    check_held_replacement("arrange");
}

#[test]
fn legacy_visibility_sync_cancels_a_held_midi_divider_without_editing_the_session() {
    check_held_replacement("visibility");
}

fn check_held_replacement(operation: &str) {
    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    app.ui.show_midi_editor = true;
    app.run("edit.select", json!({"tracks":["Keys"]})).unwrap();
    app.ui.show_memory_locations = operation == "arrange";
    let session = app.engine.session().clone();
    let ctx = egui::Context::default();
    for _ in 0..6 {
        frame(&ctx, &mut app, vec![]);
    }
    let initial = panel(&ctx);
    let start = pos2(initial.center().x, initial.top() - 28.0);
    frame(&ctx, &mut app, vec![Event::PointerMoved(start)]);
    frame(&ctx, &mut app, vec![button(start, true)]);
    let first = start - vec2(0.0, 80.0);
    frame(&ctx, &mut app, vec![Event::PointerMoved(first)]);
    frame(&ctx, &mut app, vec![]);
    assert!((panel(&ctx).height() - 400.0).abs() < 1.0, "first drag must resize");
    let second = start - vec2(0.0, 120.0);
    frame(&ctx, &mut app, vec![Event::PointerMoved(second)]);
    frame(&ctx, &mut app, vec![]);
    assert!((panel(&ctx).height() - 440.0).abs() < 1.0, "a normal second held move must continue resizing");
    match operation {
        "reset" => {
            app.run("ui.dock.reset", json!({})).unwrap();
        }
        "arrange" => {
            app.run("window.arrange_cascade", json!({})).unwrap();
        }
        "visibility" => {
            app.run("window.midi_editor", json!({})).unwrap();
            frame(&ctx, &mut app, vec![]);
            app.run("window.midi_editor", json!({})).unwrap();
            frame(&ctx, &mut app, vec![]);
        }
        _ => panic!("unknown synthetic operation"),
    }
    let reset = serde_json::to_value(&app.ui.docking).unwrap();
    let after_reset = start - vec2(0.0, 160.0);
    frame(&ctx, &mut app, vec![Event::PointerMoved(after_reset)]);
    frame(&ctx, &mut app, vec![]);
    assert_eq!(serde_json::to_value(&app.ui.docking).unwrap(), reset, "old held divider must not resize the replacement layout");
    let expected = if operation == "reset" { 320.0 } else { 440.0 };
    assert!((panel(&ctx).height() - expected).abs() < 1.0);
    frame(&ctx, &mut app, vec![button(after_reset, false)]);
    for _ in 0..4 {
        frame(&ctx, &mut app, vec![]);
    }
    assert_eq!(serde_json::to_value(&app.ui.docking).unwrap(), reset);
    assert_eq!(*app.engine.session(), session);
}

#[test]
fn a_selected_video_clip_is_not_a_midi_target() {
    use soundcraft_model::{ChannelFormat, Clip, ClipContent, SourceId, Track, TrackKind};
    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    app.run("edit.select", json!({"tracks": ["Keys"]})).unwrap();
    let midi = midi_editor::target_clip(&app).unwrap();
    let s = app.engine.session_mut();
    let track = s.add_track(TrackKind::Video, ChannelFormat::Mono, None);
    let video = s.new_clip_id();
    let source = SourceId(s.alloc());
    s.track_mut(track).and_then(Track::playlist_mut).unwrap().clips.push(Clip::video(video, "picture", source, 0, 0, 48_000));

    app.run("edit.select", json!({"clips": [video.0]})).unwrap();
    assert_eq!(midi_editor::target_clip(&app), None, "a video clip alone leaves the MIDI editors without a target");
    app.run("edit.select", json!({"clips": [video.0, midi.0]})).unwrap();
    assert_eq!(midi_editor::target_clip(&app), Some(midi), "the MIDI clip after a video clip is the target");
    let (_, clip) = app.engine.session().find_clip(midi).unwrap();
    assert!(matches!(clip.content, ClipContent::Midi { .. }));
}

#[test]
fn only_pencil_inserts_notes_in_the_midi_roll() {
    use soundcraft_model::{ClipContent, Tool};

    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    app.ui.show_midi_editor = true;
    let created = app.run("midi.new_clip", json!({"track": "Keys", "start": 0, "length": 96000})).unwrap();
    let clip = created["clip"].as_u64().unwrap();
    app.run("edit.select", json!({"clips": [clip]})).unwrap();
    let ctx = egui::Context::default();
    for _ in 0..6 {
        frame(&ctx, &mut app, vec![]);
    }
    let body = panel(&ctx);
    let point = pos2(body.left() + 200.0, body.top() + 60.0);
    for tool in Tool::ALL.into_iter().filter(|tool| *tool != Tool::Pencil).chain([Tool::Pencil]) {
        app.run("edit.tool", json!({"tool": tool.id()})).unwrap();
        let before = app.engine.session().clone();
        frame(&ctx, &mut app, vec![Event::PointerMoved(point)]);
        frame(&ctx, &mut app, vec![button(point, true)]);
        frame(&ctx, &mut app, vec![button(point, false)]);
        let (_, edited) = app.engine.session().find_clip(soundcraft_model::ClipId(clip)).unwrap();
        let ClipContent::Midi { sequence } = &edited.content else { panic!("MIDI clip required") };
        if tool == Tool::Pencil {
            assert_eq!(sequence.notes.len(), 1);
            app.run("edit.undo", json!({})).unwrap();
        }
        assert!(*app.engine.session() == before, "{tool:?} click must not change the session");
    }
}

#[test]
fn secondary_click_deletes_only_the_note_under_the_pointer_and_undo_restores_it() {
    use soundcraft_model::ClipContent;

    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    app.ui.show_midi_editor = true;
    let created = app.run("midi.new_clip", json!({"track": "Keys", "start": 0, "length": 96000})).unwrap();
    let clip = created["clip"].as_u64().unwrap();
    app.run("edit.select", json!({"clips": [clip]})).unwrap();
    app.run("edit.tool", json!({"tool": "pencil"})).unwrap();
    let ctx = egui::Context::default();
    for _ in 0..6 {
        frame(&ctx, &mut app, vec![]);
    }
    let body = panel(&ctx);
    let point = pos2(body.left() + 200.0, body.top() + 60.0);
    for pressed in [true, false] {
        frame(&ctx, &mut app, vec![Event::PointerMoved(point), button(point, pressed)]);
    }
    let note_count = |app: &SoundApp| {
        let (_, edited) = app.engine.session().find_clip(soundcraft_model::ClipId(clip)).unwrap();
        let ClipContent::Midi { sequence } = &edited.content else { panic!("MIDI clip required") };
        sequence.notes.len()
    };
    assert_eq!(note_count(&app), 1);
    for pressed in [true, false] {
        frame(&ctx, &mut app, vec![Event::PointerMoved(point), secondary_button(point, pressed)]);
    }
    assert_eq!(note_count(&app), 0);
    app.run("edit.undo", json!({})).unwrap();
    assert_eq!(note_count(&app), 1);
}

#[test]
fn held_secondary_drag_erases_every_crossed_note_as_one_undo_step() {
    use soundcraft_model::ClipContent;

    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    app.ui.show_midi_editor = true;
    let created = app.run("midi.new_clip", json!({"track": "Keys", "start": 0, "length": 96000})).unwrap();
    let clip = created["clip"].as_u64().unwrap();
    app.run("edit.select", json!({"clips": [clip]})).unwrap();
    app.run("edit.tool", json!({"tool": "pencil"})).unwrap();
    let ctx = egui::Context::default();
    for _ in 0..6 {
        frame(&ctx, &mut app, vec![]);
    }
    let body = panel(&ctx);
    let first = pos2(body.left() + 200.0, body.top() + 60.0);
    let second = pos2(first.x + 220.0, first.y);
    for point in [first, second] {
        frame(&ctx, &mut app, vec![Event::PointerMoved(point), button(point, true)]);
        frame(&ctx, &mut app, vec![Event::PointerMoved(point), button(point, false)]);
    }
    let note_count = |app: &SoundApp| {
        let (_, edited) = app.engine.session().find_clip(soundcraft_model::ClipId(clip)).unwrap();
        let ClipContent::Midi { sequence } = &edited.content else { panic!("MIDI clip required") };
        sequence.notes.len()
    };
    assert_eq!(note_count(&app), 2);

    frame(&ctx, &mut app, vec![Event::PointerMoved(first), secondary_button(first, true)]);
    for step in 1..=8 {
        frame(&ctx, &mut app, vec![Event::PointerMoved(first.lerp(second, step as f32 / 8.0))]);
    }
    frame(&ctx, &mut app, vec![Event::PointerMoved(second), secondary_button(second, false)]);
    assert_eq!(note_count(&app), 0);

    app.run("edit.undo", json!({})).unwrap();
    assert_eq!(note_count(&app), 2, "one undo must restore the complete erase pass");
}

#[test]
fn right_edge_resize_uses_press_origin_snaps_the_end_and_keeps_the_group_selected() {
    use soundcraft_model::{ChannelFormat, ClipContent, TrackKind};

    let mut engine = soundcraft_engine::Engine::default();
    let track = engine.session_mut().add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
    let clip = engine.execute("midi.new_clip", &json!({"track": track.0, "start": 0, "length": 96_000})).unwrap()["clip"].as_u64().unwrap();
    for (pitch, start, length) in [(60, 0, 500), (64, 960, 300)] {
        engine.execute("midi.note_add", &json!({"clip": clip, "pitch": pitch, "start_ticks": start, "length_ticks": length})).unwrap();
    }
    engine.execute("edit.select", &json!({"clips": [clip]})).unwrap();
    let mut app = SoundApp::new(engine, None, Services::default());
    app.ui.show_midi_editor = true;
    let ctx = egui::Context::default();
    for _ in 0..6 {
        frame(&ctx, &mut app, vec![]);
    }
    app.midi.selected = vec![0, 1];

    let body = panel(&ctx);
    let viewport = app.midi.viewport.clone();
    let start = pos2(
        body.left() + 48.0 + ((500.0 - viewport.start_tick) / viewport.ticks_per_pixel) as f32 - 1.0,
        body.top() + 22.0 + (127 - 60) as f32 * viewport.row_height - app.midi.scroll + viewport.row_height / 2.0,
    );
    let end = start + vec2(83.0, 0.0);
    frame(&ctx, &mut app, vec![Event::PointerMoved(start), button(start, true)]);
    for step in 1..=4 {
        frame(&ctx, &mut app, vec![Event::PointerMoved(start.lerp(end, step as f32 / 4.0))]);
    }
    frame(&ctx, &mut app, vec![Event::PointerMoved(end), button(end, false)]);

    let lengths = |app: &SoundApp| {
        let (_, edited) = app.engine.session().find_clip(soundcraft_model::ClipId(clip)).unwrap();
        let ClipContent::Midi { sequence } = &edited.content else { panic!("MIDI clip required") };
        sequence.notes.iter().map(|note| note.length).collect::<Vec<_>>()
    };
    let resized = lengths(&app);
    assert!(resized[0] > 500, "the note must grow from its pressed right edge");
    assert_eq!(resized[0] - 500, resized[1] - 300, "the selected group must receive one shared duration delta");
    assert_eq!(resized[0] % 240, 0, "the dragged note end must land on the MIDI grid");
    assert_eq!(app.midi.selected, vec![0, 1], "FL-style resizing keeps the resized group selected");
    app.run("edit.undo", json!({})).unwrap();
    assert_eq!(lengths(&app), vec![500, 300]);
}

#[test]
fn windows_ctrl_only_copy_and_paste_reaches_selected_midi_notes() {
    use soundcraft_model::{ChannelFormat, ClipContent, TrackKind};

    let mut engine = soundcraft_engine::Engine::default();
    let track = engine.session_mut().add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
    let clip = engine.execute("midi.new_clip", &json!({"track": track.0, "start": 0, "length": 96_000})).unwrap()["clip"].as_u64().unwrap();
    engine.execute("midi.note_add", &json!({"clip": clip, "pitch": 60, "start_ticks": 0, "length_ticks": 480})).unwrap();
    engine.execute("edit.select", &json!({"clips": [clip]})).unwrap();
    let mut app = SoundApp::new(engine, None, Services::default());
    app.ui.show_midi_editor = true;
    let ctx = egui::Context::default();
    ctx.set_os(OperatingSystem::Windows);
    for _ in 0..6 {
        frame(&ctx, &mut app, vec![]);
    }
    app.midi.selected = vec![0];
    app.midi.keyboard_focus = true;
    let ctrl = Modifiers::CTRL;
    frame(&ctx, &mut app, vec![key(Key::C, true, ctrl), key(Key::C, false, ctrl)]);
    assert_eq!(app.engine.clipboard.midi_notes.len(), 1);
    frame(&ctx, &mut app, vec![key(Key::V, true, ctrl), key(Key::V, false, ctrl)]);
    let (_, edited) = app.engine.session().find_clip(soundcraft_model::ClipId(clip)).unwrap();
    let ClipContent::Midi { sequence } = &edited.content else { panic!("MIDI clip required") };
    assert_eq!(sequence.notes.len(), 2);
    assert_eq!(sequence.notes.iter().map(|note| note.start).collect::<Vec<_>>(), vec![0, 3_840]);
    assert!(edited.length > 96_000, "pasting into the next bar must extend a one-bar MIDI clip");
}

#[test]
fn clicking_inside_and_outside_the_piano_roll_switches_clipboard_focus() {
    let mut app = SoundApp::new(soundcraft_engine::demo::demo_engine(), None, Services::default());
    app.ui.show_midi_editor = true;
    app.run("edit.select", json!({"tracks": ["Keys"]})).unwrap();
    let ctx = egui::Context::default();
    for _ in 0..6 {
        frame(&ctx, &mut app, vec![]);
    }
    let inside = panel(&ctx).center();
    frame(&ctx, &mut app, vec![Event::PointerMoved(inside), button(inside, true)]);
    frame(&ctx, &mut app, vec![Event::PointerMoved(inside), button(inside, false)]);
    assert!(app.midi.keyboard_focus);

    let timeline = pos2(800.0, 180.0);
    assert!(!panel(&ctx).contains(timeline));
    frame(&ctx, &mut app, vec![Event::PointerMoved(timeline), button(timeline, true)]);
    frame(&ctx, &mut app, vec![Event::PointerMoved(timeline), button(timeline, false)]);
    assert!(!app.midi.keyboard_focus);
}
