use crate::machines::machine::State;
use ggez::graphics::Image;

#[derive(Debug, Clone)]
/// Contains all the sprites associated with one Machine
pub struct MachineSprite(Vec<Image>);
impl From<&[Image]> for MachineSprite {
    fn from(value: &[Image]) -> Self {
        Self(value.to_vec())
    }
}

impl MachineSprite {
    ///Grabs the correct Image depending on the passed in State
    pub fn get(&self, state: State) -> &Image {
        self.0.get(state as usize).unwrap_or(&self.0[0])
    }
}
