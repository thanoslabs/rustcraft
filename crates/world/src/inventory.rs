//! Deterministic inventory rules for classic survival gameplay.

pub const HOTBAR_SIZE: usize = 9;
pub const PLAYER_INVENTORY_SIZE: usize = 36;
pub const DEFAULT_STACK_LIMIT: u8 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemStack {
    pub item: u16,
    pub count: u8,
}

impl ItemStack {
    pub fn new(item: u16, count: u8) -> Option<Self> {
        (item != 0 && (1..=DEFAULT_STACK_LIMIT).contains(&count))
            .then_some(Self { item, count })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory<const N: usize> {
    slots: [Option<ItemStack>; N],
}

impl<const N: usize> Default for Inventory<N> {
    fn default() -> Self {
        Self {
            slots: [None; N],
        }
    }
}

impl<const N: usize> Inventory<N> {
    pub fn slots(&self) -> &[Option<ItemStack>; N] {
        &self.slots
    }

    /// Insert items, filling matching stacks before empty slots.
    /// Returns the count that could not be inserted.
    pub fn insert(&mut self, item: u16, count: u32) -> u32 {
        if item == 0 {
            return count;
        }
        let mut remaining = count;
        for slot in &mut self.slots {
            if let Some(stack) = slot {
                if stack.item == item {
                    let room = u32::from(DEFAULT_STACK_LIMIT - stack.count);
                    let added = remaining.min(room);
                    stack.count += added as u8;
                    remaining -= added;
                }
            }
        }
        for slot in &mut self.slots {
            if remaining == 0 {
            break;
        }
            if slot.is_none() {
                let added = remaining.min(u32::from(DEFAULT_STACK_LIMIT));
                *slot = ItemStack::new(item, added as u8);
                remaining -= added;
            }
        }
        remaining
    }

    /// Remove up to count items; returns the number removed.
    pub fn remove(&mut self, item: u16, count: u32) -> u32 {
        let mut remaining = count;
        for slot in &mut self.slots {
            if remaining == 0 {
            break;
        }
            if let Some(stack) = slot {
                if stack.item == item {
                    let removed = remaining.min(u32::from(stack.count));
                    stack.count -= removed as u8;
                    remaining -= removed;
                    if stack.count == 0 {
                        *slot = None;
                    }
                }
            }
        }
        count - remaining
    }

    pub fn count(&self, item: u16) -> u32 {
        self.slots
            .iter()
            .flatten()
            .filter(|s| s.item == item)
            .map(|s| u32::from(s.count))
            .sum()
    }
}

pub type PlayerInventory = Inventory<PLAYER_INVENTORY_SIZE>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_stacks_before_occupying_slots() {
        let mut inventory = Inventory::<2>::default();
        assert_eq!(inventory.insert(3, 70), 0);
        assert_eq!(inventory.slots()[0].unwrap().count, 64);
        assert_eq!(inventory.slots()[1].unwrap().count, 6);
        assert_eq!(inventory.insert(3, 70), 12);
        assert_eq!(inventory.count(3), 128);
    }

    #[test]
    fn remove_clears_empty_slots() {
        let mut inventory = Inventory::<1>::default();
        inventory.insert(2, 5);
        assert_eq!(inventory.remove(2, 10), 5);
        assert_eq!(inventory.slots()[0], None);
    }

    #[test]
    fn invalid_items_are_rejected() {
        let mut inventory = Inventory::<1>::default();
        assert_eq!(inventory.insert(0, 5), 5);
        assert_eq!(ItemStack::new(1, 65), None);
    }
}
