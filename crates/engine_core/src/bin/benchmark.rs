use engine_core::ecs::{component::Component, world::World};
use std::time::Instant;

macro_rules! define_components {
    ($($name:ident),*) => {
        $(
            struct $name(u32);
            impl Component for $name {}
        )*
    };
}

define_components!(
    Comp0, Comp1, Comp2, Comp3, Comp4, Comp5, Comp6, Comp7, Comp8, Comp9, Comp10, Comp11, Comp12,
    Comp13, Comp14, Comp15, Comp16, Comp17, Comp18, Comp19, Comp20, Comp21, Comp22, Comp23, Comp24,
    Comp25, Comp26, Comp27, Comp28, Comp29, Comp30, Comp31, Comp32, Comp33, Comp34, Comp35, Comp36,
    Comp37, Comp38, Comp39, Comp40, Comp41, Comp42, Comp43, Comp44, Comp45, Comp46, Comp47, Comp48,
    Comp49
);

macro_rules! insert_dummy {
    ($world:expr, $entity:expr, $($name:ident),*) => {
        $(
            $world.insert_component($entity, $name(0));
        )*
    };
}

fn main() {
    let mut world = World::new();

    // Register a bunch of component storages
    let dummy_entity = world.spawn();
    insert_dummy!(
        world,
        dummy_entity,
        Comp0,
        Comp1,
        Comp2,
        Comp3,
        Comp4,
        Comp5,
        Comp6,
        Comp7,
        Comp8,
        Comp9,
        Comp10,
        Comp11,
        Comp12,
        Comp13,
        Comp14,
        Comp15,
        Comp16,
        Comp17,
        Comp18,
        Comp19,
        Comp20,
        Comp21,
        Comp22,
        Comp23,
        Comp24,
        Comp25,
        Comp26,
        Comp27,
        Comp28,
        Comp29,
        Comp30,
        Comp31,
        Comp32,
        Comp33,
        Comp34,
        Comp35,
        Comp36,
        Comp37,
        Comp38,
        Comp39,
        Comp40,
        Comp41,
        Comp42,
        Comp43,
        Comp44,
        Comp45,
        Comp46,
        Comp47,
        Comp48,
        Comp49
    );

    let mut entities = Vec::new();
    let num_entities = 100_000;

    println!("Spawning {} entities...", num_entities);
    for i in 0..num_entities {
        let e = world.spawn();
        if i % 2 == 0 {
            world.insert_component(e, Comp0(1));
        }
        if i % 3 == 0 {
            world.insert_component(e, Comp1(1));
        }
        entities.push(e);
    }

    let start_despawn = Instant::now();
    for e in entities {
        world.despawn(e);
    }
    let elapsed = start_despawn.elapsed();
    println!("Despawned {} entities in {:?}", num_entities, elapsed);
}
