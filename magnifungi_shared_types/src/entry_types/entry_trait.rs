use leptos::prelude::*;



/// Trait that all Find entries must derive to be displayed
pub trait Entry {
    fn into_any_view(&self) -> AnyView { ().into_any() }
}

