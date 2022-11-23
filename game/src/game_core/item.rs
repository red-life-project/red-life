use serde::{Deserialize, Serialize};
pub const GEDRUCKTESTEIL: [&str; 3] = [
    "3D-gedrucktes-Teil",
    "Ein 3D-gedrucktes-Teil, welches zur Reparatur des Kommunikationsmoduls verwendet werden kann",
    "3D-gedrucktes-Teil.png",
];
pub const SUPER_GLUE: [&str; 3] = [
    "SuperGlue",
    "SuperGlue kann zur Reparatur der Maschinen oder Löcher verwendet werden",
    "SuperGlue.png",
];
pub const BENZIN: [&str; 3] = [
    "Benzin",
    "Benzin kann mit dem Notstromgenerator verwendet werden um Strom zu generieren",
    "Benzin.png",
];
/// Defines an item in the inventory of the player
/// Contains the name of the item, information about the item and the image
#[derive(Clone, Default, Eq, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    info_text: String,
    img: String,
}
impl Item {
    pub(crate) fn new(item: [&str; 3]) -> Self {
        Self {
            name: item[0].to_string(),
            info_text: item[1].to_string(),
            img: item[2].to_string(),
        }
    }
}
