use crate::backend::gamestate::GameState;

use ggez::graphics::Image;

use crate::RLResult;
use std::fs;
use tracing::info;

#[derive(Debug, Clone)]
pub struct MachineSprite {
    name: String,
    pub idle: Image,
    pub broken: Image,
    pub running: Image,
}

impl Default for MachineSprite {
    fn default() -> Self {
        let bytes = fs::read("assets/error.png").unwrap();
        let ctx = ggez::ContextBuilder::new("img_default", "sander")
            .window_setup(ggez::conf::WindowSetup::default())
            .build()
            .unwrap();
        let error = Image::from_bytes(&ctx.0, bytes.as_slice()).unwrap();
        Self {
            name: "".to_string(),
            idle: error.clone(),
            broken: error.clone(),
            running: error,
        }
    }
}

impl MachineSprite {
    pub fn new(gs: &GameState, name: &str) -> RLResult<Self> {
        //test_Broken.png
        info!("Creating new MachineSprite: name: {}", name);

        let broken = gs
            .get_asset(format!("{}_Broken.png", name).as_str())?
            .clone();
        let idle = gs.get_asset(format!("{}_Idle.png", name).as_str())?.clone();
        let running = gs
            .get_asset(format!("{}_Running.png", name).as_str())?
            .clone();
        Ok(Self {
            name: name.to_string(),
            idle,
            broken,
            running,
        })
    }
}
