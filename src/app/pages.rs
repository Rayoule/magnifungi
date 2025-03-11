use leptos::prelude::*;

use super::find::Find;
use crate::app::find::entry_types::entry_trait::Entry;



#[component]
pub fn DisplayFindPage(find: ReadSignal<Find>) -> impl IntoView {

    view! {
        { move || find.get().into_any_view() }
    }
}