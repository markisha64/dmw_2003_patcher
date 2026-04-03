use std::path::PathBuf;

use base64::prelude::*;
use create_dmw_2003_patch::{Patch, PatchJSON};
use dioxus::prelude::*;
use once_cell::sync::Lazy;
use tokio::fs::{self, create_dir_all};

use crate::{Args, InfoState, json::Preset, mkpsxiso};

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

macro_rules! update_count {
    ($count:ident, $max_count:expr, $state:expr) => {{
        $count += 1;
        $state.set(Some(100 * $count / $max_count));
    }};
}

#[component]
pub fn patch() -> Element {
    let args_state = use_context::<Signal<Args>>();
    let preset_state = use_context::<Signal<Preset>>();
    let mut info_state = use_context::<Signal<InfoState>>();
    let mut randomizing_state: Signal<Option<i32>> = use_signal(|| None);

    let args = args_state();
    let preset = preset_state();
    let info = info_state();
    let randomizing = randomizing_state();

    rsx! {
        div { class: "column-no-stretch",
            label { r#for: "patch", class: "patch",
                if let Some(percent) = randomizing {
                    div { r#style: "height: 100%; width:{percent}%;",
                        div { class: "progress" }
                    }
                } else {
                    "Patch"
                }
            }
            input {
                r#type: "button",
                id: "patch",
                onclick: move |_| {
                    to_owned![args];

                    if randomizing.is_some() {
                        return;
                    }

                    let max_count = (preset.count_enabled() + 2) as i32;
                    let mut count = 0i32;

                    if max_count == 2 {
                        info_state
                            .set(InfoState {
                                info: Some("No Patches Selected".to_string()),
                            });
                        return;
                    }
                    match args.source_bin {
                        Some(file_path) => {
                            randomizing_state.set(Some(100 * count / max_count));
                            info_state.set(InfoState { info: None });
                            spawn(async move {
                                let task: Result<(), anyhow::Error> = async move {
                                    update_count!(count, max_count, randomizing_state);
                                    mkpsxiso::extract(&file_path).await?;
                                    let rom_name = file_path
                                        .file_name()
                                        .context("Failed file name get")?
                                        .to_str()
                                        .context("Failed to_str conversion")?;
                                    if preset.fast_text {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&FAST_TEXT_PATCH, rom_name).await?;
                                    }
                                    if preset.fixed_fields {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&FIXED_FIELD_PATCH, rom_name).await?;
                                    }
                                    if preset.improved_hp_proxy {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&IMPROVED_HP_PROXY_PATCH, rom_name).await?;
                                    }
                                    if preset.ntsc {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&NTSC_PATCH, rom_name).await?;
                                    }
                                    if preset.uncapped_dv_exp {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&UNCAPPED_DV_EXP_PATCH, rom_name).await?;
                                    }
                                    if preset.card_battle_disable {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&CARD_BATTLE_DISABLE_PATCH, rom_name).await?;
                                    }
                                    if preset.disable_script_items {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&DISABLE_SCRIPT_ITEMS_PATCH, rom_name).await?;
                                    }
                                    if preset.fast_admin_center {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&FAST_ADMIN_CENTER_PATCH, rom_name).await?;
                                    }
                                    if preset.fast_baronmon {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&FAST_BARONMON_PATCH, rom_name).await?;
                                    }
                                    if preset.fast_sepikmon {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&FAST_SEPIKMON_PATCH, rom_name).await?;
                                    }
                                    if preset.fast_start {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&FAST_START_PATCH, rom_name).await?;
                                    }
                                    if preset.disable_fishing_kicking {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&FISHING_KICKING_DISABLE_PATCH, rom_name).await?;
                                    }
                                    if preset.folder_bag_cutscene_skip {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&FOLDER_BAG_CUTSCENE_SKIP_PATCH, rom_name)
                                            .await?;
                                    }
                                    if preset.forced_encounter_disable {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&FORCED_ENCOUNTERS_DISABLE_PATCH, rom_name)
                                            .await?;
                                    }
                                    if preset.no_counter_crest {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&NO_CC_PATCH, rom_name).await?;
                                    }
                                    if preset.no_running_away {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&NO_RUNNING_AWAY_PATCH, rom_name).await?;
                                    }
                                    if preset.post_game_unlock {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&POST_GAME_UNLOCK_PATCH, rom_name).await?;
                                    }
                                    if preset.random_encounter_disable {
                                        update_count!(count, max_count, randomizing_state);
                                        apply_patch(&RANDOM_ENCOUNTERS_DISABLE_PATCH, rom_name)
                                            .await?;
                                    }
                                    update_count!(count, max_count, randomizing_state);
                                    let filename = args.filename.unwrap_or("default".to_string());
                                    create_dir_all(format!("patched/{}/{}", rom_name, filename))
                                        .await?;
                                    mkpsxiso::build(&rom_name, filename).await?;
                                    update_count!(count, max_count, randomizing_state);
                                    Ok(())
                                }
                                    .await;
                                if let Err(err) = task {
                                    info_state
                                        .set(InfoState {
                                            info: Some(err.to_string()),
                                        });
                                }
                                randomizing_state.set(None);
                            });
                        }
                        None => {
                            info_state
                                .set(InfoState {
                                    info: Some("No ROM Selected".to_string()),
                                });
                        }
                    }
                },
            }
            if let Some(info) = info.info {
                span { style: "text-align: center; color: red;", "Err: {info}" }
            }
        }
    }
}
