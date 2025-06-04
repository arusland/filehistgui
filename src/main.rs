use eframe::{egui, epi};
use rfd::FileDialog;
use std::fs;

struct HistogramApp {
    histogram: Option<[u32; 256]>,
}

impl Default for HistogramApp {
    fn default() -> Self {
        Self { histogram: None }
    }
}

impl epi::App for HistogramApp {
    fn name(&self) -> &str {
        "File Histogram"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Choose File").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    if let Ok(bytes) = fs::read(&path) {
                        let mut hist = [0u32; 256];
                        for &b in &bytes {
                            hist[b as usize] += 1;
                        }
                        self.histogram = Some(hist);
                    }
                }
            }

            if let Some(hist) = &self.histogram {
                ui.label("Histogram:");
                let max = *hist.iter().max().unwrap_or(&1) as f32;
                egui::plot::Plot::new("histogram")
                    .height(200.0)
                    .show(ui, |plot_ui| {
                        let points: Vec<_> = hist.iter().enumerate()
                            .map(|(i, &v)| [i as f64, v as f64])
                            .collect();
                        plot_ui.line(egui::plot::Line::new(egui::plot::PlotPoints::from(points)));
                    });
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "File Histogram",
        options,
        Box::new(|_cc| Box::new(HistogramApp::default())),
    );
}