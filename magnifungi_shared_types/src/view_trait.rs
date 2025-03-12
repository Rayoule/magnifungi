use leptos::prelude::*;


// UI Refs
// https://dribbble.com/shots/20412819-Add-Owner-Details-Modal
// 

/// Trait that all Find entries must derive to be displayed
pub trait IntoFindView {
    fn into_any_view(&self) -> AnyView;
}
