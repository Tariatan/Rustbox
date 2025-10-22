#![allow(unused)]

use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

trait CountChar {
    fn count_char(&self, c: char) -> usize;
}
impl CountChar for String {
    fn count_char(&self, c: char) -> usize {
        self.chars().filter(|&x| x == c).count()
    }
}

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
// Accepts any type that implements Pet
pub fn walk(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// The above 'impl Trait' is a syntax sugar for 'trait bounds'

// Trait bounds
pub fn mate<T: Pet>(pet1: &T, pet2: &T) {
    println!("Hello, we are {} and {}", pet1.name(), pet2.name());
}

// Specifying multiple trait bounds
pub fn real_cat(kitty: &(impl Animal + Pet)) {
    println!("Hello, I am {}, I have {} legs", kitty.name(), kitty.leg_count());
}

// Specifying multiple trait bounds with generic types
pub fn real_dog<T: Animal + Pet>(doggy: &T) {
    println!("Hello, I am {}, I have {} legs", doggy.name(), doggy.leg_count());
}

// Specifying 'where clauses'
pub fn unreal_cat<T, U>(t: &T, u: &U) -> i32
    where T: Animal + Pet,
          U: Animal
{
    42
}

// Returning types that implement traits without naming the  concrete type
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
    
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Conditionally implementing methods on a generic type depending on trait bounds
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
#[derive(PartialEq, Debug, Clone)]
pub struct GBP(i32);
#[derive(PartialEq, Debug, Clone)]
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

impl Display for USD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = (self.0 as f32) / 100.;
        if r < 0. {
            return write!(f, "-${:.2}", -r);
        }
        write!(f, "{:.2}", r)
    }
}

impl Clone for USD {
    fn clone(&self) -> USD {
        USD(self.0)
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

impl FromStr for GBP {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GBP(s.parse::<i32>()?))
    }
}


pub fn converter() {
    let g = GBP(200);
    let u = g.to_usd();
    println!("{:?}", u.to_string());    // implemented Display trait comes into play here
    let c = CAD::from_usd(&u);
    println!("{:?}", c);

    let c2:CAD = g.convert();
    println!("{:?}", c2);
}

pub trait ToUSDv<F> {
    fn to_uv(&self, g:F) -> f32;
}

pub trait FromUSDv<F> {
    #[allow(clippy::wrong_self_convention)]
    fn from_uv(&self, g:f32) -> F;
}

pub struct Ex {
    account_id: i32,
    cad: f32,
    gbp: f32,
}

pub trait Exchange<F, T> {
    fn convert(&self, f: F) -> T;
}

#[derive(PartialEq, Debug, Clone)]
pub struct Transaction<A> {
    from_id:i32,
    to_id:i32,
    amount:A,
}

pub trait Account {
    fn id(&self) -> i32;
}

impl Account for Ex {
    fn id(&self) -> i32 {
        self.account_id
    }
}
impl<E, F, T> Exchange<F, T> for E
    where E: ToUSDv<F> + FromUSDv<T>
{
    fn convert(&self, f: F) -> T {
        self.from_uv(self.to_uv(f))
    }
}
pub  trait ExchangeAccount<F, T> {
    fn exchange(&self, f_id:i32, t_id:i32, amount:F) -> (Transaction<F>, Transaction<T>);
}

impl<E, F, T> ExchangeAccount<F, T> for E
    where E:Exchange<F, T> + Account,
          F:Clone
{
    fn exchange(&self, f_id: i32, t_id: i32, amount: F) -> (Transaction<F>, Transaction<T>) {
        let ft = Transaction {from_id:f_id, to_id:self.id(), amount:amount.clone()};
        let tt = Transaction {from_id:self.id(), to_id:t_id, amount:self.convert(amount)};
        (ft, tt)
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

pub fn exchanger() {
    let g = GBP(200);
    let ex = Ex{ account_id: 0, cad:0.7, gbp:1.3};
    let c = ex.from_uv(ex.to_uv(g));
    println!("{:?}", c);

    // or
    let g = GBP(200);
    let c = ex.convert(g);
    println!("{:?}", c);
    
    let ex = Ex {account_id:30, cad:0.7, gbp:1.3};
    let (ft, tt) = ex.exchange(20, 40, GBP(200));
}

// Supertraits
trait Amphibious : LandCapable + WaterCapable{}
trait LandCapable {
    fn drive(&self) { println!("Drive"); }
}
trait WaterCapable {
    fn float(&self) { println!("Float"); }
}
struct Sedan;
impl LandCapable for Sedan {}
struct SUV;
impl LandCapable for SUV {}

struct Hovercraft;
impl Amphibious for Hovercraft {}
impl LandCapable for Hovercraft {}
impl WaterCapable for Hovercraft {}

fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}
fn traverse_frozen_lake(vehicle: &impl Amphibious) {
    vehicle.drive();
    vehicle.float();
}
fn trip() {
    let hc = Hovercraft;
    traverse_frozen_lake(&hc);
}