use clap::Parser;
use std::path::PathBuf;

use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/style.css");

#[derive(Clone, Copy)]
enum ChecksumStatus {
    DigimonWorld2003,
    DigimonWorld3US,
    DigimonWorld3J,
    Unknown,
    Checking,
}

#[derive(Clone)]
struct RomState {
    source_bin: Option<PathBuf>,
}

fn app() -> Element {
    let mut rom_state = use_signal(|| RomState { source_bin: None });
    let mut checksum_state = use_signal(|| ChecksumStatus::Checking);

    let rom = rom_state();
    let checksum = checksum_state();

    let file_name_cl: String = match &rom.source_bin {
        Some(file) => String::from(file.file_name().unwrap().to_str().unwrap()),
        None => String::from("Rom File"),
    };

    let checksum_status = rom.source_bin.map(|_| match checksum {
        ChecksumStatus::DigimonWorld2003 => ("✓ Digimon World 2003", "lawngreen"),
        ChecksumStatus::DigimonWorld3US => ("x Digimon World 3", "red"),
        ChecksumStatus::DigimonWorld3J => ("x Digimon World 3", "red"),
        ChecksumStatus::Unknown => ("⚠ Unknown Checksum", "yellow"),
        ChecksumStatus::Checking => ("⚠ Verifying Checksum", "yellow"),
    });

    rsx! {
        div {
            Stylesheet { href: CSS }
            div { class: "inline",
                div { class: "center",
                    label { class: "file-upload", r#for: "file-upload", "{file_name_cl}" }
                    input {
                        id: "file-upload",
                        r#type: "file",
                        accept: ".bin",
                        onchange: move |x: Event<FormData>| {
                            if let Some(file) = x.files().first() {
                                let fpath = file.path();
                                rom_state
                                    .set(RomState {
                                        source_bin: Some(fpath.clone()),
                                    });
                                checksum_state.set(ChecksumStatus::Checking);

                                spawn(async move {
                                    let hash = tokio::spawn(async {
                                        let data = async_fs::read(fpath).await.unwrap();

                                        let mut hasher = blake3::Hasher::new();
                                        hasher.update(&data[..]);

                                        hasher.finalize()
                                    }).await.unwrap();

                                    checksum_state.set(match hash.to_string().as_str() {
                                        "e87062e5408447c77033feb8b8393c9b02e407e71aa0c9bb56b3339f6e47571e" => ChecksumStatus::DigimonWorld2003,
                                        "4838e14a32313e5b59ce613f9a9d72a8de762bec2dcdb46f085620283656b79f" => ChecksumStatus::DigimonWorld3US,
                                        "2e95551f709dfe8b3ac9bd245a4fb5dd036ed73baaa431f91ce03c119719a2e8" => ChecksumStatus::DigimonWorld3J,
                                        _ => ChecksumStatus::Unknown
                                    });
                                });
                            }
                        },
                    }
                    if let Some((string, color)) = checksum_status {
                        div {
                            style: "color: {color};",
                            "{string}"
                        }
                    }
                },
            }
        }
    }
}

#[derive(Parser)]
struct Args {
    source_bin: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    match &args.source_bin {
        Some(_) => todo!("need to make this"),
        None => {
            launch(app);
        }
    }
}
