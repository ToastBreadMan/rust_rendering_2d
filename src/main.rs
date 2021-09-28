use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use models::{Rectangle::Rect, World::World, Circle::Circle, Polygon::Polygon, Vector::Vec2d};

fn main() {//use vector
    let WIDTH:u32 = 600;
    let HEIGHT:u32 = 600;
    let size = LogicalSize::new(WIDTH, HEIGHT);
    let event_loop = EventLoop::new();
    let mut world = World::new(&WIDTH, &HEIGHT, Box::new([0xff,0,0,0xff]));
    let window = WindowBuilder::new().with_inner_size(size).with_max_inner_size(size).build(&event_loop).unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };
    let rgba = [0,0xff,0,0xff];
    let mut a = Vec2d{x:200, y:500};
    let mut b = Vec2d{x:200, y:300};
    let mut c = Vec2d{x:600, y:200};
    let d = Vec2d{x:400, y:600};
    let mut Polygon:Polygon = Polygon::new(vec![a,b,c,d], Box::new(rgba), HEIGHT as usize);
    world.add(Box::new(Polygon));
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        if let Event::RedrawRequested(_) = event{ 
            a.x += 1;
            world.update(pixels.get_frame());
            let new_rgba = [0,0xff,0xff,0xff];
            let cirlce:Circle = Circle::new(a, 50.0, Box::new(new_rgba));
            world.change(0, Box::new(cirlce));
            pixels.render().unwrap();
        }
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event:WindowEvent::KeyboardInput { input, .. },
                window_id
            } => {
                println!("{:?}", input.virtual_keycode.unwrap() == winit::event::VirtualKeyCode::W)
            }
            _ => (),
        }
        window.request_redraw();
    });
}  