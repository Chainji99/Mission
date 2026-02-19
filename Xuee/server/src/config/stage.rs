use anyhow::Result;
use std::fmt;
use std::convert::TryFrom;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stage = match self {
            Stage::Local => "Local",
            Stage::Development => "Dev",
            Stage::Production => "Prod",
        };
        write!(f, "{}", stage)
    }
}

impl TryFrom<&str> for Stage {
    type Error = anyhow::Error;

    fn try_from(stage: &str) -> Result<Self, Self::Error> {
        match stage {
            "Local" => Ok(Self::Local),
            "Dev" => Ok(Self::Development),
            "Prod" => Ok(Self::Production),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}
