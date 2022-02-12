use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameInfoType {
    pub black_name: &'static [u8],
    pub white_name: &'static [u8],
    pub tournament: &'static [u8],
    pub year: i32,
    pub black_actual_score: i32,
    pub black_corrected_score: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DatabaseInfoType {
    pub year: i32,
    pub count: i32,
}
pub type PlayerFilterType = u32;
pub const WHITE_SELECTED_FILTER: PlayerFilterType = 3;
pub const BLACK_SELECTED_FILTER: PlayerFilterType = 2;
pub const BOTH_SELECTED_FILTER: PlayerFilterType = 1;
pub const EITHER_SELECTED_FILTER: PlayerFilterType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlayerType {
    pub lex_order: i32,
    pub is_program: i32,
    pub selected: i32,
    pub name: &'static [u8],
}
impl Default for PlayerType {
    fn default() -> Self {
        PlayerType {
            lex_order: 0,
            is_program: 0,
            selected: 0,
            name: &[]
        }
    }
}
#[derive(Clone)]
#[repr(C)]
pub struct PlayerDatabaseType {
    pub prolog: PrologType,
    pub name_buffer: &'static [u8],
    pub player_list: Vec<PlayerType>,
}
impl PlayerDatabaseType {
    pub fn count(&self) -> i32 {
        return self.player_list.len() as i32;
    }

    /*
      GET_PLAYER_NAME
      Returns the name of the INDEXth player if available.
    */
    pub fn get_player_name(&self, index: i32) -> &'static [u8] {
        if index < 0 || index >= self.count() {
            return b"< Not available >"
        } else {
            return self.name_buffer[(20 * index as usize)..].split(|&c| c == 0).next().unwrap();
        };
    }
    /*
      PLAYER_LEX_ORDER
      Returns the index into the lexicographical order of the
      INDEXth player if available, otherwise the last index + 1.
    */
    pub fn player_lex_order(&self, index: i32) -> i32 {
        if index < 0 || index >= self.count() {
            return self.count()
        } else {
            return self.player_list[index as usize].lex_order
        };
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrologType {
    pub creation_century: i32,
    pub creation_year: i32,
    pub creation_month: i32,
    pub creation_day: i32,
    pub game_count: i32,
    pub item_count: i32,
    pub origin_year: i32,
    pub reserved: i32,
}
#[derive(Clone)]
#[repr(C)]
pub struct DatabaseType {
    pub prolog: PrologType,
    pub games: Vec<GameType>,
    pub count: i32,
    pub next: Option<Box<DatabaseType>>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameType {
    pub tournament_no: i16,
    pub black_no: i16,
    pub white_no: i16,
    pub actual_black_score: i16,
    pub perfect_black_score: i16,
    pub moves: [i8; 60],
    pub move_count: i16,
    pub black_disc_count: [i8; 61],
    pub opening: OpeningNodeRef,
    // pub database: &'static DatabaseType, -> replaced by origin_year field below

    // replacement for `database` field, because we only need a year from it
    pub origin_year: i32,
    pub shape_hi: u32,
    pub shape_lo: u32,
    pub shape_state_hi: i16,
    pub shape_state_lo: i16,
    pub corner_descriptor: u32,
    pub sort_order: i32,
    pub matching_symmetry: i16,
    pub passes_filter: i16,
}

impl GameType {
    pub const fn new() -> Self {
        GameType {
            tournament_no: 0,
            black_no: 0,
            white_no: 0,
            actual_black_score: 0,
            perfect_black_score: 0,
            moves: [0; 60],
            move_count: 0,
            black_disc_count: [0; 61],
            opening: OpeningNodeRef::root(),
            origin_year: 0,
            shape_hi: 0,
            shape_lo: 0,
            shape_state_hi: 0,
            shape_state_lo: 0,
            corner_descriptor: 0,
            sort_order: 0,
            matching_symmetry: 0,
            passes_filter: 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThorOpeningNode {
    pub hash1: u32,
    pub hash2: u32,
    pub current_match: i32,
    pub frequency: i32,
    pub matching_symmetry: i32,
    pub child_move: i8,
    pub sibling_move: i8,
    pub child_node: Option<OpeningNodeRef>,
    pub sibling_node: Option<OpeningNodeRef>,
    pub parent_node: Option<OpeningNodeRef>,
}

impl ThorOpeningNode {
    pub fn new() -> Self {
        ThorOpeningNode {
            hash1: 0,
            hash2: 0,
            current_match: 0,
            frequency: 0,
            matching_symmetry: 0,
            child_move: 0,
            sibling_move: 0,
            child_node: None,
            sibling_node: None,
            parent_node: None
        }
    }
}

#[derive(Copy, Clone)]
pub struct OpeningNodeRef {
    index: usize,
}

impl OpeningNodeRef {
    const fn root() -> OpeningNodeRef {
        OpeningNodeRef { index: 0 }
    }
}

pub struct ThorOpeningTree {
    nodes: Vec<ThorOpeningNode>,
    root_node: Option<OpeningNodeRef>,
}

impl ThorOpeningTree {
    pub const fn new() -> Self {
        ThorOpeningTree { nodes: vec![], root_node: None }
    }
    pub fn add(&mut self, node: ThorOpeningNode) -> OpeningNodeRef {
        self.nodes.push(node);
        return OpeningNodeRef {
            index: self.nodes.len() - 1
        }
    }
    pub fn root(&self) -> Option<OpeningNodeRef> {
        return self.root_node
    }
    pub fn set_root(&mut self, node: OpeningNodeRef) {
        self.root_node = Some(node)
    }
}

impl Index<OpeningNodeRef> for ThorOpeningTree {
    type Output = ThorOpeningNode;

    fn index(&self, index: OpeningNodeRef) -> &Self::Output {
        return &self.nodes[index.index];
    }
}

impl IndexMut<OpeningNodeRef> for ThorOpeningTree {
    fn index_mut(&mut self, index: OpeningNodeRef) -> &mut Self::Output {
        &mut self.nodes[index.index]
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TournamentType {
    pub lex_order: i32,
    pub selected: i32,
    pub name: &'static [u8],
}

pub type Int32 = i32;
pub type Int16 = i16;
pub type Int8 = i8;

#[derive(Clone)]
#[repr(C)]
pub struct TournamentDatabaseType {
    pub prolog: PrologType,
    pub name_buffer: &'static [u8],
    pub tournament_list: Vec<TournamentType>,
}
impl TournamentDatabaseType {
    pub fn count(&self) -> i32 {
        return self.tournament_list.len() as i32;
    }

    /*
      TOURNAMENT_NAME
      Returns the name of the INDEXth tournament if available.
    */
    pub fn tournament_name(&self, index: i32) -> &'static [u8] {
        if index < 0 || index >= self.count() {
            return b"<Not available>"
        } else {
            return self.name_buffer[(26 * index as usize)..].split(|&c| c == 0).next().unwrap()
        };
    }

    /*
      TOURNAMENT_LEX_ORDER
      Returns the index into the lexicographical order of the
      INDEXth tournament if available, otherwise the last
      index + 1.
    */
    pub fn tournament_lex_order(&self, index: i32) -> i32 {
        if index < 0 || index >= self.count() {
            self.count()
        } else {
            self.tournament_list[index as usize].lex_order
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub move_0: i32,
    pub frequency: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilterType {
    pub game_categories: i32,
    pub first_year: i32,
    pub last_year: i32,
    pub player_filter: PlayerFilterType,
}
#[derive(Clone)]
#[repr(C)]
pub struct SearchResultType {
    pub average_black_score: f64,
    pub next_move_score: [f64; 100],
    pub match_count: i32,
    pub black_wins: i32,
    pub draws: i32,
    pub white_wins: i32,
    pub median_black_score: i32,
    pub allocation: i32,
    pub next_move_frequency: [i32; 100],
    pub match_list: Vec<*mut GameType>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThorOpening {
    pub first_unique: i32,
    pub frequency: i32,
    pub move_str: &'static [u8],
}

impl SearchResultType {
    /*
      GET_MATCH_COUNT
      GET_BLACK_WIN_COUNT
      GET_DRAW_COUNT
      GET_WHITE_WIN_COUNT
      GET_BLACK_MEDIAN_SCORE
      GET_AVERAGE_BLACK_SCORE
      GET_MOVE_FREQUENCY
      GET_MOVE_WIN_RATE
      Accessor functions which return statistics from the last
      query to DATABASE_SEARCH.
    */

    pub fn get_match_count(&self) -> i32 {
        return self.match_count;
    }

    pub fn get_black_win_count(&self) -> i32 {
        return self.black_wins;
    }

    pub fn get_draw_count(&self) -> i32 {
        return self.draws;
    }

    pub fn get_white_win_count(&self) -> i32 {
        return self.white_wins;
    }

    pub fn get_black_median_score(&self) -> i32 {
        return self.median_black_score;
    }

    pub fn get_black_average_score(&self) -> f64 {
        return self.average_black_score;
    }

    pub fn get_move_frequency(&self, move_0: i32) -> i32 {
        return self.next_move_frequency[move_0 as usize];
    }

    pub fn get_move_win_rate(&self, move_0: i32) -> f64 {
        return if self.next_move_frequency[move_0 as usize] == 0 {
            0.0f64
        } else {
            self.next_move_score[move_0 as usize] / self.next_move_frequency[move_0 as usize] as f64
        };
    }

    /*
      GET_THOR_GAME_MOVE_COUNT
      Returns the number of moves in the INDEXth game
      in the list of matching games generated by DATABASE_SEARCH.
    */

    unsafe fn get_thor_game_move_count(&self, index: i32) -> i32 {
        if index < 0 || index >= self.match_count {
            /* Bad index */
            return -1
        } else {
            return (*self.match_list[index as usize]).move_count as i32
        };
    }
}