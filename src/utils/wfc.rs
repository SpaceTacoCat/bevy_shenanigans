//! Wave function collapse

use std::collections::HashMap;
use std::num::NonZeroU64;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rules {
    chances: HashMap<NonZeroU64, HashMap<NonZeroU64, u64>>,
}

pub struct Matrix<D, const T: usize>
where
    [(); T * T]:,
{
    buffer: Box<[D; T * T]>,
}

pub type Image<const T: usize> = Matrix<NonZeroU64, T>;
pub type Data<const T: usize> = Matrix<Option<NonZeroU64>, T>;

impl Rules {
    pub fn from_data<const T: usize>(data: Image<T>) -> Rules
    where
        [(); T * T]:,
    {
        let mut rules = Rules::default();

        for x in 0..T {
            for y in 0..T {
                let current = data.get_unchecked(x, y);
                let neighbors = rules.chances.entry(*current).or_default();

                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        if let Some(neighbor) = data.get_isize((x as isize) + dx, (y as isize) + dy)
                        {
                            let probability = neighbors.entry(*neighbor).or_default();
                            *probability += 1;
                        }
                    }
                }
            }
        }

        rules
    }

    pub fn collapse<const T: usize>(&self, _: &Data<T>) -> Image<T>
    where
        [(); T * T]:,
    {
        todo!()
    }
}

impl<D, const T: usize> Matrix<D, T>
where
    [(); T * T]:,
{
    pub fn new(buffer: Box<[D; T * T]>) -> Self {
        Self { buffer }
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> &D {
        &self.buffer[x + y * T]
    }

    pub fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut D {
        &mut self.buffer[x + y * T]
    }

    pub fn get_isize(&self, x: isize, y: isize) -> Option<&D> {
        if x >= (T as isize) || x < 0 || y < 0 || y >= (T as isize) {
            None
        } else {
            Some(&self.buffer[(x + y * (T as isize)) as usize])
        }
    }

    // pub fn get_mut(&self, x: usize, y: usize) -> Option<&mut D> {
    //     if x > T || y > T {
    //         None
    //     } else {
    //         Some(&mut self.buffer[x + y * T])
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::collection;
    use crate::utils::wfc::{Image, Rules};
    use std::num::NonZeroU64;

    macro_rules! num {
        ($number:expr) => {
            unsafe { NonZeroU64::new_unchecked($number) }
        };
    }

    fn text_to_data<const T: usize>(text: &[u8]) -> Image<T>
    where
        [(); T * T]:,
    {
        let mut idx = 0;
        let mut image = Image::new(Box::new([num!(1); T * T]));

        for x in 0..T {
            for y in 0..T {
                while text[idx] == b' ' || text[idx] == b'\n' {
                    idx += 1;
                }

                *image.get_unchecked_mut(x, y) = num!((text[idx] - b'0').into());

                idx += 1;
            }
        }

        eprintln!("{:?}", image.buffer);

        image
    }

    #[test]
    fn produces_expected_ruleset_for_only_ones() {
        let rules = Rules::from_data(Image::<2>::new(Box::new([num!(1); 4])));
        assert_eq!(
            rules,
            Rules {
                chances: collection!( num!(1) => collection!( num!(1) => 12 ) )
            }
        )
    }

    #[test]
    fn produces_expected_ruleset_for_checkerboard() {
        const TEXT_REPR: &[u8] = b"1 2 1 2\
                                   2 1 2 1\
                                   1 2 1 2\
                                   2 1 2 1";

        let rules = Rules::from_data(text_to_data::<4>(TEXT_REPR));
        assert_eq!(
            rules,
            Rules {
                chances: collection!( num!(1) => collection!( num!(2) => 24, num!(1) => 18 ), num!(2) => collection!( num!(1) => 24, num!(2) => 18 ) )
            }
        )
    }
}
