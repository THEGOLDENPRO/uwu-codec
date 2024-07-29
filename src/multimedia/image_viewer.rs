use std::error::Error;

use eframe::egui::{self, Image};

use crate::{uwu_bytes::UwUBytes, uwu_decode};

#[derive(Default)]
struct UwUImageViewer<'a> {
    image: Option<Image<'a>>
}

impl eframe::App for UwUImageViewer<'_> {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.add(
                    self.image.clone().unwrap().rounding(10.0),
                );
            });
        });
    }
}

pub fn open_image(uwu_bytes: &UwUBytes) -> Result<(), Box<dyn Error>> {
    let options = eframe::NativeOptions::default();

    let bytes = match uwu_decode(uwu_bytes) {
        Ok(value) => value,
        Err(e) => return Err(e)
    };

    let image = Image::from_bytes(
        format!("bytes://image.{}", uwu_bytes.file_type.clone().expect("uwu bytes MUST have a file type to open in UwU Image Viewer!")),
        bytes,
    );

    let result = eframe::run_native(
        "UwU Image Viewer", 
        options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); // This gives us image support.
            let mut jeff = Box::<UwUImageViewer>::default();

            jeff.image = Some(image);

            Ok(jeff)
        }),
    );

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(e.into())
    }
}