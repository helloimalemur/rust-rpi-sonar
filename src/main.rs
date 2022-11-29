use std::thread;
use std::time::Duration;
use std::time::{SystemTime};

use rppal::gpio::Gpio;

fn pulse_in(pin:rppal::gpio::InputPin, time_out:u64) -> f64 {
    let t0 = SystemTime::now();
    let dur = Duration::from_secs(time_out);

    while pin.is_low() {
        if t0.elapsed().unwrap() > dur {
            return 0.0;
        }
    }

    let t0 = SystemTime::now();
    while pin.is_high() {
        if t0.elapsed().unwrap() > dur {
            return 0.0;
        }
    }

    // println!("{}", "y");
    // println!("{}", pin.read());
    return t0.elapsed().unwrap().as_millis() as f64*1000000.0;
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

    println!("{}", pulse_in(pin17, 10));
}
