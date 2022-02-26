// use gpio_cdev::*;

// fn main() {
//     let chip_iterator = match chips() {
//         Ok(chips) => chips,
//         Err(e) => {
//             println!("Failed to get chip iterator: {:?}", e);
//             return;
//         }
//     };

//     for chip in chip_iterator {
//         if let Ok(chip) = chip {
//             println!(
//                 "GPIO chip: {}, \"{}\", \"{}\", {} GPIO Lines",
//                 chip.path().to_string_lossy(),
//                 chip.name(),
//                 chip.label(),
//                 chip.num_lines()
//             );
//             for line in chip.lines() {
//                 match line.info() {
//                     Ok(info) => {
//                         let mut flags = vec![];

//                         if info.is_kernel() {
//                             flags.push("kernel");
//                         }

//                         if info.direction() == LineDirection::Out {
//                             flags.push("output");
//                         }

//                         if info.is_active_low() {
//                             flags.push("active-low");
//                         }
//                         if info.is_open_drain() {
//                             flags.push("open-drain");
//                         }
//                         if info.is_open_source() {
//                             flags.push("open-source");
//                         }

//                         let usage = if !flags.is_empty() {
//                             format!("[{}]", flags.join(" "))
//                         } else {
//                             "".to_owned()
//                         };

//                         println!(
//                             "\tline {lineno:>3}: {name} {consumer} {usage}",
//                             lineno = info.line().offset(),
//                             name = info.name().unwrap_or("unused"),
//                             consumer = info.consumer().unwrap_or("unused"),
//                             usage = usage,
//                         );
//                     }
//                     Err(e) => println!("\tError getting line info: {:?}", e),
//                 }
//             }
//             println!();
//         }
//     }
// }

// use gpio_cdev::{Chip, LineRequestFlags};
// use quicli::prelude::*;
// use std::thread::sleep;
// use std::time::{Duration, Instant};
// use structopt::StructOpt;

// #[derive(Debug, StructOpt)]
// struct Cli {
//     /// The gpiochip device (e.g. /dev/gpiochip0)
//     chip: String,
//     /// The offset of the GPIO line for the provided chip
//     line: u32,
//     /// Period in milliseconds
//     period_ms: u64,
//     /// Duration over which to blink in milliseconds
//     duration_ms: u64,
// }

// fn do_main(args: Cli) -> std::result::Result<(), gpio_cdev::Error> {
//     let mut chip = Chip::new(args.chip)?;

//     // NOTE: we set the default value to the desired state so
//     // setting it separately is not required
//     let handle = chip
//         .get_line(args.line)?
//         .request(LineRequestFlags::OUTPUT, 1, "blinky")?;

//     let duration = Duration::from_millis(args.duration_ms);
//     let start_time = Instant::now();
//     while start_time.elapsed() < duration {
//         sleep(Duration::from_millis(args.period_ms));
//         handle.set_value(0)?;
//         sleep(Duration::from_millis(args.period_ms));
//         handle.set_value(1)?;
//     }

//     Ok(())
// }

// fn main() -> CliResult {
//     let args = Cli::from_args();
//     do_main(args).or_else(|e| {
//         error!("{:?}", e);
//         Ok(())
//     })
// }

#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;

use gpio_cdev::{Chip, LineRequestFlags};
use std::thread::sleep;
use std::time::{Duration, Instant};

enum LedState {
    On,
    Off,
}

fn do_main() -> std::result::Result<(), gpio_cdev::Error> {
    let mut chip = Chip::new("/dev/gpiochip0")?;

    // NOTE: we set the default value to the desired state so
    // setting it separately is not required
    let handle = chip
        .get_line(20)?
        .request(LineRequestFlags::OUTPUT, 0, "blinky")?;

    let duration = Duration::from_millis(10000);
    let start_time = Instant::now();
    while start_time.elapsed() < duration {
        sleep(Duration::from_millis(500));
        handle.set_value(0)?;
        sleep(Duration::from_millis(500));
        handle.set_value(1)?;
    }

    Ok(())
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
    // match do_main() {
    //     Ok(_) => "Ok",
    //     Err(e) => "",
    // }
    //"Hello, world!"
    led_toggle(LedState::On);
}
#[get("/off")]
fn led_off() {
    // match do_main() {
    //     Ok(_) => "Ok",
    //     Err(e) => "",
    // }
    //"Hello, world!"
    led_toggle(LedState::Off);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![led_on, led_off])
}
