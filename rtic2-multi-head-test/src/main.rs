#![no_main]
#![no_std]

use panic_halt as _;

include!(concat!(env!("OUT_DIR"), "/", env!("RTIC_GENERATED_APP")));
