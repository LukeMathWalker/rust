use crate::display;
use graphql_parser::query::{FragmentDefinition, SelectionSet};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::serialize;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPlan<'a>(#[serde(borrow)] pub Option<PlanNode<'a>>);

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchNode<'a> {
    pub service_name: String,
    #[serde(borrow)]
    pub selection_set: SelectionSet<'a>,
    pub variable_usages: Vec<String>,
    #[serde(borrow)]
    pub requires: Option<SelectionSet<'a>>,
    #[serde(borrow)]
    pub internal_fragments: IndexSet<FragmentDefinition<'a>>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlanNode<'a> {
    #[serde(borrow)]
    Sequence(Vec<PlanNode<'a>>),
    #[serde(borrow)]
    Parallel(Vec<PlanNode<'a>>),
    #[serde(borrow)]
    Fetch(Box<FetchNode<'a>>),
    Flatten {
        path: Vec<ResponsePathElement>,
        #[serde(borrow)]
        node: Box<PlanNode<'a>>,
    },
}

impl<'a> Display for QueryPlan<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(display::display(self).as_str())
    }
}
