use crate::components::{intro::{Intro, IntroProps}, posts::{Posts, PostsProps}};
use leptos::{component, view, IntoView, Scope};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Intro/>
        <Posts/>
    }
}
