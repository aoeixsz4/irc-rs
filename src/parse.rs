use std::collections::HashMap;

use nom::bytes::complete::{tag, take_till, take_while1};
use nom::character::{is_space, is_alphanumeric};
use nom::combinator::{iterator, opt};
use nom::sequence::{delimited, terminated, pair};

pub type Input<'a> = &'a [u8];
pub type Result<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;

pub const SPECIAL: &[u8; 9] = b"[]\\`_^{|}";

fn hostname(i: Input) -> Result<&[u8]> {
    let mut it = iterator(i, terminated(shortname, tag(b".")));
    let (rest, _) = it.finish()?;
    let (end, _) = shortname(i)?;
    Ok((end, i))
}

fn is_control(chr: u8) -> bool {
    chr == b' ' || chr == b'\0' || chr == b'\r' || chr == b'\n'
}

fn is_special(chr: u8) -> bool {
    SPECIAL.iter().any(|c| *c == chr)
}

fn is_valid_tag_key(i: Input) -> bool {
    match tag_key_strict(i) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_valid_tag_value(i: Input) -> bool {
    match tag_value_strict(i) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn lower_digit(i: Input) -> Result<&[u8]> {
    take_while1(|c| is_alphanumeric(c) && !(c >= b'A' && c <= b'Z'))(i)
}

fn lower_digit_hyphen(i: Input) -> Result<&[u8]> {
    take_while1(|c| (is_alphanumeric(c) && !(c >= b'A' && c <= b'Z')) || c == b'-')(i)
}

fn tag_key(i: Input) -> Result<&[u8]> {
    take_while1(|c| c != b' ' && c != b';' && c != b'=')(i)
}

fn tag_key_strict(i: Input) -> Result<&[u8]> {
    let (rest, _) = opt(tag(b"+"))(i)?;
    let (rest, _) = opt(terminated(hostname, tag(b"/")))(rest)?;
    lower_digit_hyphen(rest)
}

fn tag_value(i: Input) -> Result<&[u8]> {
    take_while1(|c| c != b' ')(i)
}

fn tag_value_strict(i: Input) -> Result<&[u8]> {
    take_while1(|c| !is_control(c))(i)
}

fn shortname(i: Input) -> Result<&[u8]> {
    delimited(lower_digit, lower_digit_hyphen, lower_digit)(i)
}

pub fn tags(i: Input) -> Result<HashMap<&[u8], &[u8]>> {
    let (i, inner) = delimited(tag(b"@"), take_till(is_space), tag(b" "))(i)?;
    let mut it = iterator(
        inner,
        terminated(pair(tag_key, opt(pair(tag(b"="), tag_value))), tag(b";")),
    );
    let tags = it.filter_map(|(k, v)| {
        if !is_valid_tag_key(k) { return None; }
        if let Some(val) = v {
            if !is_valid_tag_value(val.1) { return None; }
            Some((k, val.1))
        } else {
            Some((k, &k[0..0]))
        }
    }).collect::<HashMap<_, _>>();
    Ok((i, tags))
}
