use::leptos::prelude::*;
use super::{Entry, IntoFindView};


#[derive(Default, Clone)]
pub struct EntryList<T: IntoFindView> {
    pub entries: Vec<Entry<T>>,
}
impl<T: IntoFindView> IntoFindView for EntryList<T> {
    fn into_any_view(&self) -> AnyView {
        let list = self
            .entries
            .iter()
            .map(|e| view! {
                <li>{ e.into_any_view() }</li>
            })
            .collect_view();

        view! {
            <ul>{ list }</ul>
        }.into_any()
    }
}