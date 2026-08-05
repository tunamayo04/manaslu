#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::time::Duration;
// hide console window on Windows in release
use eframe::egui;
use eframe::egui::Color32;
use rand::RngExt;
use backend::cpu;
use backend::cpu::registers::Flag;
use backend::manaslu::Manaslu;

const FB_WIDTH: usize = 256;
const FB_HEIGHT: usize = 240;

const PATTERN_TABLE_DIM: usize = 128;
const NUM_PALETTES: usize = 8;
const COLORS_PER_PALETTE: usize = 4;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Manaslu",
        options,
        Box::new(|_| Ok(Box::new(MyApp::with_startup_rom("roms/nestest.nes")) as Box<dyn eframe::App>)),
    )
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum LeftTab {
    Cpu,
    Ppu,
}

struct MyApp {
    backend: Option<Manaslu>,
    framebuffer_texture: Option<egui::TextureHandle>,
    pattern_table_left_texture: Option<egui::TextureHandle>,
    pattern_table_right_texture: Option<egui::TextureHandle>,
    scale: usize,
    rom_error: Option<String>,
    palettes: [[Color32; COLORS_PER_PALETTE]; NUM_PALETTES],
    selected_palette: usize,
    left_tab: LeftTab,
}

impl MyApp {
    fn with_startup_rom(path: impl AsRef<std::path::Path>) -> Self {
        let mut app = Self::default();
        match Manaslu::new(path.as_ref().to_path_buf()) {
            Ok(m) => {
                app.backend = Some(m)
            },
            Err(e) => app.rom_error = Some(format!("Failed to load ROM: {e}")),
        }

        app
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            backend: None,
            framebuffer_texture: None,
            pattern_table_left_texture: None,
            pattern_table_right_texture: None,
            scale: 4,
            rom_error: None,
            palettes: placeholder_palettes(),
            selected_palette: 0,
            left_tab: LeftTab::Cpu,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(backend) = &mut self.backend {
            for _ in 0..1 {
                backend.tick();
            }
        }

        self.setup_menu_bar(ui);
        self.update_framebuffer(ui.ctx());
        self.update_pattern_tables(ui.ctx());
        self.show_error_dialog(ui.ctx());

        self.show_left_panel(ui);

        egui::CentralPanel::default().show(ui, |ui| {
            if let Some(tex) = &self.framebuffer_texture {
                // Stretch to fill whatever space is left after the side panel.
                let size = ui.available_size();
                ui.add(egui::Image::new((tex.id(), size)));
            }
        });
        ui.ctx().request_repaint();
    }
}

impl MyApp {
    fn setup_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open ROM").clicked() {
                        let Some(path) = rfd::FileDialog::new().pick_file() else {
                            return;
                        };
                        match Manaslu::new(path) {
                            Ok(m) => {
                                self.backend = Some(m);
                                self.rom_error = None;
                            }
                            Err(e) => {
                                self.rom_error = Some(format!("Failed to load ROM: {e}"));
                            }
                        }
                    }
                    if ui.button("Close ROM").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Debug", |ui| {
                    if ui.button("Debugger").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
    }

    fn show_left_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("left_panel")
            .resizable(true)
            .default_size(280.0)
            .min_size(220.0)
            .show(ui, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(self.left_tab == LeftTab::Cpu, "CPU")
                        .clicked()
                    {
                        self.left_tab = LeftTab::Cpu;
                    }
                    if ui
                        .selectable_label(self.left_tab == LeftTab::Ppu, "PPU")
                        .clicked()
                    {
                        self.left_tab = LeftTab::Ppu;
                    }
                });
                ui.separator();
                ui.add_space(4.0);

                match self.left_tab {
                    LeftTab::Cpu => self.show_cpu_tab(ui),
                    LeftTab::Ppu => self.show_ppu_tab(ui),
                }
            });
    }

    fn show_cpu_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Registers");
        ui.add_space(4.0);

        let Some(backend) = self.backend.as_ref() else {
            ui.label("No ROM loaded.");
            return;
        };

        let regs = backend.cpu().registers();

        ui.horizontal(|ui| {
            ui.label("PC");
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!("${:04X}", regs.program_counter)).monospace(),
                ),
            );
        });
        ui.horizontal(|ui| {
            ui.label("A");
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!("${:02X}", regs.accumulator)).monospace(),
                ),
            );
        });
        ui.horizontal(|ui| {
            ui.label("X");
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!("${:02X}", regs.x)).monospace(),
                )
            )
        });
        ui.horizontal(|ui| {
            ui.label("Y");
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!("${:02X}", regs.y)).monospace(),
                )
            )
        });
        ui.horizontal(|ui| {
            ui.label("SP");
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!("${:02X}", regs.stack_pointer)).monospace(),
                )
            )
        });

        ui.add_space(16.0);
        ui.heading(egui::RichText::new(format!("Status Flags (P: ${:02X})", regs.flags)).monospace());
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.add_enabled(false, egui::Checkbox::new(&mut regs.get_flag(cpu::registers::Flag::Negative), egui::RichText::new("N").monospace()));
            ui.add_space(4.0);
            ui.add_enabled(false, egui::Checkbox::new(&mut regs.get_flag(cpu::registers::Flag::Overflow), egui::RichText::new("V").monospace()));
            ui.add_space(4.0);
            ui.add_enabled(false, egui::Checkbox::new(&mut regs.get_flag(cpu::registers::Flag::B), egui::RichText::new("B").monospace()));
        });
        ui.horizontal(|ui| {
            ui.add_enabled(false, egui::Checkbox::new(&mut regs.get_flag(cpu::registers::Flag::DecimalMode), egui::RichText::new("D").monospace()));
            ui.add_space(4.0);
            ui.add_enabled(false, egui::Checkbox::new(&mut regs.get_flag(cpu::registers::Flag::InterruptDisable), egui::RichText::new("I").monospace()));
            ui.add_space(4.0);
            ui.add_enabled(false, egui::Checkbox::new(&mut regs.get_flag(cpu::registers::Flag::Zero), egui::RichText::new("Z").monospace()));
            ui.add_space(4.0);
            ui.add_enabled(false, egui::Checkbox::new(&mut regs.get_flag(cpu::registers::Flag::Carry), egui::RichText::new("C").monospace()));
        });
    }

    fn show_ppu_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Pattern Tables");
        ui.add_space(4.0);

        ui.label("Left ($0000)");
        if let Some(tex) = &self.pattern_table_left_texture {
            let display_size = ui.available_width().min(256.0);
            ui.add(egui::Image::new((tex.id(), egui::vec2(display_size, display_size))));
        }

        ui.add_space(8.0);

        ui.label("Right ($1000)");
        if let Some(tex) = &self.pattern_table_right_texture {
            let display_size = ui.available_width().min(256.0);
            ui.add(egui::Image::new((tex.id(), egui::vec2(display_size, display_size))));
        }

        ui.add_space(12.0);
        ui.separator();
        ui.heading("Palettes");
        ui.add_space(4.0);

        for pal_idx in 0..NUM_PALETTES {
            ui.horizontal(|ui| {
                let is_selected = self.selected_palette == pal_idx;
                let label = if pal_idx < 4 {
                    format!("BG {}", pal_idx)
                } else {
                    format!("SPR {}", pal_idx - 4)
                };

                if ui.selectable_label(is_selected, label).clicked() {
                    self.selected_palette = pal_idx;
                }

                for color in &self.palettes[pal_idx] {
                    let (rect, _response) =
                        ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::click());
                    ui.painter().rect_filled(rect, 0.0, *color);
                }
            });
        }
    }

    fn show_error_dialog(&mut self, ctx: &egui::Context) {
        if self.rom_error.is_none() {
            return;
        }

        let mut open = true;
        let mut dismissed = false;

        egui::Window::new("Error")
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                if let Some(err) = &self.rom_error {
                    ui.label(err);
                }
                ui.add_space(8.0);
                if ui.button("OK").clicked() {
                    dismissed = true;
                }
            });

        if !open || dismissed {
            self.rom_error = None;
        }
    }

    fn update_framebuffer(&mut self, ctx: &egui::Context) {
        let mut rng = rand::rng();
        let mut pixels = Vec::with_capacity(FB_WIDTH * FB_HEIGHT * 4);
        for _ in 0..FB_WIDTH * FB_HEIGHT {
            let grey: u8 = rng.random();
            pixels.extend_from_slice(&[grey, grey, grey, 255]);
        }
        let image = egui::ColorImage::from_rgba_unmultiplied([FB_WIDTH, FB_HEIGHT], &pixels);
        match &mut self.framebuffer_texture {
            Some(tex) => tex.set(image, egui::TextureOptions::NEAREST),
            None => {
                self.framebuffer_texture =
                    Some(ctx.load_texture("framebuffer", image, egui::TextureOptions::NEAREST));
            }
        }
    }

    fn update_pattern_tables(&mut self, ctx: &egui::Context) {
        let mut rng = rand::rng();

        let make_noise = || -> Vec<u8> {
            let mut px = Vec::with_capacity(PATTERN_TABLE_DIM * PATTERN_TABLE_DIM * 4);
            for _ in 0..PATTERN_TABLE_DIM * PATTERN_TABLE_DIM {
                let grey: u8 = rand::rng().random();
                px.extend_from_slice(&[grey, grey, grey, 255]);
            }
            px
        };
        let _ = &mut rng;

        let left_pixels = make_noise();
        let right_pixels = make_noise();

        let left_image = egui::ColorImage::from_rgba_unmultiplied(
            [PATTERN_TABLE_DIM, PATTERN_TABLE_DIM],
            &left_pixels,
        );
        let right_image = egui::ColorImage::from_rgba_unmultiplied(
            [PATTERN_TABLE_DIM, PATTERN_TABLE_DIM],
            &right_pixels,
        );

        match &mut self.pattern_table_left_texture {
            Some(tex) => tex.set(left_image, egui::TextureOptions::NEAREST),
            None => {
                self.pattern_table_left_texture = Some(ctx.load_texture(
                    "pattern-table-left",
                    left_image,
                    egui::TextureOptions::NEAREST,
                ));
            }
        }

        match &mut self.pattern_table_right_texture {
            Some(tex) => tex.set(right_image, egui::TextureOptions::NEAREST),
            None => {
                self.pattern_table_right_texture = Some(ctx.load_texture(
                    "pattern-table-right",
                    right_image,
                    egui::TextureOptions::NEAREST,
                ));
            }
        }
    }
}

fn placeholder_palettes() -> [[Color32; COLORS_PER_PALETTE]; NUM_PALETTES] {
    std::array::from_fn(|_| {
        [
            Color32::from_gray(20),
            Color32::from_gray(90),
            Color32::from_gray(160),
            Color32::from_gray(230),
        ]
    })
}