#![feature(concat_idents, proc_macro_hygiene)]
use smashline::{
    *,
    api::*,
};
use smashline::smash::{
    app::{lua_bind::*, utility},
    lib::lua_const::*,
    phx::Hash40,
};
use smashline::smash_script::macros::*;

static mut PREV_DAMAGE: [f32; 8] = [0.0; 8];

unsafe extern "C" fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if entry_id >= 8 {
        return;
    }

    let current_damage = DamageModule::damage(fighter.module_accessor, 0);

    if current_damage > PREV_DAMAGE[entry_id] {
        let kind = utility::get_kind(&mut *fighter.module_accessor);
        let name = get_fighter_name_from_kind(kind);
        let voice_label = format!("vc_{}_damage01", name);
        PLAY_SE(fighter, Hash40::new(&voice_label));
    }

    PREV_DAMAGE[entry_id] = current_damage;
}

unsafe fn get_fighter_name_from_kind(kind: i32) -> &'static str {
    match kind {
        x if x == *FIGHTER_KIND_MARIO => "mario",
        x if x == *FIGHTER_KIND_DONKEY => "donkey",
        x if x == *FIGHTER_KIND_LINK => "link",
        x if x == *FIGHTER_KIND_SAMUS => "samus",
        x if x == *FIGHTER_KIND_SAMUSD => "samusd",
        x if x == *FIGHTER_KIND_YOSHI => "yoshi",
        x if x == *FIGHTER_KIND_KIRBY => "kirby",
        x if x == *FIGHTER_KIND_FOX => "fox",
        x if x == *FIGHTER_KIND_PIKACHU => "pikachu",
        x if x == *FIGHTER_KIND_LUIGI => "luigi",
        x if x == *FIGHTER_KIND_NESS => "ness",
        x if x == *FIGHTER_KIND_CAPTAIN => "captain",
        x if x == *FIGHTER_KIND_PURIN => "purin",
        x if x == *FIGHTER_KIND_PEACH => "peach",
        x if x == *FIGHTER_KIND_DAISY => "daisy",
        x if x == *FIGHTER_KIND_KOOPA => "koopa",
        x if x == *FIGHTER_KIND_POPO => "popo",
        x if x == *FIGHTER_KIND_NANA => "nana",
        x if x == *FIGHTER_KIND_SHEIK => "sheik",
        x if x == *FIGHTER_KIND_ZELDA => "zelda",
        x if x == *FIGHTER_KIND_MARIOD => "mariod",
        x if x == *FIGHTER_KIND_PICHU => "pichu",
        x if x == *FIGHTER_KIND_FALCO => "falco",
        x if x == *FIGHTER_KIND_MARTH => "marth",
        x if x == *FIGHTER_KIND_LUCINA => "lucina",
        x if x == *FIGHTER_KIND_YOUNGLINK => "younglink",
        x if x == *FIGHTER_KIND_GANON => "ganon",
        x if x == *FIGHTER_KIND_MEWTWO => "mewtwo",
        x if x == *FIGHTER_KIND_ROY => "roy",
        x if x == *FIGHTER_KIND_CHROM => "chrom",
        x if x == *FIGHTER_KIND_GAMEWATCH => "gamewatch",
        x if x == *FIGHTER_KIND_METAKNIGHT => "metaknight",
        x if x == *FIGHTER_KIND_PIT => "pit",
        x if x == *FIGHTER_KIND_PITB => "pitb",
        x if x == *FIGHTER_KIND_SZEROSUIT => "szerosuit",
        x if x == *FIGHTER_KIND_WARIO => "wario",
        x if x == *FIGHTER_KIND_SNAKE => "snake",
        x if x == *FIGHTER_KIND_IKE => "ike",
        x if x == *FIGHTER_KIND_PZENIGAME => "pzenigame",
        x if x == *FIGHTER_KIND_PFUSHIGISOU => "pfushigisou",
        x if x == *FIGHTER_KIND_PLIZARDON => "plizardon",
        x if x == *FIGHTER_KIND_DIDDY => "diddy",
        x if x == *FIGHTER_KIND_LUCAS => "lucas",
        x if x == *FIGHTER_KIND_SONIC => "sonic",
        x if x == *FIGHTER_KIND_DEDEDE => "dedede",
        x if x == *FIGHTER_KIND_PIKMIN => "pikmin",
        x if x == *FIGHTER_KIND_LUCARIO => "lucario",
        x if x == *FIGHTER_KIND_ROBOT => "robot",
        x if x == *FIGHTER_KIND_TOONLINK => "toonlink",
        x if x == *FIGHTER_KIND_WOLF => "wolf",
        x if x == *FIGHTER_KIND_MURABITO => "murabito",
        x if x == *FIGHTER_KIND_ROCKMAN => "rockman",
        x if x == *FIGHTER_KIND_WIIFIT => "wiifit",
        x if x == *FIGHTER_KIND_ROSETTA => "rosetta",
        x if x == *FIGHTER_KIND_LITTLEMAC => "littlemac",
        x if x == *FIGHTER_KIND_GEKKOUGA => "gekkouga",
        x if x == *FIGHTER_KIND_MIIFIGHTER => "miifighter",
        x if x == *FIGHTER_KIND_MIISWORDSMAN => "miiswordsman",
        x if x == *FIGHTER_KIND_MIIGUNNER => "miigunner",
        x if x == *FIGHTER_KIND_PALUTENA => "palutena",
        x if x == *FIGHTER_KIND_PACMAN => "pacman",
        x if x == *FIGHTER_KIND_REFLET => "reflet",
        x if x == *FIGHTER_KIND_SHULK => "shulk",
        x if x == *FIGHTER_KIND_KOOPAJR => "koopajr",
        x if x == *FIGHTER_KIND_DUCKHUNT => "duckhunt",
        x if x == *FIGHTER_KIND_RYU => "ryu",
        x if x == *FIGHTER_KIND_KEN => "ken",
        x if x == *FIGHTER_KIND_CLOUD => "cloud",
        x if x == *FIGHTER_KIND_KAMUI => "kamui",
        x if x == *FIGHTER_KIND_BAYONETTA => "bayonetta",
        x if x == *FIGHTER_KIND_INKLING => "inkling",
        x if x == *FIGHTER_KIND_RIDLEY => "ridley",
        x if x == *FIGHTER_KIND_SIMON => "simon",
        x if x == *FIGHTER_KIND_RICHTER => "richter",
        x if x == *FIGHTER_KIND_KROOL => "krool",
        x if x == *FIGHTER_KIND_SHIZUE => "shizue",
        x if x == *FIGHTER_KIND_GAOGAEN => "gaogaen",
        x if x == *FIGHTER_KIND_PACKUN => "packun",
        x if x == *FIGHTER_KIND_JACK => "jack",
        x if x == *FIGHTER_KIND_BRAVE => "brave",
        x if x == *FIGHTER_KIND_BUDDY => "buddy",
        x if x == *FIGHTER_KIND_DOLLY => "dolly",
        x if x == *FIGHTER_KIND_MASTER => "master",
        x if x == *FIGHTER_KIND_TANTAN => "tantan",
        x if x == *FIGHTER_KIND_PICKEL => "pickel",
        x if x == *FIGHTER_KIND_EDGE => "edge",
        x if x == *FIGHTER_KIND_EFLAME => "eflame",
        x if x == *FIGHTER_KIND_ELIGHT => "elight",
        x if x == *FIGHTER_KIND_DEMON => "demon",
        x if x == *FIGHTER_KIND_TRAIL => "trail",
        _ => "mario",
    }
}

#[skyline::main(name = "smash_damage_voice")]
pub fn main() {
    Agent::new("fighter")
        .on_line(Main, global_fighter_frame)
        .install();
}
