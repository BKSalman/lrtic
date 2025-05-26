#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    unsafe {
        // reset gpio
        mcx_pac::syscon::SYSCON0::instance()
            .regs()
            .PRESETCTRL0()
            .modify(|r| {
                r.set_PORT0_RST(true);
                r.set_GPIO0_RST(true);
            });

        mcx_pac::syscon::SYSCON0::instance()
            .regs()
            .PRESETCTRL0()
            .modify(|r| {
                r.set_PORT0_RST(false);
                r.set_GPIO0_RST(false);
            });

        // enable GPIO0 clock
        mcx_pac::syscon::SYSCON0::instance()
            .regs()
            .AHBCLKCTRL0()
            .modify(|ahb_ctrl0| {
                ahb_ctrl0.set_GPIO0(true);
                ahb_ctrl0.set_PORT0(true);
            });

        mcx_pac::port::PORT0::instance()
            .regs()
            .PCR(10)
            .modify(|pcr| {
                pcr.set_MUX(0);
                pcr.set_LK(true);

                defmt::debug!("write: {}", pcr);
            });

        // set pin 10 in port 0 direction to output
        mcx_pac::gpio::GPIO0::instance().regs().PDDR().modify(|r| {
            r.set_PDD(10, true);
        });

        loop {
            // set pin 10 in port 0 to HIGH
            mcx_pac::gpio::GPIO0::instance()
                .regs()
                .PSOR()
                .write(|gpio| {
                    gpio.set_PTSO(10, true);
                });

            defmt::info!("off");

            for _ in 0..200_000 {
                cortex_m::asm::nop();
            }

            // set pin 10 in port 0 to LOW
            mcx_pac::gpio::GPIO0::instance()
                .regs()
                .PCOR()
                .write(|gpio| {
                    gpio.set_PTCO(10, true);
                });

            defmt::info!("on");

            for _ in 0..200_000 {
                cortex_m::asm::nop();
            }
        }
    }
}
