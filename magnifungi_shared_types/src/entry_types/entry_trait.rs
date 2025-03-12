use leptos::prelude::*;


// UI Refs
// https://dribbble.com/shots/20412819-Add-Owner-Details-Modal
// 

/// Trait that all Find entries must derive to be displayed
pub trait IntoFindView {
    fn into_any_view(&self) -> AnyView {
        ().into_any()
    }
}

pub trait IntoFindCategoryView: IntoFindView {
    fn into_any_view(&self) -> AnyView {
        view! {
            <div
                class="find-category"
            >
                { IntoFindView::into_any_view(self) }
            </div>
        }.into_any()
    }
}

pub trait IntoFindPartView: IntoFindView {
    fn into_any_view(&self) -> AnyView {
        view! {
            <div
                class="find-part"
            >
                { IntoFindView::into_any_view(self) }
            </div>
        }.into_any()
    }
}

pub trait IntoFindSubPartView: IntoFindView {
    fn into_any_view(&self) -> AnyView {
        view! {
            <div
                class="find-subpart"
            >
                { IntoFindView::into_any_view(self) }
            </div>
        }.into_any()
    }
}

pub trait IntoFindEntryView: IntoFindView {
    fn into_any_view(&self) -> AnyView {
        view! {
            <div
                class="find-entry"
            >
                { IntoFindView::into_any_view(self) }
            </div>
        }.into_any()
    }
}

