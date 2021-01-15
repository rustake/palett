use std::fmt;

use aryth::Bound;
use aryth::duobound::matrix::duobound;
use num::Float;
use veho::matrix::{Mappers, trizipper, zipper};
use veho::matrix::Matrix;

use crate::enums::Effect;
use crate::projector::ProjectorFactory;
use crate::types::Preset;

pub fn fluo_rendered<M, R>(
    matrix: M,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> Matrix<String> where
    M: IntoIterator<Item=R> + Copy,
    R: IntoIterator + Copy,
    R::Item: fmt::Display
{
    let mx_v: Matrix<String> = matrix.mapper(|el| el.to_string());
    let ((mx_x, dye_fac_x),
        (mx_y, dye_fac_y)) = make_projector(matrix, presets, effects);
    trizipper(mx_x, mx_y, mx_v, |x, y, tx| {
        if let Some(v) = x { return dye_fac_x.render(v, &tx); }
        if let Some(v) = y { return dye_fac_y.render(v, &tx); }
        return dye_fac_x.render(f32::nan(), &tx);
    })
}

pub fn fluo_colorant<M, R>(
    matrix: M,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> Matrix<impl Fn(&str) -> String> where
    M: IntoIterator<Item=R> + Copy,
    R: IntoIterator + Copy,
    R::Item: fmt::Display
{
    let ((vec_x, dye_fac_x),
        (vec_y, dye_fac_y)) = make_projector(matrix, presets, effects);
    zipper(vec_x, vec_y, |x, y| {
        if let Some(v) = x { return dye_fac_x.make(v); }
        if let Some(v) = y { return dye_fac_y.make(v); }
        return dye_fac_x.make(f32::nan());
    })
}

fn make_projector<M, R>(
    vec: M,
    presets: &(Preset, Preset),
    effects: &[Effect],
) -> ((Matrix<Option<f32>>, ProjectorFactory), (Matrix<Option<f32>>, ProjectorFactory)) where
    M: IntoIterator<Item=R> + Copy,
    R: IntoIterator + Copy,
    R::Item: fmt::Display
{
    let (pre_x, pre_y) = presets;
    let (mb_x, mb_y) = duobound(vec);
    let (matrix_x, bound_x) = mb_x.move_to_tuple();
    let (matrix_y, bound_y) = mb_y.move_to_tuple();
    let (bound_x, bound_y) = (
        bound_x.unwrap_or_else(Bound::<f32>::default),
        bound_y.unwrap_or_else(Bound::<f32>::default),
    );
    let (fac_x, fac_y) = (
        ProjectorFactory::build(&bound_x, &pre_x, effects),
        ProjectorFactory::build(&bound_y, &pre_y, effects)
    );
    return ((matrix_x, fac_x), (matrix_y, fac_y));
}

#[cfg(test)]
mod tests {
    use veho::matrix::zipper;
    use veho::vector::mapper as mapper_vector;

    use crate::presets::{FRESH, OCEAN};

    use super::*;

    #[test]
    fn test_fluo_matrix_rendered() {
        let candidates = vec![
            vec![
                ["1", "2", "3"],
                ["foo", "bar", "zen"],
            ],
            vec![
                ["1", "2", "3"]
            ],
            vec![
                ["foo", "bar", "zen"]
            ]
        ];
        for matrix in &candidates {
            let fluoed = fluo_rendered(
                matrix,
                &(OCEAN, FRESH),
                &[Effect::Bold],
            );
            let tx = mapper_vector(fluoed, |row| format!("[ {} ]", row.join(", "))).join(", ");
            println!("[ {} ]", tx);
        }
    }

    #[test]
    fn test_fluo_matrix_colorant() {
        let candidates = vec![
            vec![
                ["1", "2", "3"],
                ["foo", "bar", "zen"],
            ],
            vec![
                ["1", "2", "3"]
            ],
            vec![
                ["foo", "bar", "zen"]
            ]
        ];
        for matrix in &candidates {
            let fluoed = fluo_colorant(
                matrix,
                &(OCEAN, FRESH),
                &[Effect::Bold],
            );
            let results = zipper(matrix, fluoed, |x, f| {
                f(x)
            });
            let tx = mapper_vector(results, |row| format!("[ {} ]", row.join(", "))).join(", ");
            println!("[ {} ]", tx);
        }
    }

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
