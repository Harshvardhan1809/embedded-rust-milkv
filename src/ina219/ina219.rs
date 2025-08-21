#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

use byteorder::{BigEndian, ByteOrder};
use core::fmt;

use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use i2cdev::core::I2CDevice;

use crate::{
    physic, physic::ToStringPhysic_current, physic::ToStringPhysic_potential,
    physic::ToStringPhysic_power,
};

pub const INA219_ADDR: u8 = 0x40;

// Since physic electrical is in nano units we need to scale taking care to not
// overflow int64 or loose resolution.
const calibratescale: i64 = (((physic::Ampere as i64) * (physic::Ohm as i64)) / 100000) << 12;

pub struct Opts {
    Address: u8,
    SenseResistor: physic::ElectricResistance,
    MaxCurrent: physic::ElectricCurrent,
}

impl Opts {
    pub fn new(
        add: u8,
        sense: physic::ElectricResistance,
        maxCurrent: physic::ElectricCurrent,
    ) -> Opts {
        Opts {
            Address: add,
            SenseResistor: sense,
            MaxCurrent: maxCurrent,
        }
    }
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            Address: INA219_ADDR,
            SenseResistor: 100 * physic::MilliOhm, // 0.1Ohm
            MaxCurrent: 1 * physic::Ampere,
        }
    }
}

pub struct PowerMonitor {
    pub Shunt: String,
    pub Voltage: String,
    pub Current: String,
    pub Power: String,
}

impl PowerMonitor {
    pub fn new(shunt: String, voltage: String, current: String, power: String) -> PowerMonitor {
        PowerMonitor {
            Shunt: shunt,
            Voltage: voltage,
            Current: current,
            Power: power,
        }
    }
}

impl fmt::Display for PowerMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Voltage = {},\nShunt_Voltage = {},\nCurrent = {},\nPower = {}",
            self.Voltage, self.Shunt, self.Current, self.Power
        )
    }
}

impl fmt::Debug for PowerMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: PowerMonitor \n{{\n\tVoltage = {},\n\tShunt_Voltage = {},\n\tCurrent = {},\n\tPower = {} \n}}",
        self.Voltage,
        self.Shunt,
        self.Current,
        self.Power
    )
    }
}
pub struct Register;

impl Register {
    pub const Configuration: u8 = 0x00;
    pub const ShuntVoltage: u8 = 0x01;
    pub const BusVoltage: u8 = 0x02;
    pub const Power: u8 = 0x03;
    pub const Current: u8 = 0x04;
    pub const Calibration: u8 = 0x05;
}

pub struct INA219 {
    i2c: LinuxI2CDevice,
    opt: Opts,
    CurrentLSB: physic::ElectricCurrent,
    PowerLSB: physic::Power,
}

impl INA219 {
    pub fn new(i2c: LinuxI2CDevice, opts: Opts) -> Self {
        INA219 {
            i2c: i2c,
            opt: opts,
            CurrentLSB: 0,
            PowerLSB: 0,
        }
    }

    pub fn init(&mut self) -> Result<(), LinuxI2CError> {
        let ReConfigVal: u16 = 0x1FFF;
        self.calibrate(self.opt.SenseResistor, self.opt.MaxCurrent)?;
        self.i2c.write(&[Register::Calibration, (ReConfigVal >> 8) as u8, ReConfigVal as u8])?;
        Ok(())
    }

    pub fn calibrate(
        &mut self,
        sense: physic::ElectricResistance,
        maxCurrent: physic::ElectricCurrent,
    ) -> Result<(), LinuxI2CError> {
        /*  to do
        if sense <= 0 {
            return Err(E::SenseResistorValueInvalid);
        }
        if maxCurrent <= 0 {
            return Err(E::MaxCurrentInvalid);
        }
        */
        self.CurrentLSB = maxCurrent / (1 << 15);
        self.PowerLSB = (((maxCurrent * 20 + (1 << 14)) / (1 << 15)) as physic::Power);

        // Calibration Register = 0.04096 / (current LSB * Shunt Resistance)
        // Where lsb is in Amps and resistance is in ohms.
        // Calibration register is 16 bits.
        let cal = calibratescale / ((self.CurrentLSB as i64) * (sense as i64));
        //to do
        /*
        if cal >= ( 1<< 16) {
            return Err();
        }
        */
        self.i2c.write(&[Register::Calibration, (cal >> 8) as u8, cal as u8])?;
        Ok(())
    }

    pub fn shunt_voltage(&mut self) -> Result<String, LinuxI2CError> {
        self.calibrate(self.opt.SenseResistor, self.opt.MaxCurrent)?;
        let value = self.read(Register::ShuntVoltage)?;
        let str = value as i16;
        let tmp: String =
            (str as physic::ElectricPotential * physic::MicroVolt).to_string_physic_potential();
        Ok(tmp)
    }

    pub fn voltage(&mut self) -> Result<String, LinuxI2CError> {
        self.calibrate(self.opt.SenseResistor, self.opt.MaxCurrent)?;
        let value = self.read(Register::BusVoltage)?;
        let str = ((value >> 3) * 4) as i16;
        let tmp: String =
            (str as physic::ElectricPotential * physic::MilliVolt).to_string_physic_potential();
        Ok(tmp)
    }

    pub fn voltage_raw(&mut self) -> Result<u16, LinuxI2CError> {
        self.calibrate(self.opt.SenseResistor, self.opt.MaxCurrent)?;
        let value = self.read(Register::BusVoltage)?;
        Ok((value >> 3) * 4)
    }

    pub fn power(&mut self) -> Result<String, LinuxI2CError> {
        self.calibrate(self.opt.SenseResistor, self.opt.MaxCurrent);
        let value = self.read(Register::Power)?;
        let str = value as i16;
        let tmp: String = (str as physic::Power * physic::MilliVolt).to_string_physic_power();
        Ok(tmp)
    }

    pub fn current(&mut self) -> Result<String, LinuxI2CError> {
        self.calibrate(self.opt.SenseResistor, self.opt.MaxCurrent)?;
        let value = self.read(Register::Current)?;
        let str = value as i16;
        let tmp: String =
            (str as physic::ElectricCurrent * physic::MilliAmpere).to_string_physic_current();
        Ok(tmp)
    }

    pub fn current_raw(&mut self) -> Result<i16, LinuxI2CError> {
        self.calibrate(self.opt.SenseResistor, self.opt.MaxCurrent)?;
        let value = self.read(Register::Current)?;
        Ok(value as i16)
    }

    pub fn sense(&mut self) -> Result<PowerMonitor, LinuxI2CError> {
        self.calibrate(self.opt.SenseResistor, self.opt.MaxCurrent)?;
        let shunt = self.shunt_voltage()?;
        let voltage = self.voltage()?;
        let current = self.current()?;
        let power = self.power()?;
        let pm = PowerMonitor::new(shunt, voltage, current, power);
        Ok(pm)
    }


    fn read(&mut self, register: u8) -> Result<u16, LinuxI2CError> {
        self.i2c.write(&[register])?;
        let mut buf: [u8; 2] = [0x00; 2];
        self.i2c.read(&mut buf)?;
        Ok(BigEndian::read_u16(&buf))
    }
}