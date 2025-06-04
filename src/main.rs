use eframe::{Frame, egui};
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;
use egui_plot::{Line, Plot, PlotPoints};

struct HistogramApp {
    selected_file: Option<PathBuf>,
    histogram: Option<[u32; 256]>,
    error_message: Option<String>,
}

impl Default for HistogramApp {
    fn default() -> Self {
        Self {
            selected_file: None,
            histogram: None,
            error_message: None,
        }
    }
}

impl eframe::App for HistogramApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Histogram Viewer");
            ui.add_space(8.0);

            if ui.button("Choose File").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    match fs::read(&path) {
                        Ok(bytes) => {
                            let mut hist = [0u32; 256];
                            for &b in &bytes {
                                hist[b as usize] += 1;
                            }
                            self.histogram = Some(hist);
                            self.selected_file = Some(path);
                            self.error_message = None;
                        }
                        Err(err) => {
                            self.error_message = Some(format!("Error reading file: {}", err));
                            self.histogram = None;
                        }
                    }
                }
            }

            if let Some(path) = &self.selected_file {
                ui.add_space(8.0);
                ui.label(format!("Selected file: {}", path.display()));
            }

            if let Some(error) = &self.error_message {
                ui.add_space(8.0);
                ui.colored_label(egui::Color32::RED, error);
            }

            if let Some(hist) = &self.histogram {
                ui.add_space(16.0);
                ui.label("Byte Frequency Histogram:");

                Plot::new("histogram")
                    .height(300.0)
                    .allow_zoom(true)
                    .allow_drag(true)
                    .x_axis_label("Byte Value (0-255)")
                    .y_axis_label("Frequency")
                    .show(ui, |plot_ui| {
                        let points: Vec<[f64; 2]> = hist
                            .iter()
                            .enumerate()
                            .map(|(i, &v)| [i as f64, v as f64])
                            .collect();

                        let line = Line::new("File", PlotPoints::from(points), /* series */)
                            .width(2.0)
                            .name("Byte frequency");

                        plot_ui.line(line);
                    });

                // Show statistics
                ui.add_space(8.0);
                let non_zero_bytes = hist.iter().filter(|&&count| count > 0).count();
                let max_value = hist.iter().copied().max().unwrap_or(1);
                let total_bytes: u32 = hist.iter().sum();

                ui.label(format!(
                    "Statistics: {} total bytes, {} unique bytes, max frequency: {}",
                    total_bytes, non_zero_bytes, max_value
                ));
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "File Histogram",
        options,
        Box::new(|_cc| Ok(Box::new(HistogramApp::default()))),
    )
    .unwrap();
}
