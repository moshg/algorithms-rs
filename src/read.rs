/// Extension of io module.
pub mod io_ext {
    use std::mem;
    use std::io::BufRead;

    pub struct Reader<R> {
        buf: String,
        inner: R,
    }

    impl<R> Reader<R> {
        #[inline]
        pub fn new(inner: R) -> Self {
            Reader { buf: String::new(), inner: inner }
        }

        #[inline]
        pub fn into_inner(self) -> R {
            self.inner
        }
    }

    impl<R: BufRead> Reader<R> {
        #[allow(deprecated)]
        #[inline]
        pub fn read_line(&mut self) -> &str {
            self.buf.clear();
            self.inner.read_line(&mut self.buf).unwrap_or_else(|e| panic!("{}", e));
            self.buf.trim_right()
        }

        pub fn read_lines(&mut self, n: usize) -> Lines<R> {
            Lines { reader: self, n: n }
        }
    }

    pub struct Lines<'a, R> {
        reader: &'a mut Reader<R>,
        n: usize,
    }

    impl<'a, R: BufRead> Iterator for Lines<'a, R> {
        type Item = &'a str;

        fn next(&mut self) -> Option<&'a str> {
            if self.n > 0 {
                self.n -= 1;
                unsafe { Some(mem::transmute::<_, &'a str>(self.reader.read_line())) }
            } else {
                None
            }
        }
    }
}

/// Module supporting to parse strings.
pub mod parse {
    use std::str::FromStr;
    use std::borrow::Borrow;

    pub trait FromStrIterator {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(i: I) -> Self;
    }

    pub trait ParseAll {
        fn parse_all<F: FromStrIterator>(self) -> F;
    }

    impl<S: Borrow<str>, I: Iterator<Item=S>> ParseAll for I {
        #[inline]
        fn parse_all<F: FromStrIterator>(self) -> F {
            F::from_str_iter(self)
        }
    }

    fn parse<S: Borrow<str>, I: Iterator<Item=S>, F: FromStr>(i: &mut I) -> F {
        i.next().unwrap_or_else(|| panic!("too few strings error")).borrow().parse().unwrap_or_else(|_| panic!("parse error"))
    }

	// To avoid conflict, this is not implemented for `A` but `(A,)`.
	impl<A: FromStr> FromStrIterator for (A,) {
		fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(mut i: I) -> Self {
			let a = parse(&mut i);
			if i.next().is_some() {
				panic!("too many strings error");
			}
			(a,)
		}
	}

    impl<A: FromStr, B: FromStr> FromStrIterator for (A, B) {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(mut i: I) -> Self {
            let a = parse(&mut i);
            let b = parse(&mut i);
            if i.next().is_some() {
                panic!("too many strings error");
            }
            (a, b)
        }
    }

    impl<A: FromStr, B: FromStr, C: FromStr> FromStrIterator for (A, B, C) {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(mut i: I) -> Self {
            let a = parse(&mut i);
            let b = parse(&mut i);
            let c = parse(&mut i);
            if i.next().is_some() {
                panic!("too many strings error");
            }
            (a, b, c)
        }
    }

    impl<T: FromStr> FromStrIterator for Vec<T> {
        fn from_str_iter<S: Borrow<str>, I: Iterator<Item=S>>(i: I) -> Self {
            i.map(|s| s.borrow().parse().unwrap_or_else(|_| panic!("parse error"))).collect()
        }
    }
}
