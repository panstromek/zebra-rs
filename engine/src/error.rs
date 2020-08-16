
pub trait FrontEnd : FatalError {

}
pub trait FatalError {
    fn invalid_move(curr_move: i32) -> !;
    fn unrecognized_character(unrecognized: i8) -> !;
    unsafe fn cannot_open_game_file(file_name: *const i8) -> !;
    fn memory_allocation_failure(block_count_: i32) -> !;
    fn invalid_move_in_move_sequence(curr_move: i32) -> !;
    fn error_in_map(i: i32, pos: i32, symmetry_map_item: i32) -> !;
    fn internal_error_in_book_code() -> !;
    fn book_node_list_allocation_failure(size: i32, to_report: u64) -> !;
    fn book_hash_table_allocaiton_failure(new_size: i32, new_memory: i32) -> !;
    fn safe_malloc_failure(size: u64) -> !;
    fn safe_realloc_failure(size: u64) -> !;
    fn error_in_map_thor(i: i32, pos: i32, to_report: i32) -> !;
    fn unexpected_character_in_a_move_string() -> !;
    fn invalid_move_string_provided() -> !;
}
