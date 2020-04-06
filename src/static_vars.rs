use dotenv;
use influx_db_client::{Client, Value};
use lazy_static::lazy_static;
use openweather::LocationSpecifier;
use openweather::{Language, Settings, Unit};
// const SETTINGS: &Settings = &Settings {
//     unit: Some(Unit::Metric),
//     lang: Some(Language::German),
// };

lazy_static! {
    pub static ref WEATHER_LOCATION: LocationSpecifier = {
        LocationSpecifier::CityAndCountryName {
            city: dotenv::var("CITY")
                .expect("get CITY (e.g. 'Tübingen') key from .env file"),

            country: dotenv::var("COUNTRY")
                .expect("get COUNTRY (e.g. 'DE') key from .env file"),
        }
    };
    pub static ref OPENWEATHER_API_KEY: String = {
        let key = "API_KEY";
        dotenv::var(key).expect("get api key from .env file")
    };
    pub static ref OPENWEATHER_SETTINGS: Settings = Settings {
        unit: Some(Unit::Metric),
        lang: Some(Language::German),
    };


    pub static ref INFLUX_CLIENT: Client = {
        // default with "http://127.0.0.1:8086", db with "test"
        let addr = dotenv::var("INFLUX_ADDRESS")
            .expect("get INFLUX_ADDRESS key from .env file");
        let user =
            dotenv::var("INFLUX_USER").expect("get INFLUX_USER key from .env file");
        let password = dotenv::var("INFLUX_PASSWORD")
            .expect("get INFLUX_PASSWORD key from .env file");
        let db = dotenv::var("INFLUX_DATABASE")
            .expect("get INFLUX_DATABASE key from .env file");

        Client::new(addr.parse().unwrap(), db).set_authentication(user, password)
    };
    pub static ref SENSOR: Value = Value::String(
        dotenv::var("SENSOR").expect("get SENSOR name key from .env file")
    );
    pub static ref LOCATION: Value = Value::String(
        dotenv::var("LOCATION").expect("get location name key from .env file"),
    );
    pub static ref DISPLAY: Value = Value::String(
        dotenv::var("DISPLAY").expect("get display name key from .env file"),
    );
}
