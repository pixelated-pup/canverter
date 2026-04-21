#![no_std]
#![no_main]

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
    time::ExtU32,
    timer::Timer,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let pwr = dp.PWR.constrain().freeze();
    let mut rcc = dp.RCC.freeze(Config::hsi(), pwr);

    let gpioa = dp.GPIOA.split(&mut rcc);

    let mut led = gpioa.pa5.into_push_pull_output();

    let timer2 = Timer::new(dp.TIM2, &rcc.clocks);
    let mut delay_tim2 = timer2.start_count_down(100.millis()).delay();

    let analog_pin = gpioa.pa7.into_analog();
    let adc12_common = dp.ADC12_COMMON.claim(ClockMode::default(), &mut rcc);
    let mut adc = adc12_common.claim_and_configure(
        dp.ADC2,
        AdcConfig::default(),
        &mut cp.SYST.delay(&rcc.clocks),
    );

    loop {
        let sample = adc.convert(&analog_pin, SampleTime::Cycles_640_5);
        let millivolts = adc.sample_to_millivolts(sample);

        led.toggle();
        delay_tim2.delay_ms(1000);
        led.toggle();
        delay_tim2.delay_ms(1000);
    }
}
