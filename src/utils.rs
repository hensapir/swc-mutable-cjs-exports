use swc_core::{
    common::{Span, Spanned, DUMMY_SP},
    ecma::{
        ast::*,
        atoms::{JsWord},
        utils::{
            ExprFactory
        },
    },
};

/// {
///     "key": ident,
/// }
pub(crate) struct ObjPropKeyIdent(JsWord, Span, Ident);

impl From<((JsWord, Span), Ident)> for ObjPropKeyIdent {
    fn from(((key, span), ident): ((JsWord, Span), Ident)) -> Self {
        Self(key, span, ident)
    }
}

impl From<(JsWord, Span, Ident)> for ObjPropKeyIdent {
    fn from((key, span, ident): (JsWord, Span, Ident)) -> Self {
        Self(key, span, ident)
    }
}

impl Spanned for ObjPropKeyIdent {
    fn span(&self) -> Span {
        self.1
    }
}

impl ObjPropKeyIdent {
    pub fn key(&self) -> &JsWord {
        &self.0
    }
}

pub(crate) fn emit_export_stmts(exports: Ident, prop_list: Vec<ObjPropKeyIdent>) -> Vec<Stmt> {
    match prop_list.len() {
        _ => prop_list
            .into_iter()
            .map(|obj_prop| {
                let member = MemberExpr {obj: exports.to_owned().as_pat_or_expr().expect_expr(), prop: MemberProp::Ident(Ident::new(obj_prop.key().clone(), DUMMY_SP)), span: DUMMY_SP};
                Expr::Assign(AssignExpr { left: member.as_pat_or_expr(), right: Box::new(Expr::from(obj_prop.2.clone())), op: AssignOp::Assign, span: DUMMY_SP}).into_stmt()
            })
            .into_iter()
            .collect(),
    }
}
