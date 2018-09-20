pub struct Triangle {
    a: u64,
    b: u64,
    c: u64
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if is_triangle( &sides ) {
            Some( Triangle { a: sides[0], b: sides[1], c: sides[2] })
        } else { None }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.c != self.a
    }

    pub fn is_isosceles(&self) -> bool {
        (self.a == self.b && self.b != self.c) ||
            (self.b == self.c && self.c != self.a) ||
            (self.c == self.a && self.a != self.b)
    }
}

fn is_triangle(sides: &[u64; 3]) -> bool {
    match (sides[0], sides[1], sides[2]) {
        (0, 0, 0) => false,
        (a, b, c) => ((a + b) >= c) && ((b + c) >= a) && ((c + a) >= b)
    }
}
