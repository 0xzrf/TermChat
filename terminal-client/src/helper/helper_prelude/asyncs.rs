use futures::future::{self, Either};
use std::pin::pin;

pub async fn race<A, B, F1, F2>(f1: F1, f2: F2) -> Either<A, B>
where
    F1: Future<Output = A>,
    F2: Future<Output = B>,
{
    let f1 = pin!(f1);
    let f2 = pin!(f2);
    match future::select(f1, f2).await {
        Either::Left((a, _f2)) => Either::Left(a),
        Either::Right((b, _f1)) => Either::Right(b),
    }
}
