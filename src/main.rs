use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use models::{Rectangle::Rect, World::World, Circle::Circle, Triangle::Triangle, Vector::Vec2d};

fn main() {//use vector
    let WIDTH:u32 = 600;
    let HEIGHT:u32 = 600;
    let size = LogicalSize::new(WIDTH, HEIGHT);
    let event_loop = EventLoop::new();
    let mut world = World::new(&WIDTH, &HEIGHT);
    let window = WindowBuilder::new().with_inner_size(size).with_max_inner_size(size).build(&event_loop).unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };
    let rgba = [0,0xff,0,0xff];
    let mut a = Vec2d{x:200, y:500};
    let mut b = Vec2d{x:400, y:400};
    let mut c = Vec2d{x:500, y:200};
    let mut triangle:Triangle = Triangle::new(vec![a,b,c], Box::new(rgba), WIDTH as usize);//everything has to happen after closure 
    let mut circle:Circle = Circle::new(Vec2d{x:200, y:500}, 10.0, Box::new([0xff,0xff,0,0xff]));
    world.add(Box::new(circle));
    world.add(Box::new(triangle));
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::RedrawRequested(_) = event{ 
            world.update(pixels.get_frame());
            pixels.render().unwrap();
        }
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
        window.request_redraw();
    });
}  