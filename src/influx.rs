use crate::*;
// use futures_executor;
use influx_db_client::{Point, Precision, Value};
use tokio;

#[cfg(not(feature = "simulator"))]
pub(crate) fn sensor_to_influx(temp: f32, humidity: f32, pressure: f32, gas_resistance: u32) {
    let point = Point::new("sensor")
        .add_tag("location", LOCATION.clone())
        .add_tag("sensor", SENSOR.clone())
        .add_field("temperature", Value::Float(temp as f64))
        .add_field("humidity", Value::Float(humidity as f64))
        .add_field("pressure", Value::Float(pressure as f64))
        .add_field("gasresistence", Value::Integer(gas_resistance as i64));

    tokio_helper(point)
}

pub fn err_influx(msg: String) {
    let point = Point::new("error")
        .add_tag("location", LOCATION.clone())
        .add_tag("sensor", SENSOR.clone())
        .add_tag("display", DISPLAY.clone())
        .add_field("error", Value::String(msg));

    tokio_helper(point)
}

pub fn status_influx(status: Status, msg: Option<String>) {
    let point = Point::new("status")
        .add_tag("location", LOCATION.clone())
        .add_tag("sensor", SENSOR.clone())
        .add_tag("display", DISPLAY.clone())
        .add_tag("status", Value::String(status.to_string()))
        .add_field("message", Value::String(msg.unwrap_or_default()));

    tokio_helper(point)
}

fn tokio_helper(point: Point) {
    let fut_values = async {
        INFLUX_CLIENT
            .write_point(point, Some(Precision::Seconds), None)
            .await
    };

    // let _ = futures_executor::block_on(fut_values);
    if let Err(e) = match tokio::runtime::Runtime::new() {
        Ok(mut t) => t.block_on(fut_values),
        Err(e) => Err(e.into()),
    } {
        log::error!("Error with tokio runtime for error transmission: {}", e);
    }
}
