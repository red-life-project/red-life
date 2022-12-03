use crate::backend::gamestate::GameState;

use ggez::graphics::Image;

use crate::RLResult;
use std::fs;
use tracing::info;

#[derive(Debug, Clone)]
pub struct MachineSprite {
    pub idle: Image,
    pub broken: Image,
    pub running: Image,
}

impl From<&[Image]> for MachineSprite {
    fn from(value: &[Image]) -> Self {
        Self {
            idle: value[0].clone(),
            broken: value[1].clone(),
            running: value[2].clone(),
        }
    }
}
/*
impl MachineSprite {

    pub fn new(images: &[Image; 3]) -> Self {
        Self {
            idle: images[0].clone(),
            broken: images[1].clone(),
            running: images[2].clone(),
        }
    }

    // IN Process of removing this function do not use
    /* fn new(gs: &GameState, name: &str) -> RLResult<Self> {
        //test_Broken.png
        info!("Creating new MachineSprite: name: {}", name);


        let broken = gs.get_asset(format!("{name}_Broken.png").as_str())?.clone();
        let idle = gs.get_asset(format!("{name}_Idle.png").as_str())?.clone();
        let running = gs
            .get_asset(format!("{name}_Running.png").as_str())?
            .clone();
        //    }

        Ok(Self {
            idle,
            broken,
            running,
        })
    }*/
}
*/
