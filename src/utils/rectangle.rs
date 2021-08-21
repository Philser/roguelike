pub struct Rectangle {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
        Rectangle {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        }
    }

    pub fn intersects(&self, other_rect: &Rectangle) -> bool {
        self.x1 <= other_rect.x2
            && self.x2 >= other_rect.x1
            && self.y1 >= other_rect.y2
            && self.y2 <= other_rect.y1
    }

    pub fn get_center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
