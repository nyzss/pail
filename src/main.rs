#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::{Path, PathBuf};

use chrono::prelude::*;
use eframe::egui;

struct Table {
    cols: Vec<String>,
    rows: Vec<Row>,
}

#[derive(Debug, Clone)]
struct Row {
    filename: String,
    file_type: String,
    created_at: DateTime<Utc>,
    file_path: PathBuf,
}
//
#[derive(Debug)]
struct Files {
    list: Vec<Row>,
    path: PathBuf,
}

impl Files {
    fn new() -> Self {
        Files {
            list: vec![],
            path: PathBuf::from("/"),
        }
    }

    fn get_files(&mut self) {
        let raw = std::fs::read_dir(&self.path).unwrap();

        let mut rows: Vec<Row> = vec![];
        for file in raw {
            let file = file.unwrap();

            let filename = get_filename(&file);

            let time = get_systemtime(&file);

            let file_type = get_filetype(&file);

            let created_at: DateTime<Utc> = time.clone().into();

            let file_path = get_filepath(&file);

            let row = Row {
                filename,
                file_type,
                created_at,
                file_path,
            };

            // println!("{:?}", &row);

            rows.push(row);
        }
        self.list = rows
    }
}

fn main() -> Result<(), eframe::Error> {
    // list_files();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(960., 720.)),
        ..Default::default()
    };
    let cols = vec![
        "Name".to_owned(),
        "Type".to_owned(),
        "Created At".to_owned(),
    ];
    // let mut files = Files::get_files("/Users/OkanT/Desktop/dev/".to_owned());
    let mut files = Files::new();
    files.get_files();

    let mut search: String = String::new();
    // println!("debug {:?}", &d);

    eframe::run_simple_native("pail", options, move |ctx, _| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("pail");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name ");
            //     ui.text_edit_singleline(&mut name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            // if ui.button("Click to add").clicked() {
            //     age += 1;
            // }
            // ui.label(format!("Hello '{name}', age '{age}'"));
            //
            // ui.columns(2, |cols| {
            //     for (i, col) in cols.iter_mut().enumerate() {
            //         col.label(format!("col 1"));
            //         if col.button("click").clicked() {
            //             age += 1;
            //         }
            //     }
            // });
            ui.heading("pail");
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("folder_table")
                    .striped(true)
                    .min_col_width(320.0)
                    .show(ui, |ui| {
                        ui.label("filter");
                        ui.text_edit_singleline(&mut search);
                        ui.label(&search);
                        if files.path != PathBuf::from("/") {
                            if ui.button("..").clicked() {
                                println!("hello guy, {:?}", &files.path);
                                // println!("strip prefix: {:?}", &files.path.strip_prefix());
                                files.path = files.path.parent().unwrap().to_path_buf();
                                search = String::new();

                                // change_list(&mut files, f.parent().unwrap().to_str().unwrap());

                                files.get_files();
                            }
                        }

                        for col in &cols {
                            ui.label(col);
                        }
                        ui.end_row();

                        // let list = &mut files.list;

                        for file in &files.list {
                            if file
                                .filename
                                .to_lowercase()
                                .contains(&search.to_lowercase())
                                || &search == ""
                            {
                                if ui.selectable_label(false, &file.filename).clicked() {
                                    if &file.file_type == "Folder" {
                                        files.path = file.file_path.clone();
                                    }
                                    println!("clicked on: {:?}", &file.file_path);

                                    search = String::new();

                                    // change_list(&mut files, file.file_path.as_str());
                                }
                                // ui.label(&file.filename);
                                ui.label(&file.file_type);
                                ui.label(&file.created_at.to_string());
                                ui.end_row();
                            }
                        }
                        files.get_files();
                    });
            });
        });
    })
}

fn get_filename(file: &std::fs::DirEntry) -> String {
    file.file_name().into_string().unwrap()
}
fn get_systemtime(file: &std::fs::DirEntry) -> std::time::SystemTime {
    file.metadata().unwrap().created().unwrap()
}

fn get_filetype(file: &std::fs::DirEntry) -> String {
    let f = file.file_type().unwrap();

    match f.is_dir() {
        true => "Folder".to_owned(),
        false => "File".to_owned(),
    }
}
fn get_filepath(file: &std::fs::DirEntry) -> PathBuf {
    file.path().to_path_buf()
}
