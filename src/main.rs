use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Mpu6050Error::I2c).expect("");

    let mut delay = Delay;
    let mut mpu = Mpu6050::new(i2c);

    mpu.init(&mut delay).unwrap();

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