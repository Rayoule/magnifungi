use::leptos::prelude::*;
use super::IntoFindView;


#[derive(Clone)]
pub struct EntryImage {
    pub img: String,
}
impl IntoFindView for EntryImage {
    fn into_any_view(&self) -> leptos::prelude::AnyView {
        // TODO avoid cloning the image
        view! {
            <p>{  self.img.clone() }</p>
        }.into_any()
    }
}

