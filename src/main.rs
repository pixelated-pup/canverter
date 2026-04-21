#![no_std]
#![no_main]

use color::palette::css;
use cortex_m_rt::entry;
use panic_halt as _; // Halts on panic

use stm32g4xx_hal::{
    adc::{
        AdcClaim, AdcCommonExt,
        config::{AdcConfig, ClockMode, SampleTime},
    },
    prelude::*,
    pwr::PwrExt,
    rcc::Config,
    stm32,
    time::{ExtU32, RateExtU32},
    timer::Timer,
};

use crate::led::LED;

mod led;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let pwr = dp.PWR.constrain().freeze();
    let mut rcc = dp.RCC.freeze(Config::hsi(), pwr);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let led_pins = (
        gpioa.pa0.into_alternate(),
        gpioa.pa1.into_alternate(),
        gpiob.pb10.into_alternate(),
    );
    let mut led = LED::new(dp.TIM2.pwm(led_pins, 100.Hz(), &mut rcc));

    let timer = Timer::new(dp.TIM1, &rcc.clocks);
    let mut delay_timer = timer.start_count_down(100.millis()).delay();

    // let analog_pin = gpioa.pa7.into_analog();
    // let adc12_common = dp.ADC12_COMMON.claim(ClockMode::default(), &mut rcc);
    // let mut adc = adc12_common.claim_and_configure(
    //     dp.ADC2,
    //     AdcConfig::default(),
    //     &mut cp.SYST.delay(&rcc.clocks),
    // );

    loop {
        // let sample = adc.convert(&analog_pin, SampleTime::Cycles_640_5);
        // let millivolts = adc.sample_to_millivolts(sample);

        led.set(css::RED);

        delay_timer.delay_ms(1000);

        led.set(css::BLACK);

        delay_timer.delay_ms(1000);
    }
}
