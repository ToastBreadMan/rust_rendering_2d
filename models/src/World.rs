use crate::Rectangle::Rect;

pub trait Funnel{
    fn draw(&self, x:usize, y:usize)->(bool, Box<[u8]>);
}

pub struct World {
    width:usize,
    height:usize,
    pub world:Vec<Box<Funnel>>
}

impl World {
    pub fn new(width:&u32, height:&u32)->World{
        let proc_width = *width as usize;
        let proc_height = *height as usize;
        World {width:proc_width, height:proc_height, world:Vec::new()}
    }
    pub fn add(&mut self, object:Box<Funnel>){
        self.world.push(object);   
    }

    pub fn change(&mut self, index: usize, object:Box<Funnel>)->Box<Funnel> {
        let val = std::mem::replace(&mut self.world[index], object);
        val
    }
    pub fn update(&self, frame:&mut [u8]){
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % self.width;
            let y = i / self.height;
            for (i,object) in self.world.iter().enumerate() {
                let drawer = object.draw(x,y);
                if drawer.0 {
                    let rgba = drawer.1;
                    pixel.copy_from_slice(&rgba);   
                }
            }
        }
    }
}