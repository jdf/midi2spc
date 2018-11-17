use std::io::Write;
use byteorder::*;
use midi::MidiHandler;

use ghakuf::messages::*;

const TEMPO_FACTOR: f32 = 0.2;

// instruments
//0. Unknown (00)
const UNKNOWN: u8 = 0;
//1. Rain (01)
const RAIN: u8 = 1;
//2. Tympani   (02)
const TIMPANI: u8 = 2;
//3. Square wave (03)
const SQUARE_WAVE: u8 = 3;
//4. Saw wave (04)
const SAW_WAVE: u8 = 4;
//5. Sine wave (05)
const SINE_WAVE: u8 = 5;
//6. Double saw wave 1 (06)
const DOUBLE_SAW_WAVE_1: u8 = 6;
//7. Double saw wave 2 (07)
const DOUBLE_SAW_WAVE_2: u8 = 7;
//8. Tweet (08)
const TWEET: u8 = 8;
//9. Strings (09)
const STRINGS: u8 = 9;
//10. Same as 9 (0A)
//11. Trombone (0B)
const TROMBONE: u8 = 11;
//12. Cymbal (0C)
const CYMBAL: u8 = 12;
//13. Ocarina (0D)
const OCARINA: u8 = 13;
//14. Chime (0E)
const CHIME: u8 = 14;
//15. Harp (0F)
const HARP: u8 = 15;
//16. Splash (10)
const SPLASH: u8 = 16;
//17. Trumpet (11)
const TRUMPET: u8 = 17;
//18. Horn (12)
const HORN: u8 = 18;
//19. Snare (13)
const SNARE: u8 = 19;
//20. Same as 19 (14)
//21. Choir (15)
const CHOIR: u8 = 21;
//22. Flute (16)
const FLUTE: u8 = 22;
//23. "Oof" (17)
const OOF: u8 = 23;
//24. Guitar (18)
const GUITAR: u8 = 24;

const INSTRUMENT_MAP: [u8;128] = [
    CHIME, //    0 Acoustic Grand Piano
    CHIME, //    1 Bright Acoustic Piano
    CHIME, //    2 Electric Grand Piano
    CHIME, //    3 Honky-tonk Piano
    CHIME, //    4 Electric Piano 1
    CHIME, //    5 Electric Piano 2
    CHIME, //    6 Harpsichord
    CHIME, //    7 Clavinet
    CHIME, //    8 Celesta
    CHIME, //    9 Glockenspiel
    CHIME, //    10 Music Box
    CHIME, //    11 Vibraphone
    CHIME, //    12 Marimba
    CHIME, //    13 Xylophone
    CHIME, //    14 Tubular Bells
    CHIME, //    15 Dulcimer
    CHOIR, //    16 Drawbar Organ
    CHOIR, //    17 Percussive Organ
    CHOIR, //    18 Rock Organ
    CHOIR, //    19 Church Organ
    CHOIR, //    20 Reed Organ
    STRINGS, //    21 Accordion
    OCARINA, //    22 Harmonica
    OOF,
    GUITAR, //    24 Acoustic Guitar (nylon)
    GUITAR, //    25 Acoustic Guitar (steel)
    GUITAR, //    26 Electric Guitar (jazz)
    GUITAR, //    27 Electric Guitar (clean)
    GUITAR, //    28 Electric Guitar (muted)
    GUITAR, //    29 Overdriven Guitar
    GUITAR, //    30 Distortion Guitar
    GUITAR, //    31 Guitar Harmonics
    GUITAR, //    32 Acoustic Bass
    GUITAR, //    33 Electric Bass (finger)
    GUITAR, //    34 Electric Bass (pick)
    GUITAR, //    35 Fretless Bass
    GUITAR, //    36 Slap Bass 1
    GUITAR, //    37 Slap Bass 2
    GUITAR, //    38 Synth Bass 1
    GUITAR, //    39 Synth Bass 2
    STRINGS, //    40 Violin
    STRINGS, //    41 Viola
    STRINGS, //    42 Cello
    STRINGS, //    43 Contrabass
    STRINGS, //    44 Tremolo Strings
    STRINGS, //    45 Pizzicato Strings
    HARP, //    46 Orchestral Harp
    TIMPANI, //    47 Timpani
    STRINGS, //    48 String Ensemble 1
    STRINGS, //    49 String Ensemble 2
    STRINGS, //    50 Synth Strings 1
    STRINGS, //    51 Synth Strings 2
    CHOIR, //    52 Choir Aahs
    CHOIR, //    53 Voice Oohs
    CHOIR, //    54 Synth Choir
    STRINGS, //    55 Orchestra Hit
    TRUMPET, //    56 Trumpet
    TROMBONE, //    57 Trombone
    HORN, //    58 Tuba
    TRUMPET, //    59 Muted Trumpet
    HORN, //    60 French Horn
    HORN, //    61 Brass Section
    HORN, //    62 Synth Brass 1
    HORN, //    63 Synth Brass 2
    TRUMPET, //    64 Soprano Sax
    TRUMPET, //    65 Alto Sax
    TROMBONE, //    66 Tenor Sax
    TROMBONE, //    67 Baritone Sax
    HORN, //    68 Oboe
    HORN, //    69 English Horn
    HORN, //    70 Bassoon
    FLUTE, //    71 Clarinet
    FLUTE, //    72 Piccolo
    FLUTE, //    73 Flute
    OCARINA, //    74 Recorder
    OCARINA, //    75 Pan Flute
    OCARINA, //    76 Blown bottle
    OCARINA, //    77 Shakuhachi
    TWEET, //    78 Whistle
    OCARINA, //    79 Ocarina
    SQUARE_WAVE, //    80 Lead 1 (square)
    SAW_WAVE, //    81 Lead 2 (sawtooth)
    OCARINA, //    82 Lead 3 (calliope)
    DOUBLE_SAW_WAVE_1, //    83 Lead 4 (chiff)
    DOUBLE_SAW_WAVE_1, //    84 Lead 5 (charang)
    DOUBLE_SAW_WAVE_1, //    85 Lead 6 (voice)
    DOUBLE_SAW_WAVE_2, //    86 Lead 7 (fifths)
    DOUBLE_SAW_WAVE_2, //    87 Lead 8 (bass + lead)
    SINE_WAVE, //    88 Pad 1 (new age)
    SINE_WAVE, //    89 Pad 2 (warm)
    SINE_WAVE, //    90 Pad 3 (polysynth)
    CHOIR, //    91 Pad 4 (choir)
    SINE_WAVE, //    92 Pad 5 (bowed)
    SINE_WAVE, //    93 Pad 6 (metallic)
    SINE_WAVE, //    94 Pad 7 (halo)
    SINE_WAVE, //    95 Pad 8 (sweep)
    RAIN, //    96 FX 1 (rain)
    RAIN, //    97 FX 2 (soundtrack)
    RAIN, //    98 FX 3 (crystal)
    RAIN, //    99 FX 4 (atmosphere)
    RAIN, //    100 FX 5 (brightness)
    RAIN, //    101 FX 6 (goblins)
    RAIN, //    102 FX 7 (echoes)
    RAIN, //    103 FX 8 (sci-fi)
    GUITAR, //    104 Sitar
    GUITAR, //    105 Banjo
    GUITAR, //    106 Shamisen
    GUITAR, //    107 Koto
    GUITAR, //    108 Kalimba
    HORN, //    109 Bagpipe
    STRINGS, //    110 Fiddle
    HORN, //    111 Shanai
    CHIME, //    112 Tinkle Bell
    CHIME, //    113 Agogo
    TIMPANI, //    114 Steel Drums
    SNARE, //    115 Woodblock
    TIMPANI, //    116 Taiko Drum
    TIMPANI, //    117 Melodic Tom
    TIMPANI, //    118 Synth Drum
    CYMBAL, //    119 Reverse Cymbal
    OOF, //    120 Guitar Fret Noise
    OOF, //    121 Breath Noise
    SPLASH, //    122 Seashore
    TWEET, //    123 Bird Tweet
    CHIME, //    124 Telephone Ring
    OOF, //    125 Helicopter
    RAIN, //    126 Applause
    OOF //    127 Gunshot
];

const PREAMBLE_TRACK_0: [u8;10] = [
    0xfa, 0x19,  // percussion offset
    0xe5, 0xc8,  // global volume
    0xed, 0xc8,  // channel volume
    0xe9, 0x00,  // global transpose
    0xea, 0x00,  // channel transpose
];

const PREAMBLE_OTHER_TRACK: [u8;4] = [
    0xed, 0xc8,  // channel volume
    0xea, 0x00,  // channel transpose
];

#[derive(Clone, Debug)]
enum Command {
    Note(u8),
    Rest,
    SetInstrument(u8),
    Pan(u8),
    PanFade(u8, u8),
    Vibrato(u8, u8, u8),
    VibratoOff,
    MasterVolume(u8),
    MasterVolumeFade(u8, u8),
    Tempo(u8),
    TempoFade(u8, u8),
    GlobalTranspose(u8),
    ChannelTranspose(u8),
    Tremolo(u8, u8, u8),
    TremoloOff,
    ChannelVolume(u8),
    ChannelVolumeFade(u8),
    CallLoop(u8, u8, u8),
    VibratoFade(u8),
    PitchEnvelopeTo(u8, u8, u8),
    PitchEnvelopeFrom(u8, u8, u8),
    PitchEnvelopeOff,
    Tuning(u8),
    EchoVolume(u8, u8, u8),
    EchoOff,
    EchoParams(u8, u8, u8),
    EchoVolumeFade(u8, u8, u8),
    PitchSlide(u8, u8, u8),
    PercussionPatchBase(u8),
}

impl Command {
    fn write(&self, out: &mut Write) {
        match *self {
            Command::Note(note) => {
                out.write_u8(note);
            },
            Command::Rest => {
                out.write_u8(0xc9);
            },
            Command::SetInstrument(p1) => {
                out.write_u8(0xe0);
                out.write_u8(p1);
            },
            Command::Pan(p1) => {
                out.write_u8(0xe1);
                out.write_u8(p1);
            },
            Command::PanFade(p1, p2) => {
                out.write_u8(0xe2);
                out.write_u8(p1);
                out.write_u8(p2);
            },
            Command::Vibrato(p1, p2, p3) => {
                out.write_u8(0xe3);
                out.write_u8(p1);
                out.write_u8(p2);
                out.write_u8(p3);
            },
            Command::VibratoOff => {
                out.write_u8(0xe4);
            },
            Command::MasterVolume(p1) => {
                out.write_u8(0xe5);
                out.write_u8(p1);
            },
            Command::MasterVolumeFade(p1, p2) => {
                out.write_u8(0xe6);
                out.write_u8(p1);
                out.write_u8(p2);
            },
            Command::Tempo(p1) => {
                out.write_u8(0xe7);
                out.write_u8(p1);
            },
            Command::TempoFade(p1, p2) => {
                out.write_u8(0xe8);
                out.write_u8(p1);
                out.write_u8(p2);
            },
            Command::GlobalTranspose(p1) => {
                out.write_u8(0xe9);
                out.write_u8(p1);
            },
            Command::ChannelTranspose(p1) => {
                out.write_u8(0xea);
                out.write_u8(p1);
            },
            Command::Tremolo(p1, p2, p3) => {
                out.write_u8(0xeb);
                out.write_u8(p1);
                out.write_u8(p2);
                out.write_u8(p3);
            },
            Command::TremoloOff => {
                out.write_u8(0xec);
            },
            Command::ChannelVolume(p1) => {
                out.write_u8(0xed);
                out.write_u8(p1);
            },
            Command::ChannelVolumeFade(p1) => {
                out.write_u8(0xee);
                out.write_u8(p1);
            },
            Command::CallLoop(p1, p2, p3) => {
                out.write_u8(0xef);
                out.write_u8(p1);
                out.write_u8(p2);
                out.write_u8(p3);
            },
            Command::VibratoFade(p1) => {
                out.write_u8(0xf0);
                out.write_u8(p1);
            },
            Command::PitchEnvelopeTo(p1, p2, p3) => {
                out.write_u8(0xf1);
                out.write_u8(p1);
                out.write_u8(p2);
                out.write_u8(p3);
            },
            Command::PitchEnvelopeFrom(p1, p2, p3) => {
                out.write_u8(0xf2);
                out.write_u8(p1);
                out.write_u8(p2);
                out.write_u8(p3);
            },
            Command::PitchEnvelopeOff => {
                out.write_u8(0xf3);
            },
            Command::Tuning(p1) => {
                out.write_u8(0xf4);
                out.write_u8(p1);
            },
            Command::EchoVolume(p1, p2, p3) => {
                out.write_u8(0xf5);
                out.write_u8(p1);
                out.write_u8(p2);
                out.write_u8(p3);
            },
            Command::EchoOff => {
                out.write_u8(0xf6);
            },
            Command::EchoParams(p1, p2, p3) => {
                out.write_u8(0xf7);
                out.write_u8(p1);
                out.write_u8(p2);
                out.write_u8(p3);
            },
            Command::EchoVolumeFade(p1, p2, p3) => {
                out.write_u8(0xf8);
                out.write_u8(p1);
                out.write_u8(p2);
                out.write_u8(p3);
            },
            Command::PitchSlide(p1, p2, p3) => {
                out.write_u8(0xf9);
                out.write_u8(p1);
                out.write_u8(p2);
                out.write_u8(p3);
            },
            Command::PercussionPatchBase(p1) => {
                out.write_u8(0xfa);
                out.write_u8(p1);
            },
        };
    }
}

#[derive(Clone, Debug)]
struct ParameterizedCommand {
    duration: Option<u8>,
    velocity: Option<u8>,
    command: Command,
}

impl ParameterizedCommand {
    fn write(&self, out: &mut Write, prev_duration: u8, prev_velocity: Option<u8>) -> (u8, Option<u8>) {
        let mut duration_out = prev_duration;
        let mut velocity_out = prev_velocity;
        match self.duration {
            Some(duration) => {
                if duration != prev_duration {
                    out.write_u8(duration);
                    match self.velocity {
                        Some(velocity) => {
                            if prev_velocity.is_none() || prev_velocity.unwrap() != velocity {
                                out.write_u8(velocity);
                                velocity_out = Some(velocity);
                            }
                        },
                        _ => {
                            if prev_velocity.is_none() {
                                out.write_u8(0x7d);
                                velocity_out = Some(0x7d);
                            }
                        },
                    }
                    duration_out = duration;
                }
            },
            _ => {},
        }
        self.command.write(out);
        (duration_out, velocity_out)
    }
}

#[derive(Clone, Debug)]
struct Track {
    commands: Vec<ParameterizedCommand>,
}

impl Track {

    fn get_duration(ticks: u32, ticks_per_beat: u16) -> u8 {
        let length_beats = (ticks as f32) / (ticks_per_beat as f32);
        ((length_beats * 24.0) as u8).max(1)
    }

    fn insert_rest(commands: &mut Vec<ParameterizedCommand>, mut last_note_end: u32, abs_time: u32, ticks_per_beat: u16) {
        if abs_time > last_note_end {
            commands.push(ParameterizedCommand {
                duration: Some(Track::get_duration(abs_time - last_note_end, ticks_per_beat)),
                velocity: None,
                command: Command::Rest,
            });
            last_note_end = abs_time;
        }
    }

    fn new(events: Box<Vec<(u32, Message)>>, ticks_per_beat: u16) -> Track {
        let mut commands = Vec::new();
        let mut note_start: Option<u32> = None;
        let mut last_note_end = 0u32;
        for &(abs_time, ref message) in events.as_ref() {
            match *message {
                Message::MetaEvent { delta_time, ref event, ref data } => {
                    if let MetaEvent::SetTempo = *event {
                        Track::insert_rest(&mut commands, last_note_end, abs_time, ticks_per_beat);
                        let usec_per_beat = (data[0] as u32) * 0x10000 + (data[1] as u32) * 0x100 + (data[2] as u32);
                        let bpm = usec_per_beat / 6000;
                        println!("ticks per beat {} usec per beat {} bpm {}", ticks_per_beat, usec_per_beat, bpm);
                        commands.push(ParameterizedCommand {
                            duration: None,
                            velocity: None,
                            command: Command::Tempo((bpm as f32 * TEMPO_FACTOR) as u8)
                        })
                    }
                }
                Message::MidiEvent { delta_time, ref event } => {
                    match *event {
                        MidiEvent::NoteOff { ch, note, velocity } => {
                            if let Some(start) = note_start {
                                commands.push(ParameterizedCommand {
                                    duration: Some(Track::get_duration(abs_time - start, ticks_per_beat)),
                                    velocity: None,
                                    command: Command::Note(note + 0x68)
                                });
                                note_start = None;
                                last_note_end = abs_time;
                            }
                        }
                        MidiEvent::NoteOn { ch, note, velocity } => {
                            Track::insert_rest(&mut commands, last_note_end, abs_time, ticks_per_beat);
                            if note_start.is_some() {
                                panic!("More than one voice needed on channel {}", ch);
                            }
                            note_start = Some(abs_time)
                        }
                        MidiEvent::PolyphonicKeyPressure { ch, note, velocity } => {
                            // TODO
                        }
                        MidiEvent::ControlChange { ch, control, data } => {
                            // TODO
                        }
                        MidiEvent::ProgramChange { ch, program } => {
                            Track::insert_rest(&mut commands, last_note_end, abs_time, ticks_per_beat);
                            commands.push(ParameterizedCommand {
                                duration: None,
                                velocity: None,
                                command: Command::SetInstrument(INSTRUMENT_MAP[program as usize])
                            })
                        }
                        MidiEvent::ChannelPressure { ch, pressure } => {
                            // TODO
                        }
                        MidiEvent::PitchBendChange { ch, data } => {
                            // TODO
                        }
                        _ => {}
                    }
                }
                Message::SysExEvent { delta_time, ref event, ref data } => {
                    // TODO (global volume?)
                }
                _ => {}
            }
        }
        Track {
            commands
        }
    }

    fn write(&self, out: &mut Write) {
        let mut duration = 0xff;
        let mut velocity = None;
        self.commands.iter().for_each(|c| {
            let (duration_out, velocity_out) = c.write(out, duration, velocity);
            duration = duration_out;
            velocity = velocity_out;
        });
    }
}

#[derive(Debug)]
struct Part {
    tracks: Vec<Track>,
}

impl Part {
    fn write(&self, out: &mut Write) {
        self.tracks.iter().for_each(|t| t.write(out));
    }

}

#[derive(Debug)]
pub struct Song {
    parts: Vec<Part>,
}

impl Song {
    pub fn from_midi(midi: &MidiHandler) -> Song {
        let mut tracks = midi.channel_map.iter().map(|channel| {
            Track::new(midi.events_for_channel(*channel), midi.ticks_per_beat)
        }).collect();
        let mut parts = Vec::new();
        let mut part = Part {
            tracks,
        };
        parts.push(part);
        Song {
            parts,
        }
    }

    pub fn write_part_zero_track(&self, out: &mut Write, track_idx: usize) {
        let track = &self.parts[0].tracks[track_idx];
        if !track.commands.is_empty() {
            if track_idx == 0 {
                out.write(&PREAMBLE_TRACK_0);
            } else {
                out.write(&PREAMBLE_OTHER_TRACK);
            }
            track.write(out);
            out.write_u8(0x00);
            out.write_u8(0x00);
        }
    }
}