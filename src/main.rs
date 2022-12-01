use std::thread;
use std::time::Duration;
use std::time::{SystemTime};

use rppal::gpio::Gpio;

fn time() -> u128 {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
}

fn pulse_in(pin:rppal::gpio::InputPin) -> u128 {
    //13200
    let dur = Duration::from_millis(13200);
    // let dur = Duration::from_secs(time_out);

    let t0 = time();
    while pin.is_low() {
        if (time() - t0) > dur.as_millis() * 0.000001 as u128 {
            return 0;
        }
    }

    let t0 = time();
    while pin.is_high() {
        if (time() - t0) > dur.as_millis() * 0.000001 as u128 {
            return 0;
        }
    }

    return (time()-t0)*1000000
}

// def pulseIn(pin,level,timeOut): # function pulseIn: obtain pulse time of a pin
// t0 = time.time()
// while(GPIO.input(pin) != level):
// if((time.time() - t0) > timeOut*0.000001):
// return 0;
// t0 = time.time()
// while(GPIO.input(pin) == level):
// if((time.time() - t0) > timeOut*0.000001):
// return 0;
// pulseTime = (time.time() - t0)*1000000
// return pulseTime


fn main() {


    let gpio = Gpio::new().unwrap();
    let mut pin17 = gpio.get(23).unwrap().into_input();
    let mut pin22 = gpio.get(22).unwrap().into_output();

    //sonar distance
    pin22.set_high();
    thread::sleep(Duration::from_micros(10));
    pin22.set_low();

    println!("{}", pulse_in(pin17));
}
