#![no_std]

extern crate cortex_m;
extern crate f3;

use f3::{L3gd20, l3gd20};
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::hal::spi::Spi;

fn main() {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constraint();
    let mut rcc = p.RCC.constraint();

    // Try the other clock configuration
    let clocks = rcc.CFGR.freeze(&mut flash.ACR);
    // let clocks = rcc.CFGR.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.ACR);

    let mut gpioa = p.GPIOA.split(&mut rcc.AHB);
    let mut gpioe = p.GPIOE.split(&mut rcc.AHB);

    let mut nss = gpioe
        .PE3
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    nss.set_high();

    let sck = gpioa.PA5.as_af5(&mut gpioa.MODER, &mut gpioa.AFRL);
    let miso = gpioa.PA6.as_af5(&mut gpioa.MODER, &mut gpioa.AFRL);
    let mosi = gpioa.PA7.as_af5(&mut gpioa.MODER, &mut gpioa.AFRL);

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz(),
        clocks,
        &mut rcc.APB2,
    );

    let mut l3gd20 = L3gd20::new(spi, nss).unwrap();

    assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);

    let _m = l3gd20.all().unwrap();

    cortex_m::asm::bkpt();
}