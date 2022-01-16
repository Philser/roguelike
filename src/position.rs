use pathfinding::num_traits::{abs, Float};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    /// Gets the airline distance between two points using pythagoras' theorem
    pub fn get_airline_distance(&self, dest: &Position) -> i32 {
        let mid_vertex = Position::new(self.x, dest.y);

        let a = abs(self.y - mid_vertex.y);
        let b = abs(mid_vertex.x - dest.x);

        ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32
    }

    pub fn is_adjacent_to(&self, other_pos: &Position) -> bool {
        if self == other_pos {
            return false;
        }

        let dist_x = (self.x - other_pos.x).abs();
        let dist_y = (self.y - other_pos.y).abs();

        if dist_x < 2 && dist_y < 2 {
            return true;
        }

        false
    }
}

#[test]
fn can_get_airline_distance() {
    let a = Position::new(1, 1);
    let b = Position::new(2, 2);
    let c = Position::new(3, 3);
    let d = Position::new(4, 3);

    assert_eq!(a.get_airline_distance(&b), 1);
    assert_eq!(b.get_airline_distance(&a), 1);

    assert_eq!(a.get_airline_distance(&c), 2);
    assert_eq!(c.get_airline_distance(&a), 2);

    assert_eq!(a.get_airline_distance(&d), 3);
}

#[test]
fn can_determine_adjacents() {
    let point = Position::new(0, 0);

    let adjacent_right = Position::new(1, 0);
    let adjacent_down = Position::new(0, 1);
    let adjacent_diagonal = Position::new(1, 1);

    assert!(point.is_adjacent_to(&adjacent_right));
    assert!(point.is_adjacent_to(&adjacent_down));
    assert!(point.is_adjacent_to(&adjacent_diagonal));

    let too_far_right = Position::new(2, 0);
    let too_far_down = Position::new(0, 2);
    let too_far_vertical = Position::new(2, 2);
    let too_far_whereever = Position::new(1, 2);

    assert!(!point.is_adjacent_to(&too_far_right));
    assert!(!point.is_adjacent_to(&too_far_down));
    assert!(!point.is_adjacent_to(&too_far_vertical));
    assert!(!point.is_adjacent_to(&too_far_whereever));
}
