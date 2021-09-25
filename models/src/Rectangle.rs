use crate::World::Funnel;

pub struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: Box<[u8]>
}

impl Funnel for Rect {
    fn draw(&self, x: usize, y:usize)->(bool, Box<[u8]>) {
            let in_rect: bool = {
                 x < self.x2 && x > self.x1 && y > self.y1 && y < self.y2 
            };
            (in_rect, self.color.clone())
    }
}

impl Rect {
    pub fn new(x1: usize, y1: usize, x2:usize, y2:usize, color:Box<[u8]>)->Rect{
        Rect {x1, y1, x2, y2, color}
    }
} 
