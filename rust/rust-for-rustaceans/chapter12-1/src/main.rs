use std::marker::PhantomData;

struct ArrayVec<T, const N: usize> {
    values: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    fn try_push(&mut self, t: T) -> Result<(), T> {
        if self.len == N {
            return Err(t);
        }
        self.values[self.len] = Some(t);
        self.len += 1;
        Ok(())
    }
}

mod registers;

pub struct On;
pub struct Off;

pub struct Pair<R1, R2>(PhantomData<(R1, R2)>);

impl Pair<Off, Off>  {
    pub fn get() -> Option<Self> {
        static mut PAIR_TAKEN: bool = false;
        if unsafe {PAIR_TAKEN} {
            None
        } else {
            registers::off("r1");
            registers::off("r2");
            unsafe { PAIR_TAKEN = true; }
            Some(Pair(PhantomData))
        }
    }

    pub fn first_on(self) -> Pair<On, Off> {
        registers::set_on("r1");
        Pair(PhantomData)
    }
}

impl Pair<On, Off> {
    pub fn off(self) -> Pair<Off, Off> {
        registers::set_off("r1");
        Pair(PhantomData)
    }
}

fn main() {
    println!("Hello, world!");
}
