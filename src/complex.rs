use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Complex<T: Float> {
    pub re: T,
    pub im: T,
}

impl<T: Float> Complex<T> {
    fn norm_sqrt(&self) -> T {
        self.re * self.re + self.im * self.im
    }
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

// (a+bi)*(c+di)
// (ac-bd)+(ad+bc)i
impl<T: Float> Mul<Self> for Complex<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let re = self.re * other.re - self.im * other.im;
        let im = self.re * other.im + self.im * other.re;
        Self::Output { re, im }
    }
}

// (a+bi)/(c+di)
// ((ac+bd)+(bc-ad)i) /c^2 + d^2
impl<T: Float> Div<Self> for Complex<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let norm_sqrt = other.norm_sqrt();
        let re = (self.re * other.re + self.im * other.im) / norm_sqrt;
        let im = (self.im * other.re - self.re * other.im) / norm_sqrt;
        Self::Output { re, im }
    }
}

macro_rules! impl_complex_debug {
    (write $self:ident: $f: ty => $to: ident) => {unsafe{
        let re: &$f = std::mem::transmute(&$self.re);
        let im: &$f = std::mem::transmute(&$self.im);
        write!(
            $to,
            "({} {} {}im)",
            re,
            if im.into_primitive().is_sign_positive() {
                "+"
            } else {
                "-"
            },
            im.into_primitive().abs()
        )
    }};
	(impl $($trait: tt)*) => {
        $(
            impl<F: PrimitiveFloat + Display, T: FloatWrapper<InnerFloat = F> + Float> $trait for Complex<T> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let float_size = std::mem::size_of::<F>();
                    if float_size == 4{
                        impl_complex_debug!(write self: f32 => f)
                    }else if float_size == 8{
                        impl_complex_debug!(write self: f64 => f)
                    }else{
                        unimplemented!()
                    }
                }
            }
        )*
	};
}

use std::fmt::*;
impl_complex_debug!(impl Debug Display);

impl<T: Float> From<T> for Complex<T> {
    fn from(n: T) -> Self {
        Self { re: n, im: T::_0 }
    }
}

impl<T: Float> FloatWrapper for Complex<T> {
    type InnerFloat = T;

    fn from_primitive(f: Self::InnerFloat) -> Self {
        Self { re: f, im: T::_0 }
    }

    fn into_primitive(self) -> T {
        eprint!("Converting complex number to primitive should be done with self.re, instead of self.into_primitive.");
        self.re
    }

    fn from_f64(f: f64) -> Self {
        Self {
            re: T::from_f64(f),
            im: T::_0,
        }
    }
}

impl<T: Float> Neg for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            im: -self.im,
            re: -self.re,
        }
    }
}

impl<T: Float> Float for Complex<T> {
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

    fn exp_(self) -> Self {
        (re(self.im.cos_()) + im(self.im.sin_())) * re(self.re.exp_())
    }

    fn cos_(self) -> Self {
        let mut neg_self = self;
        neg_self.re = self.re * -T::_1;
        (self.exp_() + (neg_self).exp_()) / Self::_2
    }

    fn sin_(self) -> Self {
        let mut neg_self = self;
        neg_self.re = self.re * -T::_1;
        (self.exp_() - (neg_self).exp_()) / (Self::_2 * self.re.into())
    }

    fn is_nan_(self) -> bool {
        self.re.is_nan_() || self.im.is_nan_()
    }

    fn is_infinite_(self) -> bool {
        self.re.is_infinite_() || self.im.is_infinite_()
    }
}

impl<T: Float> From<[T; 2]> for Complex<T> {
    fn from(n: [T; 2]) -> Self {
        Complex { re: n[0], im: n[1] }
    }
}

/// Same as `Complex{re: 0., im: f}`
pub const fn im<F: Float + Sized>(f: F) -> Complex<F> {
    Complex { re: F::_0, im: f }
}

/// Same as `Complex{re: f, im: 0.}`
pub const fn re<F: Float + Sized>(f: F) -> Complex<F> {
    Complex { re: f, im: F::_0 }
}

#[test]
fn complex_neg() {
    let a = im(3.212f32);
    let b = re(4.36f32);
    assert_eq!(-(a + b), re(-1.) * (a + b));
    assert_eq!(-(a - b), re(-1.) * (a - b));
}

macro_rules! impl_op_assign {
	($op: tt $name: ident $fn: ident) => {
        impl<T:Float> $name for Complex<T>{
            fn $fn(&mut self, rhs: Self) {
                *self = *self $op rhs
            }
        }
	};
}

impl_op_assign!(* MulAssign mul_assign);
impl_op_assign!(/ DivAssign div_assign);
impl_op_assign!(- SubAssign sub_assign);
impl_op_assign!(+ AddAssign add_assign);

#[test]
fn opposites() {
    let z1 = re(-2.3) + im(3.);
    let z2 = re(4.1) - im(2.);

    let test_1_error = (((z1 * z2) / z2) - z1).norm_sqrt();
    assert!(
        test_1_error < 0.000000000000000000000000000001,
        "error: {test_1_error}"
    );
    assert_eq!((z1 + z2) - z2, z1);
}
