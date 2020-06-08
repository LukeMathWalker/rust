use graphql_parser::query::{FragmentDefinition, SelectionSet, VariableDefinition};
use indexmap::IndexSet;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::serialize;

pub enum ResponsePathElement {
    Field(String),
    Idx(u32),
}

impl ToString for ResponsePathElement {
    fn to_string(&self) -> String {
        match self {
            ResponsePathElement::Field(str) => str.to_string(),
            ResponsePathElement::Idx(i) => i.to_string(),
        }
    }
}

pub struct QueryPlan<'a>(pub Option<PlanNode<'a>>);

pub struct FetchNode<'a> {
    pub service_name: String,
    pub selection_set: SelectionSet<'a>,
    pub variable_usages: Option<HashMap<String, VariableDefinition<'a>>>,
    pub requires: Option<SelectionSet<'a>>,
    pub internal_fragments: IndexSet<FragmentDefinition<'a>>,
    pub source: String,
}

pub enum PlanNode<'a> {
    Sequence(Vec<PlanNode<'a>>),
    Parallel(Vec<PlanNode<'a>>),
    Fetch(Box<FetchNode<'a>>),
    Flatten {
        path: Vec<ResponsePathElement>,
        node: Box<PlanNode<'a>>,
    },
}

impl<'a> QueryPlan<'a> {
    pub fn serialize(&self) -> String {
        serialize::serialize(self)
    }
}

impl<'a> Display for QueryPlan<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.serialize().as_str())
    }
}