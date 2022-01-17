mod parse;
mod rfc_defs;
use std::collections::HashMap;

type Tags<'a> = HashMap<&'a [u8], &'a [u8]>;

pub enum Command {
    Nick,
    User,
    Privmsg,
    Notice,
    Quit,
    Join,
    Part,
    Topic,
}

pub struct Source {
    pub nick: Option<String>,
    pub user: Option<String>,
    pub host: String,
}

pub struct Message<'a> {
    pub tags: Option<Tags<'a>>,
    pub source: Option<Source>,
    pub command: Command,
    pub args: Vec<String>,
}

impl<'a> Message<'a> {
    pub fn parse (i: &'a [u8]) -> parse::Result<Self> {
        use nom::{
            bytes::complete::{tag, take},
            combinator::{cond, opt, peek},
            error::context,
            sequence::tuple,
        };
        let (i, tags) = opt(parse::tags)(i)?;
        //let (i, prefix) = opt(Source::parse)(i)?;
        //let (i, command) = parse::command(i)?;
        let source = Source { nick: None, user: None, host: String::from("localhost") };
        let command = Command::Nick;
        let args: Vec<String> = Vec::new();
        Ok((i, Self { tags, command, args, source: Some(source) }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
