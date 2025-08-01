// https://github.com/ublox-rs/ublox/blob/1de591a894005117556cb2a5d523bbf63e88752a/examples/ublox-device/src/lib.rs
use ublox::*;
use std::time::Duration;
use nmea_parser::*;


pub trait UbxPacketHandler {
    fn handle(&mut self, _packet: PacketRef<'_>) {}
}

/// Implement handler for simple callbacks / closures
impl<F: FnMut(PacketRef)> UbxPacketHandler for F {
    fn handle(&mut self, package: PacketRef) {
        self(package)
    }
}

pub struct NEO6M {
    port: Box<dyn serialport::SerialPort>,
    // parser: Parser<Vec<u8>>
    parser: NmeaParser,
}

impl NEO6M {
    pub fn new(port: Box<dyn serialport::SerialPort>) -> NEO6M {
        let parser = NmeaParser::new();
        NEO6M { port, parser }
    }

    pub fn write_all(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.port.write_all(data)
    }

    pub fn read_gps_data(&mut self) -> std::io::Result<()> {

        const MAX_PAYLOAD_LEN: usize = 1240;
        let mut local_buf: Vec<u8> = vec![];
        self.read_port(&mut local_buf)?;
        if local_buf.len() == 0 {
            ()
        }

        println!("GPS Parse Data");
        let sentences: Vec<&[u8]> = local_buf.split(|&e| e == b'\n').filter(|v| !v.is_empty()).collect();
        for sentence in sentences {
            let mut nmea_sentence = std::str::from_utf8(&sentence).unwrap().to_string();
            nmea_sentence = nmea_sentence + "\n";

            match self.parser.parse_sentence(&mut nmea_sentence) {
                Ok(message) => match message {
                    ParsedMessage::Gga(gga) => {
                        println!("GPS GGA DATA  Source: {}    Latitude: {:?}°    Longitude: {:?}°", gga.source, gga.latitude, gga.longitude);
                    },
                    ParsedMessage::Rmc(rmc) => {
                        println!("GPS RMC DATA  Source: {}    Speed: {:?} kts    Bearing: {:?}°", rmc.source, rmc.sog_knots, rmc.bearing);
                    },
                    _ => ()
                },
                Err(_) => {
                    println!("GPS Parse Error");
                }
            };

        }
        Ok(())
    }

    /// Reads the serial port, converting timeouts into "no data received"
    fn read_port(&mut self, output: &mut Vec<u8>) -> std::io::Result<usize> {
        match self.port.read_to_end(output) {
            Ok(b) => Ok(b),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    Ok(0)
                } else {
                    Err(e)
                }
            },
        }
    }
}