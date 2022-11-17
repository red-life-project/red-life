use ggez::graphics::{Image};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaschineSprite {
    name:String,
    idel:Image,
    broken: Image,
    running: Image,
}

impl Default for MaschineSprite{
    fn default() -> Self {
        Self{
            name:  String::default("Machiene ohne namen"),
            idel: Image(),
            broken: Image(),
            running: Image()
        }
    }
}

impl MaschineSprite {

    pub fn name(&self) -> String {
        self.name
    }
    pub fn new(name: String, idel: Image, broken: Image, running: Image) -> Self {
        Self { name, idel, broken, running }
    }

}

