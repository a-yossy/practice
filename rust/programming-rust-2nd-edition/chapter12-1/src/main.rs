use std::{
    ops::{Add, AddAssign, Neg},
    vec,
};

use num::Complex;

// impl<L, R> Add<Complex<R>> for Complex<L>
// where
//     L: Add<R>,
// {
//     type Output = Complex<L::Output>;
//     fn add(self, rhs: Complex<R>) -> Self::Output {
//         Complex {
//             re: self.re + rhs.re,
//             im: self.im + rhs.im,
//         }
//     }
// }

// impl<T> Neg for Complex<T>
// where
//     T: Neg<Output = T>,
// {
//     type Output = Complex<T>;
//     fn neg(self) -> Self::Output {
//         Complex {
//             re: -self.re,
//             im: -self.im,
//         }
//     }
// }

// impl<T> AddAssign for Complex<T>
// where
//     T: AddAssign<T>,
// {
//     fn add_assign(&mut self, rhs: Self) {
//         self.re += rhs.re;
//         self.im += rhs.im;
//     }
// }

// trait PartialEq<Rhs = Self>
// where
//     Rhs: ?Sized,
// {
//     fn eq(&self, other: &Rhs) -> bool;
//     fn ne(&self, other: &Rhs) -> bool {
//         !self.eq(other)
//     }
// }

// impl<T: PartialEq> PartialEq for Complex<T> {
//     fn eq(&self, other: &Complex<T>) -> bool {
//         self.re == other.re && self.im == other.im
//     }
// }

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T,
    upper: T,
}

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(std::cmp::Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

trait Index<Idx> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}

trait IndexMut<Idx>: Index<Idx> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}

struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

fn main() {
    assert!(
        Interval {
            lower: 10,
            upper: 20
        } < Interval {
            lower: 20,
            upper: 40
        }
    );

    let mut desserts = vec![
        "cheesecake".to_string(),
        "tiramisu".to_string(),
        "pavlova".to_string(),
        "chocolate mousse".to_string(),
    ];
    desserts[0].push_str(" with strawberries");
    desserts[1].push_str(" with coffee");
}
