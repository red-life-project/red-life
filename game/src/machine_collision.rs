use ggez::{event, Context};
use ggez::graphics::{self, Color, DrawParam, Mesh, Rect};

fn board_collision() -> bool {
    player_pos = (40, 30);
    board_pos = vec![0,0];
        if player_pos == board.pos {
            return true;
        }
    false
}