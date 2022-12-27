pub(crate) mod english;
pub(crate) mod german;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Lang {
    De,
    En,
}

impl Default for Lang {
    fn default() -> Self {
        Self::En
    }
}

macro_rules! t {
    ($lng: expr, $target: ident) => {
        match $lng {
            Lang::En => &english::$target,
            Lang::De => &german::$target,
        }
    };

    ($name: ident, $len: expr, $target: ident) => {
        pub fn $name(lng: Lang) -> &'static [&'static str; $len] {
            t!(lng, $target)
        }
    };

    ($name: ident => $target: ident) => {
        pub fn $name(lng: Lang) -> &'static str {
            t!(lng, $target)
        }
    };
}

t!(petrol, 3, PETROL);
t!(printed_part, 3, GEDRUCKTESTEIL);
t!(super_glue, 3, SUPER_GLUE);
t!(machine_names, 7, MACHINE_NAMES);
t!(game_info, 1, GAME_INFO);
t!(mars_info, 5, MARS_INFO);
t!(nasa_info, 5, NASA_INFO);
t!(warnings, 4, WARNINGS);
t!(button_text, 4, BUTTON_TEXT);
t!(trade_conflict_popup, 1, TRADE_CONFLICT_POPUP);
t!(
    first_milestone_handbook_text,
    10,
    FIRST_MILESTONE_HANDBOOK_TEXT
);
t!(
    second_milestone_handbook_text,
    7,
    SECOND_MILESTONE_HANDBOOK_TEXT
);
t!(time_name, 1, TIME_NAME);
t!(informations_popup_mars, 2, INFORMATIONS_POPUP_MARS);
t!(informations_popup_nasa, 2, INFORMATIONS_POPUP_NASA);
t!(sandstorm, 2, SANDSTORM);
t!(cometa_strike, 2, COMETA_STRIKE);
t!(power_failure, 2, POWER_FAILURE);
t!(resource_name, 3, RESOURCE_NAME);

t!(button_info => BUTTON_INFO);
t!(winning_text => WINNING_TEXT);
t!(additional_info_string => ADDITIONAL_INFO_STRING);
t!(resume_error_string => RESUME_ERROR_STRING);
t!(air_string => AIR_STRING);
t!(energy_string => ENERGY_STRING);
t!(air_and_energy_string => AIR_AND_ENERGY_STRING);
t!(death_reason_string => DEATH_REASON_STRING);
t!(intro_text => INTRO_TEXT);
t!(tutorial_text => TUTORIAL_TEXT);
t!(send_msg_failure => SEND_MSG_FAILURE);
