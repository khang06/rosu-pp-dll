use alloc::boxed::Box;
use rosu_pp::{Beatmap, GameMods, model::mode::GameMode};

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_parse(data: *const u8, len: usize) -> *mut Beatmap {
    let slice = unsafe { core::slice::from_raw_parts(data, len) };
    Box::into_raw(Box::new(Beatmap::from_bytes(slice)))
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_free(map: *mut Beatmap) {
    unsafe { core::mem::drop(Box::from_raw(map)) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_convert(map: *mut Beatmap, mode: u8, mods: u32) -> bool {
    unsafe {
        let map = &mut *map;
        map.convert_mut(GameMode::from(mode), &GameMods::from(mods))
            .is_ok()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_get_stars(map: *const Beatmap, mods: u32) -> f64 {
    let map = unsafe { &*map };
    let diff_attrs = rosu_pp::Difficulty::new()
        .mods(mods)
        .lazer(false)
        .calculate(map);

    diff_attrs.stars()
}

#[repr(C)]
pub struct RosuPPSummary {
    max_combo: u32,
    pp100: f64,
    pp98: f64,
    pp95: f64,
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_get_pp_summary(
    map: *const Beatmap,
    mods: u32,
    summary: *mut RosuPPSummary,
) {
    let map = unsafe { &*map };
    let diff_attrs = rosu_pp::Difficulty::new()
        .mods(mods)
        .lazer(false)
        .calculate(map);
    let max_combo = diff_attrs.max_combo();

    let mut perf_attrs = diff_attrs.performance().mods(mods).lazer(false).calculate();
    let pp100 = perf_attrs.pp();

    perf_attrs = perf_attrs
        .performance()
        .mods(mods)
        .lazer(false)
        .accuracy(98.0)
        .calculate();
    let pp98 = perf_attrs.pp();

    perf_attrs = perf_attrs
        .performance()
        .mods(mods)
        .lazer(false)
        .accuracy(95.0)
        .calculate();
    let pp95 = perf_attrs.pp();

    unsafe {
        *summary = RosuPPSummary {
            max_combo,
            pp100,
            pp98,
            pp95,
        };
    }
}
