// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::abstract_value::AbstractValue;
use crate::constant_domain::ConstantDomain;
use crate::path::Path;

use log_derive::logfn_inputs;
use rustc::ty::TyKind;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;
use syntax::ast;

/// Closely based on the expressions found in MIR.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Expression {
    /// An expression that represents any possible value
    Top,

    /// An expression that represents an impossible value, such as the value returned by function
    /// that always panics.
    Bottom,

    /// An expression that represents a block of memory allocated from the heap.
    /// The value of expression is an ordinal used to distinguish this allocation from
    /// other allocations. Because this is static analysis, a given allocation site will
    /// always result in the same ordinal.
    AbstractHeapAddress(usize),

    /// An expression that is the sum of left and right. +
    Add {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is false if left + right overflows.
    AddOverflows {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
        // The type of the result of left + right.
        result_type: ExpressionType,
    },

    /// An expression that is true if both left and right are true. &&
    And {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is the bitwise and of left and right. &
    BitAnd {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is the bitwise or of left and right. |
    BitOr {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is the bitwise xor of left and right. ^
    BitXor {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is the operand cast to the target_type. as
    Cast {
        operand: Rc<AbstractValue>,
        target_type: ExpressionType,
    },

    /// An expression that is a compile time constant value, such as a numeric literal or a function.
    CompileTimeConstant(ConstantDomain),

    /// An expression that is either if_true or if_false, depending on the value of condition.
    ConditionalExpression {
        // A condition that results in a Boolean value
        condition: Rc<AbstractValue>,
        // The value of this expression if join_condition is true.
        consequent: Rc<AbstractValue>,
        // The value of this expression if join_condition is false.
        alternate: Rc<AbstractValue>,
    },

    /// An expression that is the left value divided by the right value. /
    Div {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is true if left and right are equal. ==
    Equals {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is true if left is greater than or equal to right. >=
    GreaterOrEqual {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is true if left is greater than right. >
    GreaterThan {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// Either left or right without a condition to tell them apart.
    /// This can happen when there is loss of precision because of a loop fixed point computation.
    /// For instance inside a loop body after the second fixed point iteration, a counter may have
    /// either its initial value or the value computed in the first loop body iteration and we
    /// don't have a way to tell which value it is.
    Join {
        /// The path of the location where the two flows join together.
        path: Rc<Path>,
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is true if left is less than or equal to right. <=
    LessOrEqual {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is true if left is less than right. <
    LessThan {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is left multiplied by right. *
    Mul {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is false if left multiplied by right overflows.
    MulOverflows {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
        // The type of the result of left * right.
        result_type: ExpressionType,
    },

    /// An expression that is true if left and right are not equal. !=
    Ne {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is the arithmetic negation of its parameter. -x
    Neg { operand: Rc<AbstractValue> },

    /// An expression that is true if the operand is false.
    Not { operand: Rc<AbstractValue> },

    /// An expression that is true if either one of left or right are true. ||
    Or {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is left offset with right. ptr.offset
    Offset {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// The corresponding concrete value is the runtime address of location identified by the path.
    Reference(Rc<Path>),

    /// An expression that is the remainder of left divided by right. %
    Rem {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is the result of left shifted left by right bits. <<
    Shl {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is false if left shifted left by right bits would shift way all bits. <<
    ShlOverflows {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
        // The type of the result of left << right.
        result_type: ExpressionType,
    },

    /// An expression that is the result of left shifted right by right bits. >>
    Shr {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
        // The type of the left argument and the result
        result_type: ExpressionType,
    },

    /// An expression that is false if left shifted right by right bits would shift way all bits. >>
    ShrOverflows {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
        // The type of the result of left >> right.
        result_type: ExpressionType,
    },

    /// An expression that is the right subtracted from left. -
    Sub {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
    },

    /// An expression that is false if right subtracted from left overflows. -
    SubOverflows {
        // The value of the left operand.
        left: Rc<AbstractValue>,
        // The value of the right operand.
        right: Rc<AbstractValue>,
        // The type of the result of left - right.
        result_type: ExpressionType,
    },

    /// Like a variable, but during refinement the default value is used
    /// when the qualifier becomes a known value without a defined value for the model field.
    UnknownModelField {
        /// Must be a qualified path with a model field selector.
        path: Rc<Path>,
        default: Rc<AbstractValue>,
    },

    /// The unknown value of a place in memory.
    /// This is distinct from Top in that we known something: the place and the type.
    /// This is a useful distinction because it allows us to simplify some expressions
    /// like x == x. The type is needed to prevent this particular optimization if
    /// the variable is a floating point number that could be NaN.
    Variable {
        path: Rc<Path>,
        var_type: ExpressionType,
    },

    /// The partly known value of a place in memory.
    /// The value in operand will be the join of several expressions that all reference
    /// the path of this value. This models a variable that is assigned to from inside a loop
    /// body.
    Widen {
        /// The path of the location where an indeterminate number of flows join together.
        path: Rc<Path>,
        /// The join of some of the flows to come together at this path.
        /// The first few iterations do joins. Once widening happens, further iterations
        /// all result in the same widened value.
        operand: Rc<AbstractValue>,
    },
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Top => f.write_str("TOP"),
            Expression::Bottom => f.write_str("BOTTOM"),
            Expression::AbstractHeapAddress(ordinal) => {
                f.write_fmt(format_args!("heap_{}", ordinal))
            }
            Expression::Add { left, right } => {
                f.write_fmt(format_args!("({:?}) + ({:?})", left, right))
            }
            Expression::AddOverflows { left, right, .. } => {
                f.write_fmt(format_args!("overflows(({:?}) + ({:?}))", left, right))
            }
            Expression::And { left, right } => {
                f.write_fmt(format_args!("({:?}) && ({:?})", left, right))
            }
            Expression::BitAnd { left, right } => {
                f.write_fmt(format_args!("({:?}) & ({:?})", left, right))
            }
            Expression::BitOr { left, right } => {
                f.write_fmt(format_args!("({:?}) | ({:?})", left, right))
            }
            Expression::BitXor { left, right } => {
                f.write_fmt(format_args!("({:?}) ^ ({:?})", left, right))
            }
            Expression::Cast {
                operand,
                target_type,
            } => f.write_fmt(format_args!("({:?}) as {:?}", operand, target_type)),
            Expression::CompileTimeConstant(c) => c.fmt(f),
            Expression::ConditionalExpression {
                condition,
                consequent,
                alternate,
            } => f.write_fmt(format_args!(
                "if {:?} {{ {:?} }} else {{ {:?} }}",
                condition, consequent, alternate
            )),
            Expression::Div { left, right } => {
                f.write_fmt(format_args!("({:?}) / ({:?})", left, right))
            }
            Expression::Equals { left, right } => {
                f.write_fmt(format_args!("({:?}) == ({:?})", left, right))
            }
            Expression::GreaterOrEqual { left, right } => {
                f.write_fmt(format_args!("({:?}) >= ({:?})", left, right))
            }
            Expression::GreaterThan { left, right } => {
                f.write_fmt(format_args!("({:?}) > ({:?})", left, right))
            }
            Expression::Join { path, left, right } => f.write_fmt(format_args!(
                "({:?}) join ({:?}) at {:?}",
                left, right, path
            )),
            Expression::LessOrEqual { left, right } => {
                f.write_fmt(format_args!("({:?}) <= ({:?})", left, right))
            }
            Expression::LessThan { left, right } => {
                f.write_fmt(format_args!("({:?}) < ({:?})", left, right))
            }
            Expression::Mul { left, right } => {
                f.write_fmt(format_args!("({:?}) * ({:?})", left, right))
            }
            Expression::MulOverflows { left, right, .. } => {
                f.write_fmt(format_args!("overflows(({:?}) * ({:?}))", left, right))
            }
            Expression::Ne { left, right } => {
                f.write_fmt(format_args!("({:?}) != ({:?})", left, right))
            }
            Expression::Neg { operand } => f.write_fmt(format_args!("-({:?})", operand)),
            Expression::Not { operand } => f.write_fmt(format_args!("!({:?})", operand)),
            Expression::Or { left, right } => {
                f.write_fmt(format_args!("({:?}) || ({:?})", left, right))
            }
            Expression::Offset { left, right } => {
                f.write_fmt(format_args!("({:?})[{:?}]", left, right))
            }
            Expression::Reference(path) => f.write_fmt(format_args!("&({:?})", path)),
            Expression::Rem { left, right } => {
                f.write_fmt(format_args!("({:?}) % ({:?})", left, right))
            }
            Expression::Shl { left, right } => {
                f.write_fmt(format_args!("({:?}) << ({:?})", left, right))
            }
            Expression::ShlOverflows { left, right, .. } => {
                f.write_fmt(format_args!("overflows(({:?}) << ({:?}))", left, right))
            }
            Expression::Shr { left, right, .. } => {
                f.write_fmt(format_args!("({:?}) >> ({:?})", left, right))
            }
            Expression::ShrOverflows { left, right, .. } => {
                f.write_fmt(format_args!("overflows(({:?}) >> ({:?}))", left, right))
            }
            Expression::Sub { left, right } => {
                f.write_fmt(format_args!("({:?}) - ({:?})", left, right))
            }
            Expression::SubOverflows { left, right, .. } => {
                f.write_fmt(format_args!("overflows(({:?}) - ({:?}))", left, right))
            }
            Expression::UnknownModelField { path, default } => {
                f.write_fmt(format_args!("({:?}).default(({:?})", path, default))
            }
            Expression::Variable { path, .. } => path.fmt(f),
            Expression::Widen { path, operand } => {
                f.write_fmt(format_args!("widen({:?}) at {:?}", operand, path))
            }
        }
    }
}

impl Expression {
    /// Returns the type of value the expression should result in, if well formed.
    /// (both operands are of the same type for binary operators, conditional branches match).
    #[logfn_inputs(TRACE)]
    pub fn infer_type(&self) -> ExpressionType {
        use self::ExpressionType::*;
        match self {
            Expression::Top => NonPrimitive,
            Expression::Bottom => NonPrimitive,
            Expression::AbstractHeapAddress(_) => Reference,
            Expression::Add { left, .. } => left.expression.infer_type(),
            Expression::AddOverflows { .. } => Bool,
            Expression::And { .. } => Bool,
            Expression::BitAnd { left, .. } => left.expression.infer_type(),
            Expression::BitOr { left, .. } => left.expression.infer_type(),
            Expression::BitXor { left, .. } => left.expression.infer_type(),
            Expression::Cast { target_type, .. } => target_type.clone(),
            Expression::CompileTimeConstant(c) => c.into(),
            Expression::ConditionalExpression { consequent, .. } => {
                consequent.expression.infer_type()
            }
            Expression::Div { left, .. } => left.expression.infer_type(),
            Expression::Equals { .. } => Bool,
            Expression::GreaterOrEqual { .. } => Bool,
            Expression::GreaterThan { .. } => Bool,
            Expression::Join { left, .. } => left.expression.infer_type(),
            Expression::LessOrEqual { .. } => Bool,
            Expression::LessThan { .. } => Bool,
            Expression::Mul { left, .. } => left.expression.infer_type(),
            Expression::MulOverflows { .. } => Bool,
            Expression::Ne { .. } => Bool,
            Expression::Neg { operand } => operand.expression.infer_type(),
            Expression::Not { .. } => Bool,
            Expression::Or { .. } => Bool,
            Expression::Offset { .. } => Reference,
            Expression::Reference(_) => Reference,
            Expression::Rem { left, .. } => left.expression.infer_type(),
            Expression::Shl { left, .. } => left.expression.infer_type(),
            Expression::ShlOverflows { .. } => Bool,
            Expression::Shr { left, .. } => left.expression.infer_type(),
            Expression::ShrOverflows { .. } => Bool,
            Expression::Sub { left, .. } => left.expression.infer_type(),
            Expression::SubOverflows { .. } => Bool,
            Expression::UnknownModelField { default, .. } => default.expression.infer_type(),
            Expression::Variable { var_type, .. } => var_type.clone(),
            Expression::Widen { operand, .. } => operand.expression.infer_type(),
        }
    }

    /// Determines if the given expression is the compile time constant 1u128.
    #[logfn_inputs(TRACE)]
    pub fn is_one(&self) -> bool {
        if let Expression::CompileTimeConstant(c) = self {
            if let ConstantDomain::U128(c) = c {
                return *c == 1u128;
            }
        }
        false
    }

    /// Determines if the given expression is the compile time constant 0u128.
    #[logfn_inputs(TRACE)]
    pub fn is_zero(&self) -> bool {
        if let Expression::CompileTimeConstant(c) = self {
            if let ConstantDomain::U128(c) = c {
                return *c == 0u128;
            }
        }
        false
    }

    /// Adds any abstract heap addresses found in the associated expression to the given set.
    #[logfn_inputs(TRACE)]
    pub fn record_heap_addresses(&self, result: &mut HashSet<usize>) {
        match &self {
            Expression::AbstractHeapAddress(ordinal) => {
                result.insert(*ordinal);
            }
            Expression::Add { left, right }
            | Expression::And { left, right }
            | Expression::BitAnd { left, right }
            | Expression::BitOr { left, right }
            | Expression::BitXor { left, right }
            | Expression::Div { left, right }
            | Expression::Equals { left, right }
            | Expression::GreaterOrEqual { left, right }
            | Expression::GreaterThan { left, right }
            | Expression::LessOrEqual { left, right }
            | Expression::LessThan { left, right }
            | Expression::Mul { left, right }
            | Expression::Ne { left, right }
            | Expression::Offset { left, right }
            | Expression::Or { left, right }
            | Expression::Rem { left, right }
            | Expression::Shl { left, right }
            | Expression::Shr { left, right, .. }
            | Expression::Sub { left, right } => {
                left.expression.record_heap_addresses(result);
                right.expression.record_heap_addresses(result);
            }
            Expression::ConditionalExpression {
                condition,
                consequent,
                alternate,
            } => {
                condition.expression.record_heap_addresses(result);
                consequent.expression.record_heap_addresses(result);
                alternate.expression.record_heap_addresses(result);
            }
            Expression::Join { left, right, .. } => {
                left.expression.record_heap_addresses(result);
                right.expression.record_heap_addresses(result);
            }
            Expression::Neg { operand } | Expression::Not { operand } => {
                operand.expression.record_heap_addresses(result);
            }
            Expression::Reference(path) => path.record_heap_addresses(result),
            Expression::Variable { path, .. } => path.record_heap_addresses(result),
            _ => (),
        }
    }
}

/// The type of a place in memory, as understood by MIR.
/// For now, we are only really interested to distinguish between
/// floating point values and other values, because NaN != NaN.
/// In the future the other distinctions may be helpful to SMT solvers.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ExpressionType {
    Bool,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    NonPrimitive,
    Reference,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}

impl From<&ConstantDomain> for ExpressionType {
    #[logfn_inputs(TRACE)]
    fn from(cv: &ConstantDomain) -> ExpressionType {
        use self::ExpressionType::*;
        match cv {
            ConstantDomain::Bottom => NonPrimitive,
            ConstantDomain::Char(..) => Char,
            ConstantDomain::False => Bool,
            ConstantDomain::Function { .. } => Reference,
            ConstantDomain::I128(..) => I128,
            ConstantDomain::F64(..) => F64,
            ConstantDomain::F32(..) => F32,
            ConstantDomain::Str(..) => Reference,
            ConstantDomain::True => Bool,
            ConstantDomain::U128(..) => U128,
            ConstantDomain::Unimplemented => Reference,
        }
    }
}

impl<'a> From<&TyKind<'a>> for ExpressionType {
    #[logfn_inputs(TRACE)]
    fn from(ty_kind: &TyKind<'a>) -> ExpressionType {
        match ty_kind {
            TyKind::Bool => ExpressionType::Bool,
            TyKind::Char => ExpressionType::Char,
            TyKind::Int(ast::IntTy::Isize) => ExpressionType::Isize,
            TyKind::Int(ast::IntTy::I8) => ExpressionType::I8,
            TyKind::Int(ast::IntTy::I16) => ExpressionType::I16,
            TyKind::Int(ast::IntTy::I32) => ExpressionType::I32,
            TyKind::Int(ast::IntTy::I64) => ExpressionType::I64,
            TyKind::Int(ast::IntTy::I128) => ExpressionType::I128,
            TyKind::Uint(ast::UintTy::Usize) => ExpressionType::Usize,
            TyKind::Uint(ast::UintTy::U8) => ExpressionType::U8,
            TyKind::Uint(ast::UintTy::U16) => ExpressionType::U16,
            TyKind::Uint(ast::UintTy::U32) => ExpressionType::U32,
            TyKind::Uint(ast::UintTy::U64) => ExpressionType::U64,
            TyKind::Uint(ast::UintTy::U128) => ExpressionType::U128,
            TyKind::Float(ast::FloatTy::F32) => ExpressionType::F32,
            TyKind::Float(ast::FloatTy::F64) => ExpressionType::F64,
            TyKind::Closure(..)
            | TyKind::Dynamic(..)
            | TyKind::Foreign(..)
            | TyKind::FnDef(..)
            | TyKind::FnPtr(..)
            | TyKind::Generator(..)
            | TyKind::GeneratorWitness(..)
            | TyKind::RawPtr(..)
            | TyKind::Ref(..)
            | TyKind::Slice(..)
            | TyKind::Str => ExpressionType::Reference,
            _ => ExpressionType::NonPrimitive,
        }
    }
}

impl ExpressionType {
    /// Returns true if this type is one of the floating point number types.
    #[logfn_inputs(TRACE)]
    pub fn is_floating_point_number(&self) -> bool {
        use self::ExpressionType::*;
        match self {
            F32 | F64 => true,
            _ => false,
        }
    }

    /// Returns true if this type is one of the integer types.
    #[logfn_inputs(TRACE)]
    pub fn is_integer(&self) -> bool {
        use self::ExpressionType::*;
        match self {
            I8 | I16 | I32 | I64 | I128 | Isize | U8 | U16 | U32 | U64 | U128 | Usize => true,
            _ => false,
        }
    }

    /// Returns true if this type is not a primitive type. References are not regarded as
    /// primitives for this purpose.
    #[logfn_inputs(TRACE)]
    pub fn is_primitive(&self) -> bool {
        use self::ExpressionType::*;
        match self {
            NonPrimitive | Reference => false,
            _ => true,
        }
    }

    /// Returns true if this type is one of the signed integer types.
    #[logfn_inputs(TRACE)]
    pub fn is_signed_integer(&self) -> bool {
        use self::ExpressionType::*;
        match self {
            I8 | I16 | I32 | I64 | I128 | Isize => true,
            _ => false,
        }
    }

    /// Returns true if this type is one of the unsigned integer types.
    #[logfn_inputs(TRACE)]
    pub fn is_unsigned_integer(&self) -> bool {
        use self::ExpressionType::*;
        match self {
            U8 | U16 | U32 | U64 | U128 | Usize => true,
            _ => false,
        }
    }

    /// Returns the number of bits used to represent the given type, if primitive.
    /// For non primitive types the result is just 0.
    #[logfn_inputs(TRACE)]
    pub fn bit_length(&self) -> u8 {
        use self::ExpressionType::*;
        match self {
            Bool => 1,
            Char => 16,
            F32 => 32,
            F64 => 64,
            I8 => 8,
            I16 => 16,
            I32 => 32,
            I64 => 64,
            I128 => 128,
            Isize => 64,
            U8 => 8,
            U16 => 16,
            U32 => 32,
            U64 => 64,
            U128 => 128,
            Usize => 64,
            Reference => 128,
            NonPrimitive => 128,
        }
    }

    /// Returns the maximum value for this type, as a ConstantDomain element.
    /// If the type is not a primitive value, the result is Bottom.
    #[allow(clippy::cast_lossless)]
    #[logfn_inputs(TRACE)]
    pub fn max_value(&self) -> ConstantDomain {
        use self::ExpressionType::*;
        match self {
            Bool => ConstantDomain::U128(1 as u128),
            Char => ConstantDomain::U128(std::char::MAX as u128),
            F32 => ConstantDomain::F32(std::f32::MAX.to_bits()),
            F64 => ConstantDomain::F64(std::f64::MAX.to_bits()),
            I8 => ConstantDomain::I128(std::i8::MAX as i128),
            I16 => ConstantDomain::I128(std::i16::MAX as i128),
            I32 => ConstantDomain::I128(std::i32::MAX as i128),
            I64 => ConstantDomain::I128(std::i64::MAX as i128),
            I128 => ConstantDomain::I128(std::i128::MAX),
            Isize => ConstantDomain::I128(std::isize::MAX as i128),
            U8 => ConstantDomain::U128(std::u8::MAX as u128),
            U16 => ConstantDomain::U128(std::u16::MAX as u128),
            U32 => ConstantDomain::U128(std::u32::MAX as u128),
            U64 => ConstantDomain::U128(std::u64::MAX as u128),
            U128 => ConstantDomain::U128(std::u128::MAX),
            Usize => ConstantDomain::U128(std::usize::MAX as u128),
            _ => ConstantDomain::Bottom,
        }
    }

    /// Returns the minimum value for this type, as a ConstantDomain element.
    /// If the type is not a primitive value, the result is Bottom.
    #[allow(clippy::cast_lossless)]
    #[logfn_inputs(TRACE)]
    pub fn min_value(&self) -> ConstantDomain {
        use self::ExpressionType::*;
        match self {
            Bool => ConstantDomain::U128(0 as u128),
            Char => ConstantDomain::U128(0 as u128),
            F32 => ConstantDomain::F32(std::f32::MIN.to_bits()),
            F64 => ConstantDomain::F64(std::f64::MIN.to_bits()),
            I8 => ConstantDomain::I128(std::i8::MIN as i128),
            I16 => ConstantDomain::I128(std::i16::MIN as i128),
            I32 => ConstantDomain::I128(std::i32::MIN as i128),
            I64 => ConstantDomain::I128(std::i64::MIN as i128),
            I128 => ConstantDomain::I128(std::i128::MIN),
            Isize => ConstantDomain::I128(std::isize::MIN as i128),
            U8 => ConstantDomain::U128(std::u8::MIN as u128),
            U16 => ConstantDomain::U128(std::u16::MIN as u128),
            U32 => ConstantDomain::U128(std::u32::MIN as u128),
            U64 => ConstantDomain::U128(std::u64::MIN as u128),
            U128 => ConstantDomain::U128(std::u128::MIN),
            Usize => ConstantDomain::U128(std::usize::MIN as u128),
            _ => ConstantDomain::Bottom,
        }
    }
}
