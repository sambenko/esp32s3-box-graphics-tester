use embedded_graphics_simulator::{ OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window };

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{ Size, RgbColor };
use embedded_graphics::draw_target::DrawTarget;

//paste your packages here (only those unrelated to chip initialisation and those that are not here already):



// end of packages

fn main() -> Result<(), std::convert::Infallible>  {

    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("ESP32-S3-BOX", &output_settings);
    display.clear(Rgb565::WHITE)?;

    //paste all the code after initialisation here (usually everything after you initialize the display):
    


    // end of source code

    'running: loop {
        window.update(&display);
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
    }
}
