/// This struct describes the current change rate of the player's resources.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResourcesCR {
    pub(crate) oxygen: i16,
    pub(crate) energy: i16,
    pub(crate) life: i16,
}
