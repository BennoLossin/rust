use std::ops::ControlFlow;

use rustc_abi::{FieldIdx, VariantIdx};
use rustc_macros::{HashStable, TyEncodable};
use rustc_type_ir::FieldPathVisitor;

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

impl<'tcx> FieldPath<'tcx> {
    pub fn iter(self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn visit<V: FieldPathVisitor<TyCtxt<'tcx>>>(
        self,
        container: Ty<'tcx>,
        mut visitor: V,
        tcx: TyCtxt<'tcx>,
    ) -> V::Output {
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
        let mut last = None;
        for (variant, field) in self {
            // cur = infcx.shallow_resolve(cur);
            match cur.kind() {
                ty::Adt(def, args) => {
                    let variant = def.variant(variant);
                    let field = &variant.fields[field];
                    last = Some(field);
                    let field_ty = field.ty(tcx, args);
                    if let ControlFlow::Break(val) =
                        visitor.visit_segment(cur, field.name, field_ty)
                    {
                        return val;
                    }
                    cur = field_ty;
                }
                _ => bug!("only ADTs are currently supported by `field_of!`"),
            }
        }
        // cur = infcx.shallow_resolve(cur);
        let Some(last) = last else { bug!("field paths must have at least one segment") };
        visitor.visit_final(cur, last.name)
    }
}

impl<'tcx> rustc_type_ir::inherent::FieldPath<TyCtxt<'tcx>> for FieldPath<'tcx> {
    fn visit<V: FieldPathVisitor<TyCtxt<'tcx>>>(
        self,
        container: Ty<'tcx>,
        visitor: V,
        tcx: TyCtxt<'tcx>,
    ) -> V::Output {
        Self::visit(self, container, visitor, tcx)
    }
}
