use crate::models::UrlData;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CsvTableProps {
    pub header: Vec<String>,
    pub rows: Vec<UrlData>,
}

#[function_component(CsvTable)]
pub fn csv_table(props: &CsvTableProps) -> Html {
    let tbody = props.rows.iter().map(|row| {
        html! {
            <tr>
                <td>{row.catchdate.to_string()}</td>
                <td>{&row.url_raw}</td>
            </tr>
        }
    });

    html! {
        <div class="csv-table">
            <table>
                <thead>
                    <tr>
                        { for props.header.iter().map(|header| html! { <th>{header}</th> }) }
                    </tr>
                </thead>
                <tbody>
                    { for tbody }
                </tbody>
            </table>
        </div>
    }
}
