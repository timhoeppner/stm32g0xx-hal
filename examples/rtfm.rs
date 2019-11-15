#![no_std]
#![no_main]
#![deny(warnings)]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate rtfm;
extern crate stm32g0xx_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::exti::Event;
use hal::gpio::gpioa::PA5;
use hal::gpio::{Output, PushPull, SignalEdge};
use hal::prelude::*;
use hal::stm32;
use hal::timer::Timer;
use rtfm::app;

#[app(device = hal::stm32, peripherals = true)]
const APP: () = {

    struct Resources {
        exti: stm32::EXTI,
        timer: Timer<stm32::TIM17>,
        led: PA5<Output<PushPull>>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> init::LateResources {
        let mut rcc = ctx.device.RCC.constrain();
        let gpioa = ctx.device.GPIOA.split(&mut rcc);
        let gpioc = ctx.device.GPIOC.split(&mut rcc);

        let mut timer = ctx.device.TIM17.timer(&mut rcc);
        timer.start(3.hz());
        timer.listen();

        gpioc.pc13.listen(SignalEdge::Falling, &mut ctx.device.EXTI);

        init::LateResources {
            timer,
            exti: ctx.device.EXTI,
            led: gpioa.pa5.into_push_pull_output(),
        }
    }

    #[task(binds = TIM17, resources = [led, timer])]
    fn timer_tick(ctx: timer_tick::Context) {
        ctx.resources.led.toggle().unwrap();
        ctx.resources.timer.clear_irq();
    }

    #[task(binds = EXTI4_15, resources = [exti])]
    fn button_click(ctx: button_click::Context) {
        hprintln!("Button pressed").unwrap();
        ctx.resources.exti.unpend(Event::GPIO13);
    }
};
