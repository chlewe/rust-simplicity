// Rust Simplicity Library
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Manual type specification
//!
//! Source and target types of jet nodes need to be specified manually.

use crate::core::types::VariableType;
use crate::core::types::{RcVar, Variable};

/// Byte-based specification of a Simplicity type.
///
/// Because jets are black boxes, the type inference engine has no access to their
/// source and target types. Therefore, these types need to be specified manually.
///
/// The types are written in prefix (aka Polish) notation,
/// where `+` and `*` represent sum and product types, respectively,
/// and base types are represented by the following:
///
/// | char | type         |
/// |------|--------------|
/// | `1`  | unit         |
/// | `2`  | single bit   |
/// | `i`  | 32-bit word  |
/// | `l`  | 64-bit word  |
/// | `h`  | 256-bit word |
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TypeName(pub &'static [u8]);

impl TypeName {
    // b'1' = 49
    // b'2' = 50
    // b'i' = 105
    // b'l' = 108
    // b'h' = 104
    // b'+' = 43
    // b'*' = 42
    /// Convert the type name into a type.
    pub(crate) fn to_type(&self, pow2s: &[RcVar]) -> VariableType {
        let it = self.0.iter().rev();
        let mut stack = Vec::new();

        for c in it {
            match c {
                b'1' => stack.push(VariableType::Unit),
                b'2' => {
                    let unit = Variable::bound(VariableType::Unit);
                    stack.push(VariableType::Sum(unit.clone(), unit))
                }
                b'i' => stack.push(VariableType::Product(pow2s[4].clone(), pow2s[4].clone())),
                b'l' => stack.push(VariableType::Product(pow2s[5].clone(), pow2s[5].clone())),
                b'h' => stack.push(VariableType::Product(pow2s[7].clone(), pow2s[7].clone())),
                b'+' | b'*' => {
                    let left = Variable::bound(stack.pop().expect("Illegal type name syntax!"));
                    let right = Variable::bound(stack.pop().expect("Illegal type name syntax!"));

                    match c {
                        b'+' => stack.push(VariableType::Sum(left, right)),
                        b'*' => stack.push(VariableType::Product(left, right)),
                        _ => unreachable!(),
                    }
                }
                _ => panic!("Illegal type name syntax!"),
            }
        }

        if stack.len() == 1 {
            stack.pop().unwrap()
        } else {
            panic!("Illegal type name syntax!")
        }
    }
}
