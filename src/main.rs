use windows::Gaming::Input::RawGameController;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => control_flow.set_exit(),
        Event::MainEventsCleared => {
            print_gamepads();
            window.request_redraw();
        }
        _ => (),
    });
}

fn print_gamepads() {
    let gamepads = match RawGameController::RawGameControllers() {
        Ok(gamepads) => gamepads,
        Err(e) => panic!("Error while fetching gamepads {e}"),
    };

    match gamepads.Size() {
        Ok(0) => println!("No Gamepads found"),
        _ => {
            for controller in gamepads {
                let name = match controller.DisplayName() {
                    Ok(hstring) => hstring.to_string_lossy(),
                    Err(_) => "unknown".to_string(),
                };
                println!("Found controller: {name}");
            }
        }
    }
}
