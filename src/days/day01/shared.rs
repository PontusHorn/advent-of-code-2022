use crate::input::Lines;

pub struct ElfCalories(pub Lines);
impl Iterator for ElfCalories {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut current = 0;
        let Self(lines) = self;
        for line in lines {
            let food_item = line.ok()?;
            let res = food_item.parse::<Self::Item>();
            match res {
                Ok(calories) => current += calories,
                Err(_) => return Some(current),
            }
        }
        None
    }
}
