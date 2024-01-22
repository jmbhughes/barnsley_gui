use std::sync::mpsc::{channel, Receiver, Sender};

use barnsley::animation::AnimationSequence;
use barnsley::config::{Config, ImageSettings, EvaluationSettings, self};
use barnsley::ifs::IFS;
use barnsley::image::Image;
use barnsley::transform::{
    AffineTransform, LinearTransform, Transform, Transformable, InverseJuliaTransform,
};
use barnsley::util::Color;
use egui::{self, Ui, Vec2, FontId, RichText};
use egui_extras::install_image_loaders;
use std::fs::{File, self};
use std::future::Future;
//use core::slice::SlicePattern;
use std::sync::Arc;
use std::io::{Cursor, Write, Read};
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
    text_channel: (Sender<String>, Receiver<String>),
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
            text_channel: channel(),
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

        if let Ok(text) = self.text_channel.1.try_recv() {
            let config: Config = serde_json::from_str(&text).unwrap();

            let mut ifs0: IFS = IFS::new();

            for transform in config.transforms.into_iter() {
                ifs0.add_transform(transform);
            }
            self.animation_sequence.ifs_vec[0] = ifs0;

            let mut target_ifs = IFS::new();
            target_ifs.add_transform(LinearTransform::new(0.07927406, 0.4419875, -0.64647937, 0.19174504,Color{r: 0.13267994, g: 0.49911928, b:0.9295654},0.93828845).into());
            target_ifs.add_transform(InverseJuliaTransform::new(1.1700816, 2.9560707, Color{r: 0.9284186, g: 0.4638964, b:0.20791459}, 0.95615274).into());
            target_ifs.add_transform(InverseJuliaTransform::new(1.0998807, 1.9877317, Color{r: 0.41831225, g: 0.5540522, b:0.46177816}, 1.0506994).into());

            self.animation_sequence.ifs_vec[1] = target_ifs;
            self.width = config.image_settings.width as usize;
            self.height = config.image_settings.height as usize;
            self.num_iterations = config.evaluation_settings.num_iterations as usize;
            self.num_points = config.evaluation_settings.num_points as usize;
            self.rerender = true;
        }

        egui::SidePanel::left("controls")
            .exact_width(400.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("Welcome to Barnsley!").font(FontId::proportional(30.0)));
                ui.label("This tool allows you to explore iterated function systems (IFS). These are mathematical structures related to fractals.");
                ui.label("To start, try clicking the 'Randomize' button. Then, experiment with changing parameters or adding/deleting transforms.");

                ui.hyperlink_to("See a short user guide", "https://jmbhughes.com/posts/fractals/intro-barnsley/");
                ui.hyperlink_to("See the Rust code", "https://github.com/jmbhughes/barnsley");
                ui.hyperlink_to("Made by Marcus Hughes", "https://jmbhughes.com/");

                ui.separator();
                ui.heading("Create");
                if ui.button("Randomize").clicked() {
                    // for ifs in self.animation_sequence.ifs_vec.iter_mut() {
                    //     ifs.randomize();
                    // }
                    
                    self.animation_sequence.ifs_vec.get_mut(0).unwrap().randomize();

                    self.rendered_image = self.animation_sequence.animate_single_step(
                        self.width,
                        self.height,
                        self.num_iterations,
                        self.num_points,
                        1,
                    );
                };

                #[cfg(not(target_arch = "wasm32"))]
                if ui.button("Open parameters").clicked() {
                    let path = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file().unwrap(); 
                    
                    let mut file = File::open(path).unwrap();
                    let mut data = String::new();
                    file.read_to_string(&mut data).unwrap();
                    let config: Config = serde_json::from_str(&data).unwrap();

                    let mut ifs0: IFS = IFS::new();

                    for transform in config.transforms.into_iter() {
                        ifs0.add_transform(transform);
                    }
                    self.animation_sequence.ifs_vec[0] = ifs0;

                    let mut target_ifs = IFS::new();
                    target_ifs.add_transform(LinearTransform::new(0.07927406, 0.4419875, -0.64647937, 0.19174504,Color{r: 0.13267994, g: 0.49911928, b:0.9295654},0.93828845).into());
                    target_ifs.add_transform(InverseJuliaTransform::new(1.1700816, 2.9560707, Color{r: 0.9284186, g: 0.4638964, b:0.20791459}, 0.95615274).into());
                    target_ifs.add_transform(InverseJuliaTransform::new(1.0998807, 1.9877317, Color{r: 0.41831225, g: 0.5540522, b:0.46177816}, 1.0506994).into());

                    self.animation_sequence.ifs_vec[1] = target_ifs;
                    self.width = config.image_settings.width as usize;
                    self.height = config.image_settings.height as usize;
                    self.num_iterations = config.evaluation_settings.num_iterations as usize;
                    self.num_points = config.evaluation_settings.num_points as usize;
                    self.rerender = true;
                }

                #[cfg(not(target_arch = "wasm32"))]
                if ui.button("Save image").clicked() {
                    let mut bytes: Vec<u8> = Vec::new();
                    let save_scale =
                        1.max((self.num_points * self.num_iterations) / (self.width * self.height));
                    let buffer = array_to_image(self.rendered_image.to_u8(save_scale));
                    let _ = buffer.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png);

                    let file = rfd::FileDialog::new().add_filter("png", &["png"]).save_file().unwrap(); 
                    
                    let _ = image::save_buffer(file.to_str().unwrap(), 
                        &buffer, 
                        self.height as u32, 
                        self.width as u32, 
                        image::ColorType::Rgb8);
                }

                #[cfg(not(target_arch = "wasm32"))]
                if ui.button("Save parameters").clicked() {
                    let path = rfd::FileDialog::new().add_filter("json", &["json"]).save_file().unwrap(); 
                    
                    let config = Config {
                        image_settings: ImageSettings{ 
                            width: self.width as u32,
                            height: self.height as u32,
                            path: "empty.png".into()
                        },
                        evaluation_settings: EvaluationSettings{
                            num_iterations: self.num_iterations as u32,
                            num_points: self.num_points as u32
                        },
                        transforms: self.animation_sequence.ifs_vec.get(0).unwrap().transforms.clone()
                    };

                
                    let _ = File::create(path.to_str().unwrap()).unwrap();
                    fs::write(path.to_str().unwrap(), serde_json::to_string(&config).unwrap()).unwrap();
                }

                #[cfg(target_arch = "wasm32")]
                if ui.button("Open parameters").clicked() {
                    let sender = self.text_channel.0.clone();
                    let task = rfd::AsyncFileDialog::new().pick_file();
                    // Context is wrapped in an Arc so it's cheap to clone as per:
                    // > Context is cheap to clone, and any clones refers to the same mutable data (Context uses refcounting internally).
                    // Taken from https://docs.rs/egui/0.24.1/egui/struct.Context.html
                    let ctx = ui.ctx().clone();
                    execute(async move {
                        let file = task.await;
                        if let Some(file) = file {
                            let text = file.read().await;
                            let _ = sender.send(String::from_utf8_lossy(&text).to_string());
                            ctx.request_repaint();
                        }
                    });


                    // let future = async {
                    //     let file = rfd::AsyncFileDialog::new().add_filter("json", &["json"]).pick_file().await;
                    //     let data = file.unwrap().read().await;
                    //     // let config: Config = serde_json::from_slice(&file.unwrap().read().await.as_slice()).unwrap();
                    // };
                    // let _ = async_std::task::block_on(future);
                    // ui.close_menu();

                    // let config = future;
                    // let mut file = File::open(file).unwrap();
                    // let mut data = String::new();
                    // file.read_to_string(&mut data).unwrap();
                    // let config: Config = serde_json::from_str(&data).unwrap();

                    // let mut ifs0: IFS = IFS::new();

                    // for transform in config.transforms.into_iter() {
                    //     ifs0.add_transform(transform);
                    // }
                    // self.animation_sequence.ifs_vec[0] = ifs0;

                    // let mut target_ifs = IFS::new();
                    // target_ifs.add_transform(LinearTransform::new(0.07927406, 0.4419875, -0.64647937, 0.19174504,Color{r: 0.13267994, g: 0.49911928, b:0.9295654},0.93828845).into());
                    // target_ifs.add_transform(InverseJuliaTransform::new(1.1700816, 2.9560707, Color{r: 0.9284186, g: 0.4638964, b:0.20791459}, 0.95615274).into());
                    // target_ifs.add_transform(InverseJuliaTransform::new(1.0998807, 1.9877317, Color{r: 0.41831225, g: 0.5540522, b:0.46177816}, 1.0506994).into());

                    // self.animation_sequence.ifs_vec[1] = target_ifs;
                    // self.width = config.image_settings.width as usize;
                    // self.height = config.image_settings.height as usize;
                    // self.num_iterations = config.evaluation_settings.num_iterations as usize;
                    // self.num_points = config.evaluation_settings.num_points as usize;
                    // self.rerender = true;
                }

                #[cfg(target_arch = "wasm32")]
                if ui.button("Save image").clicked() {
                    let mut bytes: Vec<u8> = Vec::new();
                    let save_scale =
                        1.max((self.num_points * self.num_iterations) / (self.width * self.height));
                    let buffer = array_to_image(self.rendered_image.to_u8(save_scale));
                    let _ = buffer.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png);
                    
                    let future = async move {
                        let file = rfd::AsyncFileDialog::new().add_filter("png", &["png"]).save_file().await;
                        file.unwrap().write(&bytes).await
                    };
                    let data = async_std::task::block_on(future);
                    ui.close_menu();
                }

                #[cfg(target_arch = "wasm32")]
                if ui.button("Save parameters").clicked() {                    
                    let config = Config {
                        image_settings: ImageSettings{ 
                            width: self.width as u32,
                            height: self.height as u32,
                            path: "empty.png".into()
                        },
                        evaluation_settings: EvaluationSettings{
                            num_iterations: self.num_iterations as u32,
                            num_points: self.num_points as u32
                        },
                        transforms: self.animation_sequence.ifs_vec.get(0).unwrap().transforms.clone()
                    };

                    let future = async move {
                        let file = rfd::AsyncFileDialog::new().add_filter("json", &["json"]).save_file().await;
                        file.unwrap().write(&serde_json::to_vec(&config).unwrap().as_slice()).await
                    };
                    let data = async_std::task::block_on(future);
                    ui.close_menu();
            
                }

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
                ui.separator();
                ui.heading("Add transform");
                egui::ComboBox::from_label("")
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

                if ui.button("Add").clicked() {
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

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
