enum TemperatureError {
    Sensor(u8),
    Conversion(String),
}

fn main() {}

fn temperature_from_sensors(sensor_id: u32) -> Result<f64, u8> {
    match sensor_id {
        1 => Ok(25.0),
        2 => Ok(30.5),
        _ => Err(42),
    }
}

fn convert_to_fahrenheit(celsius: f64) -> Result<f64, String> {
    if celsius < -100.00 || celsius > 100.00 {
        Err("Out of range".to_string())
    } else {
        Ok(celsius * 1.8 + 32.0)
    }
}

fn get_temperature_fahrenheit(sensor_id: u32) -> Result<f64, TemperatureError> {
    let temp_celsius = temperature_from_sensors(sensor_id).map_err(TemperatureError::Sensor)?;
    let temp_fahrenheit =
        convert_to_fahrenheit(temp_celsius).map_err(TemperatureError::Conversion)?;
    Ok(temp_fahrenheit)
}
