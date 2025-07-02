// use mpu6050::device::{WHOAMI, DEFAULT_SLAVE_ADDR, PWR_MGMT_1};
use linux_embedded_hal::{I2cdev, Delay};
use embedded_hal::{delay::DelayNs, i2c::Error as I2CError};
use i2cdev::linux::{LinuxI2CError, LinuxI2CDevice};
use i2cdev::core::I2CDevice;
use std::thread;
use std::time::Duration;

use ina219_rs::ina219::{Opts, INA219};
use ina219_rs::physic;

fn main() -> Result<(), LinuxI2CError> {

    let device = I2cdev::new("/dev/i2c-1").unwrap();
    let opt = Opts::new(0x42, 100 * physic::MilliOhm, 1 * physic::Ampere);
    //let opt = Opts::default();
    let mut ina = INA219::new(device, opt);
    ina.init().unwrap();
    let pm = ina.sense().unwrap();
    println!("{:?}", pm);
    /* output
        Debug: PowerMonitor
    {
            Voltage = 8.228V,
            Shunt_Voltage = 534µV,
            Current = 1.750A,
            Power = 744mW
    }
        */

    Ok(())
}