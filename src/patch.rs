use std::path::PathBuf;

use base64::prelude::*;
use create_dmw_2003_patch::{Patch, PatchJSON};
use dioxus::prelude::*;
use once_cell::sync::Lazy;
use tokio::fs::{self, create_dir_all};

use crate::{RomState, json::Preset, mkpsxiso};

async fn patch_file(patch: &Patch, rom_name: &str) -> anyhow::Result<()> {
    let mut path = PathBuf::from("./extract");

    path.push(rom_name);
    path.push(&patch.target);

    let patch = BASE64_STANDARD.decode(&patch.patch)?;
    let source_file = fs::read(&path).await?;

    let output = flips::IpsPatch::new(patch).apply(source_file)?;

    fs::write(path, output).await?;

    Ok(())
}

async fn apply_patch(patch_json: &PatchJSON, rom_name: &str) -> anyhow::Result<()> {
    for patch in &patch_json.changes {
        patch_file(&patch, rom_name).await?;
    }

    Ok(())
}

static FAST_TEXT_PATCH: Lazy<PatchJSON> =
    Lazy::new(|| serde_json::from_str(include_str!("../Dmw2003FastText/patcher.json")).unwrap());
static FIXED_FIELD_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw2003fixed_fields/patcher.json")).unwrap()
});
static IMPROVED_HP_PROXY_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw2003improved_hp_proxy/patcher.json")).unwrap()
});
static NTSC_PATCH: Lazy<PatchJSON> =
    Lazy::new(|| serde_json::from_str(include_str!("../dmw2003NTSC/patcher.json")).unwrap());
static UNCAPPED_DV_EXP_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw2003uncapped_dv_exp/patcher.json")).unwrap()
});
static CARD_BATTLE_DISABLE_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw_2003_card_battle_disable/patcher.json")).unwrap()
});
static DISABLE_SCRIPT_ITEMS_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!(
        "../dmw_2003_disable_script_items/patcher.json"
    ))
    .unwrap()
});
static FAST_ADMIN_CENTER_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw_2003_fast_admin_center/patcher.json")).unwrap()
});
static FAST_BARONMON_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw_2003_fast_baronmon/patcher.json")).unwrap()
});
static FAST_SEPIKMON_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw_2003_fast_sepikmon/patcher.json")).unwrap()
});
static FAST_START_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw_2003_fast_start/patcher.json")).unwrap()
});
static FISHING_KICKING_DISABLE_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!(
        "../dmw_2003_fishing_kicking_disable/patcher.json"
    ))
    .unwrap()
});
static FOLDER_BAG_CUTSCENE_SKIP_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!(
        "../dmw_2003_folder_bag_cutscene_skip/patcher.json"
    ))
    .unwrap()
});
static FORCED_ENCOUNTERS_DISABLE_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!(
        "../dmw_2003_forced_encounters_disable/patcher.json"
    ))
    .unwrap()
});
static NO_CC_PATCH: Lazy<PatchJSON> =
    Lazy::new(|| serde_json::from_str(include_str!("../dmw_2003_no_cc/patcher.json")).unwrap());
static NO_RUNNING_AWAY_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw_2003_no_running_away/patcher.json")).unwrap()
});
static POST_GAME_UNLOCK_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!("../dmw_2003_post_game_unlock/patcher.json")).unwrap()
});
static RANDOM_ENCOUNTERS_DISABLE_PATCH: Lazy<PatchJSON> = Lazy::new(|| {
    serde_json::from_str(include_str!(
        "../dmw_2003_random_encounters_disable/patcher.json"
    ))
    .unwrap()
});

#[component]
pub fn patch() -> Element {
    let rom_state = use_context::<Signal<RomState>>();
    let preset_state = use_context::<Signal<Preset>>();
    let mut info_state: Signal<Option<String>> = use_signal(|| None);

    let rom = rom_state();
    let preset = preset_state();
    let info = info_state();

    rsx! {
        div { class: "column",
            label { r#for: "patch", class: "patch", "Patch" }
            input {
                r#type: "button",
                id: "patch",
                onclick: move |_| {
                    to_owned![rom];

                    if let Some(file_path) = rom.source_bin {
                        spawn(async move {
                            let task: Result<(), anyhow::Error> = async move {
                                info_state.set(Some("Extracting".to_string()));

                                mkpsxiso::extract(&file_path).await?;

                                info_state.set(Some("Building".to_string()));

                                let rom_name = file_path
                                    .file_name()
                                    .context("Failed file name get")?
                                    .to_str()
                                    .context("Failed to_str conversion")?;
                                if preset.fast_text {
                                    apply_patch(&FAST_TEXT_PATCH, rom_name).await?;
                                }
                                if preset.fixed_fields {
                                    apply_patch(&FIXED_FIELD_PATCH, rom_name).await?;
                                }
                                if preset.improved_hp_proxy {
                                    apply_patch(&IMPROVED_HP_PROXY_PATCH, rom_name).await?;
                                }
                                if preset.ntsc {
                                    apply_patch(&NTSC_PATCH, rom_name).await?;
                                }
                                if preset.uncapped_dv_exp {
                                    apply_patch(&UNCAPPED_DV_EXP_PATCH, rom_name).await?;
                                }
                                if preset.card_battle_disable {
                                    apply_patch(&CARD_BATTLE_DISABLE_PATCH, rom_name).await?;
                                }
                                if preset.disable_script_items {
                                    apply_patch(&DISABLE_SCRIPT_ITEMS_PATCH, rom_name).await?;
                                }
                                if preset.fast_admin_center {
                                    apply_patch(&FAST_ADMIN_CENTER_PATCH, rom_name).await?;
                                }
                                if preset.fast_baronmon {
                                    apply_patch(&FAST_BARONMON_PATCH, rom_name).await?;
                                }
                                if preset.fast_sepikmon {
                                    apply_patch(&FAST_SEPIKMON_PATCH, rom_name).await?;
                                }
                                if preset.fast_start {
                                    apply_patch(&FAST_START_PATCH, rom_name).await?;
                                }
                                if preset.disable_fishing_kicking {
                                    apply_patch(&FISHING_KICKING_DISABLE_PATCH, rom_name).await?;
                                }
                                if preset.folder_bag_cutscene_skip {
                                    apply_patch(&FOLDER_BAG_CUTSCENE_SKIP_PATCH, rom_name).await?;
                                }
                                if preset.forced_encounter_disable {
                                    apply_patch(&FORCED_ENCOUNTERS_DISABLE_PATCH, rom_name).await?;
                                }
                                if preset.no_counter_crest {
                                    apply_patch(&NO_CC_PATCH, rom_name).await?;
                                }
                                if preset.no_running_away {
                                    apply_patch(&NO_RUNNING_AWAY_PATCH, rom_name).await?;
                                }
                                if preset.post_game_unlock {
                                    apply_patch(&POST_GAME_UNLOCK_PATCH, rom_name).await?;
                                }
                                if preset.random_encounter_disable {
                                    apply_patch(&RANDOM_ENCOUNTERS_DISABLE_PATCH, rom_name).await?;
                                }
                                create_dir_all(format!("patched/{}/testing", rom_name)).await?;
                                mkpsxiso::build(&rom_name, "testing").await?;
                                info_state.set(Some("Done".to_string()));
                                Ok(())
                            }
                                .await;
                            if let Err(err) = task {
                                info_state.set(Some(format!("{}", err)));
                            }
                        });
                    }
                },
            }
            if let Some(info) = info {
                span { style: "text-align: center;", "{info}" }
            }
        }
    }
}
