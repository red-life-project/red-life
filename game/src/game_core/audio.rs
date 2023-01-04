use ggez::audio::{AudioContext, SoundSource};
use ggez::context::Has;
use ggez::{audio, Context, GameResult};

/// Audio controller responsible for managing which sound should be played now
pub struct AudioState {
    /// Main theme played most of the time and interrupted by machines sounds
    main: audio::Source,
}

impl AudioState {
    pub fn new(ctx: &impl Has<AudioContext>) -> GameResult<AudioState> {
        let data = audio::SoundData::from_bytes(include_bytes!("../../../assets/Red_Planet.mp3"));
        let sound = audio::Source::from_data(ctx, data)?;
        let s = AudioState { main: sound };
        Ok(s)
    }

    /// Plays the sound multiple times
    pub fn play_main_theme(&mut self, ctx: &mut Context) {
        self.main.play_detached(ctx).ok();
    }

    /// Plays the sound multiple times
    pub fn pause_main_theme(&mut self, _ctx: &mut Context) {
        self.main.pause();
    }
}
