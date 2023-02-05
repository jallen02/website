use crate::components::intro::{Intro, IntroProps};
use leptos::{mount_to_body, view};

mod components;
mod lib;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <Intro/>
        }
    })
}
