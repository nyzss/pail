#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::prelude::*;
// hide console window on Windows in release
use eframe::egui;

struct Table {
    cols: Vec<String>,
    rows: Vec<Row>,
}

#[derive(Debug)]
struct Row {
    filename: String,
    file_type: String,
    created_at: DateTime<Utc>,
    file_path: String,
}
//
#[derive(Debug)]
struct Files {
    list: Vec<Row>,
}

impl Files {
    fn new() -> Self {
        Files { list: vec![] }
    }

    fn get_files(path: String) -> Files {
        let raw = std::fs::read_dir(path).unwrap();

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

            println!("{:?}", &row);

            rows.push(row);
        }
        Files { list: rows }
    }
}

fn main() -> Result<(), eframe::Error> {
    // list_files();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320., 240.)),
        ..Default::default()
    };
    let cols = vec![
        "Name".to_owned(),
        "Type".to_owned(),
        "Created At".to_owned(),
    ];

    // let mut files = std::fs::read_dir("/Users/OkanT/Desktop/dev/").unwrap();
    let mut files = list_files("/Users/OkanT/Desktop/dev/".to_owned());

    eframe::run_simple_native("pail", options, move |ctx, frame| {
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
            egui::Grid::new("some_unique_id")
                .striped(true)
                .min_col_width(180.0)
                .show(ui, |ui| {
                    for col in &cols {
                        ui.label(col);
                    }
                    ui.end_row();

                    for file in &files {
                        if ui.selectable_label(false, &file.filename).clicked() {
                            // files = list_files(file.file_path);
                            println!("{}", &file.file_path)
                        }
                        // ui.label(&file.filename);
                        ui.label(&file.file_type);
                        ui.label(&file.created_at.to_string());
                        ui.end_row();
                    }
                });
        });
    })
}

fn list_files(path: String) -> Vec<Row> {
    let raw = std::fs::read_dir(path).unwrap();

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

        println!("{:?}", &row);

        rows.push(row);
    }
    rows
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
fn get_filepath(file: &std::fs::DirEntry) -> String {
    let f: String = file.path().into_os_string().into_string().unwrap();
    f
}
