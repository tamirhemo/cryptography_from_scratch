use super::*;

/// The operation used is the unified formulas from section 3.1. of the paper
/// "Twisted Edwards Curves Revisited" by Hisil, Wong, Carter, Dawson, and Dahab.
///  http://eprint.iacr.org/2008/522
pub struct EdwardsGeneralUnifiedOperations<P: TwistedEdwardsGeneral> {
    _marker: cryp_std::marker::PhantomData<P>,
}
