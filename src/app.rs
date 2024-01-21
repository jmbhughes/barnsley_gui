use barnsley::animation::AnimationSequence;
use barnsley::ifs::IFS;
use barnsley::image::Image;
use barnsley::transform::{
    AffineTransform, LinearTransform, MoebiusTransform, Transform, Transformable, InverseJuliaTransform,
};
use barnsley::util::Color;
use egui::{self, Ui, Vec2};
use egui_extras::install_image_loaders;
use std::io::Cursor;
use strum::IntoEnumIterator;

use crate::transform::Visualize;

// #[derive(PartialEq)]
pub struct MyApp {
    animation_sequence: AnimationSequence,
    rendered_image: Image,
    num_points: usize,
    num_iterations: usize,
    width: usize,
    height: usize,
    selected_transform_to_add: Transform,
    pub(crate) delete_triggered: bool,
    pub(crate) transform_to_delete: usize,
    pub(crate) rerender: bool,
    counter: u8,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut ifs_vec = vec![IFS::new(), IFS::new()];
        ifs_vec[0].add_transform(LinearTransform::new(0.07927406, 0.4419875, -0.64647937, 0.19174504,Color{r: 0.13267994, g: 0.49911928, b:0.9295654},0.93828845).into());
        ifs_vec[0].add_transform(InverseJuliaTransform::new(1.1700816, 2.9560707, Color{r: 0.9284186, g: 0.4638964, b:0.20791459}, 0.95615274).into());
        ifs_vec[0].add_transform(InverseJuliaTransform::new(1.0998807, 1.9877317, Color{r: 0.41831225, g: 0.5540522, b:0.46177816}, 1.0506994).into());

        ifs_vec[1] = IFS::new();
        ifs_vec[1].add_transform(LinearTransform::new(0.07927406, 0.4419875, -0.64647937, 0.19174504,Color{r: 0.13267994, g: 0.49911928, b:0.9295654},0.93828845).into());
        ifs_vec[1].add_transform(InverseJuliaTransform::new(1.1700816, 2.9560707, Color{r: 0.9284186, g: 0.4638964, b:0.20791459}, 0.95615274).into());
        ifs_vec[1].add_transform(InverseJuliaTransform::new(1.0998807, 1.9877317, Color{r: 0.41831225, g: 0.5540522, b:0.46177816}, 1.0506994).into());

        Self {
            animation_sequence: AnimationSequence {
                ifs_vec,
                step_counts: vec![2],
            },
            rendered_image: Image::new(1024, 1024),
            num_points: 1000,
            num_iterations: 1000,
            width: 1024,
            height: 1024,
            selected_transform_to_add: Transform::AffineTransform(AffineTransform::default()),
            delete_triggered: false,
            transform_to_delete: 0,
            rerender: true,
            counter: 0,
        }
    }
}

impl MyApp {
    fn render_transform_ui(&mut self, ui: &mut Ui, index: usize) {
        let show_delete = self.animation_sequence.ifs_vec.get(0).unwrap().len() > 1;
        for (transform_counter, transform) in &mut self
            .animation_sequence
            .ifs_vec
            .get_mut(index)
            .unwrap()
            .transforms
            .iter_mut()
            .enumerate()
        {
            let (rerender_update, delete_trigger_update) = match transform {
                Transform::LinearTransform(t) => t.ui(ui, format!("Linear: {transform_counter}"), show_delete),
                Transform::AffineTransform(t) => t.ui(ui, format!("Affine: {transform_counter}"), show_delete),
                Transform::MoebiusTransform(t) => t.ui(ui, format!("Moebius: {transform_counter}"), show_delete),
                Transform::InverseJuliaTransform(t) => {
                    t.ui(ui, format!("InverseJulia: {transform_counter}"), show_delete)
                }
            };

            self.rerender |= rerender_update;
            self.delete_triggered |= delete_trigger_update;

            if delete_trigger_update {
                self.transform_to_delete = transform_counter;
            }
        }
    }
}

use image::RgbImage;
use ndarray::Array3;

fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        install_image_loaders(ctx);

        egui::SidePanel::left("controls")
            .exact_width(400.0)
            .show(ctx, |ui| {
                if ui.button("Randomize the IFS").clicked() {
                    for ifs in self.animation_sequence.ifs_vec.iter_mut() {
                        ifs.randomize();
                    }

                    self.rendered_image = self.animation_sequence.animate_single_step(
                        self.width,
                        self.height,
                        self.num_iterations,
                        self.num_points,
                        1,
                    );
                };

                ui.separator();
                ui.heading("Generation controls");
                if ui
                    .add(egui::Slider::new(&mut self.width, 1..=4096).text("Height"))
                    .changed()
                {
                    self.rerender = true;
                }
                if ui
                    .add(egui::Slider::new(&mut self.height, 1..=4096).text("Width"))
                    .changed()
                {
                    self.rerender = true;
                }
                if ui
                    .add(egui::Slider::new(&mut self.num_points, 1..=5000).text("Points"))
                    .changed()
                {
                    self.rerender = true;
                }
                if ui
                    .add(egui::Slider::new(&mut self.num_iterations, 1..=5000).text("Iterations"))
                    .changed()
                {
                    self.rerender = true;
                }

                // Render transform UI
                ui.separator();
                ui.heading("Transforms");
                self.render_transform_ui(ui, 0);

                if self.rerender {
                    self.rendered_image = self.animation_sequence.animate_single_step(
                        self.width,
                        self.height,
                        self.num_iterations,
                        self.num_points,
                        1,
                    );
                    self.counter += 1;
                    self.rerender = false;
                }

                if self.delete_triggered {
                    if self.animation_sequence.ifs_vec.get(0).unwrap().len() > 1 {
                        for ifs in self.animation_sequence.ifs_vec.iter_mut() {
                            ifs.delete_transform(self.transform_to_delete);
                        }
                        self.delete_triggered = false;
                        self.rendered_image = self.animation_sequence.animate_single_step(
                            self.width,
                            self.height,
                            self.num_iterations,
                            self.num_points,
                            1,
                        );
                    } else {  // cannot delete since there's only one transform left
                        self.delete_triggered = false;
                    } 
                }

                egui::ComboBox::from_label("Add a transform")
                    .selected_text(self.selected_transform_to_add.get_name())
                    .show_ui(ui, |ui| {
                        for t in Transform::iter() {
                            ui.selectable_value(
                                &mut self.selected_transform_to_add,
                                t,
                                t.get_name(),
                            );
                        }
                    });

                if ui.button("Add this transform").clicked() {
                    for ifs in &mut self.animation_sequence.ifs_vec.iter_mut() {
                        ifs.add_transform(self.selected_transform_to_add);
                    }
                    self.rendered_image = self.animation_sequence.animate_single_step(
                        self.width,
                        self.height,
                        self.num_iterations,
                        self.num_points,
                        1,
                    );
                }

                ui.end_row();
            });

        egui::SidePanel::right("right panel").show(ctx, |ui| {
            ui.label("Barnsley");
            ui.label("This tool allows you to explore iterated function systems. For more see");
            ui.hyperlink_to("the Rust library", "https://github.com/jmbhughes/barnsley");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut bytes: Vec<u8> = Vec::new();
            let save_scale =
                1.max((self.num_points * self.num_iterations) / (self.width * self.height));
            let buffer = array_to_image(self.rendered_image.to_u8(save_scale));
            let _ = buffer.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png);
            ui.ctx().forget_image("bytes://ifs.png"); // since uris are cached, we have to clear it
            ui.add(
                egui::Image::from_bytes("bytes://ifs.png", bytes)
                    .max_size(Vec2::new(10000.0, 10000.0))
                    .fit_to_exact_size(Vec2::new(self.height as f32, self.width as f32)),
            );
        });
    }
}
