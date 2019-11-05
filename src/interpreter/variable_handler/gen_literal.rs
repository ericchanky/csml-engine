use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    data::Data,
    json_to_rust::{Event},
    variable_handler::{
        get_string_from_complexstring, get_var, interval::interval_from_expr,
        memory::get_memory_action, object::decompose_object,
    },
};
use crate::parser::{
    ast::{Expr, Identifier, Interval},
    literal::Literal,
    tokens::{MEMORY, METADATA, PAST},
};

pub fn search_str(name: &str, expr: &Expr) -> bool {
    match expr {
        Expr::IdentExpr(Identifier { ident, .. }) if ident == name => true,
        _ => false,
    }
}

pub fn gen_literal_form_expr(expr: &Expr, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::LitExpr(literal) => Ok(literal.clone()),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        e => Err(ErrorInfo {
            message: "Expression must be a literal or an identifier".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}

pub fn gen_literal_form_builder(expr: &Expr, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::BuilderExpr(elem, expr) if search_str(PAST, elem) => {
            get_memory_action(data.memory, elem, expr, data)
        }
        Expr::BuilderExpr(elem, expr) if search_str(MEMORY, elem) => {
            get_memory_action(data.memory, elem, expr, data)
        }
        Expr::BuilderExpr(elem, expr) if search_str(METADATA, elem) => {
            get_memory_action(data.memory, elem, expr, data)
        }
        Expr::BuilderExpr(elem, expr) => {
            let elem: &Expr = elem;
            if let Expr::IdentExpr(ident) = elem {
                let literal = get_var(ident.clone(), data)?;
                decompose_object(&literal, expr, &ident.interval, data)
            } else {
                Err(ErrorInfo {
                    message: "Error in Object builder".to_owned(),
                    interval: interval_from_expr(elem),
                })
            }
        }
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data)),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        e => Err(ErrorInfo {
            message: "Error in Expression builder".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}

pub fn gen_literal_form_event(
    event: &Option<Event>,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match event {
        Some(Event{payload}) => Ok(Literal::string(payload.to_owned(), interval)),
        None => Ok(Literal::null(interval)),
    }
}