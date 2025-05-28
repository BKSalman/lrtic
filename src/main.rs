#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use mcx_pac::interrupt;
static FLAG_BTN_PRESSED: AtomicBool = AtomicBool::new(false);

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
                r.set_PORT1_RST(true);
                r.set_GPIO1_RST(true);
            });

        mcx_pac::syscon::SYSCON0::instance()
            .regs()
            .PRESETCTRL0()
            .modify(|r| {
                r.set_PORT0_RST(false);
                r.set_GPIO0_RST(false);
                r.set_PORT1_RST(false);
                r.set_GPIO1_RST(false);
            });

        // enable GPIO0 clock
        mcx_pac::syscon::SYSCON0::instance()
            .regs()
            .AHBCLKCTRL0()
            .modify(|ahb_ctrl0| {
                ahb_ctrl0.set_GPIO0(true);
                ahb_ctrl0.set_PORT0(true);
                ahb_ctrl0.set_GPIO1(true);
                ahb_ctrl0.set_PORT1(true);
            });

        mcx_pac::port::PORT0::instance()
            .regs()
            .PCR(10)
            .modify(|pcr| {
                pcr.set_MUX(0);
                pcr.set_LK(true);

                defmt::debug!("write: {}", pcr);
            });

        mcx_pac::port::PORT0::instance()
            .regs()
            .PCR(10)
            .modify(|pcr| {
                pcr.set_MUX(0);
                pcr.set_LK(true);
            });

        mcx_pac::port::PORT1::instance()
            .regs()
            .PCR(3)
            .modify(|pcr| {
                pcr.set_MUX(0);
                pcr.set_LK(true);
                pcr.set_PE(false);
                pcr.set_IBE(true);
            });

        // set pin 10 in port 0 direction to output
        mcx_pac::gpio::GPIO0::instance().regs().PDDR().modify(|r| {
            r.set_PDD(10, true);
        });

        mcx_pac::gpio::GPIO1::instance().regs().ICR(3).modify(|r| {
            r.set_ISF(true);
            r.set_IRQC(10);
        });

        mcx_pac::gpio::GPIO1::instance().regs().PDDR().modify(|r| {
            r.set_PDD(3, false);
        });

        cortex_m::peripheral::NVIC::unmask(interrupt::GPIO10);
        cortex_m::interrupt::enable();

        loop {
            if FLAG_BTN_PRESSED.swap(false, Ordering::Relaxed) {
                mcx_pac::gpio::GPIO0::instance()
                    .regs()
                    .PTOR()
                    .write(|gpio| {
                        gpio.set_PTTO(10, true);
                    });
            }
        }
    }
}

#[interrupt]
unsafe fn GPIO10() {
    defmt::info!("GPIO10");

    unsafe {
        mcx_pac::gpio::GPIO1::instance()
            .regs()
            .ICR(3)
            .modify(|r| r.set_ISF(true));
    }

    // cortex_m::interrupt::free(|cs| {
    //     BTN.borrow(cs)
    //         .borrow_mut()
    //         .as_mut()
    //         .unwrap()
    //         .clear_interrupt_flag()
    // });

    FLAG_BTN_PRESSED.store(true, Ordering::Relaxed);
}
