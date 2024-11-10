#![allow(non_snake_case)]

use css_in_rs::{make_styles, use_style_provider_quickstart, Classes, EmptyTheme};
use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use js_sys::{wasm_bindgen::JsValue, Date};
use serde::Deserialize;

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

make_styles! {
    (_theme: EmptyTheme) -> MyClasses {
        ":root" {
            __rosewater: "#ff8389",
            __flamingo: "#ff8389",
            __red: "#ff8389",
            __maroon: "#ff8389",
            __pink: "#ff7eb6",
            __mauve: "#be95ff",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#08bdba",
            __teal: "#33b1ff",
            __sky: "#33b1ff",
            __sapphire: "#33b1ff",
            __blue: "#78a9ff",
            __lavender: "#78a9ff",
            __text: "#ffffff",
            __subtext1: "#f4f4f4",
            __subtext0: "#e0e0e0",
            __overlay2: "#adadad",
            __overlay1: "#949494",
            __overlay0: "#7a7a7a",
            __surface2: "#4f4f4f",
            __surface1: "#383838",
            __surface0: "#2e2e2e",
            __base: "#161616",
            __mantle: "#0d0d0d",
            __crust: "#000000",
        },
    "@media (prefers-color-scheme: light)" {
        ":root" {
            __rosewater: "#da1e28",
            __flamingo: "#da1e28",
            __red: "#da1e28",
            __maroon: "#da1e28",
            __pink: "#d02670",
            __mauve: "#8a3ffc",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#007d79",
            __teal: "#1192e8",
            __sky: "#1192e8",
            __sapphire: "#1192e8",
            __blue: "#0f62fe",
            __lavender: "#0f62fe",
            __text: "#000000",
            __subtext1: "#404040",
            __subtext0: "#474747",
            __overlay2: "#575757",
            __overlay1: "#595959",
            __overlay0: "#737373",
            __surface2: "#8c8c8c",
            __surface1: "#d1d1d1",
            __surface0: "#e6e6e6",
            __base: "#ffffff",
            __mantle: "#f2f2f2",
            __crust: "#ebebeb",
            }
        },
        ":root" {
            background_color: "var(--base)",
            color: "var(--text)",
            line_height: "1.6",
        },
        "@media (hover: hover) and (pointer: fine)" {
            ".animated_list li" {
                all: "unset",
                transition_property: "all",
                transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
                transition_duration: "300ms",
            },
            ".animated_list:hover li" {
                opacity: "0.5",
            },
            ".animated_list:hover li:hover" {
                opacity: "1",
            }
        },
        ".item" {
            display: "flex",
            padding_top: "0.75rem",
            padding_bottom: "0.75rem",
            flex_direction: "column",
            gap: "0.25rem",
        },
        "@media(min-width: 850px)" {
            ".item" {
                flex_direction: "row",
                gap: "36px",
            }
        },
        ".input" {
            all: "unset",
            padding_top: "0.5rem",
            padding_bottom: "0.5rem",
            padding_left: "1rem",
            padding_right: "1rem",
            border_radius: "0.375rem",
            border: "1px solid var(--surface0)",
            text_transform: "capitalize",
            transition_property: "color, background-color, border-color, text-decoration-color, fill, stroke",
            transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
            transition_duration: "300ms",
            color: "var(--text)",
        },
        ".input:hover" {
            border_color: "var(--surface1)",
        },
        ".input:focus" {
            border_color: "var(--surface2)",
        },
        ".input:placeholder" {
            color: "var(--overlay0)",
        },
        ".section" {
            padding_top: "24px",
        },
        "@media(min-width: 950px)" {
            ".section" {
                padding_top: "64px",
            }
        },
        ".span" {
            color: "var(--overlay0)",
        },
        "@media (min-width: 640px)" {
            ".span" {
                width: "80px",
            }
        },
        "@media(min-width: 768px)" {
            ".data_temp" { width: "48px", },
        },
    }
}

#[component]
fn App() -> Element {
    use_style_provider_quickstart(|| EmptyTheme);
    let cls: &MyClasses = &MyClasses::use_style();

    let mut location =
        use_synced_storage::<LocalStorage, String>("location".to_string(), || "".to_string());
    let weather = use_resource(move || async move { get_weather(location.to_string()).await });
    let forecast = use_resource(move || async move { get_forecast(location.to_string()).await });

    rsx! {
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: &cls.section as &str,
                input {
                    aria_label: "Enter city",
                    placeholder: "Enter city",
                    spellcheck: false,
                    value: location,
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| location.set(event.value()),
                    class: &cls.input as &str,
                }
                div { margin_top: "24px",
                    if let Some(Ok(data)) = weather.read().as_ref() {
                        h2 {
                            font_size: "20px",
                            font_style: "bold",
                            text_transform: "capitalize",
                            "{data.weather[0].description.as_str()}"
                        }
                        h1 {
                            font_style: "bold",
                            margin_top: "12px",
                            margin_bottom: "12px",
                            line_height: "1",
                            font_size: "48px",
                            "{data.main.temp.round()}°C"
                        }
                        if data.main.temp.round() != data.main.feels_like.round() {
                            p {
                                color: "var(--overlay0)",
                                margin_bottom: "20px",
                                "Feels like: {data.main.feels_like.round()}°C"
                            }
                        }
                        ul {
                            class: &cls.animated_list as &str,
                            all: "unset",
                            display: "grid",
                            grid_template_columns: "repeat(2, minmax(0px, 1fr))",
                            margin_top: "20px",
                            li { padding_right: "16px",
                                div { class: &cls.item as &str,
                                    span { class: &cls.span as &str, "{formatTime(data.sys.sunrise)}" }
                                    span { font_style: "bold", "Sunrise" }
                                }
                            }
                            li { padding_right: "16px",
                                div { class: &cls.item as &str,
                                    span { class: &cls.span as &str, "{formatTime(data.sys.sunset)}" }
                                    span { font_style: "bold", "Sunset" }
                                }
                            }
                            if let Some(Ok(data)) = forecast.read().as_ref() {
                                {data.list.iter().map(|forecast_data| rsx! {
                                    li { padding_right: "16px",
                                        div { class: &cls.item as &str,
                                            span { class: &cls.span as &str, "{formatTime(forecast_data.dt)}" }
                                            span { font_style: "bold", class: &cls.data_temp as &str,
                                                "{forecast_data.main.temp.round()}°C"
                                            }
                                            span { text_transform: "capitalize", "{forecast_data.weather[0].description}" }
                                        }
                                    }
                                })}
                            }
                        }
                    }
                }
            }
        }
    }
}
