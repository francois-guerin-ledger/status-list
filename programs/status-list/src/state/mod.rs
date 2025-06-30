use anchor_lang::prelude::*;

use crate::{errors::StatusListError, types::ListPurpose};

#[account]
#[derive(InitSpace, Debug, Eq, PartialEq)]
pub struct StatusList {
    #[max_len(512)]
    pub list: Vec<u8>,
    pub size: u16,
    pub purpose: ListPurpose,
}

impl StatusList {
    pub const MAX_SIZE: u16 = 512;

    pub fn new(size: u16, purpose: ListPurpose) -> Result<Self> {
        require!(size <= StatusList::MAX_SIZE, StatusListError::SizeTooLarge);

        Ok(Self {
            list: vec![0u8; size as usize],
            size,
            purpose,
        })
    }

    fn require_in_bounds(&self, location: u16) -> Result<()> {
        require!(location < self.size << 3, StatusListError::OutOfBounds);

        Ok(())
    }

    pub fn get(&self, location: u16) -> Result<bool> {
        self.require_in_bounds(location)?;

        let position = location >> 3;
        let flag = 1u8 << (location & 0b111);

        Ok(self.list[position as usize] & flag != 0)
    }

    fn set(&mut self, location: u16, value: bool) {
        let position = location >> 3;
        let flag = 1u8 << (location & 0b111);
        let v = self.list[position as usize];

        self.list[position as usize] = if value { v | flag } else { v & !flag };
    }

    pub fn toggle(&mut self, location: u16) -> Result<()> {
        let value = self.get(location)?;

        require!(
            !(self.purpose == ListPurpose::Revocation && value),
            StatusListError::StatusNotReversible
        );

        self.set(location, !value);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_new_size_too_large() {
        assert_eq!(
            StatusList::new(600, ListPurpose::Revocation),
            Err(error!(StatusListError::SizeTooLarge))
        );
    }

    #[test]
    fn list_new() {
        assert_eq!(
            StatusList::new(6, ListPurpose::Revocation),
            Ok(StatusList {
                list: vec![0u8; 6],
                size: 6,
                purpose: ListPurpose::Revocation
            })
        );
    }

    #[test]
    fn list_get_out_of_bounds() {
        let list = StatusList {
            list: vec![0b00001000, 0b00010000],
            size: 2,
            purpose: ListPurpose::Revocation,
        };

        assert_eq!(list.get(16), Err(error!(StatusListError::OutOfBounds)));
        assert_eq!(list.get(50), Err(error!(StatusListError::OutOfBounds)));
    }

    #[test]
    fn list_get() {
        let list = StatusList {
            list: vec![0b00001000, 0b00010000],
            size: 2,
            purpose: ListPurpose::Revocation,
        };

        assert_eq!(list.get(3), Ok(true));
        assert_eq!(list.get(12), Ok(true));
        assert_eq!(list.get(4), Ok(false));
        assert_eq!(list.get(13), Ok(false));
    }

    #[test]
    fn list_toggle_out_of_bounds() {
        let mut list = StatusList {
            list: vec![0b00001000, 0b00010000],
            size: 2,
            purpose: ListPurpose::Revocation,
        };

        assert_eq!(list.toggle(16), Err(error!(StatusListError::OutOfBounds)));
        assert_eq!(list.toggle(50), Err(error!(StatusListError::OutOfBounds)));
    }

    #[test]
    fn list_toggle() {
        let mut list = StatusList {
            list: vec![0b00001000, 0b00010000],
            size: 2,
            purpose: ListPurpose::Revocation,
        };

        list.toggle(8).unwrap();
        assert_eq!(list.list, vec![0b00001000, 0b00010001]);

        list.toggle(15).unwrap();
        assert_eq!(list.list, vec![0b00001000, 0b10010001]);

        list.toggle(3).unwrap();
        assert_eq!(list.list, vec![0b00000000, 0b10010001]);

        list.toggle(8).unwrap();
        assert_eq!(list.list, vec![0b00000000, 0b10010000]);
    }
}
