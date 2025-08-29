use std::ops::ControlFlow;

use rustc_abi::{FieldIdx, VariantIdx};
use rustc_macros::{HashStable, TyEncodable};
use rustc_span::Symbol;

use crate::ty::{self, List, Ty, TyCtxt};

#[derive(Clone, Copy, PartialEq, Eq, Debug, HashStable, Hash, TyEncodable)]
pub struct FieldPath<'tcx> {
    pub path: &'tcx List<(VariantIdx, FieldIdx)>,
}

impl<'tcx> IntoIterator for FieldPath<'tcx> {
    type Item = <&'tcx List<(VariantIdx, FieldIdx)> as IntoIterator>::Item;

    type IntoIter = <&'tcx List<(VariantIdx, FieldIdx)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl<'tcx> IntoIterator for &FieldPath<'tcx> {
    type Item = <&'tcx List<(VariantIdx, FieldIdx)> as IntoIterator>::Item;

    type IntoIter = <&'tcx List<(VariantIdx, FieldIdx)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

fn handle_segment<'tcx, T>(
    tcx: TyCtxt<'tcx>,
    cur: &mut Ty<'tcx>,
    handler: impl FnOnce(Ty<'tcx>, Symbol, Ty<'tcx>) -> ControlFlow<T>,
    variant: VariantIdx,
    field: FieldIdx,
) -> ControlFlow<T> {
    // TODO(field_projections): add shallow ty resolving?
    // cur = infcx.shallow_resolve(cur);
    match cur.kind() {
        ty::Adt(def, args) => {
            let variant = def.variant(variant);
            let field = &variant.fields[field];
            let field_ty = field.ty(tcx, args);
            *cur = field_ty;
            handler(*cur, field.name, field_ty)
        }
        _ => bug!("only ADTs are currently supported by `field_of!`"),
    }
}

impl<'tcx> FieldPath<'tcx> {
    pub fn iter(self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn walk<T>(
        self,
        tcx: TyCtxt<'tcx>,
        container: Ty<'tcx>,
        mut walker: impl FnMut(Ty<'tcx>, Symbol, Ty<'tcx>) -> ControlFlow<T>,
    ) -> Option<T> {
        // TODO(field_projections): add shallow ty resolving?
        //
        // let infcx_;
        // let infcx = if let Some(infcx) = tcx.infcx() {
        //     infcx
        // } else {
        //     assert!(!container.has_infer());
        //     infcx_ = tcx.infer_ctxt().build(ty::TypingMode::non_body_analysis());
        //     &infcx_
        // };

        let mut cur = container;
        for (variant, field) in self {
            match handle_segment(
                tcx,
                &mut cur,
                |base, name, ty| walker(base, name, ty),
                variant,
                field,
            ) {
                ControlFlow::Break(val) => return Some(val),
                ControlFlow::Continue(()) => {}
            }
        }
        None
    }

    pub fn walk_split<T>(
        self,
        tcx: TyCtxt<'tcx>,
        container: Ty<'tcx>,
        mut segment: impl FnMut(Ty<'tcx>, Symbol, Ty<'tcx>) -> ControlFlow<T>,
        tail: impl FnOnce(Ty<'tcx>, Symbol, Ty<'tcx>) -> T,
    ) -> T {
        // TODO(field_projections): add shallow ty resolving?
        //
        // let infcx_;
        // let infcx = if let Some(infcx) = tcx.infcx() {
        //     infcx
        // } else {
        //     assert!(!container.has_infer());
        //     infcx_ = tcx.infer_ctxt().build(ty::TypingMode::non_body_analysis());
        //     &infcx_
        // };

        let mut cur = container;
        for (variant, field) in self.iter().take(self.path.len() - 1) {
            match handle_segment(
                tcx,
                &mut cur,
                |base, name, ty| segment(base, name, ty),
                variant,
                field,
            ) {
                ControlFlow::Break(val) => return val,
                ControlFlow::Continue(()) => {}
            }
        }
        let Some(&(variant, field)) = self.path.last() else {
            bug!("field paths must have at least one segment")
        };
        handle_segment(
            tcx,
            &mut cur,
            |base, name, ty| ControlFlow::Break(tail(base, name, ty)),
            variant,
            field,
        )
        .break_value()
        .expect("value must be break")
    }

    pub fn field_ty(self, tcx: TyCtxt<'tcx>, container: Ty<'tcx>) -> Ty<'tcx> {
        rustc_type_ir::inherent::FieldPath::field_ty(self, tcx, container)
    }
}

impl<'tcx> rustc_type_ir::inherent::FieldPath<TyCtxt<'tcx>> for FieldPath<'tcx> {
    fn walk_split<T>(
        self,
        interner: TyCtxt<'tcx>,
        container: Ty<'tcx>,
        segment: impl FnMut(Ty<'tcx>, Symbol, Ty<'tcx>) -> ControlFlow<T>,
        tail: impl FnOnce(Ty<'tcx>, Symbol, Ty<'tcx>) -> T,
    ) -> T {
        self.walk_split(interner, container, segment, tail)
    }

    fn walk<T>(
        self,
        interner: TyCtxt<'tcx>,
        container: Ty<'tcx>,
        walker: impl FnMut(Ty<'tcx>, Symbol, Ty<'tcx>) -> ControlFlow<T>,
    ) -> Option<T> {
        self.walk(interner, container, walker)
    }
}
