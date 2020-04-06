use chrono::prelude::*;
use core::fmt::{self, Debug};
use embedded_graphics::pixelcolor::BinaryColor::{Off as White, On as Black};
use embedded_graphics::{
    drawable::Drawable,
    fonts::*,
    geometry::Point,
    pixelcolor::BinaryColor,
    primitives::{Line, Primitive, Rectangle},
    style::{PrimitiveStyle, Styled},
    text_style, DrawTarget,
};

mod weather;
pub use weather::*;
#[cfg(feature = "epd4in2")]
mod forecast;
#[cfg(feature = "epd4in2")]
pub use forecast::*;
mod sensor;
pub use sensor::*;
mod time;
pub use time::*;
mod influx;
pub use influx::*;
mod static_vars;
pub use static_vars::*;

pub fn height() -> i32 {
    #[cfg(feature = "epd2in9")]
    return epd_waveshare::epd2in9::WIDTH as i32;
    #[cfg(feature = "epd4in2")]
    return epd_waveshare::epd4in2::HEIGHT as i32;
}

pub fn width() -> i32 {
    #[cfg(feature = "epd2in9")]
    return epd_waveshare::epd2in9::HEIGHT as i32;
    #[cfg(feature = "epd4in2")]
    return epd_waveshare::epd4in2::WIDTH as i32;
}

pub fn style_def() -> PrimitiveStyle<BinaryColor> {
    PrimitiveStyle::with_stroke(Black, 1)
}

pub fn line(start: Point, end: Point) -> Styled<Line, PrimitiveStyle<BinaryColor>> {
    Line::new(start, end).into_styled(style_def())
}

pub fn rectangle(start: Point, end: Point) -> Styled<Rectangle, PrimitiveStyle<BinaryColor>> {
    Rectangle::new(start, end).into_styled(PrimitiveStyle::with_fill(White))
}

pub fn text_6x8<T: DrawTarget<BinaryColor>>(display: &mut T, text: &str, top_left: Point) {
    draw_text(display, text, top_left, Font6x8);
}

/// Doesn't support as many different ascii chars
pub fn text_6x12<T: DrawTarget<BinaryColor>>(display: &mut T, text: &str, top_left: Point) {
    draw_text(display, text, top_left, Font6x12);
}

pub fn text_8x16<T: DrawTarget<BinaryColor>>(display: &mut T, text: &str, top_left: Point) {
    draw_text(display, text, top_left, Font8x16);
}

pub fn text_12x16<T: DrawTarget<BinaryColor>>(display: &mut T, text: &str, top_left: Point) {
    draw_text(display, text, top_left, Font12x16);
}
pub fn text_24x32<T: DrawTarget<BinaryColor>>(display: &mut T, text: &str, top_left: Point) {
    draw_text(display, text, top_left, Font24x32);
}

pub fn draw_text<T: DrawTarget<BinaryColor>, F: Copy + Font>(
    display: &mut T,
    text: &str,
    top_left: Point,
    font: F,
) {
    // epd4in2 doesn't fail there
    let _ = Text::new(text, top_left)
        .into_styled(text_style!(
            font = font,
            text_color = Black,
            background_color = White
        ))
        .draw(display);
}

fn daystr(day: &Weekday) -> &str {
    match day {
        Weekday::Mon => "Mon",
        Weekday::Tue => "Tue",
        Weekday::Wed => "Wed",
        Weekday::Thu => "Thu",
        Weekday::Fri => "Fri",
        Weekday::Sat => "Sat",
        Weekday::Sun => "Sun",
    }
}

pub fn error<T: core::fmt::Display>(desc: &str, error: T) {
    let fmt = format!("Error in {}: {}", desc, error);
    log::error!("{}", &fmt);
    err_influx(fmt);
}

#[derive(Debug)]
pub enum Status {
    STARTUP,
    SHUTDOWN,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Status::*;
        write!(
            f,
            "{}",
            match self {
                STARTUP => "STARTUP",
                SHUTDOWN => "SHUTDOWN",
            }
        )
    }
}
