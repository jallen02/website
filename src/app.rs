use egui::Layout;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("This website is written fully in Rust using ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
                    ui.label(". Code used to build is available to inspect on ");
                    ui.hyperlink_to("github", "https://github.com/jallen02/website")
                })
            });
            ui.with_layout(Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label("Written by Jason Allen - jason@jallen.info");
            });
        });
    }

}
