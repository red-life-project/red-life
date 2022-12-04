use ggez::graphics::Image;

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
