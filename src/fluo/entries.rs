use std::fmt;

use veho::entries::{MoveUnwind, RefUnwind};

use crate::enums::Effect;
use crate::fluo::vector::{fluo_colorant as fluo_vector_colorant, fluo_rendered as fluo_vector_rendered};
use crate::types::Preset;

pub fn fluo_rendered<K, V, KVS>(
    entries: KVS,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> Vec<(String, String)> where
    K: fmt::Display,
    V: fmt::Display,
    KVS: IntoIterator<Item=(K, V)>
{
    let (keys, items) = entries.move_unwind();
    let keys = fluo_vector_rendered(&keys, presets, effects);
    let items = fluo_vector_rendered(&items, presets, effects);
    let rendered = keys.into_iter().zip(items).collect::<Vec<(String, String)>>();
    rendered
}

pub fn fluo_rendered_ref<'a, K, V, KVS>(
    entries: KVS,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> Vec<(String, String)> where
    K: 'a + fmt::Display,
    V: 'a + fmt::Display,
    KVS: IntoIterator<Item=&'a (K, V)>
{
    let (keys, items) = entries.ref_unwind();
    let keys = fluo_vector_rendered(&keys, presets, effects);
    let items = fluo_vector_rendered(&items, presets, effects);
    let rendered = keys.into_iter().zip(items).collect::<Vec<(String, String)>>();
    rendered
}

pub type Dye = impl Fn(&str) -> String;

pub fn fluo_colorant<K, V, KVS>(
    entries: KVS,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> Vec<(Dye, Dye)> where
    K: fmt::Display,
    V: fmt::Display,
    KVS: IntoIterator<Item=(K, V)>
{
    let (keys, items) = entries.move_unwind();
    let keys = fluo_vector_colorant(&keys, presets, effects);
    let items = fluo_vector_colorant(&items, presets, effects);
    keys.into_iter().zip(items).collect()
}

pub fn fluo_colorant_ref<'a, K, V, KVS>(
    entries: KVS,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> Vec<(Dye, Dye)> where
    K: 'a + fmt::Display,
    V: 'a + fmt::Display,
    KVS: IntoIterator<Item=&'a (K, V)>
{
    let (keys, items) = entries.ref_unwind();
    let keys = fluo_vector_colorant(&keys, presets, effects);
    let items = fluo_vector_colorant(&items, presets, effects);
    keys.into_iter().zip(items).collect()
}


#[cfg(test)]
mod tests {
    use veho::hashmap::MoveInit;
    use veho::vector::{Mappers, zipper};

    use crate::presets::{FRESH, OCEAN};

    use super::*;

    #[test]
    fn test_fluo_entries_rendered() {
        let candidates = vec![
            vec![("one", "1"), ("two", "2"), ("three", "3")].into_hashmap(),
        ];
        for entries in candidates {
            let fluoed = fluo_rendered(
                &mut entries.into_iter(),
                &(OCEAN, FRESH),
                &[Effect::Bold],
            );
            let tx = fluoed.mapper(|(k, v)| format!("[{}] ({})", k, v)).join(", ");
            println!("[ {} ]", tx);
        }
    }

    #[test]
    fn test_fluo_entries_colorant() {
        let candidates = vec![
            vec![("one", "1"), ("two", "2"), ("three", "3")],
        ];
        for entries in &candidates {
            let fluoed = fluo_colorant_ref(
                entries.into_iter(),
                &(OCEAN, FRESH),
                &[Effect::Bold],
            );
            let results = zipper(entries, fluoed, |kv, fs| {
                format!("[{}] ({})", (fs.0)(kv.0), (fs.1)(kv.1))
            });
            let tx = results.join(", ");
            println!("test_fluo_entries_colorant: [ {} ]", tx);
        }
    }
    //
    // #[test]
    // fn test_fluo_vector_colorant2() {
    //     let candidates = vec![
    //         ["1", "2", "3", "foo", "bar", "zen"],
    //         ["1", "2", "3", "4", "5", "6"],
    //         ["foo", "bar", "zen", "-", "--", "---"]
    //     ];
    //     for vec in &candidates {
    //         let fluoed = fluo_colorant(
    //             vec,
    //             &(OCEAN, FRESH),
    //             &[Effect::Bold],
    //         );
    //         let results = zipper(vec, fluoed, |x, f| {
    //             f(x)
    //         });
    //         let tx = results.join(", ");
    //         println!("[ {} ]", tx);
    //     }
    // }
}
