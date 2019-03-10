//! AST template substitution.
//!
//! This module provides functions for substituting `Bindings` into a template AST.  Placeholder
//! forms in the template AST are similar to patterns used in the `matcher` module:
//!
//!  * `__x`: An ident whose name is present in the `Bindings` will be replaced with the
//!    corresponding AST fragment.  (If the `Bindings` came from a `matcher` invocation, then most
//!    of these names will start with double underscores.)
//!
//!    This placeholder form only works if the name is present in the `Bindings` and the
//!    corresponding AST fragment is the same type as the current node.  Like in `matcher`, the
//!    substitution code tries to replace at multiple levels.  For example, if the placeholder AST
//!    is the expr `__x`, then the substitution code will first try to replace the entire `Expr`,
//!    but if this fails (because the `Bindings` have a non-`Expr` for the name `__x`), then it
//!    will continue on to try replacing the `Path` and finally just the `Ident`.
//!
//!    For itemlikes, a lone ident can't be used as a placeholder because it's not a valid
//!    itemlike.  Use a zero-argument macro invocation `__x!()` instead.

use syntax::ast::{Ident, Path, Expr, Pat, Ty, Stmt, Item, ImplItem, Label};
use syntax::ast::Mac;
use syntax::mut_visit::{self, MutVisitor};
use syntax::ptr::P;
use smallvec::SmallVec;

use crate::ast_manip::MutVisit;
use crate::ast_manip::util::PatternSymbol;
use crate::command::CommandState;
use crate::matcher::Bindings;
use crate::RefactorCtxt;

// `st` and `cx` were previously used for `def!` substitution, which has been removed.  I expect
// they'll be needed again for future subst extensions, so I've left them in to reduce API churn.
#[allow(unused)]
struct SubstFolder<'a, 'tcx: 'a> {
    st: &'a CommandState,
    cx: &'a RefactorCtxt<'a, 'tcx>,
    bindings: &'a Bindings,
}

impl<'a, 'tcx> MutVisitor for SubstFolder<'a, 'tcx> {
    fn visit_ident(&mut self, i: &mut Ident) {
        // The `Ident` case is a bit different from the others.  If `fold_stmt` finds a non-`Stmt`
        // in `self.bindings`, it can ignore the problem and hope `fold_expr` or `fold_ident` will
        // find an `Expr`/`Ident` for the symbol later on.  If `fold_ident` fails, there is no
        // lower-level construct to try.  So we report an error if a binding exists at this point
        // but is not an `Ident`.

        if let Some(sym) = i.pattern_symbol() {
            if let Some(binding) = self.bindings.get::<_, Ident>(sym) {
                *i = binding.clone();
            } else if let Some(ty) = self.bindings.get::<_, P<Ty>>(sym) {
                panic!("binding {:?} (of type {:?}) has wrong type for hole", sym, ty);
            }
            // Otherwise, fall through
        }
        mut_visit::noop_visit_ident(i, self)
    }

    fn visit_path(&mut self, p: &mut Path) {
        if let Some(binding) = p.pattern_symbol().and_then(|sym| self.bindings.get::<_, Path>(sym)) {
            *p = binding.clone();
        }

        mut_visit::noop_visit_path(p, self);
    }

    fn visit_expr(&mut self, e: &mut P<Expr>) {
        if let Some(sym) = e.pattern_symbol() {
            if let Some(binding) = self.bindings.get::<_, P<Expr>>(sym) {
                *e = binding.clone();
            } else if let Some(Some(binding)) = self.bindings.get_opt::<_, P<Expr>>(sym) {
                *e = binding.clone();
            }
        }

        mut_visit::noop_visit_expr(e, self);
    }

    fn visit_pat(&mut self, p: &mut P<Pat>) {
        if let Some(binding) = p.pattern_symbol().and_then(|sym| self.bindings.get::<_, P<Pat>>(sym)) {
            *p = binding.clone();
        }

        mut_visit::noop_visit_pat(p, self);
    }

    fn visit_ty(&mut self, ty: &mut P<Ty>) {
        if let Some(sym) = ty.pattern_symbol() {
            if let Some(binding) = self.bindings.get::<_, P<Ty>>(sym) {
                *ty = binding.clone();
            } else if let Some(Some(binding)) = self.bindings.get_opt::<_, P<Ty>>(sym) {
                *ty = binding.clone();
            }
        }

        mut_visit::noop_visit_ty(ty, self)
    }

    fn flat_map_stmt(&mut self, s: Stmt) -> SmallVec<[Stmt; 1]> {
        if let Some(stmt) = s.pattern_symbol().and_then(|sym| self.bindings.get::<_, Stmt>(sym)) {
            smallvec![stmt.clone()]
        } else if let Some(stmts) = s.pattern_symbol()
                .and_then(|sym| self.bindings.get::<_, Vec<Stmt>>(sym)) {
            SmallVec::from_vec(stmts.clone())
        } else {
            mut_visit::noop_flat_map_stmt(s, self)
        }
    }

    fn flat_map_item(&mut self, i: P<Item>) -> SmallVec<[P<Item>; 1]> {
        if let Some(item) = i.pattern_symbol().and_then(|sym| self.bindings.get::<_, P<Item>>(sym)) {
            smallvec![item.clone()]
        } else {
            mut_visit::noop_flat_map_item(i, self)
        }
    }

    fn visit_label(&mut self, l: &mut Label) {
        let ps = l.ident.pattern_symbol();
        if let Some(i) = ps.and_then(|sym| self.bindings.get::<_, Ident>(sym)) {
            l.ident = i.clone();
        } else if let Some(Some(i)) = ps.and_then(|sym| self.bindings.get_opt::<_, Ident>(sym)) {
            l.ident = i.clone();
        }

        mut_visit::noop_visit_label(l, self);
    }

    fn visit_mac(&mut self, mac: &mut Mac) {
        mut_visit::noop_visit_mac(mac, self)
    }
}


pub trait Subst {
    fn subst(self, st: &CommandState, cx: &RefactorCtxt, bindings: &Bindings) -> Self;
}

macro_rules! subst_impl {
    ($ty:ty, $fold_func:ident) => {
        impl Subst for $ty {
            fn subst(mut self, st: &CommandState, cx: &RefactorCtxt, bindings: &Bindings) -> Self {
                let mut f = SubstFolder {
                    st: st,
                    cx: cx,
                    bindings: bindings,
                };
                self.visit(&mut f);
                self
            }
        }
    };
}

macro_rules! multi_subst_impl {
    ($ty:ty, $fold_func:ident) => {
        impl Subst for Vec<$ty> {
            fn subst(self, st: &CommandState, cx: &RefactorCtxt, bindings: &Bindings) -> Self {
                let mut f = SubstFolder {
                    st: st,
                    cx: cx,
                    bindings: bindings,
                };
                let mut results = Vec::with_capacity(self.len());
                for x in self {
                    results.extend_from_slice(&x.flat_map(&mut f));
                }
                results
            }
        }
    };
}

subst_impl!(Ident, fold_ident);
subst_impl!(P<Expr>, fold_expr);
subst_impl!(P<Pat>, fold_pat);
subst_impl!(P<Ty>, fold_ty);
subst_impl!(Stmt, fold_stmt);
subst_impl!(P<Item>, fold_item);
subst_impl!(ImplItem, fold_impl_item);

multi_subst_impl!(Stmt, fold_stmt);
multi_subst_impl!(P<Item>, fold_item);
multi_subst_impl!(ImplItem, fold_impl_item);
