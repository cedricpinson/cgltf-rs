#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::c_char, mem};

    // Add the byte_strings crate as a dependency
    use ::byte_strings::c_str;

    #[test]
    fn test_reading_gltf() {
        unsafe {
            let input = c_str!("DamagedHelmet.glb");
            // let input = "/Users/trigrou/Downloads/vespa-kart.glb\0";
            let mut data: *mut cgltf_data = mem::zeroed();
            let mut options: cgltf_options = mem::zeroed();
            let result = cgltf_parse_file(&options, input.as_ptr() as *const c_char, &mut data);

            println!("Result: {}", result);
            println!("Data: {:#?}", *data);
            println!("Node Count: {:#?}", (*data).nodes_count);

            cgltf_free(data);
            assert_eq!(result, cgltf_result_cgltf_result_success);
        }
    }
}
