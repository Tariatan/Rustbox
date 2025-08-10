use std::fmt::Display;

pub trait Animal {
    fn leg_count(&self) -> u32 {
        4                           // Default trait implementation
    }
}

pub trait Pet: Animal {
    fn name(&self) -> String;
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

// It’s common to derive PartialEq/Eq traits, but uncommon to implement them.
impl PartialEq<Dog> for Cat {
    fn eq(&self, _other: &Dog) -> bool {
        false
    }
}

pub struct Dog {
    pub name: String,
    pub age: i8,
}
pub struct Cat {
    pub name: String,
    pub lives: i8,
}
impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Animal for Cat {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) -> String {
       format!("Miau! {} lives left!", self.lives)
    }
}
// Multiple impl blocks are allowed for a given type.
// Furthermore, impl blocks can even be spread across multiple modules/files.
impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) -> String {
        format!("Woof, my name is {}! I have {} legs! I'm {}", self.name(), self.leg_count(), self.age)
    }
}

// Uses generics and static dispatch.
pub fn generic(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// Uses type-erasure and dynamic dispatch.
// Represents two pointers:
//  -   to the data (an instance of a struct)
//  -   to a vtable.
pub fn dynamic(pet: &dyn Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    // Associated types (like Output) are controlled by the implementer of a trait.
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

// Traits as parameters
#[allow(unused)]
// Accepts any type that implements Pet
pub fn walk(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// The above 'impl Trait' is a syntax sugar for 'trait bounds'

// Trait bounds
#[allow(unused)]
pub fn mate<T: Pet>(pet1: &T, pet2: &T) {
    println!("Hello, we are {} and {}", pet1.name(), pet2.name());
}

// Specifying multiple trait bounds
#[allow(unused)]
pub fn real_cat(kitty: &(impl Animal + Pet)) {
    println!("Hello, I am {}, I have {} legs", kitty.name(), kitty.leg_count());
}

// Specifying multiple trait bounds with generic types
#[allow(unused)]
pub fn real_dog<T: Animal + Pet>(doggy: &T) {
    println!("Hello, I am {}, I have {} legs", doggy.name(), doggy.leg_count());
}

// Specifying 'where clauses'
#[allow(unused)]
pub fn unreal_cat<T, U>(t: &T, u: &U) -> i32
    where T: Animal + Pet,
          U: Animal
{
    42
}

// Returning types that implement traits without naming the  concrete type
#[allow(unused)]
pub fn adopt() -> impl Pet {
    Cat {
        name: "Tom".to_string(),
        lives: 9
    }
}

struct Pair<T>  {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    #[allow(unused)]
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Conditionally implementing methods on a generic type depending on trait bounds
#[allow(unused)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        }  else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct USD(i32);
#[derive(PartialEq, Debug)]
pub struct GBP(i32);
#[derive(PartialEq, Debug)]
pub struct CAD(i32);

pub trait ToUSD {
    fn to_usd(&self) -> USD;
    fn convert<T:FromUSD>(&self) -> T {
        T::from_usd(&self.to_usd())
    }
}
impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD((self.0 * 130) / 100)
    }
}

pub trait FromUSD {
    fn from_usd(u:&USD) -> Self;
}

impl FromUSD for CAD {
    fn from_usd(u:&USD) -> Self {
        CAD((u.0 * 130) / 100)
    }
}

#[allow(unused)]
pub fn converter() {
    let g = GBP(200);
    let u = g.to_usd();
    println!("{:?}", u);
    let c = CAD::from_usd(&u);
    println!("{:?}", c);
    
    let c2:CAD = g.convert();
    println!("{:?}", c2);
}

pub trait ToUSDv<F> {
    fn to_uv(&self, g:F) -> f32;
}

pub trait FromUSDv<F> {
    fn from_uv(&self, g:f32) -> F;
}

pub struct Ex {
    cad:f32,
    gbp:f32,
}

pub trait Exchange<F, T> {
    fn convert(&self, f: F) -> T;
}

impl<E, F, T> Exchange<F, T> for E
    where E: ToUSDv<F> + FromUSDv<T>
{
    fn convert(&self, f: F) -> T {
        self.from_uv(self.to_uv(f))
    }
}
impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g:GBP) -> f32 {
        (g.0 as  f32) * self.gbp
    }
}

impl FromUSDv<CAD> for Ex {
    fn from_uv(&self, g: f32) -> CAD {
        CAD((g/self.cad) as i32)
    }
}
#[allow(unused)]
pub fn exchanger() {
    let g = GBP(200);
    let ex = Ex{cad:0.7, gbp:1.3};
    let c = ex.from_uv(ex.to_uv(g));
    println!("{:?}", c);

    // or
    let g = GBP(200);
    let c = ex.convert(g);
    println!("{:?}", c);
}

