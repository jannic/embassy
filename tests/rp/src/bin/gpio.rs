#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{assert, *};
use embassy::executor::Spawner;
use embassy_rp::gpio::{Flex, Input, Level, Output, OutputOpenDrain, Pull};
use embassy_rp::Peripherals;
use {defmt_rtt as _, panic_probe as _};

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let (mut a, mut b) = (p.PIN_0, p.PIN_1);

    // Test initial output
    {
        let b = Input::new(&mut b, Pull::None);

        {
            let _a = Output::new(&mut a, Level::Low);
            delay();
            assert!(b.is_low());
        }
        {
            let _a = Output::new(&mut a, Level::High);
            delay();
            assert!(b.is_high());
        }
    }

    // Test input no pull
    {
        let b = Input::new(&mut b, Pull::None);
        // no pull, the status is undefined

        let mut a = Output::new(&mut a, Level::Low);
        delay();
        assert!(b.is_low());
        a.set_high();
        delay();
        assert!(b.is_high());
    }

    // Test input pulldown
    {
        let b = Input::new(&mut b, Pull::Down);
        delay();
        assert!(b.is_low());

        let mut a = Output::new(&mut a, Level::Low);
        delay();
        assert!(b.is_low());
        a.set_high();
        delay();
        assert!(b.is_high());
    }

    // Test input pullup
    {
        let b = Input::new(&mut b, Pull::Up);
        delay();
        assert!(b.is_high());

        let mut a = Output::new(&mut a, Level::Low);
        delay();
        assert!(b.is_low());
        a.set_high();
        delay();
        assert!(b.is_high());
    }

    // OUTPUT OPEN DRAIN
    {
        let mut b = OutputOpenDrain::new(&mut b, Level::High);
        let mut a = Flex::new(&mut a);
        a.set_as_input();

        // When an OutputOpenDrain is high, it doesn't drive the pin.
        a.set_pull(Pull::Up);
        delay();
        assert!(a.is_high());
        a.set_pull(Pull::Down);
        delay();
        assert!(a.is_low());

        b.set_low();

        // When an OutputOpenDrain is low, it drives the pin low.
        a.set_pull(Pull::Up);
        delay();
        assert!(a.is_low());
        a.set_pull(Pull::Down);
        delay();
        assert!(a.is_low());

        b.set_high();

        a.set_pull(Pull::Up);
        delay();
        assert!(a.is_high());
        a.set_pull(Pull::Down);
        delay();
        assert!(a.is_low());
    }

    // FLEX
    // Test initial output
    {
        //Flex pin configured as input
        let mut b = Flex::new(&mut b);
        b.set_as_input();

        {
            //Flex pin configured as output
            let mut a = Flex::new(&mut a); //Flex pin configured as output
            a.set_low(); // Pin state must be set before configuring the pin, thus we avoid unknown state
            a.set_as_output();
            delay();
            assert!(b.is_low());
        }
        {
            //Flex pin configured as output
            let mut a = Flex::new(&mut a);
            a.set_high();
            a.set_as_output();

            delay();
            assert!(b.is_high());
        }
    }

    // Test input no pull
    {
        let mut b = Flex::new(&mut b);
        b.set_as_input(); // no pull by default.

        let mut a = Flex::new(&mut a);
        a.set_low();
        a.set_as_output();

        delay();
        assert!(b.is_low());
        a.set_high();
        delay();
        assert!(b.is_high());
    }

    // Test input pulldown
    {
        let mut b = Flex::new(&mut b);
        b.set_as_input();
        b.set_pull(Pull::Down);
        delay();
        assert!(b.is_low());

        let mut a = Flex::new(&mut a);
        a.set_low();
        a.set_as_output();
        delay();
        assert!(b.is_low());
        a.set_high();
        delay();
        assert!(b.is_high());
    }

    // Test input pullup
    {
        let mut b = Flex::new(&mut b);
        b.set_as_input();
        b.set_pull(Pull::Up);
        delay();
        assert!(b.is_high());

        let mut a = Flex::new(&mut a);
        a.set_high();
        a.set_as_output();
        delay();
        assert!(b.is_high());
        a.set_low();
        delay();
        assert!(b.is_low());
    }

    info!("Test OK");
    cortex_m::asm::bkpt();
}

fn delay() {
    cortex_m::asm::delay(10000);
}
