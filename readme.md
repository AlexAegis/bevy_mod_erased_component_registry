# bevy_mod_erased_component_registry

[![ci](https://github.com/AlexAegis/bevy_mod_erased_component_registry/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/bevy_mod_erased_component_registry/actions/workflows/ci.yml)

Insert components into entities by their `TypeId`!

> The type must implement either `FromWorld` or `Default`, as new instances
> will be created using `from_world`.

## Usage

Register your `Component + FromWorld` type in a plugin:

```rs
app.register_erased_component::<GenericFlagComponent<A>>();
```

Create a `TypeId` of your type, and save it somewhere: into a resource, or in a
component.

```rs
let flag_a_type_id = TypeId::of::<GenericFlagComponent<A>>()
```

Then insert it into an entity using this `type_id`

> If you find yourself creating the `TypeId` right where you would call this
> insert function, then you definitely do not need to use this function.
> You know the type at the time of insertion, just insert it normally.

```rs
commands
    .spawn_empty()
    .insert_component_by_type_id(flag_a_type_id);
```

## Example

This example creates a randomly selected generic variant of a component every
time you press `Space`. The system that spawns this entity has no idea of the
actual type of this component!

```sh
cargo run -p bevy_mod_erased_component_registry --example erased_component_registry_example --features example
```

## Example Usecase

With required components, if you know that the entity where you use `Thing<T>`
will only ever have one kind of `Thing<T>` on it, you can require another
component that will hold the previous components `TypeId`! Giving you something
you can query without `T`, and create a new instance of the original
component without even knowing which kind of `Thing<T>` it was!

```rs
use core::marker::PhantomData;
use std::any::TypeId;

use bevy_ecs::component::Component;

#[derive(Component)]
#[require(ErasedThing::new::<T>())]
pub struct Thing<T> {
    _phantom_data: PhantomData<T>,
}

#[derive(Component)]
pub struct ErasedThing {
    type_id: TypeId,
}

impl ErasedThing {
    fn new<T: 'static>() -> Self {
        Self {
            type_id: TypeId::of::<Thing<T>>(),
        }
    }
}
```

## Why?

Bevy can insert components by their `ComponentId` into an entity, but it
requires you to construct the component. But there's no api to try that with
a possible `FromWorld` implementation, which would require an internal registry
of constructors. This crate does that, with the registry being a resource.

A "quick" user-space proof of concept can make a better case for upstreaming a
feature into bevy and makes it easy for users to try it out without having to
use nightly bevy!

> If that happens, migration would be very easy as the entire api is just 2
> functions, one to register a component, and one to insert it. The register
> function would just disappear and the insert function would at worst get
> renamed.

### Why `TypeId` instead of `ComponentId`?

To get a `ComponentId` you need world access, but a `TypeId` can be acquired
anywhere. And when you retrieve the `ComponentId` from the world, it uses
`TypeId` anyway, so it's fine to use `TypeId` in the user facing api.

Internally when interacting with the `World` it will use the `ComponentId`
associated with that `TypeId` in the `World`.

### Why no custom constructors?

You want consistency when inserting new components from the registry, to be
sure that if you insert a component now vs 10 minutes later, it will be created
the same way. Therefore changing the implementation of the constructor during
runtime must be prohibited. And if it's prohibited then your only option to
define it is during App build inside a plugin. At that point, using `FromWorld`
is the perfect solution do define that custom constuctor.

## Development of this repository

This repository is using `cargo-make`, it will take care of installing all
required cargo extensions and rustup components used in this repository.

1. Run `cargo install cargo-make`
2. (Optional) Install the rest of the tooling/cargo extensions using
   `cargo make setup`

### Requirements

- [Git LFS](https://git-lfs.github.com/)
- [Latest Rust Stable](https://rustup.rs/)

### `cargo-make` tasks

- `cargo make all` to run everything that could make ci fail (Everything below)
- `cargo make build` to build all crates
- `cargo make test` to test all crates
- `cargo make format` to format all crates
- `cargo make lint` to lint all crates using `clippy` and `rustfmt`
- `cargo make book-build` to build the documentation book

## Bevy Compatibility Table

| Bevy | bevy_mod_erased_component_registry |
| ---- | ---------------------------------- |
| 0.17 | 0.2                                |
| 0.16 | 0.1                                |
