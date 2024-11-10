#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use js_sys::{wasm_bindgen::JsValue, Date};
use serde::Deserialize;
use sir::{css, global_css, AppStyle};
use svg_attributes::{font_size, font_style};

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

    global_css!(
        "
       :root {
        --rosewater: #ff8389;
        --flamingo: #ff8389;
        --red: #ff8389;
        --maroon: #ff8389;
        --pink: #ff7eb6;
        --mauve: #be95ff;
        --peach: #d44a1c;
        --yellow: #ab8600;
        --green: #08bdba;
        --teal: #33b1ff;
        --sky: #33b1ff;
        --sapphire: #33b1ff;
        --blue: #78a9ff;
        --lavender: #78a9ff;
        --text: #ffffff;
        --subtext1: #f4f4f4;
        --subtext0: #e0e0e0;
        --overlay2: #adadad;
        --overlay1: #949494;
        --overlay0: #7a7a7a;
        --surface2: #4f4f4f;
        --surface1: #383838;
        --surface0: #2e2e2e;
        --base: #161616;
        --mantle: #0d0d0d;
        --crust: #000000;
    } 

    @media (prefers-color-scheme: light) {
        :root {
            --rosewater: #da1e28;
            --flamingo: #da1e28;
            --red: #da1e28;
            --maroon: #da1e28;
            --pink: #d02670;
            --mauve: #8a3ffc;
            --peach: #d44a1c;
            --yellow: #ab8600;
            --green: #007d79;
            --teal: #1192e8;
            --sky: #1192e8;
            --sapphire: #1192e8;
            --blue: #0f62fe;
            --lavender: #0f62fe;
            --text: #000000;
            --subtext1: #404040;
            --subtext0: #474747;
            --overlay2: #575757;
            --overlay1: #595959;
            --overlay0: #737373;
            --surface2: #8c8c8c;
            --surface1: #d1d1d1;
            --surface0: #e6e6e6;
            --base: #ffffff;
            --mantle: #f2f2f2;
            --crust: #ebebeb;
        }
    }

    :root {
        background-color: var(--base);
        color: var(--text);
        line-height: 1.6;
    }

    "
    );

    let animated_list = css!(
        "
    @media (hover: hover) and (pointer: fine) {
        li {
            all: unset;
            transition-property: all;
            transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
            transition-duration: 300ms;
        }
        &:hover li {
            opacity: 0.5;
        }
        &:hover li:hover {
            opacity: 1;
        }
    }
    "
    );

    let input = css!(
        "
        all: unset;
        padding-top: 0.5rem;
        padding-bottom: 0.5rem; 
        padding-left: 1rem;
        padding-right: 1rem;
        border-radius: 0.375rem; 
        border: 1px solid var(--surface0); 
        text-transform: capitalize; 
        transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 300ms; 
        color: var(--text);

        &:hover {
            border-color: var(--surface1);
        }
        &:focus {
            border-color: var(--surface2);
        }
        &:placeholder {
            color: var(--overlay0);
        }
        "
    );

    let section = css!(
        "
    padding-top: 24px;
    @media(min-width: 950px) {
        padding-top: 64px
    }
    "
    );
    let span = css!(
        "
    color: var(--overlay0);
    @media (min-width: 640px) {
            width: 80px;
    }
    "
    );
    let item = css!(
        "
        display: flex; 
        padding-top: 0.75rem;
        padding-bottom: 0.75rem; 
        flex-direction: column; 
        gap: 0.25rem;
        @media(min-width: 850px) {
            flex-direction: row;
            gap: 36px;
        }
    "
    );

    rsx! {
        AppStyle {}
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: section,
                input {
                    aria_label: "Enter city",
                    placeholder: "Enter city",
                    spellcheck: false,
                    value: location,
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| location.set(event.value()),
                    class: input,
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
                            class: animated_list,
                            all: "unset",
                            display: "grid",
                            grid_template_columns: "repeat(2, minmax(0px, 1fr))",
                            margin_top: "20px",
                            li { padding_right: "16px",
                                div { class: item,
                                    span { class: span, "{formatTime(data.sys.sunrise)}" }
                                    span { font_style: "bold", "Sunrise" }
                                }
                            }
                            li { padding_right: "16px",
                                div { class: item,
                                    span { class: span, "{formatTime(data.sys.sunset)}" }
                                    span { font_style: "bold", "Sunset" }
                                }
                            }
                            if let Some(Ok(data)) = forecast.read().as_ref() {
                                {data.list.iter().map(|forecast_data| rsx! {
                                    li { padding_right: "16px",
                                        div { class: item,
                                            span { class: span, "{formatTime(forecast_data.dt)}" }
                                            span {
                                                font_style: "bold",
                                                class: css!("@media(min-width: 768px) { width: 48px; }"),
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
