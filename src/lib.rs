extern "C" {
    fn map_driver(data: *const u8, free: bool, mdl_mode: bool, pass_allocation_ptr: bool, exit_code: *mut i32, param1: u64, param2: u64) -> i32;
}

/// Maps a driver into kernel memory and calls the entry point with parameters param1 and param2
///
/// If successful, the function will return Some with the base address the driver was mapped to and the return value of the driver, respectively
///
/// If free is set to true, the driver will automatically be unmapped after calling the entry point
/// If map_mdl is set to true, the driver will be mapped in mdl memory, avoiding being in BigPoolTable (this is recommended as BigPoolTable is detected in some kernel anticheats)
/// If pass_allocation_ptr is set to true, the driver will be called with the allocation ptr as the first param
///
/// # Safety
/// This function can cause blue screens if given malformed input or the driver causes an exception in the entrypoint, so this function is marked unsafe
pub unsafe fn kdmapper(driver: &[u8], free: bool, map_mdl: bool, pass_allocation_ptr: bool, param1: u64, param2: u64) -> Option<(i32, i32)> {
    let mut exit = 0;
    let result = map_driver(driver.as_ptr(), free, map_mdl, pass_allocation_ptr, &mut exit, param1, param2);
    match result {
        0 => None,
        n => Some((n, exit))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn kdmapper() {
        unsafe {
            let driver = include_bytes!("../kdmapper/HelloWorld.sys");
            let (_, exit) = super::kdmapper(driver.as_slice(), false, true, false, 0, 0).unwrap();
            assert_eq!(exit, 0);
        }
    }
}
