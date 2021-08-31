use pathfinding::num_traits::abs;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        return Position { x, y };
    }

    /// Gets the airline distance between two points using pythagoras' theorem
    pub fn get_airline_distance(&self, dest: &Position) -> i32 {
        let mid_vertex = Position::new(self.x, dest.y);

        let a = abs(self.y - mid_vertex.y);
        let b = abs(mid_vertex.x - dest.x);

        ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32
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
