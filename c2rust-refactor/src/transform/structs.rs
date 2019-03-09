use rustc::ty;
use syntax::ast::*;
use syntax::ptr::P;

use crate::ast_manip::{fold_blocks, fold_nodes, AstEquiv};
use crate::command::{CommandState, Registry};
use crate::driver::{Phase, parse_expr};
use crate::matcher::{fold_match, Subst};
use crate::path_edit::fold_resolved_paths;
use crate::transform::Transform;
use c2rust_ast_builder::{mk, IntoSymbol};
use crate::RefactorCtxt;


/// # `struct_assign_to_update` Command
/// 
/// Usage: `struct_assign_to_update`
/// 
/// Replace all struct field assignments with functional update expressions.
/// 
/// Example:
/// 
///     let mut x: S = ...;
///     x.f = 1;
///     x.g = 2;
/// 
/// After running `struct_assign_to_update`:
/// 
///     let mut x: S = ...;
///     x = S { f: 1, ..x };
///     x = S { g: 2, ..x };
pub struct AssignToUpdate;

impl Transform for AssignToUpdate {
    fn transform(&self, krate: Crate, st: &CommandState, cx: &RefactorCtxt) -> Crate {
        let pat = parse_expr(cx.session(), "__x.__f = __y");
        let repl = parse_expr(cx.session(), "__x = __s { __f: __y, .. __x }");

        fold_match(st, cx, pat, krate, |orig, mut mcx| {
            let x = mcx.bindings.get::<_, P<Expr>>("__x").unwrap().clone();

            let struct_def_id = match cx.node_type(x.id).sty {
                ty::TyKind::Adt(ref def, _) => def.did,
                _ => return orig,
            };
            let struct_path = cx.def_path(struct_def_id);

            mcx.bindings.add("__s", struct_path);
            repl.clone().subst(st, cx, &mcx.bindings)
        })
    }

    fn min_phase(&self) -> Phase {
        Phase::Phase3
    }
}


/// # `struct_merge_updates` Command
/// 
/// Usage: `struct_merge_updates`
/// 
/// Merge consecutive struct updates into a single update.
/// 
/// Example:
/// 
///     let mut x: S = ...;
///     x = S { f: 1, ..x };
///     x = S { g: 2, ..x };
/// 
/// After running `struct_assign_to_update`:
/// 
///     let mut x: S = ...;
///     x = S { f: 1, g: 2, ..x };
pub struct MergeUpdates;

impl Transform for MergeUpdates {
    fn transform(&self, krate: Crate, _st: &CommandState, _cx: &RefactorCtxt) -> Crate {
        fold_blocks(krate, |curs| {
            loop {
                // Find a struct update.
                curs.advance_until(|s| is_struct_update(s));
                if curs.eof() {
                    break;
                }
                let (path, mut fields, base) = unpack_struct_update(curs.remove());

                // Collect additional updates to the same struct.
                while !curs.eof() && is_struct_update_for(curs.next(), &base) {
                    let (_, mut more_fields, _) = unpack_struct_update(curs.remove());
                    fields.append(&mut more_fields)
                }

                // Build a new struct update and store it.
                curs.insert(build_struct_update(path, fields, base))
            }
        })
    }
}

fn is_struct_update(s: &Stmt) -> bool {
    let e = match_or!([s.node] StmtKind::Semi(ref e) => e; return false);
    let (lhs, rhs) = match_or!([e.node] ExprKind::Assign(ref lhs, ref rhs) => (lhs, rhs);
                               return false);
    match_or!([rhs.node] ExprKind::Struct(_, _, Some(ref base)) => lhs.ast_equiv(base);
              return false)
}

fn is_struct_update_for(s: &Stmt, base1: &Expr) -> bool {
    let e = match_or!([s.node] StmtKind::Semi(ref e) => e; return false);
    let rhs = match_or!([e.node] ExprKind::Assign(_, ref rhs) => rhs;
                        return false);
    match_or!([rhs.node] ExprKind::Struct(_, _, Some(ref base)) => base1.ast_equiv(base);
              return false)
}

fn unpack_struct_update(s: Stmt) -> (Path, Vec<Field>, P<Expr>) {
    let e = expect!([s.node] StmtKind::Semi(e) => e);
    let rhs = expect!([e.into_inner().node] ExprKind::Assign(_, rhs) => rhs);
    expect!([rhs.into_inner().node]
            ExprKind::Struct(path, fields, Some(base)) => (path, fields, base))
}

fn build_struct_update(path: Path, fields: Vec<Field>, base: P<Expr>) -> Stmt {
    mk().semi_stmt(
        mk().assign_expr(
            &base,
            mk().struct_expr_base(path, fields, Some(&base))))
}


/// # `rename_struct` Command
/// 
/// Obsolete - use `rename_items_regex` instead.
/// 
/// Usage: `rename_struct NAME`
/// 
/// Marks: `target`
/// 
/// Rename the struct marked `target` to `NAME`.  Only supports renaming a single
/// struct at a time.
pub struct Rename(pub String);

impl Transform for Rename {
    fn transform(&self, krate: Crate, st: &CommandState, cx: &RefactorCtxt) -> Crate {
        let new_ident = Ident::with_empty_ctxt((&self.0 as &str).into_symbol());
        let mut target_def_id = None;

        // Find the struct definition and rename it.
        let krate = fold_nodes(krate, |i: P<Item>| {
            if target_def_id.is_some() || !st.marked(i.id, "target") {
                return smallvec![i];
            }

            // Make sure this is actually a struct declaration, and not, say, the target
            // declaration's containing module.
            match_or!([struct_item_id(&i)] Some(x) => x; return smallvec![i]);
            target_def_id = Some(cx.node_def_id(i.id));

            smallvec![i.map(|i| {
                Item {
                    ident: new_ident.clone(),
                    .. i
                }
            })]
        });

        // Find uses of the struct and rewrite them.  We need to check everywhere a Path may
        // appear, since the struct name may be used as a scope for methods or other associated
        // items.

        let target_def_id = target_def_id
            .expect("found no struct to rename");

        let krate = fold_resolved_paths(krate, cx, |qself, mut path, def| {
            if let Some(def_id) = def.opt_def_id() {
                if def_id == target_def_id {
                    path.segments.last_mut().unwrap().ident = new_ident;
                }
            }
            (qself, path)
        });

        krate
    }

    fn min_phase(&self) -> Phase {
        Phase::Phase3
    }
}

fn struct_item_id(i: &Item) -> Option<NodeId> {
    let vd = match_or!([i.node] ItemKind::Struct(ref vd, _) => vd; return None);
    let id = match_or!([*vd] VariantData::Struct(_, id) => id; return None);
    Some(id)
}


pub fn register_commands(reg: &mut Registry) {
    use super::mk;

    reg.register("struct_assign_to_update", |_args| mk(AssignToUpdate));
    reg.register("struct_merge_updates", |_args| mk(MergeUpdates));
    reg.register("rename_struct", |args| mk(Rename(args[0].clone())));
}
