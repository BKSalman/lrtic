#![no_std]
#![no_main]

use rtic::app;

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
        (Shared {}, Local {})
    }
}
