extern crate blinkt;
extern crate toml;

use std::env;
use std::fs;
use std::path::Path;
use color_processing::Color;
use blinkt::Blinkt;
use serde_derive::Deserialize;

struct Settings {
    data_pin: u8,
    clock_pin: u8,
    debug: bool,
}

#[derive(Deserialize)]
struct LedsConfig {
    data_pin: u8,
    clock_pin: u8,
}


fn setup() -> Settings {
    let mut settings: Settings = Settings { data_pin: 14, clock_pin: 15, debug: false};
    let config_filename = Path::new(".leds");
    if config_filename.exists() {
        let config_data = fs::read_to_string(".leds")
            .expect("Failed to read config file .leds.");
        let cfg: LedsConfig = toml::from_str(&config_data.to_string())
            .expect("Error in parsing .leds config. Make sure it is in TOML format and contains only\n    data_pin = <value>\n    clock_pin = <value>\n");
        settings.data_pin = cfg.data_pin;
        settings.clock_pin = cfg.clock_pin;
    }
    match env::var("BLINK_DATA") {
        Ok(val) => match val.parse::<u8>() {
            Ok(num) => settings.data_pin = num,
            Err(_e) => println!("Could not parse BLINK_DATA={} to u8.", val),
        },
        Err(_e) => (),
    }
    match env::var("BLINK_CLOCK") {
        Ok(val) => match val.parse::<u8>() {
            Ok(num) => settings.clock_pin = num,
            Err(_e) => println!("Could not parse BLINK_CLOCK={} to u8.", val),
        },
        Err(_e) => (),
    }
    match env::var("BLINK_DEBUG") {
        Ok(_val) => settings.debug = true,
        Err(_e) => (),
    }
    // println!("Settings {{data: {}, clk: {}}}", settings.data_pin, settings.clock_pin);
    return settings;
}

fn main() {
    let settings = setup();
	let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!(r#"
    USAGE:
        leds <led1-color> [<led2-color> [<led3-color> [...]]]

        ... the colors can be in web format '#aabbcc' or simplified '#abc' or even without hash
            'aabbcc' or a named color like 'red'

        ... to setup data and clock pin set environment variables BLINK_DATA and BLINK_CLOCK or
            create '.leds' configuration file in TOML format.

            .leds file example:
            > data_pin = 13
            > clock_pin = 15
        "#);
        return;
    }
	let strip_size = args.len() - 1;	
	let mut blinkt;
    match Blinkt::with_settings(settings.data_pin, settings.clock_pin, strip_size) {
        Ok(val) => blinkt = val,
        Err(e) => panic!("Error loading smart led modul on pins data: {}, clock: {}: {}", settings.data_pin, settings.clock_pin, e),
    }
    if settings.debug {println!("Data pin: {}, clock pin: {}", settings.data_pin, settings.clock_pin)}
	blinkt.set_clear_on_drop(false);
	let mut n:usize = 0;
	for arg in &args[1..] {
        match Color::new_string(arg) {
            Some(color) => {
                blinkt.set_pixel(n, color.red, color.green, color.blue);
                if settings.debug { println!("r:{} g:{} b:{}", color.red, color.green, color.blue);}
            },
            None => println!("Could not parse color {}.", arg),
        }
		n += 1;
	}
	blinkt.show()
        .expect("Error loading communicating with leds.");

    // }
}

