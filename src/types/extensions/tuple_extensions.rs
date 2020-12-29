use num::Num;

use crate::types::TDV;

pub fn div<T: Num + Copy>(tdv: &TDV<T>, other: T) -> TDV<T> {
    let (x, y, z) = *tdv;
    return (x / other, y / other, z / other);
}