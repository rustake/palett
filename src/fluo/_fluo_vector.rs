use std::fmt;

use aryth::Bound;
use aryth::duobound::vector::duobound;
use veho::vector::Mappers;

use crate::enums::Effect;
use crate::presets::{FRESH, OCEAN};
use crate::projector::{preset_to_flat, ProjectorFactory};
use crate::types::Preset;

pub fn fluo_vector<I>(
    vec: I,
    presets: &(Preset, Preset),
    effects: &[Effect],
    colorant: bool,
) -> Vec<String> where
    I: IntoIterator + Copy,
    I::Item: fmt::Display
{
    let (pre_x, pre_y) = presets;
    let (vb_x, vb_y) = duobound(vec);
    let (vec_x, bound_x) = vb_x.move_to_tuple();
    let (vec_y, bound_y) = vb_y.move_to_tuple();
    let (bound_x, bound_y) = (
        bound_x.unwrap_or_else(Bound::<f32>::default),
        bound_y.unwrap_or_else(Bound::<f32>::default),
    );
    let (dye_fac_x, dye_fac_y) = (
        ProjectorFactory::build(&bound_x, &pre_x, effects),
        ProjectorFactory::build(&bound_y, &pre_y, effects)
    );
    let projector = to_pigment(
        &vec_x, &dye_fac_x, &vec_y, &dye_fac_y,
        preset_to_flat(&pre_x, &[Effect::Bold]),
    );
    vec.indexed_mapper(|i, x| projector(i, &x.to_string()))
}

fn to_pigment<'a>(
    vec_x: &'a Vec<Option<f32>>,
    dye_fac_x: &'a ProjectorFactory,
    vec_y: &'a Vec<Option<f32>>,
    dye_fac_y: &'a ProjectorFactory,
    default_dye: impl Fn(&str) -> String + 'a,
) -> impl Fn(usize, &str) -> String + 'a
{
    move |i, tx| {
        if let Some(v) = vec_x[i] { return dye_fac_x.make(v)(tx); }
        if let Some(v) = vec_y[i] { return dye_fac_y.make(v)(tx); }
        return default_dye(tx);
    }
}

// fn to_colorant<'a>(
//     vec_x: &'a Vec<Option<f32>>,
//     dye_fac_x: &'a ProjectorFactory,
//     vec_y: &'a Vec<Option<f32>>,
//     dye_fac_y: &'a ProjectorFactory,
//     default_dye: impl Fn(&str) -> String + 'a,
// ) -> impl Fn(usize, &str) -> ( Fn(&str) -> String + 'a)
// {
//     move |i, tx| {
//         if let Some(v) = vec_x[i] { return dye_fac_x.make(v); }
//         if let Some(v) = vec_y[i] { return dye_fac_y.make(v); }
//         return default_dye;
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluo_vector_pigment_mode() {
        let candidates = vec![
            vec!["1", "2", "3", "foo", "bar", "zen"],
            vec!["1", "2", "3", ],
            vec!["foo", "bar", "zen"]
        ];
        for vec in &candidates {
            let fluoed = fluo_vector(
                vec,
                &(OCEAN, FRESH),
                &[Effect::Bold],
                true,
            );
            let tx = fluoed.join(", ");
            println!("[ {} ]", tx);
        }
    }
}
