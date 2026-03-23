//! EPBot bridge bidding engine — Rust FFI bindings
//! 
//! # Example
//! ```rust
//! let bot = EPBot::new();
//! bot.new_hand(0, "Q5\nKJ8\nAQT9\nAKJ3", 0, 0, 1, 0);
//! let bid = bot.get_bid();
//! ```

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[link(name = "EPBot")]
extern "C" {
    fn epbot_create() -> *mut std::ffi::c_void;
    fn epbot_destroy(ptr: *mut std::ffi::c_void);

    fn epbot_new_hand(ptr: *mut std::ffi::c_void,
        position: c_int, hand: *const c_char,
        dealer: c_int, vulnerability: c_int,
        scoring: c_int, system_type: c_int) -> c_int;

    fn epbot_get_bid(ptr: *mut std::ffi::c_void) -> c_int;
    fn epbot_set_bid(ptr: *mut std::ffi::c_void, bid: c_int) -> c_int;
    fn epbot_get_str_bidding(ptr: *mut std::ffi::c_void, buf: *mut c_char, size: c_int) -> c_int;

    fn epbot_get_lead(ptr: *mut std::ffi::c_void, with_final_length: c_int) -> c_int;
    fn epbot_set_lead(ptr: *mut std::ffi::c_void, card: c_int) -> c_int;
    fn epbot_set_dummy(ptr: *mut std::ffi::c_void,
        position: c_int, hand: *const c_char, without_final_length: c_int) -> c_int;

    fn epbot_get_dealer(ptr: *mut std::ffi::c_void) -> c_int;
    fn epbot_get_vulnerability(ptr: *mut std::ffi::c_void) -> c_int;
    fn epbot_get_scoring(ptr: *mut std::ffi::c_void) -> c_int;
    fn epbot_set_scoring(ptr: *mut std::ffi::c_void, val: c_int) -> c_int;

    fn epbot_get_system_type(ptr: *mut std::ffi::c_void, position: c_int) -> c_int;
    fn epbot_set_system_type(ptr: *mut std::ffi::c_void, position: c_int, val: c_int) -> c_int;
    fn epbot_get_opponent_type(ptr: *mut std::ffi::c_void, position: c_int) -> c_int;
    fn epbot_set_opponent_type(ptr: *mut std::ffi::c_void, position: c_int, val: c_int) -> c_int;

    fn epbot_get_playing_skills(ptr: *mut std::ffi::c_void) -> c_int;
    fn epbot_set_playing_skills(ptr: *mut std::ffi::c_void, val: c_int) -> c_int;
    fn epbot_get_defensive_skills(ptr: *mut std::ffi::c_void) -> c_int;
    fn epbot_set_defensive_skills(ptr: *mut std::ffi::c_void, val: c_int) -> c_int;

    fn epbot_get_convention(ptr: *mut std::ffi::c_void,
        position: c_int, name: *const c_char) -> c_int;
    fn epbot_set_convention(ptr: *mut std::ffi::c_void,
        position: c_int, name: *const c_char, enabled: c_int) -> c_int;
    fn epbot_get_used_conventions(ptr: *mut std::ffi::c_void, position: c_int) -> c_int;
    fn epbot_set_used_conventions(ptr: *mut std::ffi::c_void, position: c_int, val: c_int) -> c_int;

    fn epbot_get_info_meaning(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_set_info_meaning(ptr: *mut std::ffi::c_void,
        bid_index: c_int, val: *const c_char) -> c_int;
    fn epbot_get_info_meaning_extended(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_get_info_alerting(ptr: *mut std::ffi::c_void, bid_index: c_int) -> c_int;
    fn epbot_set_info_alerting(ptr: *mut std::ffi::c_void, bid_index: c_int, val: c_int) -> c_int;
    fn epbot_get_info_feature(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_get_info_strength(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_get_info_min_length(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_get_info_max_length(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_get_info_probable_length(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_get_info_suit_power(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_get_info_honors(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_get_info_stoppers(ptr: *mut std::ffi::c_void,
        bid_index: c_int, buf: *mut c_char, size: c_int) -> c_int;

    fn epbot_get_probable_level(ptr: *mut std::ffi::c_void) -> c_int;
    fn epbot_get_sd_tricks(ptr: *mut std::ffi::c_void,
        contracts_buf: *mut c_char, contracts_size: c_int,
        percentages_buf: *mut c_char, percentages_size: c_int) -> c_int;
    fn epbot_get_str_longers(ptr: *mut std::ffi::c_void, buf: *mut c_char, size: c_int) -> c_int;
    fn epbot_get_pbn(ptr: *mut std::ffi::c_void, buf: *mut c_char, size: c_int) -> c_int;

    fn epbot_get_licence(ptr: *mut std::ffi::c_void) -> c_int;
    fn epbot_set_licence(ptr: *mut std::ffi::c_void, val: c_int) -> c_int;
}

// =============================================================================
// Safe Rust wrapper
// =============================================================================

const BUF_SIZE: usize = 4096;

pub struct EPBot(*mut std::ffi::c_void);

impl EPBot {
    pub fn new() -> Self {
        let ptr = unsafe { epbot_create() };
        assert!(!ptr.is_null(), "epbot_create() returned null");
        EPBot(ptr)
    }

    fn ptr(&self) -> *mut std::ffi::c_void { self.0 }

    fn read_str<F: Fn(*mut c_char, c_int) -> c_int>(f: F) -> String {
        let mut buf = vec![0u8; BUF_SIZE];
        let rc = f(buf.as_mut_ptr() as *mut c_char, BUF_SIZE as c_int);
        if rc < 0 { return String::new(); }
        let cstr = unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) };
        cstr.to_string_lossy().into_owned()
    }

    fn cstr(s: &str) -> CString {
        CString::new(s).unwrap_or_default()
    }

    // -------------------------------------------------------------------------
    // Lifecycle
    // -------------------------------------------------------------------------

    /// Inicjalizuje rękę gracza.
    /// `hand` — kolory w kolejności C.D.H.S oddzielone `\n`
    /// `scoring`: 0=MP, 1=IMP
    /// `system_type`: 0=natural, 1=polish club itd.
    pub fn new_hand(&self, position: i32, hand: &str,
        dealer: i32, vulnerability: i32, scoring: i32, system_type: i32) -> i32
    {
        let h = Self::cstr(hand);
        unsafe { epbot_new_hand(self.ptr(), position, h.as_ptr(),
            dealer, vulnerability, scoring, system_type) }
    }

    // -------------------------------------------------------------------------
    // Bidding
    // -------------------------------------------------------------------------

    pub fn get_bid(&self) -> i32 {
        unsafe { epbot_get_bid(self.ptr()) }
    }

    pub fn set_bid(&self, bid: i32) -> i32 {
        unsafe { epbot_set_bid(self.ptr(), bid) }
    }

    pub fn get_str_bidding(&self) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_str_bidding(p, buf, size) })
    }

    // -------------------------------------------------------------------------
    // Lead & Dummy
    // -------------------------------------------------------------------------

    pub fn get_lead(&self, with_final_length: bool) -> i32 {
        unsafe { epbot_get_lead(self.ptr(), with_final_length as c_int) }
    }

    pub fn set_lead(&self, card: i32) -> i32 {
        unsafe { epbot_set_lead(self.ptr(), card) }
    }

    pub fn set_dummy(&self, position: i32, hand: &str, without_final_length: bool) -> i32 {
        let h = Self::cstr(hand);
        unsafe { epbot_set_dummy(self.ptr(), position, h.as_ptr(), without_final_length as c_int) }
    }

    // -------------------------------------------------------------------------
    // State
    // -------------------------------------------------------------------------

    pub fn get_dealer(&self) -> i32        { unsafe { epbot_get_dealer(self.ptr()) } }
    pub fn get_vulnerability(&self) -> i32 { unsafe { epbot_get_vulnerability(self.ptr()) } }
    pub fn get_scoring(&self) -> i32       { unsafe { epbot_get_scoring(self.ptr()) } }
    pub fn set_scoring(&self, val: i32) -> i32 {
        unsafe { epbot_set_scoring(self.ptr(), val) }
    }

    // -------------------------------------------------------------------------
    // System & Opponent type
    // -------------------------------------------------------------------------

    pub fn get_system_type(&self, position: i32) -> i32 {
        unsafe { epbot_get_system_type(self.ptr(), position) }
    }
    pub fn set_system_type(&self, position: i32, val: i32) -> i32 {
        unsafe { epbot_set_system_type(self.ptr(), position, val) }
    }
    pub fn get_opponent_type(&self, position: i32) -> i32 {
        unsafe { epbot_get_opponent_type(self.ptr(), position) }
    }
    pub fn set_opponent_type(&self, position: i32, val: i32) -> i32 {
        unsafe { epbot_set_opponent_type(self.ptr(), position, val) }
    }

    // -------------------------------------------------------------------------
    // Skills
    // -------------------------------------------------------------------------

    pub fn get_playing_skills(&self) -> i32  { unsafe { epbot_get_playing_skills(self.ptr()) } }
    pub fn set_playing_skills(&self, val: i32) -> i32 {
        unsafe { epbot_set_playing_skills(self.ptr(), val) }
    }
    pub fn get_defensive_skills(&self) -> i32 { unsafe { epbot_get_defensive_skills(self.ptr()) } }
    pub fn set_defensive_skills(&self, val: i32) -> i32 {
        unsafe { epbot_set_defensive_skills(self.ptr(), val) }
    }

    // -------------------------------------------------------------------------
    // Conventions
    // -------------------------------------------------------------------------

    pub fn get_convention(&self, position: i32, name: &str) -> bool {
        let n = Self::cstr(name);
        unsafe { epbot_get_convention(self.ptr(), position, n.as_ptr()) != 0 }
    }
    pub fn set_convention(&self, position: i32, name: &str, enabled: bool) -> i32 {
        let n = Self::cstr(name);
        unsafe { epbot_set_convention(self.ptr(), position, n.as_ptr(), enabled as c_int) }
    }
    pub fn get_used_conventions(&self, position: i32) -> i32 {
        unsafe { epbot_get_used_conventions(self.ptr(), position) }
    }
    pub fn set_used_conventions(&self, position: i32, val: i32) -> i32 {
        unsafe { epbot_set_used_conventions(self.ptr(), position, val) }
    }

    // -------------------------------------------------------------------------
    // Bid information
    // -------------------------------------------------------------------------

    pub fn get_info_meaning(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_meaning(p, bid_index, buf, size) })
    }
    pub fn set_info_meaning(&self, bid_index: i32, val: &str) -> i32 {
        let v = Self::cstr(val);
        unsafe { epbot_set_info_meaning(self.ptr(), bid_index, v.as_ptr()) }
    }
    pub fn get_info_meaning_extended(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_meaning_extended(p, bid_index, buf, size) })
    }
    pub fn get_info_alerting(&self, bid_index: i32) -> bool {
        unsafe { epbot_get_info_alerting(self.ptr(), bid_index) != 0 }
    }
    pub fn set_info_alerting(&self, bid_index: i32, val: bool) -> i32 {
        unsafe { epbot_set_info_alerting(self.ptr(), bid_index, val as c_int) }
    }
    pub fn get_info_feature(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_feature(p, bid_index, buf, size) })
    }
    pub fn get_info_strength(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_strength(p, bid_index, buf, size) })
    }
    pub fn get_info_min_length(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_min_length(p, bid_index, buf, size) })
    }
    pub fn get_info_max_length(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_max_length(p, bid_index, buf, size) })
    }
    pub fn get_info_probable_length(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_probable_length(p, bid_index, buf, size) })
    }
    pub fn get_info_suit_power(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_suit_power(p, bid_index, buf, size) })
    }
    pub fn get_info_honors(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_honors(p, bid_index, buf, size) })
    }
    pub fn get_info_stoppers(&self, bid_index: i32) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_info_stoppers(p, bid_index, buf, size) })
    }

    // -------------------------------------------------------------------------
    // Analysis
    // -------------------------------------------------------------------------

    pub fn get_probable_level(&self) -> i32 {
        unsafe { epbot_get_probable_level(self.ptr()) }
    }

    /// Zwraca (contracts, percentages) jako CSV stringi
    pub fn get_sd_tricks(&self) -> (String, String) {
        let p = self.ptr();
        let contracts = Self::read_str(|buf, size| unsafe {
            epbot_get_sd_tricks(p, buf, size, std::ptr::null_mut(), 0)
        });
        let percentages = Self::read_str(|buf, size| unsafe {
            epbot_get_sd_tricks(p, std::ptr::null_mut(), 0, buf, size)
        });
        (contracts, percentages)
    }

    pub fn get_str_longers(&self) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_str_longers(p, buf, size) })
    }

    pub fn get_pbn(&self) -> String {
        let p = self.ptr();
        Self::read_str(|buf, size| unsafe { epbot_get_pbn(p, buf, size) })
    }

    // -------------------------------------------------------------------------
    // Licence
    // -------------------------------------------------------------------------

    pub fn get_licence(&self) -> i32  { unsafe { epbot_get_licence(self.ptr()) } }
    pub fn set_licence(&self, val: i32) -> i32 { unsafe { epbot_set_licence(self.ptr(), val) } }
}

impl Drop for EPBot {
    fn drop(&mut self) {
        unsafe { epbot_destroy(self.ptr()) }
    }
}

// Nie jest Send/Sync — EPBot nie jest thread-safe
