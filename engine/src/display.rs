
pub static mut echo: i32 = 0;
pub static mut display_pv: i32 = 0;


extern "C" {
    #[no_mangle]
    pub fn display_buffers();
    #[no_mangle]
    pub fn reset_buffer_display();
}
