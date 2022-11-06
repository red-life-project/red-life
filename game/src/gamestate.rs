use crate::screen::StackCommand;
use crate::utils::get_scale;
use crate::{screen::Screen, RedResult};
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
struct Item;

#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    inventory: Vec<Item>,
    milestone: usize,
}

impl GameState {
    pub fn tick(&mut self) {
        // do stuff
        self.milestone += 1;
    }

    fn save_game_state(game_state: &GameState) -> RedResult {
        let save_data = serde_yaml::to_string(game_state)?;
        std::fs::write("../saves/savegame.yaml", save_data)?;
        Ok(())
    }

    fn load_game_state() -> RedResult<GameState> {
        let save_data = std::fs::read_to_string("../saves/savegame.yaml")?;
        let game_state = serde_yaml::from_str(&save_data)?;
        Ok(game_state)
    }
}

impl Screen for GameState {
    fn update(&mut self, ctx: &mut Context) -> RedResult<StackCommand> {
        self.tick();
        Ok(StackCommand::None)
    }
    fn draw(&self, ctx: &mut Context) -> RedResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let background =
            graphics::Image::from_bytes(ctx, include_bytes!("../../assets/basis.png"))?;
        canvas.draw(&background, graphics::DrawParam::default().scale(scale));
        canvas.finish(ctx)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gamestate() {
        let mut gamestate = GameState::default();
    }

    #[test]
    fn test_save_game_state_yaml() {
        let gamestate = GameState::default();
        GameState::save_game_state(&gamestate);
    }

    #[test]
    fn test_load_game_state_yaml() {
        let gamestate = GameState::load_game_state().unwrap();
    }
}
