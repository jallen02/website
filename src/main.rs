#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged in the browser's console.error
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log
    tracing_wasm::set_as_global_default();

    let options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas_id",
            options,
            Box::new(|cc| Box::new(website::App::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    })
}
