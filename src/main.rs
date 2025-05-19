#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        defmt::info!("Hello");
    }
}
