use crate::regex::Pattern;

extern crate nom;
use nom::{bytes::complete::tag, character::complete::digit1, IResult};

#[derive(PartialEq, Clone, Debug)]
pub enum Error {
    UnTerminatedError(String),
    ParseError,
}

pub fn parse(s: &str) -> Result<Pattern, Error> {
    match nom::branch::alt((parse_loop, parse_digit))(s) {
        Ok(("\r\n", p)) => Ok(p),
        Ok(("\n", p)) => Ok(p),
        Ok(("\r", p)) => Ok(p),
        Ok(("", p)) => Ok(p),

        Ok((s, _)) => Err(Error::UnTerminatedError(s.to_string())),
        _ => Err(Error::ParseError),
    }
}

fn parse_loop(s: &str) -> IResult<&str, Pattern> {
    let (s, p) = parse_digit(s)?;

    let (s, _) = tag("{")(s)?;

    let (s, from) = digit1(s)?;

    let (s, to) = if let Ok((s, _)) = tag::<&str, &str, (&str, nom::error::ErrorKind)>(",")(s) {
        digit1(s)?
    } else {
        (s, from)
    };

    let (s, _) = tag("}")(s)?;

    Ok((
        s,
        Pattern::Loop(Box::new(p), from.parse().unwrap(), to.parse().unwrap()),
    ))
}

fn parse_digit(s: &str) -> IResult<&str, Pattern> {
    let (s, _) = tag("\\b")(s)?;
    Ok((s, Pattern::Digit))
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::parser::Error;
    use crate::regex::Pattern;

    #[test]
    fn test_parse() {
        assert_eq!(parse("\\b"), Ok(Pattern::Digit));

        assert_eq!(
            parse("\\b{1}"),
            Ok(Pattern::Loop(Box::new(Pattern::Digit), 1, 1))
        );
        assert_eq!(
            parse("\\b{10}"),
            Ok(Pattern::Loop(Box::new(Pattern::Digit), 10, 10))
        );
        assert_eq!(
            parse("\\b{1,1}"),
            Ok(Pattern::Loop(Box::new(Pattern::Digit), 1, 1))
        );
        assert_eq!(
            parse("\\b{7,10}"),
            Ok(Pattern::Loop(Box::new(Pattern::Digit), 7, 10))
        );

        assert_eq!(
            parse("\\b{1}\r\n"),
            Ok(Pattern::Loop(Box::new(Pattern::Digit), 1, 1))
        );

        assert_eq!(
            parse("\\b{1}\n"),
            Ok(Pattern::Loop(Box::new(Pattern::Digit), 1, 1))
        );

        assert_eq!(
            parse("\\b{1}\r"),
            Ok(Pattern::Loop(Box::new(Pattern::Digit), 1, 1))
        );
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(parse("\\"), Err(Error::ParseError));
        assert_eq!(parse("b"), Err(Error::ParseError));
        assert_eq!(
            parse("\\b{"),
            Err(Error::UnTerminatedError("{".to_string()))
        );
        assert_eq!(
            parse("\\b{1,"),
            Err(Error::UnTerminatedError("{1,".to_string()))
        );
    }
}