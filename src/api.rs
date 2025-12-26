use rosu_pp::{Beatmap, Difficulty, GameMods};
use rosu_pp::model::mode::GameMode;
use rosu_pp::any::DifficultyAttributes;

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_parse(data: *const u8, len: u32) -> *mut Beatmap {
    if data.is_null() || len == 0 {
        return core::ptr::null_mut();
    }

    let slice = unsafe { core::slice::from_raw_parts(data, len as usize) };
    let Ok(beatmap) = Beatmap::from_bytes(slice) else {
        return core::ptr::null_mut();
    };

    Box::into_raw(Box::new(beatmap))
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_free(map: *mut Beatmap) {
    if map.is_null() {
        return;
    }

    unsafe { drop(Box::from_raw(map)) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_convert(map: *mut Beatmap, mode: u8, mods: u32) -> bool {
    if map.is_null() {
        return false;
    }

    unsafe {
        let map = &mut *map;
        map.convert_mut(GameMode::from(mode), &GameMods::from(mods))
            .is_ok()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_get_stars(map: *const Beatmap, mods: u32) -> f64 {
    if map.is_null() {
        return 0.0;
    }

    let map = unsafe { &*map };

    Difficulty::new()
        .mods(mods)
        .lazer(false)
        .calculate(map)
        .stars()
}

#[repr(C)]
pub struct RosuPPSummary {
    pub max_combo: u32,
    pub pp100: f64,
    pub pp98: f64,
    pub pp95: f64,
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_get_pp_summary(
    map: *const Beatmap,
    mods: u32,
    summary: *mut RosuPPSummary,
) {
    if map.is_null() || summary.is_null() {
        return;
    }

    let map = unsafe { &*map };

    let diff_attrs = Difficulty::new()
        .mods(mods)
        .lazer(false)
        .calculate(map);

    let max_combo = diff_attrs.max_combo();

    // 100%
    let pp100 = diff_attrs
        .clone()
        .performance()
        .mods(mods)
        .lazer(false)
        .calculate()
        .pp();

    // 98%
    let pp98 = diff_attrs
        .clone()
        .performance()
        .mods(mods)
        .lazer(false)
        .accuracy(98.0)
        .calculate()
        .pp();

    // 95%
    let pp95 = diff_attrs
        .performance()
        .mods(mods)
        .lazer(false)
        .accuracy(95.0)
        .calculate()
        .pp();

    unsafe {
        *summary = RosuPPSummary {
            max_combo,
            pp100,
            pp98,
            pp95,
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_calc_difficulty_attrs(
    map: *const Beatmap,
    mods: u32,
    lazer: bool,
) -> *mut DifficultyAttributes {
    if map.is_null() {
        return core::ptr::null_mut();
    }

    let map = unsafe { &*map };

    let attrs = Difficulty::new()
        .mods(mods)
        .lazer(lazer)
        .calculate(map);

    Box::into_raw(Box::new(attrs))
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_free_difficulty_attrs(attrs: *mut DifficultyAttributes) {
    if attrs.is_null() {
        return;
    }

    unsafe { drop(Box::from_raw(attrs)) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_attrs_max_combo(attrs: *const DifficultyAttributes) -> u32 {
    if attrs.is_null() {
        return 0;
    }

    unsafe { (&*attrs).max_combo() }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_attrs_stars(attrs: *const DifficultyAttributes) -> f64 {
    if attrs.is_null() {
        return 0.0;
    }

    unsafe { (&*attrs).stars() }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_calc_pp_from_attrs(
    attrs: *const DifficultyAttributes,
    mods: u32,
    lazer: bool,
    passed_objects: u32,
    combo: u32,
    n300: u32,
    n100: u32,
    n50: u32,
    n_miss: u32,
    n_katu: u32,
    n_geki: u32,
) -> f64 {
    if attrs.is_null() {
        return 0.0;
    }

    let attrs = unsafe { &*attrs };

    attrs.clone()
        .performance()
        .mods(mods)
        .lazer(lazer)
        .passed_objects(passed_objects)
        .combo(combo)
        .n300(n300)
        .n100(n100)
        .n50(n50)
        .n_katu(n_katu)
        .n_geki(n_geki)
        .misses(n_miss)
        .calculate()
        .pp()
}