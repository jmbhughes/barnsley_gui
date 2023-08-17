use egui::{self, plot, Frame, Stroke, CollapsingHeader, Slider, Ui};            
use egui::plot::{Line, Plot, PlotPoints, Points, Text, PlotPoint, PlotBounds, Bar, BarChart, VLine, GridMark, CoordinatesFormatter};
use egui_extras::RetainedImage;

#[derive(PartialEq)]
pub struct MyApp {
    selected: usize,
    selector_vec: Vec<String>,
    max_value: usize, 
    show_add_new_ifs_window: bool,
    num_points: usize,
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
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected: 0,
            selector_vec: vec!["1".to_string(), "2".to_string(), "3".to_string()],//get_vec(),
            max_value: 3,
            show_add_new_ifs_window: false,
            num_points: 4,
            positions: vec![0, 10, 30, 100],
            //let positions = vec![[0.0, 0.0], [1.0, 0.0], [10.0, 0.0]];
            labels: vec!["A".to_string(), "weird".to_string(), "swirl".to_string(), "end".to_string()], 
            current_position: 3,
            paused: false,
            time: 0.0,
            zoom: 0.25,
            start_line_width: 2.5,
            depth: 9,
            length_factor: 0.8,
            luminance_factor: 0.8,
            width_factor: 0.9,
            line_count: 0
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
                    self.selector_vec.insert(self.selector_vec.len(), (self.max_value+1).to_string());
                    self.max_value += 1;
                    self.show_add_new_ifs_window = false;
                }


            });
        });
    }
    }

    fn options_ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>) {
        if seconds_since_midnight.is_some() {
            ui.label(format!(
                "Local time: {:02}:{:02}:{:02}.{:03}",
                (self.time % (24.0 * 60.0 * 60.0) / 3600.0).floor(),
                (self.time % (60.0 * 60.0) / 60.0).floor(),
                (self.time % 60.0).floor(),
                (self.time % 1.0 * 100.0).floor()
            ));
        } else {
            ui.label("The fractal_clock clock is not showing the correct time");
        };
        ui.label(format!("Painted line count: {}", self.line_count));

        ui.checkbox(&mut self.paused, "Paused");
        ui.add(Slider::new(&mut self.zoom, 0.0..=1.0).text("zoom"));
        ui.add(Slider::new(&mut self.start_line_width, 0.0..=5.0).text("Start line width"));
        ui.add(Slider::new(&mut self.depth, 0..=14).text("depth"));
        ui.add(Slider::new(&mut self.length_factor, 0.0..=1.0).text("length factor"));
        ui.add(Slider::new(&mut self.luminance_factor, 0.0..=1.0).text("luminance factor"));
        ui.add(Slider::new(&mut self.width_factor, 0.0..=1.0).text("width factor"));

        egui::reset_button(ui, self);

        ui.hyperlink_to(
            "Inspired by a screensaver by Rob Mayoff",
            "http://www.dqd.com/~mayoff/programs/FractalClock/",
        );
    }
}

impl eframe::App for MyApp {

    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            egui::Window::new("This app is being developed! Check back soon for some fractals.")
            .default_height(500.0)
            .default_width(500.0)
            // .collapsible(false)
            // .resizable(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.heading("This app is being developed! Check back soon for some fractals.");
                let image = RetainedImage::from_image_bytes("example.png", 
                                                                           include_bytes!("example.png")).unwrap();
                image.show(ui);
                ui.hyperlink("https://github.com/jmbhughes/barnsley");
                ui.heading("to learn more in the meantime.");

                });
            
        

            ui.heading("Select with Vectors");

            egui::ComboBox::from_label("Take your pick")
            .selected_text(format!("{}", &self.selector_vec[self.selected]))
            .show_ui(ui, |ui| {
                
                for i in 0..self.selector_vec.len() {
                    let value = ui.selectable_value(&mut &self.selector_vec[i], &self.selector_vec[self.selected], &self.selector_vec[i]);
                    if value.clicked() {
                        self.selected = i;
                    }
                }
            });

            if ui.button("add to the vec!").clicked() {
                self.show_add_new_ifs_window = true;
                println!("adding");
                // self.selector_vec.insert(self.selector_vec.len(), (self.max_value+1).to_string());
                // self.max_value += 1;
            }

            if self.show_add_new_ifs_window {
                self.add_new_ifs(ctx);
            }

            ui.end_row();



            let low_x = -5;
            let high_x = 105;
            let low_y = -5;
            let high_y = 5;
            let bounds = PlotBounds::from_min_max([low_x as f64, low_y as f64], [high_x as f64, high_y as f64]);
            let response = Plot::new("my_plot")
                .view_aspect(10.0)
                .show_y(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .allow_drag(false)
                .allow_boxed_zoom(false)
                .x_grid_spacer(|_|{vec![]})
                .label_formatter(|name, value|{
                    if !name.is_empty() {
                        format!("{} at {}", name, value.x.round() as i64)
                    } else {
                        format!("{}", value.x.round() as i64)
                    }})
                //.coordinates_formatter(plot::Corner::RightTop, CoordinatesFormatter::with_decimals(0))
                //.x_grid_spacer(move |_|{(low_x..high_x).map(|v| GridMark{value: v as f64, step_size: 1.0}).collect()})
                .y_grid_spacer(|_|{vec![]})
                //.show_background(false)
                .show(ui, |plot_ui| 
                    {
                        
                        //plot_ui.line(line); 
                        // let points = Points::new(positions)
                        //     .name("hi")
                        //     .filled(true)
                        //     .radius(10.0);
                        // plot_ui.points(points);
                        // for i in 0..self.num_points - 1 {
                        //     let points = vec![[self.positions[i], 0.0], [self.positions[i+1], 0.0]];
                        //     let line = Line::new(PlotPoints::new(points)).width(5.0);   
                        //     plot_ui.line(line);
                        // }


                        let mut bars: Vec<Bar> = (0..(self.num_points-1)).map(|i| Bar::new(0.0,(self.positions[i+1]-self.positions[i]) as f64).horizontal().base_offset(self.positions[i] as f64).width(1.0)).collect();
                        plot_ui.bar_chart(BarChart::new(bars)
                                            .element_formatter(Box::new(|bar, chart| {bar.name.clone()})));

        
                        for i in 0..self.num_points {
                            let point = PlotPoint::new(self.positions[i] as f64, 0.01);
                            let coord = vec![[self.positions[i] as f64, 0.0]];
                            //plot_ui.text(Text::new(point, labels[i]));
                            plot_ui.points(Points::new(coord).name(self.labels[i].as_str()).filled(true).radius(15.0));
                            //plot_ui.points(Points::new(vec![label.]))
                        }

                        let marker = VLine::new(self.current_position as f64).width(5.0);
                        plot_ui.vline(marker);

                        plot_ui.set_plot_bounds(bounds)
                    });

            if response.response.clicked_by(egui::PointerButton::Primary) {
                let pos = response.response.hover_pos().unwrap();
                println!("{} {}", pos.x, pos.y);
                let value = response.transform.value_from_position(pos);
                println!("(x, y) = ({}, {})",value.x, value.y);
                self.current_position = value.x.round() as i64;
            }

            if response.response.clicked_by(egui::PointerButton::Secondary) {
                let pos = response.response.hover_pos().unwrap();
                println!("{} {}", pos.x, pos.y);
                let value = response.transform.value_from_position(pos);
                println!("(x, y) = ({}, {})",value.x, value.y);
                let value = PlotPoint::new(value.x.round(), value.y.round());
                if (value.x as i64) < *self.positions.iter().min().unwrap() {
                    println!("adding a new min");
                } else if (value.x as i64) > *self.positions.iter().max().unwrap() {
                    println!("adding a new max");
                } else {
                    println!("break an animation!");
                    // self.num_points += 1;
                    // self.positions.insert(self.positions.len(), value.x.round() as i64);
                    // self.labels.insert(self.labels.len(), format!("{}", value.x).to_string());
                }
                // println!("{} {} {} {}", response.response.rect.min.x, 
                //                response.response.rect.min.y, 
                //                response.response.rect.max.x, 
                //                response.response.rect.max.y);
            }

            // if response.response.clicked_by(egui::PointerButton::Secondary) {
            //     let pos = response.response.hover_pos().unwrap();
            //     println!("DELETE {} {}", pos.x, pos.y);
            // }

            if response.response.dragged() {
                let delta = response.response.drag_delta();
                println!("drag {} {}", delta.x, delta.y);
            }
        });

    }
}