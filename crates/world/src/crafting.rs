//! Grid-based shaped crafting with translation-independent recipe matching.
use crate::inventory::ItemStack;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Recipe {
    pub width: usize,
    pub height: usize,
    pub pattern: &'static [u16],
    pub output: ItemStack,
}

impl Recipe {
    pub fn matches(&self, grid: &[u16], grid_width: usize, grid_height: usize) -> bool {
        if self.width == 0
            || self.height == 0
            || self.width > grid_width
            || self.height > grid_height
        {
            return false;
        }
        if self.pattern.len() != self.width * self.height || grid.len() != grid_width * grid_height
        {
            return false;
        }
        for offset_y in 0..=grid_height - self.height {
            for offset_x in 0..=grid_width - self.width {
                let mut valid = true;
                for y in 0..grid_height {
                    for x in 0..grid_width {
                        let expected = if x >= offset_x
                            && x < offset_x + self.width
                            && y >= offset_y
                            && y < offset_y + self.height
                        {
                            self.pattern[(y - offset_y) * self.width + x - offset_x]
                        } else {
                            0
                        };
                        if grid[y * grid_width + x] != expected {
                            valid = false;
                        }
                    }
                }
                if valid {
                    return true;
                }
            }
        }
        false
    }
}

/// IDs use the initial block registry; dedicated item IDs follow later.
pub const PLANKS_FROM_LOG: Recipe = Recipe {
    width: 1,
    height: 1,
    pattern: &[10],
    output: ItemStack { item: 5, count: 4 },
};

pub const CRAFTING_TABLE_FROM_PLANKS: Recipe = Recipe {
    width: 2,
    height: 2,
    pattern: &[5, 5, 5, 5],
    output: ItemStack { item: 14, count: 1 },
};

pub const FURNACE_FROM_COBBLESTONE: Recipe = Recipe {
    width: 3,
    height: 3,
    pattern: &[4, 4, 4, 4, 0, 4, 4, 4, 4],
    output: ItemStack { item: 15, count: 1 },
};

pub const STARTER_RECIPES: &[Recipe] = &[
    PLANKS_FROM_LOG,
    CRAFTING_TABLE_FROM_PLANKS,
    FURNACE_FROM_COBBLESTONE,
];

pub fn find_recipe(grid: &[u16], width: usize, height: usize) -> Option<ItemStack> {
    STARTER_RECIPES
        .iter()
        .find(|recipe| recipe.matches(grid, width, height))
        .map(|recipe| recipe.output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_translated_recipe() {
        assert_eq!(
            find_recipe(&[0, 0, 0, 0, 10, 0, 0, 0, 0], 3, 3),
            Some(ItemStack { item: 5, count: 4 })
        );
    }

    #[test]
    fn rejects_extra_ingredients() {
        assert_eq!(find_recipe(&[10, 5, 0, 0], 2, 2), None);
    }

    #[test]
    fn furnace_recipe_requires_center_gap() {
        assert_eq!(
            find_recipe(&[4, 4, 4, 4, 0, 4, 4, 4, 4], 3, 3),
            Some(ItemStack { item: 15, count: 1 })
        );
        assert_eq!(find_recipe(&[4; 9], 3, 3), None);
    }
}
