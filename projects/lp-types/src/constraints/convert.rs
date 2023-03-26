use super::*;

impl From<bool> for OptimizeDirection {
    fn from(value: bool) -> Self {
        match value {
            true => OptimizeDirection::Maximize,
            false => OptimizeDirection::Minimize,
        }
    }
}
