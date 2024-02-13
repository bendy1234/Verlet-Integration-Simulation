mod physics;
mod render;
mod utill;

use std::{fs, thread, time::Duration};

use eframe::{
    egui::{self, containers, menu, style::Spacing, Context, Layout},
    emath::{Align, History, Vec2},
    epaint::{Color32, ColorImage},
};
use egui_extras::image::{load_image_bytes, load_svg_bytes};
use rfd::FileDialog;

fn main() -> eframe::Result<()> {
    let img = load_image_bytes(include_bytes!("../assets/rust-logo-128x128.png")).unwrap();

    let mut min_size = Vec2::new(img.width() as f32, img.height() as f32);
    let spacing = Spacing::default();
    min_size += spacing.window_margin.sum();
    min_size += Vec2::new(0.0, 22.0) + spacing.menu_margin.sum();
    min_size += render::PADDING * Vec2::splat(2.0);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size(min_size),
        ..Default::default()
    };

    eframe::run_native(
        "Verlet Integration Simluation",
        options,
        Box::new(|_cc| {
            let app = MyApp::new(img);
            let ctx = _cc.egui_ctx.clone();
            let dur = Duration::from_secs(1).div_f32(60.0);
            thread::spawn(move || loop {
                thread::sleep(dur);
                ctx.request_repaint();
            });
            Box::new(app)
        }),
    )
}

struct MyApp {
    frame_times: History<f32>,
    img: ColorImage,
    solver: physics::Solver,
    num: u64,
}

impl MyApp {
    fn new(img: ColorImage) -> Self {
        let frame_times = History::new(10..300, 1.0);
        let solver = physics::Solver::new(Vec2::splat(128.0));

        Self {
            frame_times,
            img,
            solver,
            num: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // frame counting
        let prev_time = frame.info().cpu_usage.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            *latest = prev_time;
        }
        self.frame_times.add(ctx.input(|i| i.time), prev_time);

        self.add_menu(ctx);

        // check whether to restart and add colors
        if self.num == 0 && self.solver.objects().len() >= self.solver.max_objects() as usize {
            self.num = ctx.frame_nr();
        } else if self.num != 0 && ctx.frame_nr() - self.num >= 60 * 10 && !self.solver.has_colors()
        {
            self.num = 0;
            self.solver.set_colors(Some(utill::map_colors(
                &self.img,
                self.solver.get_grid(),
                self.solver.objects().len(),
            )));
            self.solver.reset();
        }

        self.solver.tick(1.0 / 60.0);

        containers::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(Color32::DARK_GRAY))
            .show(ctx, |ui| {
                render::draw_content(self.solver.get_size(), self.solver.objects(), ui);
            });
    }
}

impl MyApp {
    fn select_new_img(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("", &["png", "jpeg", "jpg", "ico", "webp", "gif", "svg"])
            .pick_file()
        {
            println!("file picked: {:?}", path);
            let img = if path.extension().unwrap() == "svg" {
                load_svg_bytes(&fs::read(path).unwrap())
            } else {
                load_image_bytes(&fs::read(path).unwrap())
            }
            .unwrap();

            if img.width().max(img.height()) > i16::MAX as usize {
                println!("Image too big");
            } else if img.width().min(img.height()) < 16 {
                println!("Image too small")
            } else {
                self.solver
                    .set_size(Vec2::new(img.width() as f32, img.height() as f32));

                self.img = img;
                self.num = 0;

                self.solver.set_colors(None);
            }
        }
    }

    fn add_menu(&mut self, ctx: &Context) {
        containers::TopBottomPanel::top("top bar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let open_button = &ui.button("Open");
                    let reset_button = &ui.button("Reset");

                    if open_button.clicked() {
                        self.select_new_img();
                    } else if reset_button.clicked() {
                        self.solver.reset();
                        self.solver.set_colors(None);
                        self.num = 0;
                    }
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(format!(
                        "FPS: {:3.0}   Objects: {}",
                        1.0 / self.frame_times.mean_time_interval().unwrap_or_default(),
                        self.solver.objects().len()
                    ));
                });
            });
        });
    }
}
