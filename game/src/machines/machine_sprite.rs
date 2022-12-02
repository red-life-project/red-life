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
impl MachineSprite {
    pub fn new(gs: &GameState, name: &str) -> RLResult<Self> {
        //test_Broken.png
        info!("Creating new MachineSprite: name: {}", name);
        let broken;
        let idle;
        let running;
        /*        if let Ok(img)= gs.get_asset(format!("{name}.png").as_str())
        {
            broken = img.clone();
            idle = img.clone();
            running = img.clone();
        }*/
        //   else {
        broken = gs.get_asset(format!("{name}_Broken.png").as_str())?.clone();
        idle = gs.get_asset(format!("{name}_Idle.png").as_str())?.clone();
        running = gs
            .get_asset(format!("{name}_Running.png").as_str())?
            .clone();
        //    }

        Ok(Self {
            idle,
            broken,
            running,
        })
    }
}
