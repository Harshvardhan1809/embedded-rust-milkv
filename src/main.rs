use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;
use embedded_hal::blocking::i2c::{WriteRead, Write};
use shared_bus;

struct MyI2cdev(I2cdev);

impl WriteRead for MyI2cdev {}
impl Write for MyI2cdev {}

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {

    // make linux driver, then shared bus, then mpu

    let i2c = MyI2cdev::new("/dev/i2c-1")
        .map_err(Mpu6050Error::I2c).expect("");


    println!("1\n");

    let mut delay = Delay;

    let i2c_bus = shared_bus::BusManagerSimple::new(i2c);

    println!("2\n");

    let mut mpu = Mpu6050::new(i2c_bus.acquire_i2c());

    println!("3\n");

    mpu.init(&mut delay.delay_ns(1000000000)).unwrap();

    println!("4\n");

    loop {
        // get roll and pitch estimate
        let acc = mpu.get_acc_angles().expect("");
        println!("r/p: {:?}", acc);

        // get sensor temp
        let temp = mpu.get_temp().expect("");
        println!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().expect("");
        println!("gyro: {:?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().expect("");
        println!("acc: {:?}", acc);
    }

    Ok(())
}