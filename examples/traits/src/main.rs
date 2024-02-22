use chrono::{DateTime, Utc};

fn main() {
    // We can freely access methods on our datetime value.
    let datetime: DateTime<Utc> = Utc::now();
    assert_eq!(datetime.offset(), &Utc);

    // We can also freely access methods on our boxed datetime value.
    // This is because `Box` implements the `Deref` trait so we
    // can automatically get a reference to the inner value.
    let boxed_datetime: Box<DateTime<Utc>> = Box::new(datetime);
    assert_eq!(boxed_datetime.offset(), &Utc);

    // Where boxed values get complicated is when we abstract away the original type.
    // In this case we're boxing a dynamic trait object that implements the `Debug` trait.
    // This means there is no way for our code to know what the original type was anymore.
    // It is only "something that implements the `Debug` trait" and nothing more.
    let dyn_boxed_datetime: Box<dyn std::fmt::Debug> = Box::new(datetime);
    // Can't access .offset() because there is no `fn offset(&self)` on `Debug`.
    // assert_eq!(boxed_datetime.offset(), &Utc);
    // But it does have a `fn fmt(&self, f: &mut fmt::Formatter<'_>)` we can use via `format!()`.
    assert!(format!("{dyn_boxed_datetime:?}").len() > 0);

    // So the way to get around that is to find or make our a trait that has the function we need.
    pub trait Offset {
        fn dyn_naive_utc(&self) -> chrono::NaiveDateTime;
    }
    impl Offset for chrono::DateTime<Utc> {
        fn dyn_naive_utc(&self) -> chrono::NaiveDateTime {
            self.naive_utc()
        }
    }

    // Now that we have a trait with the function we want, we can make a dynamic box of that trait.
    let dyn_boxed_datetime: Box<dyn Offset> = Box::new(datetime);
    assert_eq!(dyn_boxed_datetime.dyn_naive_utc(), datetime.naive_utc());
}
