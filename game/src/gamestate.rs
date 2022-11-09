use crate::screen::StackCommand;
use crate::utils::get_scale;
use crate::{screen::Screen, RedResult};
use ggez::graphics::Rect;
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
struct Item {
    name: String,
    info_text: String,
    //image should be a texture, didnt work yet
    img: String,
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Player {
    position: (usize, usize),
    inventory: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    player: Player,
    milestone: usize,
    machines: Vec<(Rect)>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player: Player::default(),
            milestone: 0,
            machines: vec![],
        }
    }
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
    // Game collision detection
    fn machine_collision_detection(&self, next_player_pos: (usize, usize)) -> bool {
        for machine in &self.machines {
            if (machine.x as usize..=(machine.x as usize + machine.w as usize))
                .contains(&next_player_pos.0)
                && (machine.y as usize..=(machine.y as usize + machine.h as usize))
                    .contains(&next_player_pos.1)
            {
                return true;
            }
        }
        false
    }
    fn border_collision_detection(next_player_pos: (usize, usize)) -> bool {
        next_player_pos.0 >= 1879 ||next_player_pos.1 >= 1030
    }
    fn collision_detection(&self, next_player_pos: (usize, usize)) -> bool {
        self.machine_collision_detection(next_player_pos) || Self::border_collision_detection(next_player_pos)
    }
}

impl Screen for GameState {
    fn update(&mut self, ctx: &mut Context) -> RedResult<StackCommand> {
        self.tick();
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            match key {
                VirtualKeyCode::Escape => {
                    // TODO: Save the game
                    return Ok(StackCommand::Pop);
                }
                VirtualKeyCode::W => {
                    if !self.collision_detection((self.player.position.0, self.player.position.1.saturating_sub(5))) {
                        self.player.position.1 = self.player.position.1.saturating_sub(5);
                    }
                }
                VirtualKeyCode::A => {
                    if !self.collision_detection((self.player.position.0.saturating_sub(5), self.player.position.1)) {
                        self.player.position.0 = self.player.position.0.saturating_sub(5);
                    }
                }
                VirtualKeyCode::S => {
                    if !self.collision_detection((self.player.position.0, self.player.position.1.saturating_add(5)))
                    {
                        self.player.position.1 = self.player.position.1.saturating_add(5);
                    }
                }
                VirtualKeyCode::D => {
                    if !self.collision_detection((self.player.position.0.saturating_add(5), self.player.position.1))
                    {
                        self.player.position.0 = self.player.position.0.saturating_add(5);
                    }
                }
                key => {
                    dbg!("{:?}", key);
                }
            }
        }
        Ok(StackCommand::None)
    }

    fn draw(&self, ctx: &mut Context) -> RedResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        // TODO: After asset loading we can do this again: let background = graphics::Image::from_bytes(ctx, include_bytes!("../../assets/basis.png"))?;
        //canvas.draw(&background, graphics::DrawParam::default().scale(scale));
        let player = graphics::Image::from_bytes(ctx, include_bytes!("../../assets/player.png"))?;
        canvas.draw(
            &player,
            graphics::DrawParam::default()
                .scale(scale)
                .dest([self.player.position.0 as f32, self.player.position.1 as f32]),
        );
        canvas.finish(ctx)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gamestate() {
        let gamestate = GameState::default();
    }

    #[test]
    fn test_save_game_state() {
        let gamestate = GameState::default();
        GameState::save_game_state(&gamestate);
    }

    #[test]
    fn test_load_game_state() {
        let gamestate = GameState::load_game_state().unwrap();
    }
}
