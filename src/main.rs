use clap::Parser;
use std::path::PathBuf;
use tokio::io::AsyncReadExt as _;

use dioxus::{
    desktop::{
        Config, WindowBuilder,
        wry::dpi::{PhysicalSize, Size},
    },
    prelude::*,
};

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
        ChecksumStatus::DigimonWorld2003 => (
            "✓ Digimon World 2003",
            "lawngreen",
            "Perfect ROM For Patching",
        ),
        ChecksumStatus::DigimonWorld3US => (
            "x Digimon World 3 (US)",
            "red",
            "The Patches Won't Work With This ROM",
        ),
        ChecksumStatus::DigimonWorld3J => (
            "x Digimon World 3 (Japan)",
            "red",
            "The Patches Won't Work With This ROM",
        ),
        ChecksumStatus::Unknown => (
            "⚠ Unknown Checksum",
            "yellow",
            "Unknown ROM, Proceed With Caution",
        ),
        ChecksumStatus::Checking => (
            "⚠ Verifying Checksum",
            "yellow",
            "Generating Checksum To Verify ROM",
        ),
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
                                            let mut file = tokio::fs::File::open(fpath).await.unwrap();
                                            let mut hasher = blake3::Hasher::new();
                                            let mut buffer = [0u8; 1024 * 512];
                                            loop {
                                                let n = file.read(&mut buffer).await.unwrap();
                                                if n == 0 {
                                                    break;
                                                }
                                                hasher.update(&buffer[..n]);
                                            }
                                            hasher.finalize()
                                        })
                                        .await
                                        .unwrap();
                                    checksum_state
                                        .set(
                                            match hash.to_string().as_str() {
                                                "e87062e5408447c77033feb8b8393c9b02e407e71aa0c9bb56b3339f6e47571e" => {
                                                    ChecksumStatus::DigimonWorld2003
                                                }
                                                "4838e14a32313e5b59ce613f9a9d72a8de762bec2dcdb46f085620283656b79f" => {
                                                    ChecksumStatus::DigimonWorld3US
                                                }
                                                "2e95551f709dfe8b3ac9bd245a4fb5dd036ed73baaa431f91ce03c119719a2e8" => {
                                                    ChecksumStatus::DigimonWorld3J
                                                }
                                                _ => ChecksumStatus::Unknown,
                                            },
                                        );
                                });
                            }
                        },
                    }
                    div { class: "segment tooltip",
                        if let Some((string, color, tooltip)) = checksum_status {
                            div { style: "color: {color};",
                                div { class: "tooltiptext", "{tooltip}" }
                                "{string}"
                            }
                        } else {
                            div { style: "color: yellow;",
                                "Checksum Will Be Checked After You Select A ROM"
                            }
                        }
                    }
                }
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
            LaunchBuilder::desktop()
                .with_cfg(
                    Config::default().with_window(
                        WindowBuilder::new()
                            .with_resizable(true)
                            .with_inner_size(Size::Physical(PhysicalSize {
                                width: 800,
                                height: 800,
                            })),
                    ),
                )
                .launch(app);
        }
    }
}
