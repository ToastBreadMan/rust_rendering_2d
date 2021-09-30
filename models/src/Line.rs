use crate::World::Funnel;
use crate::Vector::Vec2d;

pub struct Line {
    points: Vec<Vec2d>,
    color:Box<[u8]>,
    min_x:isize,
    min_y:isize,
    max_x:isize,
    max_y:isize
}

impl Line {
    pub fn new(points: Vec<Vec2d>, color:Box<[u8]>) -> Line {
        let mut  min_x = points[0].x;
        let mut max_x = points[0].x;
        let mut min_y = points[0].y;
        let mut max_y = points[0].y;

        for i in 0..points.len() {
            if points.get(i).unwrap().x > max_x {
                max_x = points.get(i).unwrap().x
            }
            else if points.get(i).unwrap().x < min_x {
                min_x = points.get(i).unwrap().x
            }

            if points.get(i).unwrap().y > max_y {
                max_y = points.get(i).unwrap().y
            }
            else if points.get(i).unwrap().y < min_y{
                min_y = points.get(i).unwrap().y
            }
        }
        Line {points, color, min_x, max_x, min_y,max_y}
    }
}

impl Funnel for Line {
    fn draw(&self, x: usize, y: usize) -> Option<Box<[u8]>> {
            let line_dist = ((self.points.get(0).unwrap().x - self.points.get(1).unwrap().x).pow(2) + (self.points.get(0).unwrap().y - self.points.get(1).unwrap().y).pow(2))as f32;
            let dist = line_dist.sqrt();
            let proc_dist_1 = ((self.points.get(1).unwrap().x - x as isize).pow(2) + (self.points.get(1).unwrap().y - y as isize).pow(2))as f32;
            let dist_1 = proc_dist_1.sqrt();
            let proc_dist_2 = ((self.points.get(0).unwrap().x - x as isize).pow(2) + (self.points.get(0).unwrap().y - y as isize).pow(2))as f32;
            let dist_2 = proc_dist_2.sqrt();
            if dist_1+dist_2 == dist {
                Some(self.color.clone())
            }
            else {
                None
            }
    }
}