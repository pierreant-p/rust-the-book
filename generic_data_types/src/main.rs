// generics in functions
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generics in struct
struct Point<T> {
    x: T,
    y: T,
}

struct PointWithMixedTypes<T, U> {
    x: T,
    y: U,
}

// in methods definitions

// definition for all types
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// implementation just for one type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> PointWithMixedTypes<T, U> {
    fn mixup<V, W>(self, other: PointWithMixedTypes<V, W>) -> PointWithMixedTypes<T, W> {
        PointWithMixedTypes {
            x: self.x,
            y: other.y,
        }
    }
}

// generics in enum definition
enum SomeEnum<T, U> {
    Variant(T),
    AnotherVariant(U),
}

fn main() {
    let v1 = vec![1, 17, 2, 100, -1];
    let largest_int = largest(&v1);
    println!("{}", largest_int);

    let v2 = vec!['a', 'b', 'y', 'd'];
    let largest_char = largest(&v2);
    println!("{}", largest_char);

    let float_point = Point { x: 1.0, y: 3.4 };
    let in_point = Point { x: 1, y: 8 };

    let mixed_point = PointWithMixedTypes { x: 1, y: 4.5 };

    let p1 = PointWithMixedTypes { x: 1.0, y: 5 };
    let p2 = PointWithMixedTypes { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
}

// Note: generics are not slower than specific code because Rust performs Monomorphization
// (ie the compiler generates the specific versions of the code that are used)
