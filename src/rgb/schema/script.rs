// LNP/BP Rust Library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::io;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

/// For now, Simplicity script is not implemented, so we use a byte array as a
/// placeholder for script data
pub type SimplicityScript = Vec<u8>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Display)]
#[display_from(Debug)]
pub struct Scripting {
    pub validation: Procedure,
    pub extensions: Extensions,
}

#[non_exhaustive]
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Display, ToPrimitive, FromPrimitive,
)]
#[display_from(Debug)]
#[repr(u8)]
pub enum Extensions {
    ScriptsDenied = 0,
    ScriptsExtend,
    ScriptsReplace,
}

#[non_exhaustive]
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Display, ToPrimitive, FromPrimitive,
)]
#[display_from(Debug)]
#[repr(u8)]
pub enum StandardProcedure {
    ConfidentialAmount = 1,
    IssueControl = 2,
}

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Display)]
#[display_from(Debug)]
pub enum Procedure {
    NoValidation,
    Standard(StandardProcedure),
    Simplicity(SimplicityScript),
}

mod strict_encoding {
    use super::*;
    use crate::strict_encoding::{Error, StrictDecode, StrictEncode};

    impl_enum_strict_encoding!(Extensions);
    impl_enum_strict_encoding!(StandardProcedure);

    impl StrictEncode for Scripting {
        type Error = Error;

        fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
            self.validation.strict_encode(&mut e)?;
            self.extensions.strict_encode(&mut e)
        }
    }

    impl StrictDecode for Scripting {
        type Error = Error;

        fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
            Ok(Self {
                validation: <Procedure>::strict_decode(&mut d)?,
                extensions: <Extensions>::strict_decode(&mut d)?,
            })
        }
    }

    impl StrictEncode for Procedure {
        type Error = Error;

        fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
            Ok(match self {
                Self::NoValidation => strict_encode_list!(e; 0u8),
                Self::Simplicity(code) => strict_encode_list!(e; 1u8, code),
                Self::Standard(proc_id) => strict_encode_list!(e; 0xFFu8, proc_id),
            })
        }
    }

    impl StrictDecode for Procedure {
        type Error = Error;

        fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
            Ok(match u8::strict_decode(&mut d)? {
                0u8 => Self::NoValidation,
                1u8 => Self::Simplicity(Vec::<u8>::strict_decode(&mut d)?),
                0xFFu8 => Self::Standard(StandardProcedure::strict_decode(&mut d)?),
                x => Err(Error::EnumValueNotKnown("script::Procedure".to_string(), x))?,
            })
        }
    }
}
