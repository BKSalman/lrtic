#![no_std]
#![no_main]

mod pac {
    pub const NVIC_PRIO_BITS: u8 = 2;
    pub use mcx_pac::*;
}

use {defmt_rtt as _, panic_probe as _};

#[rtic::app(device = crate::pac, peripherals = false)]
mod app {
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let gpio_version_id = unsafe { crate::pac::gpio::GPIO0::instance().regs().VERID().read() };
        defmt::info!("{}", gpio_version_id);
        (Shared {}, Local {})
    }
}
