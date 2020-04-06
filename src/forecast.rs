use crate::*;
use embedded_graphics::transform::Transform;
use embedded_graphics::{drawable::Drawable, geometry::Point, pixelcolor::BinaryColor, DrawTarget};
use log::*;

fn tmp_graph_height() -> i32 {
    120
}

fn scale(min: i32, max: i32) -> i32 {
    let tmp = tmp_graph_height() / ((max + 5) - (min - 5));
    debug!("Scale: {}", tmp);
    match tmp_graph_height() / ((max + 5) - (min - 5)) {
        x if x.le(&1) => 1,
        //x if x.ge(&5) => 5,
        x => x,
    }
}

fn pos_x(day: usize, slot: usize) -> i32 {
    let mul = 10;
    (day * 8 + slot) as i32 * mul
}

struct Range {
    //min: i32,
    //max: i32,
    scale: i32,
    offset: i32,
}

impl Range {
    fn new(min: i32, max: i32, basic_offset: Option<i32>) -> Self {
        let scale = scale(min, max);
        Self {
            // min,
            // max,
            scale,
            offset: basic_offset.unwrap_or_default()
                + if min.is_negative() {
                    (min - 5).abs() * scale
                } else {
                    0
                },
        }
    }
    fn pos_y(&self, temp: f32) -> i32 {
        //let scale = 4;
        //let dist_from_ground = tmp_graph_height() + 10;
        -(temp as i32 * self.scale) - self.offset //-dist_from_ground
    }
}

pub fn weather_forecast<T: DrawTarget<BinaryColor>>(display: &mut T, current_temp: f32) {
    let forecast = match openweather::get_5_day_forecast(
        &WEATHER_LOCATION,
        &OPENWEATHER_API_KEY,
        &OPENWEATHER_SETTINGS,
    ) {
        Ok(forecast) => forecast,
        Err(e) => {
            error("Getting 5 Day Forecast", e);
            return;
        }
    };

    let mut abs_min = current_temp;
    let mut abs_max: f32 = current_temp;
    let mut temps: Vec<f32> = Vec::new();

    for (day, day_list) in forecast.list.chunks(8).take(4).enumerate() {
        let mut min = std::f32::MAX;
        let mut max: f32 = std::f32::MIN;

        for h3_slot in day_list.iter() {
            //let tmp = day.main.temp;
            min = min.min(h3_slot.main.temp_min);
            max = max.max(h3_slot.main.temp_max);
            debug!(
                "Day {}: Norm: {} | Min: {} | Max: {}",
                day + 1,
                h3_slot.main.temp,
                h3_slot.main.temp_min,
                h3_slot.main.temp_max
            );
            temps.push(h3_slot.main.temp);
        }
        debug!("Day {}: Min: {} | Max: {}", day + 1, min, max);
        text_6x8(
            display,
            &format!("{:6.2}°C\n{:6.2}°C", min, max),
            (pos_x(day, 4), height() - 20).into(),
        );

        abs_min = abs_min.min(min);
        abs_max = abs_max.max(max);
    }
    let abs_min: i32 = abs_min as i32;
    let abs_max: i32 = abs_max as i32;
    let basic_x_offset = 35;
    let basic_y_offset = 25;
    let r = Range::new(abs_min, abs_max, Some(basic_y_offset));

    let _ = rectangle(
        Point::new(0, height() - tmp_graph_height() - basic_y_offset),
        Point::new(width(), height() - basic_y_offset),
    )
    .draw(display);

    let mut prev_temp = current_temp;
    for (counter, temp) in temps.iter().enumerate() {
        let _ = line(
            (pos_x(0, counter), r.pos_y(prev_temp)).into(),
            (pos_x(0, counter + 1), r.pos_y(*temp)).into(),
        )
        .translate((basic_x_offset, height()).into())
        .draw(display);
        prev_temp = *temp;
    }

    for temp in (-30..=50)
        .step_by(10)
        .filter(|x| (*x).ge(&(abs_min - 5)) && (*x).le(&(abs_max + 5)))
    {
        text_6x8(
            display,
            &format!("{:3.2}°C", temp),
            (0, height() + r.pos_y(temp as f32)).into(),
        );
        let _ = line(
            (pos_x(0, 0), r.pos_y(temp as f32)).into(),
            (pos_x(3, 8), r.pos_y(temp as f32)).into(),
        )
        .translate(Point::new(basic_x_offset, height()))
        .draw(display);
    }
}
