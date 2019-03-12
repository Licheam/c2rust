use syntax::ast::*;
use syntax::ptr::P;
use syntax::symbol::Symbol;

use crate::ast_manip::MutVisitNodes;
use crate::command::{CommandState, Registry};
use crate::transform::Transform;
use crate::RefactorCtxt;


/// # `bytestr_to_str` Command
/// 
/// Usage: `bytestr_to_str`
/// 
/// Marks: `target`
/// 
/// Convert bytestring literal expressions marked `target` to string literal
/// expressions.
/// 
/// Note the mark must be placed on the expression, as it is currently difficult to
/// mark a literal node.
pub struct ByteStrToStr;

impl Transform for ByteStrToStr {
    fn transform(&self, krate: &mut Crate, st: &CommandState, _cx: &RefactorCtxt) {
        MutVisitNodes::visit(krate, |e: &mut P<Expr>| {
            if !st.marked(e.id, "target") {
                return;
            }

            match e.node {
                ExprKind::Lit(l) => {
                    match l.node {
                        LitKind::ByteStr(ref bs) => {
                            let s = String::from_utf8((**bs).clone()).unwrap();
                            l.node = LitKind::Str(Symbol::intern(&s), StrStyle::Cooked)
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
    }
}


/// # `remove_null_terminator` Command
/// 
/// Usage: `remove_null_terminator`
/// 
/// Marks: `target`
/// 
/// Remove a trailing `\0` character from marked string and bytestring literal
/// expressions.
/// 
/// Note the mark must be placed on the expression, as it is currently difficult to
/// mark a literal node.
pub struct RemoveNullTerminator;

impl Transform for RemoveNullTerminator {
    fn transform(&self, krate: &mut Crate, st: &CommandState, _cx: &RefactorCtxt) {
        MutVisitNodes::visit(krate, |e: &mut P<Expr>| {
            if !st.marked(e.id, "target") {
                return;
            }

            match e.node {
                ExprKind::Lit(l) => {
                    match &mut l.node {
                        LitKind::ByteStr(bs) => {
                            if bs.last() == Some(&0) {
                                bs.pop();
                            }
                        }
                        LitKind::Str(s, _style) => {
                            if s.as_str().ends_with("\0") {
                                let end = s.as_str().len() - 1;
                                *s = Symbol::intern(&s.as_str()[..end]);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}


pub fn register_commands(reg: &mut Registry) {
    use super::mk;
    reg.register("bytestr_to_str", |_args| mk(ByteStrToStr));
    reg.register("remove_null_terminator", |_args| mk(RemoveNullTerminator));
}

