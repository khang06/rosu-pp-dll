use rosu_pp::{Beatmap, Difficulty, GameMods, GradualPerformance};
use rosu_pp::model::mode::GameMode;
use rosu_pp::any::{DifficultyAttributes, ScoreState, Strains};

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
#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_calc_pp_from_attrs_accuracy(
    attrs: *const DifficultyAttributes,
    mods: u32,
    lazer: bool,
    accuracy: f64,
    combo: u32,
    n_miss: u32,
) -> f64 {
    if attrs.is_null() {
        return 0.0;
    }

    let attrs = unsafe { &*attrs };
    attrs.clone()
        .performance()
        .mods(mods)
        .lazer(lazer)
        .combo(combo)
        .accuracy(accuracy)
        .misses(n_miss)
        .calculate()
        .pp()
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_calc_pp_from_attrs_taiko_100(
    attrs: *const DifficultyAttributes,
    mods: u32,
    lazer: bool,
    combo: u32,
    n100: u32,
    n_miss: u32,
) -> f64 {
    if attrs.is_null() {
        return 0.0;
    }

    let attrs = unsafe { &*attrs };
    attrs.clone()
        .performance()
        .mods(mods)
        .lazer(lazer)
        .combo(combo)
        .n100(n100)
        .misses(n_miss)
        .calculate()
        .pp()
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_create_gradual_performance(
    map: *const Beatmap,
    mods: u32,
    lazer: bool,
) -> *mut GradualPerformance {
    if map.is_null() {
        return core::ptr::null_mut();
    }

    let map = unsafe { &*map };
    let difficulty = Difficulty::new()
        .mods(mods)
        .lazer(lazer);
    let gradual = GradualPerformance::new(difficulty, map);

    Box::into_raw(Box::new(gradual))
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_free_gradual_performance(gradual: *mut GradualPerformance) {
    if gradual.is_null() {
        return;
    }

    unsafe { drop(Box::from_raw(gradual)) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_gradual_performance_remaining(gradual: *const GradualPerformance) -> u32 {
    if gradual.is_null() {
        return 0;
    }

    unsafe { (&*gradual).len().min(u32::MAX as usize) as u32 }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_gradual_performance_advance(
    gradual: *mut GradualPerformance,
    advance: u32,
    max_combo: u32,
    n300: u32,
    n100: u32,
    n50: u32,
    n_miss: u32,
    n_katu: u32,
    n_geki: u32,
    pp: *mut f64,
) -> bool {
    if gradual.is_null() || pp.is_null() || advance == 0 {
        return false;
    }

    let gradual = unsafe { &mut *gradual };
    let remaining = gradual.len();
    if remaining == 0 {
        return false;
    }

    let advance = (advance as usize).min(remaining);
    let mut state = ScoreState::new();
    state.max_combo = max_combo;
    state.n300 = n300;
    state.n100 = n100;
    state.n50 = n50;
    state.misses = n_miss;
    state.n_katu = n_katu;
    state.n_geki = n_geki;

    let Some(attrs) = gradual.nth(state, advance - 1) else {
        return false;
    };

    unsafe { *pp = attrs.pp() };
    true
}

#[repr(C)]
pub struct RosuStrainsResult {
    start_time: f64,
    section_len: f64,
    series: Vec<Vec<f64>>,
}

fn legacy_clock_rate(mods: u32) -> f64 {
    const DT: u32 = 1 << 6;
    const HT: u32 = 1 << 8;
    const NC: u32 = 1 << 9;

    if mods & (DT | NC) != 0 {
        1.5
    } else if mods & HT != 0 {
        0.75
    } else {
        1.0
    }
}

fn strain_start_time(map: &Beatmap, section_len: f64, clock_rate: f64) -> f64 {
    let Some(first) = map.hit_objects.first() else {
        return 0.0;
    };

    let first_time = first.start_time / clock_rate;
    ((first_time / section_len).ceil() * section_len - section_len) * clock_rate
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_calc_strains(map: *const Beatmap, mods: u32) -> *mut RosuStrainsResult {
    if map.is_null() {
        return core::ptr::null_mut();
    }

    let map = unsafe { &*map };
    let strains = Difficulty::new()
        .mods(mods)
        .lazer(false)
        .strains(map);

    let raw_section_len = strains.section_len();
    let clock_rate = legacy_clock_rate(mods);
    let start_time = strain_start_time(map, raw_section_len, clock_rate);
    let section_len = raw_section_len * clock_rate;

    let series = match strains {
        Strains::Osu(s) => vec![s.aim, s.aim_no_sliders, s.speed, s.flashlight],
        Strains::Taiko(s) => vec![s.color, s.reading, s.rhythm, s.stamina, s.single_color_stamina],
        Strains::Catch(s) => vec![s.movement],
        Strains::Mania(s) => vec![s.strains],
    };

    Box::into_raw(Box::new(RosuStrainsResult {
        start_time,
        section_len,
        series,
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_free_strains(strains: *mut RosuStrainsResult) {
    if strains.is_null() {
        return;
    }

    unsafe { drop(Box::from_raw(strains)) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_strains_start_time(strains: *const RosuStrainsResult) -> f64 {
    if strains.is_null() {
        return 0.0;
    }

    unsafe { (&*strains).start_time }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_strains_section_len(strains: *const RosuStrainsResult) -> f64 {
    if strains.is_null() {
        return 0.0;
    }

    unsafe { (&*strains).section_len }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_strains_series_count(strains: *const RosuStrainsResult) -> u32 {
    if strains.is_null() {
        return 0;
    }

    unsafe { (&*strains).series.len().min(u32::MAX as usize) as u32 }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_strains_series_len(strains: *const RosuStrainsResult, series: u32) -> u32 {
    if strains.is_null() {
        return 0;
    }

    unsafe {
        (&*strains)
            .series
            .get(series as usize)
            .map_or(0, |values| values.len().min(u32::MAX as usize) as u32)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_strains_series_values(
    strains: *const RosuStrainsResult,
    series: u32,
) -> *const f64 {
    if strains.is_null() {
        return core::ptr::null();
    }

    unsafe {
        (&*strains)
            .series
            .get(series as usize)
            .map_or(core::ptr::null(), |values| values.as_ptr())
    }
}

