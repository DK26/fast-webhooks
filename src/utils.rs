#![allow(dead_code)]
use std::borrow::Cow;
use std::collections::VecDeque;
use std::char;
// use std::string::FromUtf8Error;
use encoding::{
    DecoderTrap, 
    Encoding, 
    all
};

use crate::CFG;

// Unescape code was borrowed from: https://github.com/saghm/unescape-rs.
// I added my own `unescape_as_bytes()` function and I'll offer this to the author.
macro_rules! try_option {
    ($o:expr) => {
        match $o {
            Some(s) => s,
            None => return None,
        }
    }
}

pub const DEFAULT_DECODER_TRAP : DecoderTrap = DecoderTrap::Replace;

pub type UTF8String = String;

pub type DecodingResult = Result<UTF8String, Cow<'static, str>>;

// pub type UTF8Result = Result<UTF8String, FromUtf8Error>;

pub fn reverse_str(src: &str) -> String {
    src.chars().rev().collect()
}

pub fn to_utf8_lossy(src: &[u8]) -> UTF8String {
   String::from_utf8_lossy(src).to_string()
}

pub fn to_utf8(src: &[u8]) -> DecodingResult {
    match String::from_utf8(src.to_owned()) {
        Ok(utf_8_string) => Ok(utf_8_string),
        Err(e) => Err(Cow::Owned(e.to_string()))
    }
}

pub trait Reverse {
    fn reverse(&self) -> String;
}

pub trait DecodeUTF8 {
    fn decode(&self, encoding: &str, trap: DecoderTrap) -> DecodingResult;
}

pub trait AsUTF8Lossy {
    fn as_utf8_lossy(&self) -> UTF8String;
}

impl AsUTF8Lossy for &[u8] {
    fn as_utf8_lossy(&self) -> UTF8String {
        to_utf8_lossy(self)
    }
}

pub trait AsUTF8 {
    fn as_utf8(&self) -> DecodingResult;
}

impl AsUTF8 for &[u8] {
    fn as_utf8(&self) -> DecodingResult {
        to_utf8(self)
    }
}

impl Reverse for str {
    fn reverse(&self) -> String {
        reverse_str(&self)
    }
}

impl DecodeUTF8 for &[u8] {
    fn decode(&self, encoding: &str, trap: DecoderTrap) -> DecodingResult {
        decode_bytes(self, encoding, trap)
    }
}

pub fn decode_bytes(src: &[u8], encoding: &str, trap: DecoderTrap) -> DecodingResult  { 

    let encoding = String::from(encoding).trim().to_lowercase();

    let mut src_decoded = String::new();

    let result: String = match encoding.as_str() {
        "utf8" | "utf-8" => { src.as_utf8()? },
        "iso-8859-1" => {
            all::ISO_8859_1.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-8859-2" => {
            all::ISO_8859_2.decode_to(src, trap, &mut src_decoded)?;
            src_decoded    
        },
        "iso-8859-3" => {
            all::ISO_8859_3.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-8859-4" => {
            all::ISO_8859_4.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-8859-5" => {
            all::ISO_8859_5.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-8859-6" => {
            all::ISO_8859_6.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
         "iso-8859-7" => {
            all::ISO_8859_7.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
         "iso-8859-8" => { 
            all::ISO_8859_8.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
         "iso-8859-8-i" => {
            all::ISO_8859_8.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-8859-10" => {
            all::ISO_8859_10.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-8859-13" => {
            all::ISO_8859_13.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-8859-14" => {
            all::ISO_8859_14.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-8859-15" => {
            all::ISO_8859_15.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-8859-16" => {
            all::ISO_8859_16.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "ibm866" => {
            all::IBM866.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "koi8-r" => {
            all::KOI8_R.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "koi8-u" => {
            all::ISO_8859_8.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "macintosh" | "mac-roman" => {
            all::MAC_ROMAN.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "windows-874" => {
            all::WINDOWS_874.decode_to(src, trap, &mut src_decoded)?;
            src_decoded  
        },
        "windows-1250" => {
            all::WINDOWS_1250.decode_to(src, trap, &mut src_decoded)?;
            src_decoded  
        },
        "windows-1251" => {
            all::WINDOWS_1251.decode_to(src, trap, &mut src_decoded)?;
            src_decoded  
        },
        "windows-1252" => {
            all::WINDOWS_1252.decode_to(src, trap, &mut src_decoded)?;
            src_decoded  
        },
        "windows-1253" => {
            all::WINDOWS_1253.decode_to(src, trap, &mut src_decoded)?;
            src_decoded  
        },
        "windows-1254" => {
            all::WINDOWS_1254.decode_to(src, trap, &mut src_decoded)?;
            src_decoded  
        },
        "windows-1255" => {
            all::WINDOWS_1255.decode_to(src, trap, &mut src_decoded)?;
            src_decoded  
        },
        "windows-1256" => {
            all::WINDOWS_1256.decode_to(src, trap, &mut src_decoded)?;
            src_decoded 
        },
        "windows-1257" => {
            all::WINDOWS_1257.decode_to(src, trap, &mut src_decoded)?;
            src_decoded 
        },
        "windows-1258" => {
            all::WINDOWS_1258.decode_to(src, trap, &mut src_decoded)?;
            src_decoded 
        },
        "mac-cyrillic" | "x-mac-cyrillic" => {
            all::MAC_CYRILLIC.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "ascii" => {
            all::ASCII.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "big5-2003" => {
            all::BIG5_2003.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "euc-jp" => {
            all::EUC_JP.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "gb18030" => {
            all::GB18030.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "gbk" => {
            all::GBK.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "hz" => {
            all::HZ.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "iso-2022_jp" => {
            all::ISO_2022_JP.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "uft-16be" => {
            all::UTF_16BE.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "uft-16le" => {
            all::UTF_16LE.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "windows-31j" => {
            all::WINDOWS_31J.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        },
        "windows-949" => {
            all::WINDOWS_949.decode_to(src, trap, &mut src_decoded)?;
            src_decoded
        }
        _ => { src.as_utf8()? }
    };

    Ok(result) 
}

// Takes in a string with backslash escapes written out with literal backslash characters and
// converts it to a string with the proper escaped characters.
pub fn unescape(s: &str) -> Option<String> {
    let mut queue : VecDeque<_> = String::from(s).chars().collect();
    let mut s = String::new();

    while let Some(c) = queue.pop_front() {
        if c != '\\' {
            s.push(c);
            continue;
        }

        match queue.pop_front() {
            Some('b') => s.push('\u{0008}'),
            Some('f') => s.push('\u{000C}'),
            Some('n') => s.push('\n'),
            Some('r') => s.push('\r'),
            Some('t') => s.push('\t'),
            Some('\'') => s.push('\''),
            Some('\"') => s.push('\"'),
            Some('\\') => s.push('\\'),
            Some('u') => s.push(try_option!(unescape_unicode(&mut queue))),
            Some('x') => s.push(try_option!(unescape_byte(&mut queue))),
            Some(c) if c.is_digit(8) => s.push(try_option!(unescape_octal(c, &mut queue))),
            _ => return None
        };
    }

    Some(s)
}

pub fn unescape_as_bytes(s: &str) -> Option<Vec<u8>> {
    let mut queue : VecDeque<_> = String::from(s).chars().collect();
    let mut s = Vec::new();

    while let Some(c) = queue.pop_front() {
        if c != '\\' {
            s.push(c as u8);
            continue;
        }

        match queue.pop_front() {
            Some('b') => s.push('\u{0008}' as u8),
            Some('f') => s.push('\u{000C}' as u8),
            Some('n') => s.push('\n' as u8),
            Some('r') => s.push('\r' as u8),
            Some('t') => s.push('\t' as u8),
            Some('\'') => s.push('\'' as u8),
            Some('\"') => s.push('\"' as u8),
            Some('\\') => s.push('\\' as u8),
            Some('u') => s.push(try_option!(unescape_unicode(&mut queue)) as u8),
            Some('x') => s.push(try_option!(unescape_byte(&mut queue)) as u8),
            Some(c) if c.is_digit(8) => s.push(try_option!(unescape_octal(c, &mut queue)) as u8),
            _ => return None
        };
    }

    Some(s)
}

fn unescape_unicode(queue: &mut VecDeque<char>) -> Option<char> {
    let mut s = String::new();

    for _ in 0..4 {
        s.push(try_option!(queue.pop_front()));
    }

    let u = try_option!(u32::from_str_radix(&s, 16).ok());
    char::from_u32(u)
}

fn unescape_byte(queue: &mut VecDeque<char>) -> Option<char> {
    let mut s = String::new();

    for _ in 0..2 {
        s.push(try_option!(queue.pop_front()));
    }

    let u = try_option!(u32::from_str_radix(&s, 16).ok());
    char::from_u32(u)
}

fn unescape_octal(c: char, queue: &mut VecDeque<char>) -> Option<char> {
    match unescape_octal_leading(c, queue) {
        Some(ch) => {
            let _ = queue.pop_front();
            let _ = queue.pop_front();
            Some(ch)
        }
        None => unescape_octal_no_leading(c, queue)
    }
}

fn unescape_octal_leading(c: char, queue: &VecDeque<char>) -> Option<char> {
    if c != '0' && c != '1' && c != '2' && c != '3' {
        return None;
    }

    let mut s = String::new();
    s.push(c);
    s.push(*try_option!(queue.get(0)));
    s.push(*try_option!(queue.get(1)));

    let u = try_option!(u32::from_str_radix(&s, 8).ok());
    char::from_u32(u)
}

fn unescape_octal_no_leading(c: char, queue: &mut VecDeque<char>) -> Option<char> {
    let mut s = String::new();
    s.push(c);
    s.push(try_option!(queue.pop_front()));

    let u = try_option!(u32::from_str_radix(&s, 8).ok());
    char::from_u32(u)
}

/// Attempt to decode given `src` bytes slice into a given encoding format. 
/// If fails, attempt to use alternative encoding `alt_encoding` from `cfg.toml`. 
/// If fails again, return a lossy UTF-8.
pub fn attempt_decode(src: &[u8], encoding: &str) -> DecodingResult {

    Ok(match decode_bytes(src, &encoding, DEFAULT_DECODER_TRAP) {
        Ok(result) => result,
        Err(_) => match decode_bytes(src, &CFG.common.alt_encoding, DEFAULT_DECODER_TRAP) {
                    Ok(alt_result) => alt_result,
                    Err(_) => String::from_utf8_lossy(src).to_string()
            }
    })

    // match std::str::from_utf8(src) {
    //     Ok(utf8_result) => Ok(utf8_result.to_owned()),
    //     Err(_) => Ok(decode_bytes(src, &CFG.common.alt_encoding, DEFAULT_DECODER_TRAP)?)
    // }

}

pub fn decode_mime_subject(src: &str) -> DecodingResult {

    let mut collect = false;  

    let mut scanning_encoding = true;

    let mut has_encoding = false; // If has codec, skip scan only for base64

    let mut encoding = String::new();

    let mut payload = String::new();

    let mut decoded_payload = Vec::<u8>::new();

    // for char in String::from_utf8_lossy(&msg_1).chars() {
    for char in src.chars() {
        match char {
            '?' => {
                // We were collecting and now it's time to sum-up
                if collect { 

                    scanning_encoding = !scanning_encoding;

                    // If No encoding was found yet
                    if !has_encoding { 
                        
                        // Encoding was collected
                        has_encoding = true;

                    }  else  { // Base64 payload was collected
    
                        let decoded_item = base64::decode(&payload).unwrap();

                        payload.clear();

                        decoded_payload.extend(decoded_item);
    
                    }
                }
                collect = !collect;  // flip
            },
            '\\' => {}, 
            _ => {
                if collect {
                    if !has_encoding {
                        encoding.push(char);
                    } else if !scanning_encoding {
                        payload.push(char);
                    }
                }
            }
        } // match char
    } // for src.chars()

    attempt_decode(&decoded_payload, &encoding)
}