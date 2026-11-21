//! Native MIDI synthesis and the original packed WAV effect bank.
use rustysynth::{MidiFile, MidiFileSequencer, SoundFont, Synthesizer, SynthesizerSettings};
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, mpsc},
};

#[derive(Debug, Clone)]
pub enum AudioCommand {
    Play {
        id: u32,
        name: String,
        position: i64,
        loops: i32,
        volume: i32,
    },
    Stop(u32),
    Volume(u32, i32),
}

pub struct AudioCore {
    sequencer: MidiFileSequencer,
    midis: HashMap<String, Arc<MidiFile>>,
    midi_id: u32,
    midi_name: String,
    midi_on: bool,
    midi_volume: f32,
    effects: Vec<f32>,
    effects_rate: u32,
    effect_id: u32,
    effect_pos: f64,
    effect_on: bool,
    effect_volume: f32,
}
impl AudioCore {
    pub const RATE: i32 = 44100;
    pub fn new(assets: &Path) -> Result<Self, String> {
        let mut font = std::fs::File::open(assets.join("audio/GeneralUser-GS.sf2"))
            .map_err(|e| format!("SoundFont: {e}"))?;
        let font = Arc::new(SoundFont::new(&mut font).map_err(|e| e.to_string())?);
        let synth = Synthesizer::new(&font, &SynthesizerSettings::new(Self::RATE))
            .map_err(|e| e.to_string())?;
        let mut midis = HashMap::new();
        for e in std::fs::read_dir(assets.join("Sound/midi")).map_err(|e| e.to_string())? {
            let path = e.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|x| x == "mid") {
                let midi =
                    MidiFile::new(&mut std::fs::File::open(&path).map_err(|e| e.to_string())?)
                        .map_err(|e| format!("{}: {e}", path.display()))?;
                midis.insert(
                    format!("Sound/midi/{}", path.file_name().unwrap().to_string_lossy()),
                    Arc::new(midi),
                );
            }
        }
        let mut wav =
            hound::WavReader::open(assets.join("Sound/Effects.wav")).map_err(|e| e.to_string())?;
        let spec = wav.spec();
        let raw: Vec<f32> = if spec.sample_format == hound::SampleFormat::Float {
            wav.samples::<f32>()
                .collect::<Result<_, _>>()
                .map_err(|e| e.to_string())?
        } else {
            let scale = (1u64 << (spec.bits_per_sample - 1)) as f32;
            wav.samples::<i32>()
                .map(|x| x.map(|s| s as f32 / scale))
                .collect::<Result<_, _>>()
                .map_err(|e| e.to_string())?
        };
        let effects = raw
            .chunks_exact(spec.channels as usize)
            .map(|c| c.iter().sum::<f32>() / spec.channels as f32)
            .collect();
        Ok(Self {
            sequencer: MidiFileSequencer::new(synth),
            midis,
            midi_id: 0,
            midi_name: String::new(),
            midi_on: false,
            midi_volume: 0.5,
            effects,
            effects_rate: spec.sample_rate,
            effect_id: 0,
            effect_pos: 0.0,
            effect_on: false,
            effect_volume: 0.5,
        })
    }
    pub fn command(&mut self, c: AudioCommand) {
        match c {
            AudioCommand::Play {
                id,
                name,
                position,
                loops,
                volume,
            } => {
                if name.ends_with(".mid") {
                    if id != self.midi_id || name != self.midi_name {
                        if let Some(midi) = self.midis.get(&name) {
                            self.sequencer.play(midi, loops < 0);
                            self.midi_id = id;
                            self.midi_name = name;
                        } else {
                            eprintln!("Unknown MIDI: {name}");
                            return;
                        }
                    }
                    self.midi_on = true;
                    self.midi_volume = volume as f32 / 100.0;
                } else if name.ends_with(".wav") {
                    self.effect_id = id;
                    self.effect_pos =
                        position.max(0) as f64 * self.effects_rate as f64 / 1_000_000.0;
                    self.effect_on = true;
                    self.effect_volume = volume as f32 / 100.0;
                }
            }
            AudioCommand::Stop(id) => {
                if id == self.midi_id {
                    self.midi_on = false;
                }
                if id == self.effect_id {
                    self.effect_on = false;
                }
            }
            AudioCommand::Volume(id, v) => {
                if id == self.midi_id {
                    self.midi_volume = v as f32 / 100.0;
                }
                if id == self.effect_id {
                    self.effect_volume = v as f32 / 100.0;
                }
            }
        }
    }
    pub fn render(&mut self, frames: usize) -> Vec<f32> {
        let mut left = vec![0.0; frames];
        let mut right = vec![0.0; frames];
        if self.midi_on {
            self.sequencer.render(&mut left, &mut right);
        }
        let mut out = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let fx = if self.effect_on {
                let pos = self.effect_pos as usize;
                if pos >= self.effects.len() {
                    self.effect_on = false;
                    0.0
                } else {
                    let t = (self.effect_pos - pos as f64) as f32;
                    let a = self.effects[pos];
                    let b = *self.effects.get(pos + 1).unwrap_or(&a);
                    self.effect_pos += self.effects_rate as f64 / Self::RATE as f64;
                    (a + (b - a) * t) * self.effect_volume
                }
            } else {
                0.0
            };
            out.push((left[i] * self.midi_volume + fx).clamp(-1.0, 1.0));
            out.push((right[i] * self.midi_volume + fx).clamp(-1.0, 1.0));
        }
        out
    }
    pub fn audit(&mut self) -> Result<usize, String> {
        let tracks = self.midis.clone();
        for (name, midi) in &tracks {
            self.sequencer.play(midi, false);
            let mut l = vec![0.0; Self::RATE as usize * 2];
            let mut r = vec![0.0; l.len()];
            self.sequencer.render(&mut l, &mut r);
            if l.iter().chain(&r).any(|x| !x.is_finite()) {
                return Err(format!("Invalid samples in {name}"));
            }
        }
        self.sequencer.stop();
        Ok(tracks.len())
    }
}

pub struct Mixer {
    pub core: AudioCore,
    pub commands: mpsc::Receiver<AudioCommand>,
}
impl sdl3::audio::AudioCallback<f32> for Mixer {
    fn callback(&mut self, stream: &mut sdl3::audio::AudioStream, additional_amount: i32) {
        while let Ok(command) = self.commands.try_recv() {
            self.core.command(command);
        }
        if additional_amount > 0 {
            let frames = (additional_amount as usize).div_ceil(2);
            let samples = self.core.render(frames);
            if let Err(e) = stream.put_data_f32(&samples) {
                eprintln!("Audio output: {e}");
            }
        }
    }
}
