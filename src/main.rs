use conrod_core::{widget, Colorable, Positionable, Rect, Widget};
use conrod_glium::Renderer;
use conrod_winit::WinitWindow;
use conrod_core::image::MapTexture;
use std::io;
use rand::{Rng, thread_rng};
use glium::Surface;

fn main() {
    // Initialize the conrod GUI
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_title("Magic 8 Ball Game");
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut ui = conrod_core::UiBuilder::new([200.0, 200.0]).build();

    // Create a texture to display the Magic 8 Ball's response
    let mut image_map = MapTexture::new(&display, 128, 128).unwrap();
    let image_map_texture = image_map.texture();

    // Initialize the conrod renderer
    let mut renderer = Renderer::new(&display).unwrap();

    // Initialize the conrod window
    let mut window = WinitWindow::new().unwrap();
    let mut event_loop = window.event_loop();

    // Initialize the Magic 8 Ball's response
    let mut response = String::new();

    // Run the main GUI loop
    'main: loop {
        // Handle GUI events
        let mut events = Vec::new();
        event_loop.poll_events(|event| events.push(event));

        // Update the conrod UI
        let ui_cell = ui.set_widgets();

        // Add a widget to display the Magic 8 Ball's response
        widget::Text::new(response.as_str())
            .color(conrod_core::color::WHITE)
            .middle()
            .set(ui_cell.image_id, &mut image_map);

        // Render the conrod UI
        let primitives = ui.draw();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.fill(&display, primitives, &image_map);
        renderer.draw(&display, primitives, &image_map_texture);
        target.finish().unwrap();

        // Get the user's question
        println!("Please enter your question: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Update the Magic 8 Ball's response
        response = random_response();

        // Break the main loop if the user closes the window
        for event in events {
            if let glium::glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glium::glutin::WindowEvent::Closed => break 'main,
                    _ => (),
                }
            }
        }
    }
}

fn random_response() -> String {
    // Generate a random number between 0 and 3, which corresponds to one of the default answers.
    let mut rng = thread_rng();
    let index = rng.gen_range(0..4);
    match index {
        0 => "It is currently sunny where you are.",
        1 => "The future is uncertain, but I sense a strong possibility of financial success.",
        2 => "I cannot read minds, but I can tell you that your question has been answered by the Magic 8 Ball.",
        3 => "It is unclear whether the answer to this question will be positive or negative.",
        _ => unreachable!(),
    }
}