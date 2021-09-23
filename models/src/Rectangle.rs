use crate::World::Funnel;

pub struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    width: usize,
    height: usize
}

impl Funnel for  Rect {
    fn draw(&self, x: usize, y:usize)->(bool, Box<[u8]>) {
            let in_rect: bool = {
                 x < self.x2 && x > self.x1 && y > self.y1 && y < self.y2 
            };
            let rgba = [0xff, 0,0,0xff];
            (in_rect, Box::new(rgba))
    }
}

impl Rect {
    pub fn new(x1: usize, y1: usize, x2:usize, y2:usize, width:usize, height:usize)->Rect{
        Rect {x1, y1, x2, y2, width, height}
    }
} 
