use leptos::prelude::*;
use magnifungi_shared_types::view_trait::IntoFindView;

use super::find::Find;



#[component]
pub fn DisplayFindPage(find: ReadSignal<Find>) -> impl IntoView {

    view! {
        { move || find.get().into_any_view() }
    }
}