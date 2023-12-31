use barnsley::animation::AnimationSequence;
use barnsley::ifs::IFS;
use barnsley::image::Image;
use barnsley::transform::{AffineTransform, LinearTransform, MoebiusTransform, Transform, Transformable};
use egui::{self, Ui};
use egui_extras::install_image_loaders;
use strum::IntoEnumIterator;
use std::io::Cursor;

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
    counter: u8
}

impl Default for MyApp {
    fn default() -> Self {
        let mut ifs_vec = vec![IFS::new(), IFS::new()];
        ifs_vec[0].add_transform(LinearTransform::random().into());
        ifs_vec[0].add_transform(AffineTransform::random().into());
        ifs_vec[0].add_transform(MoebiusTransform::random().into());
        ifs_vec[0].add_transform(MoebiusTransform::random().into());

        ifs_vec[1] = IFS::new();
        ifs_vec[1].add_transform(LinearTransform::random().into());
        ifs_vec[1].add_transform(AffineTransform::random().into());
        ifs_vec[1].add_transform(MoebiusTransform::random().into());
        ifs_vec[1].add_transform(MoebiusTransform::random().into());

        Self {
            animation_sequence: AnimationSequence {
                ifs_vec: ifs_vec,
                step_counts: vec![2],
            },
            rendered_image: Image::new(500, 500),
            num_points: 1000,
            num_iterations: 100,
            width: 500,
            height: 500,
            selected_transform_to_add: Transform::AffineTransform(AffineTransform::default()),
            delete_triggered: false,
            transform_to_delete: 0,
            rerender: false,
            counter: 0
        }
    }
}

impl MyApp {
    fn render_transform_ui(&mut self, ui: &mut Ui, index: usize) {
        let mut transform_counter = 0;
        for transform in &mut self
            .animation_sequence
            .ifs_vec
            .get_mut(index)
            .unwrap()
            .transforms
            .iter_mut()
        {
            let (rerender_update, delete_trigger_update) = match transform {
                Transform::LinearTransform(t) => t.ui(ui, format!("Linear: {transform_counter}")),
                Transform::AffineTransform(t) => t.ui(ui, format!("Affine: {transform_counter}")),
                Transform::MoebiusTransform(t) => t.ui(ui, format!("Moebius: {transform_counter}")),
                Transform::InverseJuliaTransform(t) => {
                    t.ui(ui, format!("InverseJulia: {transform_counter}"))
                }
            };

            self.rerender |= rerender_update;
            self.delete_triggered |= delete_trigger_update;

            if self.delete_triggered {
                self.transform_to_delete = transform_counter;
            }

            transform_counter += 1;
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

        egui::SidePanel::left("controls").exact_width(400.0).show(ctx, |ui| {
            if ui.button("Randomize the IFS").clicked() {
                self.animation_sequence.ifs_vec[0] = IFS::new();
                self.animation_sequence.ifs_vec[0].add_transform(LinearTransform::random().into());
                self.animation_sequence.ifs_vec[0].add_transform(AffineTransform::random().into());
                self.animation_sequence.ifs_vec[0].add_transform(MoebiusTransform::random().into());
                self.animation_sequence.ifs_vec[0].add_transform(MoebiusTransform::random().into());

                self.animation_sequence.ifs_vec[1] = IFS::new();
                self.animation_sequence.ifs_vec[1].add_transform(LinearTransform::random().into());
                self.animation_sequence.ifs_vec[1].add_transform(AffineTransform::random().into());
                self.animation_sequence.ifs_vec[1].add_transform(MoebiusTransform::random().into());
                self.animation_sequence.ifs_vec[1].add_transform(MoebiusTransform::random().into());

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
            ui.add(egui::Slider::new(&mut self.width, 1..=4096).text("Width"));
            ui.add(egui::Slider::new(&mut self.height, 1..=4096).text("Height"));
            ui.add(egui::Slider::new(&mut self.num_points, 1..=10000).text("Points"));
            ui.add(egui::Slider::new(&mut self.num_iterations, 1..=10000).text("Iterations"));

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
            }

            egui::ComboBox::from_label("Add a transform")
                .selected_text(self.selected_transform_to_add.get_name())
                .show_ui(ui, |ui| {
                    for t in Transform::iter() {
                        ui.selectable_value(&mut self.selected_transform_to_add, t, t.get_name());
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
            let _= buffer.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png);
            ui.ctx().forget_image("bytes://ifs.png");  // since uris are cached, we have to clear it
            ui.add(
                egui::Image::from_bytes("bytes://ifs.png", bytes)
                    .max_height(self.height as f32)
                    .max_width(self.width as f32)
                    .shrink_to_fit());
        });
    }
}
