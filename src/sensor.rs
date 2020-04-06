use crate::*;
use anyhow::Result;
#[cfg(not(feature = "simulator"))]
use bme680::{Bme680, *};
#[cfg(not(feature = "simulator"))]
use embedded_hal::blocking::delay::DelayMs;
#[cfg(not(feature = "simulator"))]
use embedded_hal::blocking::i2c::{Read, Write};
#[cfg(not(feature = "simulator"))]
use log::*;

#[cfg(feature = "simulator")]
pub fn sensor<T: DrawTarget<BinaryColor>>(display: &mut T) -> Result<(), String> {
    let temp = 15.7;
    let pressure = 972.44;
    let humidity = 43.84;
    let gas = 538138;

    draw_sensor(display, temp, humidity, pressure, gas);

    Ok(())
}

#[cfg(not(feature = "simulator"))]
pub fn sensor<T: DrawTarget<BinaryColor>, I2C, D>(
    display: &mut T,
    bme: &mut Bme680<I2C, D>,
) -> Result<(), String>
where
    D: DelayMs<u8>,
    I2C: Read + Write,
    <I2C as embedded_hal::blocking::i2c::Read>::Error: std::fmt::Debug,
    <I2C as embedded_hal::blocking::i2c::Write>::Error: std::fmt::Debug,
{
    let power_mode = bme
        .get_sensor_mode()
        .map_err(|e| format!("Unable to get sensor mode: {:?}", e))?;
    debug!("Sensor power mode: {:?}", power_mode);
    debug!("Setting forced power modes");
    bme.set_sensor_mode(PowerMode::ForcedMode)
        .map_err(|e| format!("Unable to set sensor mode: {:?}", e))?;
    debug!("Retrieving sensor data");
    let (data, _state) = bme
        .get_sensor_data()
        .map_err(|e| format!("Unable to get sensor data: {:?}", e))?;
    debug!("Sensor Data {:?}", data);
    let temp = data.temperature_celsius();
    let pressure = data.pressure_hpa();
    let humidity = data.humidity_percent();
    let gas = data.gas_resistance_ohm();
    if humidity >= 99.9 {
        return Err(format!("Received bad sensor data: {:?}", data));
    }

    draw_sensor(display, temp, humidity, pressure, gas);

    sensor_to_influx(temp, humidity, pressure, gas);

    Ok(())
}

#[cfg(feature = "epd4in2")]
fn draw_sensor<T: DrawTarget<BinaryColor>>(
    display: &mut T,
    temp: f32,
    humidity: f32,
    pressure: f32,
    gas_resistance: u32,
) {
    text_24x32(display, &format!("{:5.1}°C", temp), (0, 100).into());
    text_8x16(
        display,
        &format!(
            "{:7.2}kOhm\n{:7.2}hPa\n{:7.2}%",
            gas_resistance as f32 / 1000.0,
            pressure,
            humidity,
        ),
        (0, 0).into(),
    );
}

#[cfg(feature = "epd2in9")]
fn draw_sensor<T: DrawTarget<BinaryColor>>(
    display: &mut T,
    temp: f32,
    humidity: f32,
    pressure: f32,
    gas_resistance: u32,
) {
    text_24x32(
        display,
        &format!("{:5.1}°", temp),
        (0, height() - 32).into(),
    );
    text_8x16(
        display,
        &format!(
            "{:7.2}kOhm\n{:7.2}hPa\n{:7.2}%",
            gas_resistance as f32 / 1000.0,
            pressure,
            humidity,
        ),
        (0, 0).into(),
    );
}
