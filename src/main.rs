use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::COPY_DEPTH_FROM_PARENT;

pub fn initialize_x11(connection: &impl Connection, screen_num: usize) -> Result<(), Box<dyn std::error::Error>> {
    let screen = &connection.setup().roots[screen_num];
    let window_id = connection.generate_id()?;

    connection.create_window(
        COPY_DEPTH_FROM_PARENT,
        window_id,
        screen.root,
        0,
        0,
        100,
        100,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel))?;

    connection.map_window(window_id)?;
    Ok(connection.flush()?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (connection, screen_num) = x11rb::connect(None).unwrap();
    initialize_x11(&connection, screen_num)?;

    loop {
        println!("Event: {:?}", connection.wait_for_event()?);
    }

    println!("Program completed successfully.");
}
