use crate::backend::gamestate::GameState;
use crate::machines::machine::Mashine;
use ggez::graphics::Image;
use ggez::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::read_dir;
use tracing::info;

#[derive(Debug, Clone)]
pub struct MaschineSprite {
    name: String,
    pub idel: Image,
    pub broken: Image,
    pub running: Image,
}

impl Default for MaschineSprite {
    fn default() -> Self {
        let bytes = fs::read("assets/error.png").unwrap();
        let ctx = ggez::ContextBuilder::new("img_default", "sander")
            .window_setup(ggez::conf::WindowSetup::default())
            .build()
            .unwrap();
        let error = Image::from_bytes(&ctx.0, bytes.as_slice()).unwrap();
        Self {
            name: "".to_string(),
            idel: error.clone(),
            broken: error.clone(),
            running: error,
        }
    }
}

impl MaschineSprite {
    pub(crate) fn default(gs: &GameState) -> Self {
        MaschineSprite::new(gs, "test")
    }
    pub fn new(gs: &GameState, name: &str) -> Self {
        //test_Broken.png
        info!("Creating new MachineSprite: name: {}", name);

        let broken = gs
            .get_asset(format!("{}_Broken.png", name).as_str())
            .unwrap()
            .clone();
        let idel = gs
            .get_asset(format!("{}_Idel.png", name).as_str())
            .unwrap()
            .clone();
        let running = gs
            .get_asset(format!("{}_Running.png", name).as_str())
            .unwrap()
            .clone();
        Self {
            name: name.to_string(),
            idel,
            broken,
            running,
        }
    }
}
