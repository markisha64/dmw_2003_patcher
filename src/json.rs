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

impl Preset {
    pub fn count_enabled(&self) -> usize {
        [
            self.card_battle_disable,
            self.disable_script_items,
            self.fast_admin_center,
            self.fast_baronmon,
            self.fast_sepikmon,
            self.fast_start,
            self.folder_bag_cutscene_skip,
            self.no_counter_crest,
            self.no_running_away,
            self.post_game_unlock,
            self.forced_encounter_disable,
            self.random_encounter_disable,
            self.disable_fishing_kicking,
            self.fast_text,
            self.fixed_fields,
            self.improved_hp_proxy,
            self.ntsc,
            self.uncapped_dv_exp,
        ]
        .iter()
        .filter(|&&b| b)
        .count()
    }
}
