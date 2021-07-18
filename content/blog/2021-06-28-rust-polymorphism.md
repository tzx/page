+++
title = "Rust Polymorphism"
+++

This is my first post. This was from handwritten notes that I took 2 years ago.
I was inspired to actually put stuff online from someone's similar post about
static and dynamic dispatch in Rust with detailed binary inspection. I wish I
can find it again so I can give credit :(. 

### What is Polymorphism?

Polymorphism allows us to provide a group of functions for multiple concrete
types, allowing shared behavior. In most languages, we can use interfaces to
implement this behavior. In Rust, we can use traits. Most examples that I see
about polymorphism are with animals, so let's try it here.

```rust
trait Animal {
    fn make_noise();
}

struct Cat;
impl Animal for Cat {
    fn make_noise(&self) {
        println!("meow")
    }
}

struct Dog;
impl Animal for Dog {
    fn make_noise(&self) {
        println!("woof")
    }
}
```

### Using Trait Bounds

In Rust, we can use trait bounds to make a generic function be any type that
has the behavior we specified. This allows us to have a function that takes any
struct that implements the Animal trait and call the shared behavior of the
trait Animal: `make_noise`.

```rust
fn pls_make_noise(a: impl Animal) {
  a.make_noise();
}

fn main() {
  pls_make_noise(Cat{});
  pls_make_noise(Dog{});
}
```

### Using Trait Objects

However, this would not work if we want to take in a vector of different types
that implement the same trait. A `Vec<impl Animal>` only allows only one type
that implements the `Animal` trait, so we can't have both `Cat` and `Dog` in that
vector. This is because trait bounds are checked during compile time, so the
types must match. To resolve this, we need to have a vector of trait objects. A
trait object is a pointer that points to the type that implements our trait and
a table to lookup the trait methods during runtime, using dynamic dispatch. A
way to create one is to make a reference `&` with the `dyn` keyword after
(technically optional but new versions of the compiler will warn you). Let's
now make a vector of trait objects and a function that takes this vector and
calls the shared trait method for each of the objects.

```rust
fn pls_all_make_noise(animals: Vec<&dyn Animal>) {
  for a in animals {
    a.make_noise();
  }
}

fn main() {
  let animals: Vec<&dyn Animal> = vec![&Dog{}, &Cat{}];
  pls_all_make_noise(animals);
}
```

If you wanted the trait objects to be owned rather than being references, you
can use `Box`.

### But Wait! Consider Enums First

In Rust, enums are allowed to have data along with them, meaning we can create
a `Animal` enum to hold our `Dog` and `Cat` structs (although we didn't hold
data for our structs in our Traits example). We also can hold a `Vec<Animal>`
without needing to use dynamic dispatch. I believe how it works under the hood
is something that is similar to tagged unions which only needs compile time
checks. To achieve polymorphism, we can implement a function on the enum with a
`match` expression to have a single place for our shared behavior.

```rust
enum Animal {
  Cat { name: &'static str },
  Dog { name: &'static str },
}

impl Animal {
  fn make_noise(&self) {
    match self {
      Animal::Cat { name } =>
        println!("{} says meow", name),
      Animal::Dog { name } =>
        println!("{} says woof", name),
    }
  }
}

fn pls_all_make_noise(animals: Vec<Animal>) {
  for a in animals {
    a.make_noise();
  }
}

fn main() {
  // I am not good with names
  let dog = Animal::Dog { name: "Dog" };
  let cat = Animal::Cat { name: "Cat" };
  let animals = vec![dog, cat];
  pls_all_make_noise(animals);
}
```

Therefore, if you know the number of variants that have shared behavior, you
should probably use an enum. Otherwise, you would have to use trait objects;
the most common case of this happening is exposing a library where people can
add their own types to have your shared behavior.
