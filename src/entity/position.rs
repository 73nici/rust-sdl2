#[derive(PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub(crate) fn new(x: Option<i32>, y: Option<i32>) -> Self {
        Position {
            x: if x.is_some() { x.unwrap() } else { 0 },
            y: if y.is_some() { y.unwrap() } else { 0 },
        }
    }

    pub(crate) fn check_collision(&self, position: &Position, ) -> bool {

        if (self.x <= position.x + 100)
            && (self.x + 100 > position.x)
            && (self.y <= position.y + 100)
            && (self.y + 100 > position.y)
        {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use crate::entity::position::Position;

    #[test]
    fn create_position() {
        let position_without_values = Position::new(None, None);
        assert_eq!(position_without_values.x, 0);
        assert_eq!(position_without_values.y, 0);

        let position_with_values = Position::new(Some(10), Some(11));
        assert_eq!(position_with_values.x, 10);
        assert_eq!(position_with_values.y, 11);
    }

    #[test]
    fn check_collision() {
        let position = Position::new(Some(100), Some(100));
        let second_position = Position::new(Some(1), Some(2));
        // position
    }
}
