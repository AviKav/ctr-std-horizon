// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use infer::canonical::{Canonical, Canonicalized, CanonicalizedQueryResult, QueryResult};
use traits::query::Fallible;
use ty::{ParamEnvAnd, Ty, TyCtxt};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Eq<'tcx> {
    pub a: Ty<'tcx>,
    pub b: Ty<'tcx>,
}

impl<'tcx> Eq<'tcx> {
    pub fn new(a: Ty<'tcx>, b: Ty<'tcx>) -> Self {
        Self { a, b }
    }
}

impl<'gcx: 'tcx, 'tcx> super::QueryTypeOp<'gcx, 'tcx> for Eq<'tcx> {
    type QueryResult = ();

    fn try_fast_path(
        _tcx: TyCtxt<'_, 'gcx, 'tcx>,
        key: &ParamEnvAnd<'tcx, Eq<'tcx>>,
    ) -> Option<Self::QueryResult> {
        if key.value.a == key.value.b {
            Some(())
        } else {
            None
        }
    }

    fn perform_query(
        tcx: TyCtxt<'_, 'gcx, 'tcx>,
        canonicalized: Canonicalized<'gcx, ParamEnvAnd<'tcx, Self>>,
    ) -> Fallible<CanonicalizedQueryResult<'gcx, ()>> {
        tcx.type_op_eq(canonicalized)
    }

    fn shrink_to_tcx_lifetime(
        v: &'a CanonicalizedQueryResult<'gcx, ()>,
    ) -> &'a Canonical<'tcx, QueryResult<'tcx, ()>> {
        v
    }
}

BraceStructTypeFoldableImpl! {
    impl<'tcx> TypeFoldable<'tcx> for Eq<'tcx> {
        a,
        b,
    }
}

BraceStructLiftImpl! {
    impl<'a, 'tcx> Lift<'tcx> for Eq<'a> {
        type Lifted = Eq<'tcx>;
        a,
        b,
    }
}

impl_stable_hash_for! {
    struct Eq<'tcx> { a, b }
}
