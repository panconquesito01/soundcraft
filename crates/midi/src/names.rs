const PITCH_CLASSES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

const ENGLISH_SHARPS: [&str; 12] = ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"];
const ENGLISH_FLATS: [&str; 12] = ["C", "D♭", "D", "E♭", "E", "F", "G♭", "G", "A♭", "A", "B♭", "B"];
const SPANISH_SHARPS: [&str; 12] = ["Do", "Do♯", "Re", "Re♯", "Mi", "Fa", "Fa♯", "Sol", "Sol♯", "La", "La♯", "Si"];
const SPANISH_FLATS: [&str; 12] = ["Do", "Re♭", "Re", "Mi♭", "Mi", "Fa", "Sol♭", "Sol", "La♭", "La", "Si♭", "Si"];

/// Language used for pitch names. This is independent from the application's UI language.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteLanguage {
    #[default]
    English,
    Spanish,
}

/// Enharmonic spelling for black keys. `Auto` currently preserves SoundCraft's sharp spelling;
/// key-signature-aware spelling can promote it without changing stored MIDI pitches.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccidentalStyle {
    #[default]
    Sharps,
    Flats,
    Auto,
}

/// Note name using the Pro Tools convention where middle C (60) is "C3" (so 0 is "C-2").
pub fn note_name(pitch: u8) -> String {
    let class = PITCH_CLASSES.get(usize::from(pitch % 12)).copied().unwrap_or("?");
    let octave = i32::from(pitch / 12) - 2;
    format!("{class}{octave}")
}

/// Localized note name using SoundCraft's octave convention (MIDI 60 = octave 3).
///
/// This function changes display only. The MIDI pitch remains the exact input value.
pub fn note_name_with(pitch: u8, language: NoteLanguage, accidentals: AccidentalStyle) -> String {
    let names = match (language, accidentals) {
        (NoteLanguage::English, AccidentalStyle::Flats) => &ENGLISH_FLATS,
        (NoteLanguage::Spanish, AccidentalStyle::Flats) => &SPANISH_FLATS,
        (NoteLanguage::English, AccidentalStyle::Sharps | AccidentalStyle::Auto) => &ENGLISH_SHARPS,
        (NoteLanguage::Spanish, AccidentalStyle::Sharps | AccidentalStyle::Auto) => &SPANISH_SHARPS,
    };
    let class = names.get(usize::from(pitch % 12)).copied().unwrap_or("?");
    let octave = i32::from(pitch / 12) - 2;
    format!("{class}{octave}")
}

const GM_DRUMS: [&str; 47] = [
    "Acoustic Bass Drum",
    "Bass Drum 1",
    "Side Stick",
    "Acoustic Snare",
    "Hand Clap",
    "Electric Snare",
    "Low Floor Tom",
    "Closed Hi-Hat",
    "High Floor Tom",
    "Pedal Hi-Hat",
    "Low Tom",
    "Open Hi-Hat",
    "Low-Mid Tom",
    "Hi-Mid Tom",
    "Crash Cymbal 1",
    "High Tom",
    "Ride Cymbal 1",
    "Chinese Cymbal",
    "Ride Bell",
    "Tambourine",
    "Splash Cymbal",
    "Cowbell",
    "Crash Cymbal 2",
    "Vibraslap",
    "Ride Cymbal 2",
    "Hi Bongo",
    "Low Bongo",
    "Mute Hi Conga",
    "Open Hi Conga",
    "Low Conga",
    "High Timbale",
    "Low Timbale",
    "High Agogo",
    "Low Agogo",
    "Cabasa",
    "Maracas",
    "Short Whistle",
    "Long Whistle",
    "Short Guiro",
    "Long Guiro",
    "Claves",
    "Hi Wood Block",
    "Low Wood Block",
    "Mute Cuica",
    "Open Cuica",
    "Mute Triangle",
    "Open Triangle",
];

/// General MIDI (level 1) percussion key-map name for a pitch on channel 10 (keys 35..=81).
pub fn gm_drum_name(pitch: u8) -> Option<&'static str> {
    let idx = usize::from(pitch).checked_sub(35)?;
    GM_DRUMS.get(idx).copied()
}

const GM_PROGRAMS: [&str; 128] = [
    "Acoustic Grand Piano",
    "Bright Acoustic Piano",
    "Electric Grand Piano",
    "Honky-tonk Piano",
    "Electric Piano 1",
    "Electric Piano 2",
    "Harpsichord",
    "Clavi",
    "Celesta",
    "Glockenspiel",
    "Music Box",
    "Vibraphone",
    "Marimba",
    "Xylophone",
    "Tubular Bells",
    "Dulcimer",
    "Drawbar Organ",
    "Percussive Organ",
    "Rock Organ",
    "Church Organ",
    "Reed Organ",
    "Accordion",
    "Harmonica",
    "Tango Accordion",
    "Acoustic Guitar (nylon)",
    "Acoustic Guitar (steel)",
    "Electric Guitar (jazz)",
    "Electric Guitar (clean)",
    "Electric Guitar (muted)",
    "Overdriven Guitar",
    "Distortion Guitar",
    "Guitar Harmonics",
    "Acoustic Bass",
    "Electric Bass (finger)",
    "Electric Bass (pick)",
    "Fretless Bass",
    "Slap Bass 1",
    "Slap Bass 2",
    "Synth Bass 1",
    "Synth Bass 2",
    "Violin",
    "Viola",
    "Cello",
    "Contrabass",
    "Tremolo Strings",
    "Pizzicato Strings",
    "Orchestral Harp",
    "Timpani",
    "String Ensemble 1",
    "String Ensemble 2",
    "SynthStrings 1",
    "SynthStrings 2",
    "Choir Aahs",
    "Voice Oohs",
    "Synth Voice",
    "Orchestra Hit",
    "Trumpet",
    "Trombone",
    "Tuba",
    "Muted Trumpet",
    "French Horn",
    "Brass Section",
    "SynthBrass 1",
    "SynthBrass 2",
    "Soprano Sax",
    "Alto Sax",
    "Tenor Sax",
    "Baritone Sax",
    "Oboe",
    "English Horn",
    "Bassoon",
    "Clarinet",
    "Piccolo",
    "Flute",
    "Recorder",
    "Pan Flute",
    "Blown Bottle",
    "Shakuhachi",
    "Whistle",
    "Ocarina",
    "Lead 1 (square)",
    "Lead 2 (sawtooth)",
    "Lead 3 (calliope)",
    "Lead 4 (chiff)",
    "Lead 5 (charang)",
    "Lead 6 (voice)",
    "Lead 7 (fifths)",
    "Lead 8 (bass + lead)",
    "Pad 1 (new age)",
    "Pad 2 (warm)",
    "Pad 3 (polysynth)",
    "Pad 4 (choir)",
    "Pad 5 (bowed)",
    "Pad 6 (metallic)",
    "Pad 7 (halo)",
    "Pad 8 (sweep)",
    "FX 1 (rain)",
    "FX 2 (soundtrack)",
    "FX 3 (crystal)",
    "FX 4 (atmosphere)",
    "FX 5 (brightness)",
    "FX 6 (goblins)",
    "FX 7 (echoes)",
    "FX 8 (sci-fi)",
    "Sitar",
    "Banjo",
    "Shamisen",
    "Koto",
    "Kalimba",
    "Bag pipe",
    "Fiddle",
    "Shanai",
    "Tinkle Bell",
    "Agogo",
    "Steel Drums",
    "Woodblock",
    "Taiko Drum",
    "Melodic Tom",
    "Synth Drum",
    "Reverse Cymbal",
    "Guitar Fret Noise",
    "Breath Noise",
    "Seashore",
    "Bird Tweet",
    "Telephone Ring",
    "Helicopter",
    "Applause",
    "Gunshot",
];

/// General MIDI (level 1) instrument name for a 0-based program number. Values above 127
/// return `"Unknown"`.
pub fn gm_program_name(program: u8) -> &'static str {
    GM_PROGRAMS.get(usize::from(program)).copied().unwrap_or("Unknown")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(note_name(60), "C3");
        assert_eq!(note_name(0), "C-2");
        assert_eq!(note_name(127), "G8");
        assert_eq!(note_name(61), "C#3");
        assert_eq!(gm_drum_name(36), Some("Bass Drum 1"));
        assert_eq!(gm_drum_name(81), Some("Open Triangle"));
        assert_eq!(gm_drum_name(34), None);
        assert_eq!(gm_drum_name(82), None);
        assert_eq!(gm_program_name(0), "Acoustic Grand Piano");
        assert_eq!(gm_program_name(127), "Gunshot");
        assert_eq!(gm_program_name(200), "Unknown");
    }

    #[test]
    fn localized_names_keep_every_midi_pitch_and_octave() {
        assert_eq!(note_name_with(60, NoteLanguage::English, AccidentalStyle::Sharps), "C3");
        assert_eq!(note_name_with(61, NoteLanguage::English, AccidentalStyle::Sharps), "C♯3");
        assert_eq!(note_name_with(61, NoteLanguage::English, AccidentalStyle::Flats), "D♭3");
        assert_eq!(note_name_with(60, NoteLanguage::Spanish, AccidentalStyle::Sharps), "Do3");
        assert_eq!(note_name_with(61, NoteLanguage::Spanish, AccidentalStyle::Flats), "Re♭3");
        assert_eq!(note_name_with(0, NoteLanguage::Spanish, AccidentalStyle::Auto), "Do-2");
        assert_eq!(note_name_with(127, NoteLanguage::Spanish, AccidentalStyle::Auto), "Sol8");

        for pitch in 0..=127u8 {
            for language in [NoteLanguage::English, NoteLanguage::Spanish] {
                for accidental in [AccidentalStyle::Sharps, AccidentalStyle::Flats, AccidentalStyle::Auto] {
                    let name = note_name_with(pitch, language, accidental);
                    assert!(!name.is_empty(), "pitch {pitch}");
                    assert!(name.ends_with(&(i32::from(pitch / 12) - 2).to_string()), "pitch {pitch}: {name}");
                }
            }
        }
    }
}
