#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    pub title: String,
    pub time: String,
    pub temp: f64,
    pub humd: f64,
    pub overview: String,
}
