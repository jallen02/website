use leptos::{component, Scope, IntoView, view};

#[component]
pub fn Posts(cx: Scope) -> impl IntoView {
    view! { cx, 
        <iframe height="1000" width="1000" src="assets/posts/2023-02-04.html" />
    }

}
