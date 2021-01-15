use std::fmt;

use veho::entries::reference::unwind as ref_unwind;
use veho::entries::unwind as move_unwind;

use crate::enums::Effect;
use crate::fluo::vector::{
    fluo_colorant as fluo_vector_colorant,
    fluo_rendered as fluo_vector_rendered,
};
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
    let (keys, items) = move_unwind(entries);
    let keys = fluo_vector_rendered(keys, presets, effects);
    let items = fluo_vector_rendered(items, presets, effects);
    keys.into_iter().zip(items).collect()
}


pub fn fluo_colorant<K, V, KVS>(
    entries: KVS,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> Vec<(impl Fn(&str) -> String, impl Fn(&str) -> String)> where
    K: fmt::Display,
    V: fmt::Display,
    KVS: IntoIterator<Item=(K, V)>
{
    let (keys, items) = move_unwind(entries);
    let keys = fluo_vector_colorant(keys, presets, effects);
    let items = fluo_vector_colorant(items, presets, effects);
    keys.into_iter().zip(items).collect()
}

#[cfg(test)]
mod tests {
    use veho::entries::IntoHashmap;
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
            vec![("one", "1"), ("two", "2"), ("three", "3")].into_hashmap(),
        ];
        for entries in &candidates {
            let fluoed = fluo_colorant(
                entries,
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
}

pub mod reference {
    use super::*;

    pub fn fluo_rendered<'a, K, V, KVS>(
        entries: KVS,
        presets: &(Preset, Preset),
        effects: &[Effect],
    ) -> Vec<(String, String)> where
        K: 'a + fmt::Display,
        V: 'a + fmt::Display,
        KVS: IntoIterator<Item=&'a (K, V)>
    {
        let (keys, items) = ref_unwind(entries);
        let keys = fluo_vector_rendered(keys, presets, effects);
        let items = fluo_vector_rendered(items, presets, effects);
        keys.into_iter().zip(items).collect()
    }

    pub fn fluo_colorant<'a, K, V, KVS>(
        entries: KVS,
        presets: &(Preset, Preset),
        effects: &[Effect],
    ) -> Vec<(impl Fn(&str) -> String + 'a, impl Fn(&str) -> String + 'a)> where
        K: 'a + fmt::Display,
        V: 'a + fmt::Display,
        KVS: IntoIterator<Item=&'a (K, V)>
    {
        let (keys, items) = ref_unwind(entries);
        let keys = fluo_vector_colorant(keys, presets, effects);
        let items = fluo_vector_colorant(items, presets, effects);
        keys.into_iter().zip(items).collect()
    }

    #[cfg(test)]
    mod tests {
        use veho::vector::zipper;

        use crate::presets::{FRESH, OCEAN};

        use super::*;

        #[test]
        fn test_fluo_entries_colorant_ref() {
            let candidates = vec![
                vec![("one", "1"), ("two", "2"), ("three", "3")],
            ];
            for entries in &candidates {
                let fluoed = fluo_colorant(
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
    }
}
