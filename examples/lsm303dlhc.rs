#![no_std]

extern crate cortex_m;
extern crate f3;
extern crate lsm303dlhc;

use f3::hal::i2c::I2c;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::Lsm303dlhc;

fn main() {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constraint();
    let mut rcc = p.RCC.constraint();

    // Try the other clock configuration
    let clocks = rcc.CFGR.freeze(&mut flash.ACR);
    // let clocks = rcc.CFGR.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.ACR);

    let mut gpiob = p.GPIOB.split(&mut rcc.AHB);
    let scl = gpiob.PB6.as_af4(&mut gpiob.MODER, &mut gpiob.AFRL);
    let sda = gpiob.PB7.as_af4(&mut gpiob.MODER, &mut gpiob.AFRL);

    let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.APB1);

    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    let _accel = lsm303dlhc.accel().unwrap();
    let _mag = lsm303dlhc.mag().unwrap();
    let _temp = lsm303dlhc.temp().unwrap();

    cortex_m::asm::bkpt();
}