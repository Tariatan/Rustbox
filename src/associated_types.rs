#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
struct Meters(i32);

#[derive(Debug)]
#[allow(dead_code)]
struct MetersSquared(i32);

#[allow(dead_code)]
trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared;
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}