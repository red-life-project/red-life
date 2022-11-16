use std::ops;

/// This struct holds data for resources
/// This is used to describe the current state and change rate of the player's resources.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Resources {
    pub(crate) oxygen: i16,
    pub(crate) energy: i16,
    pub(crate) life: i16,
}

impl IntoIterator for Resources {
    type Item = i16;
    type IntoIter = std::array::IntoIter<i16, 3>;

    fn into_iter(self) -> Self::IntoIter {
       [self.oxygen, self.energy, self.life].into_iter()
    }
}

impl ops::Add<Resources> for Resources
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            oxygen: self.oxygen + rhs.oxygen,
            energy: self.energy + rhs.energy,
            life: self.life + rhs.life,
        }
    }
}

impl ops::Sub<Resources> for Resources {
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
    fn addition()
    {
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
        let c = a + b;
        let d = Resources {
            oxygen: 5,
            energy: 7,
            life: 9,
        };
        assert_eq!(c, d)
    }

    #[test]
    fn into_it()
    {
        let a = Resources {
            oxygen: 3,
            energy: 2,
            life: 1,
        };
        let mut ait = a.into_iter();
        assert_eq!(ait.next().unwrap(),3 ) ;
        assert_eq!(ait.next().unwrap() ,2) ;
        assert_eq!(ait.next().unwrap() ,1) ;
        assert_eq!(Some(ait.next()),Some(None))
    }
}

