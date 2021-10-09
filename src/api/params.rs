#[macro_export]
macro_rules! par {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(par!(@single $rest)),*]));
    ($($key:tt: $value:expr,)+) => { par!($($key: $value),+) };
    ($($key:tt: $value:expr),*) => {{
	    let cap = par!(@count $($key),*);
	    let mut map = ::std::collections::HashMap::with_capacity(cap);
	    $(
	        let _ = map.insert($key, $value.to_string());
	    )*
	    map
    }};
}
