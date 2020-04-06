use crate::*;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::DrawTarget;

pub fn time<T: DrawTarget<BinaryColor>>(display: &mut T) {
    let local: DateTime<Local> = Local::now();

    draw(display, local);
}

#[cfg(feature = "epd4in2")]
fn draw<T: DrawTarget<BinaryColor>>(display: &mut T, local: DateTime<Local>) {
    // date and day
    text_8x16(
        display,
        &format!(
            "{:4}/{:2}/{:2}\n   {:3}/{:3}",
            local.year(),
            local.month(),
            local.day(),
            daystr(&local.weekday()),
            local.ordinal()
        ),
        (width() - 80i32, 0).into(),
    );

    // time
    text_24x32(
        display,
        &format!(
            //TODO: use 24:36
            "{:2}:{:02}", //" {:02}s ",
            local.hour(),
            local.minute(),
            //local.second()
        ),
        (width() / 2 - 60i32, 40).into(),
    );
}

#[cfg(feature = "epd2in9")]
fn draw<T: DrawTarget<BinaryColor>>(display: &mut T, local: DateTime<Local>) {
    text_8x16(
        display,
        &format!(
            "{:2}/{:2}\n  {:3}",
            local.month(),
            local.day(),
            daystr(&local.weekday()),
        ),
        (width() - 40i32, 0).into(),
    );

    text_24x32(
        display,
        &format!(
            //TODO: use 24:36
            "{:2}:{:02}", //" {:02}s ",
            local.hour(),
            local.minute(),
            //local.second()
        ),
        (width() / 2 - 60i32, height() / 3).into(),
    );
}
