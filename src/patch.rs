use dioxus::prelude::*;

use crate::{
    RomState,
    json::{self, Preset},
    mkpsxiso,
};

#[component]
pub fn patch() -> Element {
    let rom_state = use_context::<Signal<RomState>>();
    let preset_state = use_context::<Signal<Preset>>();

    let rom = rom_state();
    let preset = preset_state();

    rsx! {
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
                        mkpsxiso::extract(&file_path).await.unwrap();
                    });


                }
            }
        }
    }
}
