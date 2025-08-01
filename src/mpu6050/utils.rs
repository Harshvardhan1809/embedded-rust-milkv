use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;

use nalgebra::{Vector3, Vector2};
use libm::{powf, atan2f, sqrtf};

use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

use crate::mpu6050::device::{ACC_REGX_H, TEMP_OFFSET, TEMP_SENSITIVITY, TEMP_OUT_H, GYRO_REGX_H};
use crate::{ACCEL_SENS, GYRO_SENS, PWR_MGMT_1, WHOAMI, AccelRange, GyroRange, ACCEL_HPF, ACCEL_CONFIG, GYRO_CONFIG};
use crate::mpu6050::bits;

pub const PI: f32 = core::f32::consts::PI;
pub const PI_180: f32 = PI / 180.0;

pub struct Mpu6050 {
    i2c: Rc<RefCell<LinuxI2CDevice>>,
    acc_sensitivity: f32,
    gyro_sensitivity: f32,
}

impl Mpu6050 {

    pub fn new(i2c: Rc<RefCell<LinuxI2CDevice>>) -> Self {
        Mpu6050 {
            i2c: i2c.clone(),
            acc_sensitivity: ACCEL_SENS.0,
            gyro_sensitivity: GYRO_SENS.0,
        }
    }

    pub fn wake(&self) -> Result<(), ()> {
        self.i2c.borrow_mut().smbus_write_byte_data(PWR_MGMT_1::ADDR, 0x01).unwrap();
        thread::sleep(Duration::from_millis(100));
        Ok(())
    }

    pub fn read_slave_addr(&self) -> Result<(), ()> {
        let address = self.i2c.borrow_mut().smbus_read_byte_data(WHOAMI).unwrap();
        thread::sleep(Duration::from_millis(100));
        println!("Address : {}", address);
        if address != 0x68 {
            // panic
            println!("Wrong slave address");
            return Ok(());
        }
        Ok(())
    }

    pub fn init(&self) {
        self.set_accel_range(AccelRange::G2);
        self.set_gyro_range(GyroRange::D250);
        self.set_accel_hpf(ACCEL_HPF::_RESET);
    }

    pub fn write_read(&self, address: u8, write: &[u8], read: &mut [u8]) -> Result<(), ()> {

        self.i2c.borrow_mut().write(write).unwrap();
        Ok(self.i2c.borrow_mut().read(read).unwrap())
    }

    pub fn read_bytes(&self, slave_addr: u8, reg: u8, buf: &mut [u8]) -> Result<(), ()> {
        match self.write_read(slave_addr, &[reg], buf) {
            Ok(a) => Ok(a),
            Err(_) => Err(())
        }
    }

    pub fn write_bits(&self, reg: u8, start_bit: u8, length: u8, data: u8) {
        let mut byte: [u8; 1] = [0; 1];
        self.read_bytes(0x68, reg, &mut byte).unwrap();
        bits::set_bits(&mut byte[0], start_bit, length, data);
        self.i2c.borrow_mut().smbus_write_byte_data(reg, byte[0]).unwrap();
    }

    // pub fn read_bit(i2c: Rc<RefCell<LinuxI2CDevice>>, reg: u8, bit_n: u8) -> Result<u8, ()> {
    //     let mut byte: [u8; 1] = [0; 1];
    //     read_bytes(i2c.clone(), 0x68, reg, &mut byte).unwrap();
    //     println!("The byte is: {}", byte[0]);
    //     Ok(bits::get_bit(byte[0], bit_n))
    // }

    pub fn set_accel_range(&self, range: AccelRange) -> Result<(), ()> {
        self.write_bits(ACCEL_CONFIG::ADDR, ACCEL_CONFIG::FS_SEL.bit, ACCEL_CONFIG::FS_SEL.length, range as u8);
        // acc_sensitivity = range.sensitivity();
        Ok(())
    }

    pub fn set_gyro_range(&self, range: GyroRange) -> Result<(), ()> {
        self.write_bits(GYRO_CONFIG::ADDR, GYRO_CONFIG::FS_SEL.bit, GYRO_CONFIG::FS_SEL.length, range as u8);
        // gyro_sensitivity = range.sensitivity();
        Ok(())
    }

    pub fn set_accel_hpf(&self, mode: ACCEL_HPF) -> Result<(), ()> {
        Ok(self.write_bits(ACCEL_CONFIG::ADDR, ACCEL_CONFIG::ACCEL_HPF.bit, ACCEL_CONFIG::ACCEL_HPF.length, mode as u8))
    }

    // pub fn get_motion_detected(i2c: Rc<RefCell<LinuxI2CDevice>>) -> Result<bool, ()> {
    //     Ok(read_bit(i2c.clone(), INT_STATUS::ADDR, INT_STATUS::MOT_INT)? != 0)
    // }

    fn read_word_2c(&self, byte: &[u8]) -> i32 {
        let high: i32 = byte[0] as i32;
        let low: i32 = byte[1] as i32;
        let mut word: i32 = (high << 8) + low;

        if word >= 0x8000 {
            word = -((65535 - word) + 1);
        }

        word
    }

    fn read_rot(&self, reg: u8) -> Result<Vector3<f32>, ()> {
        let mut buf: [u8; 6] = [0; 6];
        self.read_bytes(0x68, reg, &mut buf).unwrap();

        Ok(Vector3::<f32>::new(
            self.read_word_2c(&buf[0..2]) as f32,
            self.read_word_2c(&buf[2..4]) as f32,
            self.read_word_2c(&buf[4..6]) as f32
        ))
    }

    pub fn get_acc(&self) -> Result<Vector3<f32>, ()> {
        let mut acc = self.read_rot(ACC_REGX_H)?;
        acc /= self.acc_sensitivity;

        Ok(acc)
    }

    pub fn get_acc_angles(&self) -> Result<Vector2<f32>, ()> {
        let acc = self.get_acc()?;

        Ok(Vector2::<f32>::new(
            atan2f(acc.y, sqrtf(powf(acc.x, 2.) + powf(acc.z, 2.))),
            atan2f(-acc.x, sqrtf(powf(acc.y, 2.) + powf(acc.z, 2.)))
        ))
    }

    pub fn get_temp(&self) -> Result<f32, ()> {
        let mut buf: [u8; 2] = [0; 2];
        self.read_bytes(0x68, TEMP_OUT_H, &mut buf).unwrap();
        let raw_temp = self.read_word_2c(&buf[0..2]) as f32;

        // According to revision 4.2
        Ok((raw_temp / TEMP_SENSITIVITY) + TEMP_OFFSET)
    }

    pub fn get_gyro(&self) -> Result<Vector3<f32>, ()> {
        let mut gyro = self.read_rot(GYRO_REGX_H)?;

        gyro *= PI_180 / self.gyro_sensitivity;

        Ok(gyro)
    }


}


#[cfg(test)]
mod tests {
}
