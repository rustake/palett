use num::Num;

pub fn leverage<T: Num + Copy>(xyz: &(T, T, T), delta: T) -> (T, T, T) {
    let (x, y, z) = xyz;
    return (*x / delta, *y / delta, *z / delta);
}

#[cfg(test)]
mod leverage_tests {
    use crate::projector::utils::_leverage::leverage;

    #[test]
    fn test() {
        let xyz = (12.0, 24.0, 36.0);
        let delta = 3.0;
        let result = leverage(&xyz, delta);
        println!("{:?}", result);
    }
}