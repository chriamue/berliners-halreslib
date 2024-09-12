use crate::components::CsvTable;
use crate::models::UrlData;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let urls = UrlData::default_urls().unwrap();
    let header = vec!["Date".to_string(), "URL".to_string()];
    html! {
        <div class="app">
            <h1>{"URLs"}</h1>
            <CsvTable header={header} rows={urls} />
        </div>
    }
}
