use crate::components::{intro::{Intro, IntroProps}, blog::{Blog, BlogProps}};
use leptos::{component, view, IntoView, Scope};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Intro/>
        <Blog/>
    }
}
