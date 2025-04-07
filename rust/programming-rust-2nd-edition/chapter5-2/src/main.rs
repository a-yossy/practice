static mut STASH: &i32 = &128;
static WORTH_POINTING_AT: i32 = 1000;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

struct S<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn f2<'a>(r: &'a i32, s: &'a i32) -> &'a i32 {
    r
}

fn sum_r_xy(r: &i32, s: S) -> i32 {
    r + s.x + s.y
}

fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
    (&point[0], &point[2])
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

fn main() {
    f(&WORTH_POINTING_AT);

    let parabola = [];
    let s = smallest(&parabola);
    println!("{}", s);

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }

    println!("{}", r);

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;

    let mut v = (136, 139);
    let m = &mut v;
    let r1 = &m.1;
    let m0 = &mut m.0;
    *m0 = 137;
    println!("{}", r1);
}
