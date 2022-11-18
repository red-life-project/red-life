use ggez::graphics::Image;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaschineSprite {
    name: String,
    idel: String,    //temp img for later
    broken: String,  //temp img for later
    running: String, //temp img for later
}

impl Default for MaschineSprite {
    fn default() -> Self {
        Self {
            name: "Machiene ohne namen".to_string(),
            idel: "img".to_string(),
            broken: "img".to_string(),
            running: "img".to_string(),
        }
    }
}

impl MaschineSprite {
    pub fn new(name: String, idel: String, broken: String, running: String) -> Self {
        Self {
            name,
            idel,
            broken,
            running,
        }
    }
}
