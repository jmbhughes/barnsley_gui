use barnsley::animation::AnimationSequence;
use barnsley::ifs::IFS;
use barnsley::image::Image;
use barnsley::transform::{
    self, AffineTransform, LinearTransform, MoebiusTransform, Transform, Transformable,
};
use barnsley::util::Color;
use egui::plot::{
    Bar, BarChart, CoordinatesFormatter, GridMark, Line, Plot, PlotBounds, PlotPoint, PlotPoints,
    Points, Text, VLine,
};
use egui::{self, plot, CollapsingHeader, Color32, Frame, Rgba, Slider, Stroke, Ui};
use egui_extras::RetainedImage;
use strum::IntoEnumIterator;

// #[derive(PartialEq)]
pub struct MyApp {
    selected: usize,
    selector_vec: Vec<String>,
    max_value: usize,
    show_add_new_ifs_window: bool,
    positions: Vec<i64>,
    labels: Vec<String>,
    current_position: i64,
    paused: bool,
    time: f64,
    zoom: f32,
    start_line_width: f32,
    depth: usize,
    length_factor: f32,
    luminance_factor: f32,
    width_factor: f32,
    line_count: usize,
    animation_sequence: AnimationSequence,
    rendered_image: Image,
    num_points: usize,
    num_iterations: usize,
    width: usize,
    height: usize,
    selected_transform_to_add: Transform,
    pub (crate) delete_triggered: bool,
    pub (crate) transform_to_delete: usize,
    pub (crate) rerender: bool,
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
            selected: 0,
            selector_vec: vec!["1".to_string(), "2".to_string(), "3".to_string()], //get_vec(),
            max_value: 3,
            show_add_new_ifs_window: false,
            positions: vec![0, 10, 30, 100],
            //let positions = vec![[0.0, 0.0], [1.0, 0.0], [10.0, 0.0]];
            labels: vec![
                "A".to_string(),
                "weird".to_string(),
                "swirl".to_string(),
                "end".to_string(),
            ],
            current_position: 3,
            paused: false,
            time: 0.0,
            zoom: 0.25,
            start_line_width: 2.5,
            depth: 9,
            length_factor: 0.8,
            luminance_factor: 0.8,
            width_factor: 0.9,
            line_count: 0,
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
        }
    }
}

impl MyApp {
    fn add_new_ifs(&mut self, ctx: &egui::Context) {
        if self.show_add_new_ifs_window {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_add_new_ifs_window = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.selector_vec
                                .insert(self.selector_vec.len(), (self.max_value + 1).to_string());
                            self.max_value += 1;
                            self.show_add_new_ifs_window = false;
                        }
                    });
                });
        }
    }
    
    fn render_transform_ui(&mut self,  ui: &mut Ui, index: usize) {
        let mut transform_counter = 0;
        for transform in &mut self.animation_sequence.ifs_vec.get_mut(index).unwrap().transforms.iter_mut() {
            let (rerender_update, delete_trigger_update) = match transform {
                Transform::LinearTransform(t) => 
                    t.ui(ui, format!("Linear: {transform_counter}")),
                Transform::AffineTransform(t) => 
                    t.ui(ui, format!("Affine: {transform_counter}")),
                Transform::MoebiusTransform(t) => 
                    t.ui(ui, format!("Moebius: {transform_counter}")),
                Transform::InverseJuliaTransform(t) => 
                    t.ui(ui, format!("InverseJulia: {transform_counter}")),
            };

            self.rerender |= rerender_update;
            self.delete_triggered |= delete_trigger_update;

            println!("{}", self.rerender);
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
use std::io::Cursor;

use crate::transform::Visualize;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let save_scale =
                1.max((self.num_points * self.num_iterations) / (self.width * self.height));

            let buffer = array_to_image(self.rendered_image.to_u8(save_scale));
            let mut bytes: Vec<u8> = Vec::new();
            let mut writer = Cursor::new(&mut bytes);
            buffer
                .write_to(&mut writer, image::ImageOutputFormat::Png)
                .unwrap();

            let retained_image =
                RetainedImage::from_image_bytes("test_image", bytes.as_slice()).unwrap();
            //ui.image(re, size)
            retained_image.show(ui);

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

            ui.add(egui::Slider::new(&mut self.width, 1..=4096).text("Width"));
            ui.add(egui::Slider::new(&mut self.height, 1..=4096).text("Height"));
            ui.add(egui::Slider::new(&mut self.num_points, 1..=10000).text("Points"));
            ui.add(egui::Slider::new(&mut self.num_iterations, 1..=10000).text("Iterations"));

            // Render transform UI
            self.render_transform_ui(ui, 0);

            if self.rerender {
                self.rendered_image = self.animation_sequence.animate_single_step(
                    self.width,
                    self.height,
                    self.num_iterations,
                    self.num_points,
                    1,
                );
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
                        println!("{}", t.get_name());
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
    }
}
