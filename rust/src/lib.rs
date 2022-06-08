#[link(wasm_import_module="env")]
extern {
    #[link_name = "foo"]
    fn foo();

    #[link_name = "load_texture"]
    fn load_texture(width: i32, height: i32, arr_length: i32, arr: *const u8);
}

#[no_mangle]
pub extern fn sum(x: i32, y: i32) -> i32 {
    unsafe {
        foo();
    }
    x + y
}

const TEXTURE_WIDTH: usize = 32;
const TEXTURE_HEIGHT: usize = 32;
const image_array: [u8; TEXTURE_WIDTH*TEXTURE_HEIGHT*4] = [0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 32,  0, 0, 0, 94,  0, 0, 0, 162,  0, 0, 0, 217,  0, 0, 0, 247,  0, 0, 0, 247,  0, 0, 0, 217,  0, 0, 0, 162,  0, 0, 0, 94,  0, 0, 0, 32,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 7,  0, 0, 0, 94,  0, 0, 0, 247,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 247,  0, 0, 0, 94,  0, 0, 0, 7,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 76,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 76,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 7,  0, 0, 0, 162,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 162,  0, 0, 0, 7,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 14,  0, 0, 0, 217,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  15, 14, 0, 255,  73, 70, 0, 255,  146, 140, 0, 255,  209, 201, 0, 255,  247, 238, 0, 255,  247, 238, 0, 255,  209, 201, 0, 255,  146, 140, 0, 255,  73, 70, 0, 255,  15, 14, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 217,  0, 0, 0, 14,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 7,  0, 0, 0, 217,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  30, 28, 0, 255,  160, 154, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  160, 154, 0, 255,  30, 28, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 217,  0, 0, 0, 7,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 162,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  1, 0, 0, 255,  115, 110, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  115, 110, 0, 255,  1, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 162,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 76,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  7, 6, 0, 255,  181, 174, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  181, 174, 0, 255,  7, 6, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 76,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 7,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  3, 2, 0, 255,  188, 181, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  188, 181, 0, 255,  3, 2, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 7,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 94,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  131, 126, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  131, 126, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 94,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 247,  0, 0, 0, 255,  0, 0, 0, 255,  44, 42, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  239, 230, 0, 255,  158, 152, 0, 255,  187, 180, 0, 255,  233, 224, 0, 255,  253, 244, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  236, 227, 0, 255,  137, 132, 0, 255,  137, 132, 0, 255,  236, 227, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  44, 42, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 247,  0, 0, 0, 0,  0, 0, 0, 32,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  193, 186, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  156, 150, 0, 255,  8, 7, 0, 255,  23, 22, 0, 255,  147, 141, 0, 255,  242, 233, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  134, 129, 0, 255,  5, 4, 0, 255,  7, 6, 0, 255,  145, 139, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  193, 186, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 32,  0, 0, 0, 94,  0, 0, 0, 255,  0, 0, 0, 255,  26, 25, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  180, 173, 0, 255,  24, 23, 0, 255,  8, 7, 0, 255,  150, 144, 0, 255,  253, 244, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  115, 110, 0, 255,  1, 0, 0, 255,  23, 22, 0, 255,  189, 182, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  26, 25, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 94,  0, 0, 0, 162,  0, 0, 0, 255,  0, 0, 0, 255,  97, 93, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  244, 235, 0, 255,  179, 172, 0, 255,  156, 150, 0, 255,  239, 230, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  229, 220, 0, 255,  120, 115, 0, 255,  183, 176, 0, 255,  247, 238, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  97, 93, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 162,  0, 0, 0, 217,  0, 0, 0, 255,  0, 0, 0, 255,  173, 166, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  173, 166, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 217,  0, 0, 0, 247,  0, 0, 0, 255,  0, 0, 0, 255,  230, 221, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  230, 221, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 247,  0, 0, 0, 247,  0, 0, 0, 255,  0, 0, 0, 255,  251, 242, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  251, 242, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 247,  0, 0, 0, 217,  0, 0, 0, 255,  0, 0, 0, 255,  230, 221, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  230, 221, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 217,  0, 0, 0, 162,  0, 0, 0, 255,  0, 0, 0, 255,  173, 166, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  245, 236, 0, 255,  189, 182, 0, 255,  213, 205, 0, 255,  250, 241, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  173, 166, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 162,  0, 0, 0, 94,  0, 0, 0, 255,  0, 0, 0, 255,  97, 93, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  253, 244, 0, 255,  233, 224, 0, 255,  197, 190, 0, 255,  233, 224, 0, 255,  253, 244, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  245, 236, 0, 255,  138, 133, 0, 255,  37, 35, 0, 255,  78, 75, 0, 255,  216, 208, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  97, 93, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 94,  0, 0, 0, 32,  0, 0, 0, 255,  0, 0, 0, 255,  26, 25, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  242, 233, 0, 255,  157, 151, 0, 255,  54, 52, 0, 255,  122, 117, 0, 255,  203, 195, 0, 255,  250, 241, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  250, 241, 0, 255,  216, 208, 0, 255,  155, 149, 0, 255,  32, 30, 0, 255,  28, 27, 0, 255,  78, 75, 0, 255,  216, 208, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  26, 25, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 32,  0, 0, 0, 0,  0, 0, 0, 247,  0, 0, 0, 255,  0, 0, 0, 255,  193, 186, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  253, 244, 0, 255,  233, 224, 0, 255,  155, 149, 0, 255,  48, 46, 0, 255,  80, 77, 0, 255,  156, 150, 0, 255,  212, 204, 0, 255,  250, 241, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  208, 200, 0, 255,  67, 64, 0, 255,  64, 61, 0, 255,  37, 35, 0, 255,  135, 130, 0, 255,  211, 203, 0, 255,  249, 240, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  193, 186, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 247,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 94,  0, 0, 0, 255,  0, 0, 0, 255,  44, 42, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  246, 237, 0, 255,  170, 164, 0, 255,  77, 74, 0, 255,  20, 19, 0, 255,  77, 74, 0, 255,  153, 147, 0, 255,  185, 178, 0, 255,  186, 179, 0, 255,  114, 109, 0, 255,  9, 8, 0, 255,  65, 62, 0, 255,  165, 159, 0, 255,  244, 235, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  44, 42, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 94,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 7,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  131, 126, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  250, 241, 0, 255,  212, 204, 0, 255,  156, 150, 0, 255,  76, 73, 0, 255,  19, 18, 0, 255,  34, 32, 0, 255,  34, 32, 0, 255,  21, 20, 0, 255,  118, 113, 0, 255,  207, 199, 0, 255,  250, 241, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  131, 126, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 7,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 76,  0, 0, 0, 255,  0, 0, 0, 255,  3, 2, 0, 255,  188, 181, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  250, 241, 0, 255,  212, 204, 0, 255,  168, 162, 0, 255,  137, 132, 0, 255,  137, 132, 0, 255,  172, 165, 0, 255,  244, 235, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  188, 181, 0, 255,  3, 2, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 76,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 162,  0, 0, 0, 255,  0, 0, 0, 255,  7, 6, 0, 255,  181, 174, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  253, 244, 0, 255,  240, 231, 0, 255,  240, 231, 0, 255,  253, 244, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  181, 174, 0, 255,  7, 6, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 162,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 7,  0, 0, 0, 217,  0, 0, 0, 255,  0, 0, 0, 255,  1, 0, 0, 255,  115, 110, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  115, 110, 0, 255,  1, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 217,  0, 0, 0, 7,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 14,  0, 0, 0, 217,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  30, 28, 0, 255,  160, 154, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  255, 246, 0, 255,  160, 154, 0, 255,  30, 28, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 217,  0, 0, 0, 14,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 7,  0, 0, 0, 162,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  15, 14, 0, 255,  73, 70, 0, 255,  146, 140, 0, 255,  209, 201, 0, 255,  247, 238, 0, 255,  247, 238, 0, 255,  209, 201, 0, 255,  146, 140, 0, 255,  73, 70, 0, 255,  15, 14, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 162,  0, 0, 0, 7,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 76,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 76,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 7,  0, 0, 0, 94,  0, 0, 0, 247,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 255,  0, 0, 0, 247,  0, 0, 0, 94,  0, 0, 0, 7,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 32,  0, 0, 0, 94,  0, 0, 0, 162,  0, 0, 0, 217,  0, 0, 0, 247,  0, 0, 0, 247,  0, 0, 0, 217,  0, 0, 0, 162,  0, 0, 0, 94,  0, 0, 0, 32,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0];

#[no_mangle]
pub extern fn init() {
    let img_ref = &image_array;
    let img_array_ptr = img_ref as *const u8;
    unsafe {
        load_texture(TEXTURE_WIDTH as i32, TEXTURE_HEIGHT as i32, (TEXTURE_WIDTH as i32)*(TEXTURE_HEIGHT as i32)*4, img_array_ptr);
    }
}