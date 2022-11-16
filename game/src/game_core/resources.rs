use std::ops;
use serde::{Serialize, Deserialize};
/// This struct holds data for resources
/// This is used to describe the current state and change rate of the player's resources.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resources <T> {
    pub(crate) oxygen: T,
    pub(crate) energy: T,
    pub(crate) life: T,
}

impl <T> IntoIterator for Resources <T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.oxygen, self.energy, self.life].into_iter()
    }
}

impl <T: std::ops::Add<Output = T>> ops::Add<Resources <T>> for Resources <T>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            oxygen: self.oxygen + rhs.oxygen,
            energy: self.energy + rhs.energy,
            life: self.life + rhs.life,
        }
    }
}

impl <T: std::ops::Sub<Output = T>> ops::Sub<Resources<T>> for Resources<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            oxygen: self.oxygen - rhs.oxygen,
            energy: self.energy - rhs.energy,
            life: self.life - rhs.life,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game_core::resources::Resources;

    #[test]
    fn addition() {
        let a = Resources {
            oxygen: 1,
            energy: 2,
            life: 3,
        };
        let b = Resources {
            oxygen: 4,
            energy: 5,
            life: 6,
        };
        let add_result = a + b;
        let sub_result = a - b;
        let add_controll = Resources {
            oxygen: -3,
            energy: -3,
            life: -3,
        };

        let sub_controll = Resources {
            oxygen: 5,
            energy: 7,
            life: 9,
        };
        assert_eq!(add_result, add_controll);
        assert_eq!(sub_result, sub_controll)
    }

    #[test]
    fn into_it() {
        let a = Resources {
            oxygen: 3,
            energy: 2,
            life: 1,
        };
        let mut ait = a.into_iter();
        assert_eq!(ait.next().unwrap(), 3);
        assert_eq!(ait.next().unwrap(), 2);
        assert_eq!(ait.next().unwrap(), 1);
        assert_eq!(Some(ait.next()), Some(None))
    }
}
