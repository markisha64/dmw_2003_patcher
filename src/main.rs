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

use crate::json::Preset;

mod checkbox;
mod json;
mod mkpsxiso;
mod patch;

#[derive(Clone, Copy)]
enum ChecksumStatus {
    DigimonWorld2003,
    DigimonWorld3US,
    DigimonWorld3J,
    Unknown,
    Checking,
}

#[derive(Clone)]
pub struct RomState {
    pub source_bin: Option<PathBuf>,
}

const BG: Asset = asset!(
    "assets/bg.png",
    AssetOptions::builder().with_hash_suffix(false)
);

fn app() -> Element {
    let _ = format!("{}", BG);
    use_context_provider(|| Signal::new(RomState { source_bin: None }));
    use_context_provider(|| Signal::new(Preset::default()));

    let mut rom_state = use_context::<Signal<RomState>>();
    let mut checksum_state = use_signal(|| ChecksumStatus::Checking);
    let mut preset_state = use_context::<Signal<Preset>>();

    let rom = rom_state();
    let checksum = checksum_state();
    let preset = preset_state();

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
            document::Stylesheet { href: asset!("../assets/style.css") }
            div { class: "inline",
                div { class: "center",
                    label { class: "file-upload", r#for: "file-upload", "{file_name_cl}" }
                    input {
                        id: "file-upload",
                        r#type: "file",
                        accept: ".bin",
                        onchange: move |x: Event<FormData>| {
                            if let Some(file) = &x.files().first() {
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
                            div { style: "color: yellow;", "Checksum Check" }
                        }
                    }
                    patch::patch {}
                }
            }
        }
        div { class: "left",
            div { class: "column",
                div { class: "segment",
                    div { "Quality Of Life" }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "NTSC",
                            tooltip: "Makes The Game Run At 60fps NTSC",
                            id: "ntsc_checkbox",
                            checked: preset.ntsc,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().ntsc = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Fast Admin Center",
                            tooltip: "Skips The Staff Pass Quest",
                            id: "fast_admin_center_checkbox",
                            checked: preset.fast_admin_center,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().fast_admin_center = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Fast Baronmon",
                            tooltip: "TNT Chip Baronmon Appears Right After Admin Center",
                            id: "fast_baronmon_checkbox",
                            checked: preset.fast_baronmon,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().fast_baronmon = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Fast Sepikmon",
                            tooltip: "Skips The Sepikmon Quest, Just Go Back To Jungle Grave",
                            id: "fast_sepikmon_checkbox",
                            checked: preset.fast_sepikmon,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().fast_sepikmon = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Fast Start",
                            tooltip: "Skips Most Of The Starting Cutscenes",
                            id: "fast_start_checkbox",
                            checked: preset.fast_start,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().fast_start = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Folder Bag Cutscene Skip",
                            tooltip: "Skips The Folder Bag Cutscene (Still Need To Talk To Divermon)",
                            id: "folder_bag_cutscene_skip_checkbox",
                            checked: preset.folder_bag_cutscene_skip,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().folder_bag_cutscene_skip = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Post Game Unlock",
                            tooltip: "No Longer Need To Get Legendary Weapons To Finish Post Game",
                            id: "post_game_unlock_checkbox",
                            checked: preset.post_game_unlock,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().post_game_unlock = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Fast Text",
                            tooltip: "Inbuilt Turbo For Textboxes (Not Recommended)",
                            id: "fast_text_checkbox",
                            checked: preset.fast_text,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().fast_text = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Fixed Fields",
                            tooltip: "Fixes Bugged Field Moves",
                            id: "fixed_fields_checkbox",
                            checked: preset.fixed_fields,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().fixed_fields = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Improved Hp Proxy",
                            tooltip: "Instead Of Reducing Damage By 10/20, Decreases It By 10%/20%",
                            id: "improved_hp_proxy_checkbox",
                            checked: preset.improved_hp_proxy,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().improved_hp_proxy = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Uncapped DV EXP",
                            tooltip: "You're Usually Limited To 1 Digivolution Level Up Per Battle, Removes This Cap",
                            id: "uncapped_dv_exp_checkbox",
                            checked: preset.uncapped_dv_exp,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().uncapped_dv_exp = x;
                            },
                        }
                    }
                }
            }
            div { class: "column",
                div { class: "segment",
                    div { "Ironmon" }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Card Battle Disable",
                            tooltip: "You Can No Longer Do Card Battles",
                            id: "card_battle_disable_checkbox",
                            checked: preset.card_battle_disable,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().card_battle_disable = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "No Counter Crest",
                            tooltip: "Removes Counter Crest From Drops",
                            id: "no_counter_crest_checkbox",
                            checked: preset.no_counter_crest,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().no_counter_crest = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "No Running Away",
                            tooltip: "Neither You Or Enemy Digimon Can Run Away",
                            id: "no_running_away_checkbox",
                            checked: preset.no_running_away,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().no_running_away = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Forced Encounter Disable",
                            tooltip: "Disables Forced Encounters (Like East Wire Forest Kuwagamon)",
                            id: "forced_encounter_disable_checkbox",
                            checked: preset.forced_encounter_disable,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().forced_encounter_disable = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Random Encounter Disable",
                            tooltip: "Disables Random Encounters",
                            id: "random_encounter_disable_checkbox",
                            checked: preset.random_encounter_disable,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().random_encounter_disable = x;
                            },
                        }
                    }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Disable Fishing And Kicking",
                            tooltip: "You Can No Longer Fish Or Kick Trees",
                            id: "disable_fishing_kicking_checkbox",
                            checked: preset.disable_fishing_kicking,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().disable_fishing_kicking = x;
                            },
                        }
                    }
                }
                div { class: "segment",
                    div { "Other" }
                    div { class: "left",
                        checkbox::checkbox {
                            label: "Disable Script Items",
                            tooltip: "Scripts No Longer Give Items And Bits",
                            id: "disable_script_items_checkbox",
                            checked: preset.disable_script_items,
                            disabled: false,
                            onchange: move |x: bool| {
                                preset_state.write().disable_script_items = x;
                            },
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
