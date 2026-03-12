use winit::{
    dpi::PhysicalSize, event::{Event, WindowEvent}, event_loop::{self, EventLoop}, window::{self, WindowBuilder}
};


fn main() {
    env_logger::init();


    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
    .with_title("Particle Graphics")
    .with_inner_size(PhysicalSize::new(900, 900))
    .with_resizable(false)
    .build(&event_loop)
    .unwrap();

    let _ = event_loop.run(move |event, target|{
        match event {
           Event::WindowEvent { window_id, event }
           if window_id == window.id() => match event {
            WindowEvent::CloseRequested => target.exit(),

            WindowEvent::RedrawRequested => {
                // Graphics logic here 
            }

            _ => {}
           }

           Event::AboutToWait => {
            window.request_redraw();
           }

           _ => {}
        };
    });
}