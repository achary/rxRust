use crate::observer::{complete_proxy_impl, error_proxy_impl};
use crate::prelude::*;
/// Ignore the first `count` values emitted by the source Observable.
///
/// `skip` returns an Observable that ignore the first `count` values
/// emitted by the source Observable. If the source emits fewer than `count`
/// values then 0 of its values are emitted. After that, it completes,
/// regardless if the source completes.
///
/// # Example
/// Ignore the first 5 seconds of an infinite 1-second interval Observable
///
/// ```
/// # use rxrust::{
///   ops::{Skip}, prelude::*,
/// };
///
/// observable::from_iter(0..10).skip(5).subscribe(|v| println!("{}", v));

/// // print logs:
/// // 6
/// // 7
/// // 8
/// // 9
/// // 10
/// ```
///
pub trait Skip {
  fn skip(self, count: u32) -> SkipOp<Self>
  where
    Self: Sized,
  {
    SkipOp {
      source: self,
      count,
    }
  }
}

impl<O> Skip for O {}

#[derive(Clone)]
pub struct SkipOp<S> {
  source: S,
  count: u32,
}

macro observable_impl($subscription:ty, $($marker:ident +)* $lf: lifetime) {
  fn actual_subscribe<O: Observer<Self::Item, Self::Err> + $($marker +)* $lf>(
    self,
    subscriber: Subscriber<O, $subscription>,
  ) -> Self::Unsub {
    let subscriber = Subscriber {
      observer: SkipObserver {
        observer: subscriber.observer,
        subscription: subscriber.subscription.clone(),
        count: self.count,
        hits: 0,
      },
      subscription: subscriber.subscription,
    };
    self.source.actual_subscribe(subscriber)
  }
}

impl<'a, S> Observable<'a> for SkipOp<S>
where
  S: Observable<'a>,
{
  type Item = S::Item;
  type Err = S::Err;
  type Unsub = S::Unsub;
  observable_impl!(LocalSubscription, 'a);
}

impl<S> SharedObservable for SkipOp<S>
where
  S: SharedObservable,
{
  type Item = S::Item;
  type Err = S::Err;
  type Unsub = S::Unsub;
  observable_impl!(SharedSubscription, Send + Sync + 'static);
}

pub struct SkipObserver<O, S> {
  observer: O,
  subscription: S,
  count: u32,
  hits: u32,
}

impl<Item, Err, O, U> Observer<Item, Err> for SkipObserver<O, U>
where
  O: Observer<Item, Err>,
  U: SubscriptionLike,
{
  fn next(&mut self, value: Item) {
    self.hits += 1;
    if self.hits > self.count {
      self.observer.next(value);
      if self.hits == self.count {
        self.complete();
        self.subscription.unsubscribe();
      }
    }
  }

  error_proxy_impl!(Err, observer);
  complete_proxy_impl!(observer);
}

#[cfg(test)]
mod test {
  use super::Skip;
  use crate::prelude::*;

  #[test]
  fn base_function() {
    let mut completed = false;
    let mut next_count = 0;

    observable::from_iter(0..100)
      .skip(5)
      .subscribe_complete(|_| next_count += 1, || completed = true);

    assert_eq!(next_count, 95);
    assert_eq!(completed, true);
  }

  #[test]
  fn base_empty_function() {
    let mut completed = false;
    let mut next_count = 0;

    observable::from_iter(0..100)
      .skip(101)
      .subscribe_complete(|_| next_count += 1, || completed = true);

    assert_eq!(next_count, 0);
    assert_eq!(completed, true);
  }

  #[test]
  fn skip_support_fork() {
    let mut nc1 = 0;
    let mut nc2 = 0;
    {
      let skip5 = observable::from_iter(0..100).skip(5);
      let f1 = skip5.clone();
      let f2 = skip5;

      f1.skip(5).subscribe(|_| nc1 += 1);
      f2.skip(5).subscribe(|_| nc2 += 1);
    }
    assert_eq!(nc1, 90);
    assert_eq!(nc2, 90);
  }

  #[test]
  fn into_shared() {
    observable::from_iter(0..100)
      .skip(5)
      .skip(5)
      .to_shared()
      .subscribe(|_| {});
  }
}
