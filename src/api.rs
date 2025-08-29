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
        let mode = GameMode::from(mode);
        let mods = GameMods::from(mods);
        map.convert_mut(mode, &mods).is_ok()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rosu_pp_get_stars(map: *mut Beatmap, mods: u32, pp: *mut f64) -> f64 {
    let map = unsafe { &mut *map };
    let diff_attrs = rosu_pp::Difficulty::new().mods(mods).calculate(&map);
    let stars = diff_attrs.stars();

    if let Some(pp) = unsafe { pp.as_mut() } {
        let difficulty = diff_attrs.performance().mods(mods).calculate();
        *pp = difficulty.pp();
    }

    stars
}
