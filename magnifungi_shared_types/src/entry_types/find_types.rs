use leptos::prelude::*;

use crate::view_trait::IntoFindView;



/// Find Categories
#[derive(Default, Clone)]
pub struct FCategory<T: IntoFindView>(T);
impl<T: IntoFindView> IntoFindView for FCategory<T> {
    fn into_any_view(&self) -> AnyView {
        view! {
            <div
                class="fcategory"
            >
                { self.0.into_any_view() }
            </div>
        }.into_any()
    }
}

/// Find Parts
#[derive(Default, Clone)]
pub struct FPart<T: IntoFindView>(T);
impl<T: IntoFindView> IntoFindView for FPart<T> {
    fn into_any_view(&self) -> AnyView {
        view! {
            <div
                class="fpart"
            >
                { self.0.into_any_view() }
            </div>
        }.into_any()
    }
}


/// Find Entry
#[derive(Default, Clone)]
pub struct FEntry<T: IntoFindView>(T);
impl<T:IntoFindView> IntoFindView for FEntry<T> {
    fn into_any_view(&self) -> AnyView {
        view! {
            <div
                class="fentry"
            >
                { self.0.into_any_view() }
            </div>
        }.into_any()
    }
}