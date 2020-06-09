//! Query Language Abstract Syntax Tree (AST)
//!
//! The types and fields here resemble official [graphql grammar] whenever it
//! makes sense for rust.
//!
//! [graphql grammar]: http://facebook.github.io/graphql/October2016/#sec-Appendix-Grammar-Summary
//!
pub use crate::common::{Directive, Txt, Type, Value};
use crate::position::Pos;
use serde::{Deserialize, Serialize};

/// Root of query data
#[derive(Debug, Clone, PartialEq)]
pub struct Document<'a> {
    pub definitions: Vec<Definition<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Definition<'a> {
    SelectionSet(SelectionSet<'a>),
    Operation(OperationDefinition<'a>),
    Fragment(FragmentDefinition<'a>),
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct FragmentDefinition<'a> {
    pub position: Pos,
    pub description: Option<String>,
    #[serde(borrow)]
    pub name: Txt<'a>,
    #[serde(borrow)]
    pub type_condition: Txt<'a>,
    #[serde(borrow)]
    pub directives: Vec<Directive<'a>>,
    #[serde(borrow)]
    pub selection_set: SelectionSet<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinition<'a> {
    pub position: Pos,
    pub kind: Operation,
    pub description: Option<String>,
    pub name: Option<Txt<'a>>,
    pub variable_definitions: Vec<VariableDefinition<'a>>,
    pub directives: Vec<Directive<'a>>,
    pub selection_set: SelectionSet<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Query,
    Mutation,
    Subscription,
}

impl Operation {
    /// Returns GraphQL syntax compatible name of the operation
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Query => "query",
            Self::Mutation => "mutation",
            Self::Subscription => "subscription",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct SelectionSet<'a> {
    pub span: (Pos, Pos),

    #[serde(borrow)]
    pub items: Vec<Selection<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDefinition<'a> {
    pub position: Pos,
    #[serde(borrow)]
    pub name: Txt<'a>,
    #[serde(borrow)]
    pub var_type: Type<'a>,
    #[serde(borrow)]
    pub default_value: Option<Value<'a>>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Selection<'a> {
    #[serde(borrow)]
    Field(Field<'a>),
    #[serde(borrow)]
    FragmentSpread(FragmentSpread<'a>),
    #[serde(borrow)]
    InlineFragment(InlineFragment<'a>),
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Field<'a> {
    pub position: Pos,
    #[serde(borrow)]
    pub alias: Option<Txt<'a>>,
    #[serde(borrow)]
    pub name: Txt<'a>,
    #[serde(borrow)]
    pub arguments: Vec<(Txt<'a>, Value<'a>)>,
    #[serde(borrow)]
    pub directives: Vec<Directive<'a>>,
    #[serde(borrow)]
    pub selection_set: SelectionSet<'a>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct FragmentSpread<'a> {
    pub position: Pos,
    #[serde(borrow)]
    pub fragment_name: Txt<'a>,
    #[serde(borrow)]
    pub directives: Vec<Directive<'a>>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct InlineFragment<'a> {
    pub position: Pos,
    #[serde(borrow)]
    pub type_condition: Option<Txt<'a>>,
    #[serde(borrow)]
    pub directives: Vec<Directive<'a>>,
    #[serde(borrow)]
    pub selection_set: SelectionSet<'a>,
}
