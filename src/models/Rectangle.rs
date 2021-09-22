pub struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    width: usize,
    height: usize
}

impl Rect {
    pub fn new(x1: usize, y1: usize, x2:usize, y2:usize, width:usize, height:usize)->Rect{
        Rect {x1, y1, x2, y2, width, height}
    }
    pub fn draw(&self, frame:&mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate(){
            let x = i % self.width;
            let y = i / self.height;
            let in_rect: bool = {
                 x < self.x2 && x > self.x1 && y > self.y1 && y < self.y2 
            };
            let rgba = [0,0,255,0xff];
            if in_rect {pixel.copy_from_slice(&rgba)};
        }
    }
}
