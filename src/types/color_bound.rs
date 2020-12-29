use num::Num;

pub struct ColorBound<T> {
    pub max: T,
    pub sum: T,
    pub dif: T,
}

impl<T: Num + PartialOrd + Copy> ColorBound<T> {
    pub fn from(rgb: &(T, T, T)) -> Self {
        let (r, g, b) = *rgb;
        let mut va = r;
        let mut vi = r;
        if g > r { va = g } else { vi = g }
        if b > va { va = b }
        if b < vi { vi = b }
        return Self { max: va, sum: va + vi, dif: va - vi };
    }
}