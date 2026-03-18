use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Preset {
    pub card_battle_disable: bool,
    pub disable_script_items: bool,
    pub fast_admin_center: bool,
    pub fast_baronmon: bool,
    pub fast_sepikmon: bool,
    pub fast_start: bool,
    pub folder_bag_cutscene_skip: bool,
    pub no_counter_crest: bool,
    pub no_running_away: bool,
    pub post_game_unlock: bool,
    pub forced_encounter_disable: bool,
    pub random_encounter_disable: bool,
    pub disable_fishing_kicking: bool,
    pub fast_text: bool,
    pub fixed_fields: bool,
    pub improved_hp_proxy: bool,
    pub ntsc: bool,
    pub uncapped_dv_exp: bool,
}

impl Default for Preset {
    fn default() -> Self {
        Preset {
            card_battle_disable: false,
            disable_script_items: false,
            fast_admin_center: false,
            fast_baronmon: false,
            fast_sepikmon: false,
            fast_start: false,
            folder_bag_cutscene_skip: false,
            no_counter_crest: false,
            no_running_away: false,
            post_game_unlock: false,
            forced_encounter_disable: false,
            random_encounter_disable: false,
            disable_fishing_kicking: false,
            fast_text: false,
            fixed_fields: true,
            improved_hp_proxy: false,
            ntsc: true,
            uncapped_dv_exp: false,
        }
    }
}
