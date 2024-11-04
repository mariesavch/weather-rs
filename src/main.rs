#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use js_sys::{wasm_bindgen::JsValue, Date};
use serde::Deserialize;
use tailwind_fuse::tw_merge;

const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    dioxus_sdk::storage::set_dir!();
    dioxus::launch(App);
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeatherData {
    pub main: Main,
    pub weather: Vec<Weather>,
    pub sys: Sys,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Sys {
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForecastData {
    pub list: Vec<List>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct List {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub dt: i64,
}

async fn get_weather(location: String) -> reqwest::Result<WeatherData> {
    reqwest::get(format!("https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&cnt=8&appid=7484f462f852c04cbab6a6a5ad8c9d37", location))
        .await?
        .json::<WeatherData>()
        .await
}

async fn get_forecast(location: String) -> reqwest::Result<ForecastData> {
    reqwest::get(format!("https://api.openweathermap.org/data/2.5/forecast?q={}&units=metric&cnt=8&appid=7484f462f852c04cbab6a6a5ad8c9d37", location))
        .await?
        .json::<ForecastData>()
        .await
}

pub fn formatTime(time: i64) -> String {
    let date = Date::new(&JsValue::from_f64((time * 1000) as f64));
    let hours = date.get_hours();
    let minutes = date.get_minutes();

    format!("{:02}:{:02}", hours, minutes)
}

#[component]
fn App() -> Element {
    let mut location =
        use_synced_storage::<LocalStorage, String>("location".to_string(), || "".to_string());
    let weather = use_resource(move || async move { get_weather(location.to_string()).await });
    let forecast = use_resource(move || async move { get_forecast(location.to_string()).await });

    rsx! {
        main { class: "mx-auto max-w-[850px] px-6 pb-20",
            div { class: "pt-6 min-[950px]:pt-16",
                input {
                    aria_label: "Enter city",
                    placeholder: "Enter city",
                    spellcheck: false,
                    value: location,
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| location.set(event.value()),
                    class: tw_merge!(
                        "rounded-md border border-surface0 bg-base py-2 px-4 capitalize",
                        "outline-none transition-colors duration-300",
                        "placeholder:text-overlay0 hover:border-surface1",
                        "focus:text-text focus:border-surface2"
                    )
                }
                div { class: "mt-6",
                    if let Some(Ok(data)) = weather.read().as_ref() {
                        h2 { class: "text-xl font-bold capitalize",
                            "{data.weather[0].description.as_str()}"
                        }
                        h1 { class: "my-3 text-5xl font-bold", "{data.main.temp.round()}°C" }
                        if data.main.temp.round() != data.main.feels_like.round() {
                            p { class: "mb-5 text-overlay0",
                                "Feels like: {data.main.feels_like.round()}°C"
                            }
                        }
                        ul { class: "animated-list grid grid-cols-2 mt-5",
                            li { class: "pr-4",
                                div { class: "flex flex-col gap-1 py-3 min-[820px]:flex-row min-[820px]:gap-9",
                                    span { class: "text-overlay0 sm:w-20",
                                        "{formatTime(data.sys.sunrise)}"
                                    }
                                    span { class: "font-bold", "Sunrise" }
                                }
                            }
                            li { class: "pr-4",
                                div { class: "flex flex-col gap-1 py-3 min-[820px]:flex-row min-[820px]:gap-9",
                                    span { class: "text-overlay0 sm:w-20",
                                        "{formatTime(data.sys.sunset)}"
                                    }
                                    span { class: "font-bold", "Sunset" }
                                }
                            }
                            if let Some(Ok(data)) = forecast.read().as_ref() {
                                {data.list.iter().map(|forecast_data|
                                rsx! (
                                        li { class: "pr-4",
                                            div { class: "flex flex-col gap-1 py-3 min-[820px]:flex-row min-[820px]:gap-9",
                                                span { class: "text-overlay0 sm:w-20", "{formatTime(forecast_data.dt)}" }
                                                span { class: "font-semibold sm:w-12", "{forecast_data.main.temp.round()}°C" }
                                                span { class: "capitalize", "{forecast_data.weather[0].description}" }
                                            }
                                        }
                                ))}
                            }
                        }
                    }
                }
            }
        }
    }
}
