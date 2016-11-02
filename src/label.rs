use errors::*;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum LabelInner {
    Floor,
    Hat,
    Star,
    Generic(String),
}

/// Data that identifies the Mandatory Access Control characteristics of a Subject or Object
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Label(LabelInner);

impl FromStr for Label {
    type Err = Error;
    fn from_str(s: &str) -> Result<Label> {
        if s.is_empty() || s.len() > 255 || s.contains(|c| c == '\'' || c == '"' || c == '\\' || c == '/') {
            return Err(ErrorKind::InvalidLabel(s.into()).into());
        }

        Ok(Label(match s.as_ref() {
            "_" => LabelInner::Floor,
            "^" => LabelInner::Hat,
            "*" => LabelInner::Star,
            _   => LabelInner::Generic(s.to_owned())
        }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
