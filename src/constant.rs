pub const DEFAULT_CODE: &str = "1";

pub const MIN_WINDOW_ORDER: usize = 6;

#[allow(dead_code)]
pub const MIN_WINDOW_SIZE: usize = 1 << MIN_WINDOW_ORDER; // 64

pub const DEFAULT_WINDOW_ORDER: usize = 11;

#[allow(dead_code)]
pub const DEFAULT_WINDOW_SIZE: usize = 1 << DEFAULT_WINDOW_ORDER; // 2048

pub const MAX_WINDOW_ORDER: usize = 15;

pub const MAX_WINDOW_SIZE: usize = 1 << MAX_WINDOW_ORDER; // 32768

pub const MIN_OVERLAP_ORDER: usize = 2;

#[allow(dead_code)]
pub const MIN_OVERLAP_TIMES: usize = 1 << MIN_OVERLAP_ORDER; // 4

pub const DEFAULT_OVERLAP_ORDER: usize = 4;

#[allow(dead_code)]
pub const DEFAULT_OVERLAP_TIMES: usize = 1 << DEFAULT_OVERLAP_ORDER; // 16

pub const MAX_OVERLAP_ORDER: usize = 5;

#[allow(dead_code)]
pub const MAX_OVERLAP_TIMES: usize = 1 << MAX_OVERLAP_ORDER; // 32