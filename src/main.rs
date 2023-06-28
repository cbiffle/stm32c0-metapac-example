//! STM32C011F6 example using stm32-metapac

#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::{entry, pre_init};
use stm32_metapac as device;
use device::gpio::vals::Moder;

#[entry]
fn main() -> ! {
    let rcc = device::RCC;
    rcc.gpioenr().modify(|v| {
        v.set_gpioaen(true);
    });
    cortex_m::asm::dsb(); // likely not necessary on M0

    let gpioa = device::GPIOA;
    gpioa.moder().modify(|v| {
        v.set_moder(8, Moder::OUTPUT);
    });

    loop {
        gpioa.bsrr().write(|w| {
            w.set_bs(8, true);
        });
        cortex_m::asm::delay(6_000_000);
        gpioa.bsrr().write(|w| {
            w.set_br(8, true);
        });
        cortex_m::asm::delay(6_000_000);
    }
}


/// This is working around device erratum "2.2.5 SRAM write error" where if a
/// reset is timed just right, the first write to an SRAM is treated as a read,
/// and so the write is lost.
///
/// To avoid this we need to read from each independent SRAM. There's only one
/// on C0, making this easy.
///
/// This function needs to not use the stack. As of this writing (rustc 1.69)
/// the function below generates no prologue/epilogue, but the right way to do
/// this is with a `naked_fn`, which is still unstable.
#[pre_init]
unsafe fn pre_init() {
    core::arch::asm!(
        "
            ldr {adr}, =0x20000000
            ldr {tmp}, [{adr}]
        ",
        tmp = out(reg) _,
        adr = out(reg) _,
        options(readonly, preserves_flags, nostack),
    );
}
