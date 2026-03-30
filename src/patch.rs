use dioxus::prelude::*;
use tokio::fs::create_dir_all;

use crate::{
    RomState,
    json::{self, Preset},
    mkpsxiso,
};

#[component]
pub fn patch() -> Element {
    let rom_state = use_context::<Signal<RomState>>();
    let preset_state = use_context::<Signal<Preset>>();
    let mut info_state: Signal<Option<String>> = use_signal(|| None);

    let rom = rom_state();
    let preset = preset_state();
    let info = info_state();

    rsx! {
        div {
            class: "column",
            label {
                r#for: "patch",
                class: "patch",
                "Patch"
            },
            input {
                r#type: "button",
                id: "patch",
                onclick: move |_| {
                    to_owned![rom];

                    if let Some(file_path) = rom.source_bin {
                        spawn(async move {
                            info_state.set(Some("Extracting".to_string()));

                            mkpsxiso::extract(&file_path).await.unwrap();

                            info_state.set(Some("Building".to_string()));

                            println!("here");
                            let rom_name = file_path
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap();
                            println!("here 2");

                            create_dir_all(format!("patched/{}/testing", rom_name)).await.unwrap();

                            mkpsxiso::build(&rom_name, "testing").await.unwrap();

                            info_state.set(Some("Done".to_string()));
                        });
                    }
                }
            }
            if let Some(info) = info {
                span {
                    style: "text-align: center;",
                    "{info}"
                }
            }
        }
    }
}
