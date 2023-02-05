use crate::components::intro::{Intro, IntroProps};
use leptos::{component, view, IntoView, Scope};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Intro/>
    }
}
