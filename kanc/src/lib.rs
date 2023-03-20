#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calc1);

pub mod data;

mod internal_prelude;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(calc1::TermParser::new().parse("22").is_ok());
        assert!(calc1::TermParser::new().parse("(22)").is_ok());
        assert!(calc1::TermParser::new().parse("((((22))))").is_ok());
        assert!(calc1::TermParser::new().parse("((22)").is_err());
    }
}
