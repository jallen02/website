use components::app::{App, AppProps};
use leptos::{mount_to_body, view};

mod components;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    })
}
