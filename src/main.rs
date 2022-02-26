#[macro_use]
extern crate rocket;
//extern crate rocket_dyn_templates;

use rocket_dyn_templates::Template;

use std::collections::HashMap;

use gpio_cdev::{Chip, LineRequestFlags};
use std::thread::sleep;
use std::time::{Duration, Instant};

enum LedState {
    On,
    Off,
}

fn led_toggle(led_state: LedState) -> std::result::Result<(), gpio_cdev::Error> {
    let mut chip = Chip::new("/dev/gpiochip0")?;

    let handle = chip
        .get_line(20)?
        .request(LineRequestFlags::OUTPUT, 0, "blinky")?;
    
    match led_state {
        LedState::On => handle.set_value(1)?,
        LedState::Off => handle.set_value(0)?,
    }

    Ok(())
}

#[get("/on")]
fn led_on() {
    led_toggle(LedState::On);
}
#[get("/off")]
fn led_off() {
    led_toggle(LedState::Off);
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();

    context.insert("", "");

    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/led", routes![led_on, led_off])
}
