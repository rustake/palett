use std::fmt;

use aryth::Bound;
use aryth::duobound::vector::duobound;
use num::Float;
use veho::vector::Mappers;

use crate::enums::Effect;
use crate::projector::ProjectorFactory;
use crate::types::Preset;

pub fn fluo_rendered<I>(
    vec: I,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> Vec<String> where
    I: IntoIterator + Copy,
    I::Item: fmt::Display
{
    let (
        (vec_x, dye_fac_x),
        (vec_y, dye_fac_y)
    ) = make_projector(vec, presets, effects);
    let projector = to_rendered(&vec_x, &dye_fac_x, &vec_y, &dye_fac_y);
    vec.indexed_mapper(|i, x| projector(i, &x.to_string()))
}

pub fn fluo_colorant<I>(
    vec: I,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> Vec<Dye> where
    I: IntoIterator + Copy,
    I::Item: fmt::Display
{
    let (
        (vec_x, dye_fac_x),
        (vec_y, dye_fac_y)
    ) = make_projector(vec, presets, effects);
    let projector = to_colorant(&vec_x, &dye_fac_x, &vec_y, &dye_fac_y);
    vec.indexed_mapper(|i, x| projector(i, &x))
}

fn make_projector<I>(
    vec: I,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> ((Vec<Option<f32>>, ProjectorFactory), (Vec<Option<f32>>, ProjectorFactory)) where
    I: IntoIterator,
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
    return ((vec_x, dye_fac_x), (vec_y, dye_fac_y));
}

fn to_rendered<'a>(
    vec_x: &'a Vec<Option<f32>>,
    dye_fac_x: &'a ProjectorFactory,
    vec_y: &'a Vec<Option<f32>>,
    dye_fac_y: &'a ProjectorFactory,
) -> impl Fn(usize, &str) -> String + 'a
{
    move |i, tx| {
        if let Some(v) = vec_x[i] { return dye_fac_x.render(v, tx); }
        if let Some(v) = vec_y[i] { return dye_fac_y.render(v, tx); }
        return dye_fac_x.render(f32::nan(), tx);
    }
}

pub type Dye = impl Fn(&str) -> String;

fn to_colorant<'a, T>(
    vec_x: &'a Vec<Option<f32>>,
    dye_fac_x: &'a ProjectorFactory,
    vec_y: &'a Vec<Option<f32>>,
    dye_fac_y: &'a ProjectorFactory,
) -> impl Fn(usize, &T) -> Dye + 'a
    where T: fmt::Display
{
    move |i, _| {
        if let Some(v) = vec_x[i] { return dye_fac_x.make(v); }
        if let Some(v) = vec_y[i] { return dye_fac_y.make(v); }
        return dye_fac_x.make(f32::nan());
    }
}


#[cfg(test)]
mod tests {
    use veho::vector::zipper;

    use crate::presets::{FRESH, OCEAN};

    use super::*;

    #[test]
    fn test_fluo_vector_rendered() {
        let candidates = vec![
            vec!["1", "2", "3", "foo", "bar", "zen"],
            vec!["1", "2", "3", ],
            vec!["foo", "bar", "zen"]
        ];
        for vec in &candidates {
            let fluoed = fluo_rendered(
                vec,
                &(OCEAN, FRESH),
                &[Effect::Bold],
            );
            let tx = fluoed.join(", ");
            println!("[ {} ]", tx);
        }
    }

    #[test]
    fn test_fluo_vector_colorant() {
        let candidates = vec![
            vec!["1", "2", "3", "foo", "bar", "zen"],
            vec!["1", "2", "3", ],
            vec!["foo", "bar", "zen"]
        ];
        for vec in &candidates {
            let fluoed = fluo_colorant(
                vec,
                &(OCEAN, FRESH),
                &[Effect::Bold],
            );
            let results = zipper(vec, fluoed, |x, f| {
                f(x)
            });
            let tx = results.join(", ");
            println!("[ {} ]", tx);
        }
    }

    #[test]
    fn test_fluo_vector_colorant2() {
        let candidates = vec![
            ["1", "2", "3", "foo", "bar", "zen"],
            ["1", "2", "3", "4", "5", "6"],
            ["foo", "bar", "zen", "-", "--", "---"]
        ];
        for vec in &candidates {
            let fluoed = fluo_colorant(
                vec,
                &(OCEAN, FRESH),
                &[Effect::Bold],
            );
            let results = zipper(vec, fluoed, |x, f| {
                f(x)
            });
            let tx = results.join(", ");
            println!("[ {} ]", tx);
        }
    }
}
