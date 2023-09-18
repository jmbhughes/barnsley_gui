use std::f32::consts::PI;

use barnsley::{transform::{LinearTransform, AffineTransform, MoebiusTransform, InverseJuliaTransform}, util::Color};
use egui::{Color32, Rgba, Ui};


pub trait Visualize {
    fn ui(&mut self, ui: &mut Ui, label: String) -> (bool, bool);
}

impl Visualize for LinearTransform {
    fn ui(&mut self, ui: &mut Ui, label: String) -> (bool, bool) {
        let mut rerender = false;
        let mut delete_triggered = false;

        ui.collapsing(label, |ui| {
            ui.horizontal(|ui| {
                ui.label("a");
                let result = ui.add(egui::Slider::new(&mut self.a, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("b");
                let result = ui.add(egui::Slider::new(&mut self.b, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("c");
                let result = ui.add(egui::Slider::new(&mut self.c, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("d");
                let result = ui.add(egui::Slider::new(&mut self.d, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Color");
                let mut this_color: Color32 =
                    Rgba::from_rgb(self.base_color.r, self.base_color.g, self.base_color.b).into();
                let response = ui.color_edit_button_srgba(&mut this_color);
                if response.changed() {
                    self.base_color = Color {
                        r: this_color.r() as f32 / 255.0,
                        g: this_color.g() as f32 / 255.0,
                        b: this_color.b() as f32 / 255.0,
                    };
                    rerender = true;
                }
                ui.end_row();
            });

            ui.horizontal(|ui| {
                ui.label("Weight");
                let result = ui.add(egui::Slider::new(&mut self.weight, 0.0..=10.0));
                if result.changed() {
                    rerender = true;
                }
            });

            if ui.button("delete").clicked() {
                delete_triggered = true;
            }
        });
        (rerender, delete_triggered)
    }
}

impl Visualize for AffineTransform {
    fn ui(&mut self, ui: &mut Ui, label: String) -> (bool, bool) {
        let mut rerender = false;
        let mut delete_triggered = false;

        ui.collapsing(label, |ui| {
            ui.horizontal(|ui| {
                ui.label("a");
                let result = ui.add(egui::Slider::new(&mut self.a, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("b");
                let result = ui.add(egui::Slider::new(&mut self.b, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("c");
                let result = ui.add(egui::Slider::new(&mut self.c, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("d");
                let result = ui.add(egui::Slider::new(&mut self.d, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("xshift");
                let result = ui.add(egui::Slider::new(&mut self.xshift, -2.0..=2.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("yshift");
                let result = ui.add(egui::Slider::new(&mut self.yshift, -2.0..=2.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Color");
                let mut this_color: Color32 =
                    Rgba::from_rgb(self.base_color.r, self.base_color.g, self.base_color.b).into();
                let response = ui.color_edit_button_srgba(&mut this_color);
                if response.changed() {
                    self.base_color = Color {
                        r: this_color.r() as f32 / 255.0,
                        g: this_color.g() as f32 / 255.0,
                        b: this_color.b() as f32 / 255.0,
                    };
                    rerender = true;
                }
                ui.end_row();
            });

            ui.horizontal(|ui| {
                ui.label("Weight");
                let result = ui.add(egui::Slider::new(&mut self.weight, 0.0..=10.0));
                if result.changed() {
                    rerender = true;
                }
            });

            if ui.button("delete").clicked() {
                delete_triggered = true;
            }
        });
        (rerender, delete_triggered)
    }
}

impl Visualize for MoebiusTransform {
    fn ui(&mut self, ui: &mut Ui, label: String) -> (bool, bool) {
        let mut rerender = false;
        let mut delete_triggered = false;

        ui.collapsing(label, |ui| {
            ui.horizontal(|ui| {
                ui.label("a.re");
                let result = ui.add(egui::Slider::new(&mut self.a.re, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("a.im");
                let result = ui.add(egui::Slider::new(&mut self.a.im, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });


            ui.horizontal(|ui| {
                ui.label("b.re");
                let result = ui.add(egui::Slider::new(&mut self.b.re, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("b.im");
                let result = ui.add(egui::Slider::new(&mut self.b.im, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("c.re");
                let result = ui.add(egui::Slider::new(&mut self.c.re, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("c.im");
                let result = ui.add(egui::Slider::new(&mut self.c.im, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("d.re");
                let result = ui.add(egui::Slider::new(&mut self.d.re, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("d.im");
                let result = ui.add(egui::Slider::new(&mut self.d.im, -1.0..=1.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Color");
                let mut this_color: Color32 =
                    Rgba::from_rgb(self.base_color.r, self.base_color.g, self.base_color.b).into();
                let response = ui.color_edit_button_srgba(&mut this_color);
                if response.changed() {
                    self.base_color = Color {
                        r: this_color.r() as f32 / 255.0,
                        g: this_color.g() as f32 / 255.0,
                        b: this_color.b() as f32 / 255.0,
                    };
                    rerender = true;
                }
                ui.end_row();
            });

            ui.horizontal(|ui| {
                ui.label("Weight");
                let result = ui.add(egui::Slider::new(&mut self.weight, 0.0..=10.0));
                if result.changed() {
                    rerender = true;
                }
            });

            if ui.button("delete").clicked() {
                delete_triggered = true;
            }
        });
        (rerender, delete_triggered)
    }
}


impl Visualize for InverseJuliaTransform {
    fn ui(&mut self, ui: &mut Ui, label: String) -> (bool, bool) {
        let mut rerender = false;
        let mut delete_triggered = false;

        ui.collapsing(label, |ui| {
            ui.horizontal(|ui| {
                ui.label("r");
                let result = ui.add(egui::Slider::new(&mut self.r, 0.0..=3.0));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("theta");
                let result = ui.add(egui::Slider::new(&mut self.theta, 0.0..=2.0*PI));
                if result.changed() {
                    rerender = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Color");
                let mut this_color: Color32 =
                    Rgba::from_rgb(self.base_color.r, self.base_color.g, self.base_color.b).into();
                let response = ui.color_edit_button_srgba(&mut this_color);
                if response.changed() {
                    self.base_color = Color {
                        r: this_color.r() as f32 / 255.0,
                        g: this_color.g() as f32 / 255.0,
                        b: this_color.b() as f32 / 255.0,
                    };
                    rerender = true;
                }
                ui.end_row();
            });

            ui.horizontal(|ui| {
                ui.label("Weight");
                let result = ui.add(egui::Slider::new(&mut self.weight, 0.0..=10.0));
                if result.changed() {
                    rerender = true;
                }
            });

            if ui.button("delete").clicked() {
                delete_triggered = true;
            }
        });
        (rerender, delete_triggered)
    }
}