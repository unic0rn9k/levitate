//! # Numbers
//!
//! # Thanks
//! Heavy inspiration has been taken from crates listed bellow.
//!
//! - https://lib.rs/crates/num
//! - https://lib.rs/crates/fast-floats

#![feature(const_trait_impl, core_intrinsics)]

use std::ops::*;

pub trait PrimitiveFloat: Float + BackedByPrimitive<Primitive = Self> {}

pub trait BackedByPrimitive {
    type Primitive;
    fn from_primitive(f: Self::Primitive) -> Self;
    fn into_primitive(self) -> Self::Primitive;
    fn from_f64(f: f64) -> Self;
}

pub trait Float:
    Add<Self, Output = Self>
    + Div<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Copy
    + BackedByPrimitive
    + PartialEq
{
    const _0: Self;
    const _1: Self;
    const _2: Self;

    fn sqrt_(self) -> Self;
    fn powi_(self, p: i32) -> Self;
    fn hypot_(self, other: Self) -> Self;
    fn exp_(self) -> Self::Primitive;
    fn sin_(self) -> Self::Primitive;
    fn cos_(self) -> Self::Primitive;
}

macro_rules! impl_wrapper {
	($fn: ident $( ($($arg: ident $({$($pre: tt)*})? : $t: ty),*) )? { $($finalize: tt)* } -> $re: ty) => {
		paste!{
            fn [< $fn _ >] (self $(, $($arg: $t),* )? ) -> $re {
                self . $fn($($( $arg $($($pre)*)? ),*)?) $($finalize)*
            }
        }
	};
}

macro_rules! def_primitive {
    ($t: ty) => {
        impl const BackedByPrimitive for $t {
            type Primitive = $t;

            #[inline(always)]
            fn from_primitive(f: Self::Primitive) -> Self {
                f
            }

            #[inline(always)]
            fn into_primitive(self) -> Self::Primitive {
                self
            }

            #[inline(always)]
            fn from_f64(f: f64) -> Self {
                f as $t
            }
        }

        impl const PrimitiveFloat for $t {}

        impl Float for $t {
            const _0: $t = 0.;
            const _1: $t = 1.;
            const _2: $t = 2.;

            impl_wrapper!(powi (p: i32) { .into() } -> Self);
            impl_wrapper!(hypot (other: Self) { .into() } -> Self);

            impl_wrapper!(sqrt { .into() } -> Self);
            impl_wrapper!(exp { .into() } -> Self);
            impl_wrapper!(sin { .into() } -> Self);
            impl_wrapper!(cos { .into() } -> Self);
        }
    };
}

def_primitive!(f32);
def_primitive!(f64);

#[cfg(feature = "fast-floats")]
pub mod fast_floats;
#[cfg(feature = "fast-floats")]
pub mod _fast_floats {
    use super::*;
    use fast_floats::*;

    macro_rules! def_fast_primitive {
    ($t: ty) => {
        impl const BackedByPrimitive for Fast<$t> {
            type Primitive = $t;

            #[inline(always)]
            fn from_primitive(f: Self::Primitive) -> Self {
                unsafe{ Self::new(f) }
            }

            #[inline(always)]
            fn into_primitive(self) -> Self::Primitive {
                *self
            }

            #[inline(always)]
            fn from_f64(f: f64) -> Self {
                unsafe{ Self::new(f as $t) }
            }
        }

        impl Float for Fast<$t> {
            const _0: Self = unsafe{ Fast::new(0.) };
            const _1: Self = unsafe{ Fast::new(1.) };
            const _2: Self = unsafe{ Fast::new(2.) };

            impl_wrapper!(powi (p: i32) { .into() } -> Self);
            impl_wrapper!(hypot (other { .deref().clone() } : Self) { .into() } -> Self);

            impl_wrapper!(sqrt { .into() } -> Self);
            impl_wrapper!(exp { .into() } -> $t);
            impl_wrapper!(sin { .into() } -> $t);
            impl_wrapper!(cos { .into() } -> $t);
        }
    };
}

    def_fast_primitive!(f32);
    def_fast_primitive!(f64);
}

macro_rules! impl_complex_plus_min {
    ($op: tt, $trait: ident, $fn: ident) => {
        impl<T: Float> $trait<Self> for Complex<T> {
            type Output = Self;
            fn $fn(self, other: Self) -> Self {
                Self::Output{re: self.re $op other.re, im: self.im $op other.im}
            }
        }
    };
}

impl_complex_plus_min!(+, Add, add);
impl_complex_plus_min!(-, Sub, sub);

impl<T: Float> Mul<Self> for Complex<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let re = self.re * other.re - self.im * other.im;
        let im = self.re * other.im + self.im * other.re;
        Self::Output { re, im }
    }
}
impl<T: Float> Div<Self> for Complex<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let re = self.re * other.re + self.im * other.im;
        let im = self.re * other.im - self.im * other.re;
        Self::Output { re, im }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Complex<T: Float> {
    pub re: T,
    pub im: T,
}

impl<T: Float> From<T> for Complex<T> {
    fn from(n: T) -> Self {
        Self { re: n, im: T::_0 }
    }
}

impl<T: PrimitiveFloat> BackedByPrimitive for Complex<T> {
    type Primitive = T;

    fn from_primitive(f: Self::Primitive) -> Self {
        Self { re: f, im: T::_0 }
    }

    fn into_primitive(self) -> Self::Primitive {
        //eprint!("Converting complex number to primitive should be done with self.re, instead of self.into_primitive.");
        self.re.into_primitive()
    }

    fn from_f64(f: f64) -> Self {
        Self {
            re: T::from_f64(f),
            im: T::_0,
        }
    }
}

impl<T: PrimitiveFloat> Float for Complex<T> {
    const _0: Self = Self {
        re: T::_0,
        im: T::_0,
    };
    const _1: Self = Self {
        re: T::_1,
        im: T::_0,
    };
    const _2: Self = Self {
        re: T::_2,
        im: T::_0,
    };

    /// This function os currently unimplemented for complex numbers, you can still use `number * number`.
    fn powi_(self, n: i32) -> Self {
        let mut prod = Self::_1;
        for _ in 0..n {
            prod = prod * self
        }
        prod
    }

    fn sqrt_(self) -> Self {
        let norm = self.re.hypot_(self.im);
        Self {
            re: ((norm + self.re) / T::_2).sqrt_(),
            im: ((norm - self.re) / T::_2).sqrt_(),
        }
    }

    fn hypot_(self, other: Self) -> Self {
        (self * self + other * other).sqrt_()
    }

    fn exp_(self) -> T {
        self.re.cos_() + self.im * self.re.sin_()
    }

    fn cos_(self) -> T {
        let mut neg_self = self;
        neg_self.re = self.re * T::from_f64(-1.);
        (self.exp_() + (neg_self).exp_()) / T::from_f64(2.)
    }

    //FIXME
    fn sin_(self) -> T {
        todo!()
    }
}

impl<T: Float> From<[T; 2]> for Complex<T> {
    fn from(n: [T; 2]) -> Self {
        Complex { re: n[0], im: n[1] }
    }
}

use paste::paste;
macro_rules! gen_tests {
    ($t: ty, $name: ident) => {
        paste! {
            #[cfg(test)]
            #[test]
            fn $name() {
                for x in 0..100 {
                    let x = $t::from_f64(x as f64) / 2.;
                    for y in 0..100 {
                        let y = $t::from_f64(y as f64) / 2.;
                        assert_eq!(x.hypot(y.into_primitive()), x.hypot_(y).into_primitive());
                    }
                    for n in 0..4 {
                        assert_eq!(x.powi(n), x.powi_(n).into_primitive());
                    }
                    assert_eq!(x.sqrt(), x.sqrt_().into_primitive());
                }
            }
        }
    };
}

gen_tests!(f32, trig_f32);
gen_tests!(f64, trig_f64);

#[cfg(test)]
use fast_floats::Fast;
gen_tests!(Fast::<f32>, trig_fast_f32);
gen_tests!(Fast::<f64>, trig_fast_f64);
