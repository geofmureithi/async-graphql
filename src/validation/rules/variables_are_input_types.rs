use crate::validation::visitor::{Visitor, VisitorContext};
use graphql_parser::query::VariableDefinition;

#[derive(Default)]
pub struct VariablesAreInputTypes;

impl<'a> Visitor<'a> for VariablesAreInputTypes {
    fn enter_variable_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        variable_definition: &'a VariableDefinition,
    ) {
        if let Some(ty) = ctx
            .registry
            .basic_type_by_parsed_type(&variable_definition.var_type)
        {
            if !ty.is_input() {
                ctx.report_error(
                    vec![variable_definition.position],
                    format!(
                        "Variable \"{}\" cannot be of non-input type \"{}\"",
                        &variable_definition.name,
                        ty.name()
                    ),
                );
            }
        }
    }
}
