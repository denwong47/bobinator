use std::fmt;

use bobinator_models::structs::BobinatorError;
use serde::{de, ser::SerializeMap, Deserialize, Serialize};

use conch::Lines;

use crate::common::consts;

/// One member of `messages` in [`ValidationMessages`]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationMessage {
    pub reason: String,
    pub explanations: Vec<String>,
}

/// Private, intermediary struct that provides a conveninent Serde derive for [`ValidationMessages`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct InterimValidationMessages {
    pub level: String,
    pub messages: Vec<ValidationMessage>,
}

/// Validation Messages from API end points such as Calculate Timeouts.
///
/// Typical data structure:
/// ```ignore
/// "validationMessages": {
///     "level": "INFO",
///     "messages": [
///       {
///         "reason": "You are requesting 1 day",
///         "explanations": []
///       },
///       {
///         "reason": "The forecasted remaining balance will be 24.08 days",
///         "explanations": []
///       }
///     ]
/// },
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationMessages {
    Error(Vec<ValidationMessage>),
    Information(Vec<ValidationMessage>),
}
impl ValidationMessages {
    fn name(&self) -> &str {
        macro_rules! expand_variants {
            ($(($variant:ident, $index:literal)),+) => {
                match self {
                    $(Self::$variant(_) => $index),*
                }
            };
        }

        expand_variants!((Error, "ERROR"), (Information, "INFO"))
    }

    fn messages(&self) -> Option<&[ValidationMessage]> {
        macro_rules! expand_variants {
            ($($variant:ident),+) => {
                match self {
                    $(Self::$variant(msg) => Some(msg.as_slice()),)*
                }
            };
        }

        expand_variants!(Error, Information)
    }
}
impl TryFrom<InterimValidationMessages> for ValidationMessages {
    type Error = BobinatorError;

    fn try_from(
        value: InterimValidationMessages,
    ) -> Result<Self, <ValidationMessages as TryFrom<InterimValidationMessages>>::Error> {
        macro_rules! expand_variants {
            (
                $(
                    (
                        $variant:ident,
                        $index:literal
                    )
                ),+
            ) => {
                match value.level.as_str() {
                    $(
                        $index => Ok(Self::$variant(value.messages)),
                    )*
                    other => Err(
                        BobinatorError::BobReturnedUnexpectedValidationType(other.to_string())
                    ),
                }
            };
        }

        expand_variants!((Error, "ERROR"), (Information, "INFO"))
    }
}

impl Serialize for ValidationMessages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;

        map.serialize_entry("level", self.name())?;

        map.serialize_entry("messages", &self.messages().unwrap_or(&vec![]))?;

        map.end()
    }
}
impl<'de> Deserialize<'de> for ValidationMessages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = InterimValidationMessages::deserialize(deserializer)?;

        value.try_into().map_err(|err| de::Error::custom(err))
    }
}

impl From<&ValidationMessages> for Lines {
    /// Produce [`Lines`] from a [`ValidationMessages`].
    fn from(value: &ValidationMessages) -> Self {
        consts::STANDARD_LINES.clone().title(value.name()).extend(
            value
                .messages()
                .map(|slice| {
                    slice
                        .iter()
                        // Expand all explanations with its reasons
                        .flat_map(|msg| {
                            msg.explanations
                                .iter()
                                .map(|explanation| format!("{} {}", msg.reason, explanation))
                        })
                        .collect()
                })
                .unwrap_or(vec![String::from("(No details available.)")]),
        )
    }
}
impl From<ValidationMessages> for Lines {
    /// Allow owned [`ValidationMessages`] to be transformed into [`Lines`].
    /// Useful if we don't need the message afterwards.
    fn from(value: ValidationMessages) -> Self {
        Self::from(&value)
    }
}
impl fmt::Display for ValidationMessages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Lines::from(self))
    }
}
