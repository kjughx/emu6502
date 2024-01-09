use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Not,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::hardware::cpu::Flag;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Byte(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct Addr(pub u16);
#[derive(PartialEq, PartialOrd)]
pub struct Bit(pub bool);

impl core::fmt::Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as u8)
    }
}

impl Sub<u8> for Addr {
    type Output = Addr;
    fn sub(self, rhs: u8) -> Self::Output {
        Self(self.0 - rhs as u16)
    }
}

impl Not for Flag {
    type Output = u8;
    fn not(self) -> Self::Output {
        !(1 << (self as u8))
    }
}

impl Shl<Flag> for Bit {
    type Output = Byte;
    fn shl(self, rhs: Flag) -> Self::Output {
        Byte((self.0 as u8) << (rhs as u8))
    }
}

impl BitAnd<Byte> for Byte {
    type Output = Self;
    fn bitand(self, rhs: Byte) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOrAssign<Byte> for Byte {
    fn bitor_assign(&mut self, rhs: Byte) {
        *self = Self(self.0 | rhs.0)
    }
}

impl BitXorAssign<Byte> for Byte {
    fn bitxor_assign(&mut self, rhs: Byte) {
        *self = Self(self.0 ^ rhs.0)
    }
}

impl BitAndAssign<Byte> for Byte {
    fn bitand_assign(&mut self, rhs: Byte) {
        *self = Self(self.0 & rhs.0)
    }
}

impl From<Byte> for Bit {
    fn from(value: Byte) -> Self {
        Bit(value != 0)
    }
}

impl BitOrAssign<Bit> for Byte {
    fn bitor_assign(&mut self, rhs: Bit) {
        *self = Byte(self.0 | rhs.0 as u8)
    }
}

impl BitXor<Bit> for Bit {
    type Output = Bit;
    fn bitxor(self, rhs: Bit) -> Self::Output {
        Bit(self.0 ^ rhs.0)
    }
}
impl BitAnd<Bit> for Bit {
    type Output = Bit;
    fn bitand(self, rhs: Bit) -> Self::Output {
        Bit(self.0 == rhs.0)
    }
}
impl Add<Bit> for Byte {
    type Output = Byte;
    fn add(self, rhs: Bit) -> Self::Output {
        Byte(self.0 + rhs.0 as u8)
    }
}
impl Add<Byte> for Bit {
    type Output = Byte;
    fn add(self, rhs: Byte) -> Self::Output {
        Byte(self.0 as u8 + rhs.0)
    }
}
impl BitAnd<Flag> for Byte {
    type Output = Bit;
    fn bitand(self, rhs: Flag) -> Self::Output {
        println!("{:#08b} & {:#08b}", self.0, 1 << rhs as u8);
        println!("");
        Bit((self.0 & ((1 << rhs as u8))) != 0)
    }
}

impl Add<Byte> for Byte {
    type Output = Byte;
    fn add(self, rhs: Byte) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<u8> for Addr {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Self(self.0 + rhs as u16)
    }
}

impl From<i32> for Addr {
    fn from(value: i32) -> Self {
        Self(value as u16)
    }
}

impl From<u8> for Addr {
    fn from(value: u8) -> Self {
        Self(value as u16)
    }
}
impl PartialEq<Addr> for Byte {
    fn eq(&self, other: &Addr) -> bool {
        self.0 == (other.0 & 0xff) as u8
    }
}
impl PartialEq<u8> for Byte {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl From<Addr> for usize {
    fn from(val: Addr) -> Self {
        val.0 as usize
    }
}

impl Add<Byte> for Addr {
    type Output = Self;
    fn add(self, rhs: Byte) -> Self::Output {
        Addr(self.0 + rhs.0 as u16)
    }
}

impl AddAssign<u16> for Addr {
    fn add_assign(&mut self, rhs: u16) {
        *self = Addr(self.0 + rhs)
    }
}

impl Deref for Addr {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BitOr<Byte> for Addr {
    type Output = Addr;
    fn bitor(self, rhs: Byte) -> Self::Output {
        Self(self.0 | (rhs.0 as u16))
    }
}

impl Shl<u8> for Addr {
    type Output = Addr;
    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u8> for Addr {
    type Output = Addr;
    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl BitAnd<u16> for Addr {
    type Output = Addr;
    fn bitand(self, rhs: u16) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl From<u16> for Addr {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Byte> for Addr {
    fn from(value: Byte) -> Self {
        Self(value.0 as u16)
    }
}

impl BitAndAssign<u8> for Byte {
    fn bitand_assign(&mut self, rhs: u8) {
        *self = Byte(self.0 & rhs)
    }
}

impl BitXor<u8> for Byte {
    type Output = Self;
    fn bitxor(self, rhs: u8) -> Self::Output {
        Byte(self.0 ^ rhs)
    }
}

impl AddAssign<u8> for Byte {
    fn add_assign(&mut self, rhs: u8) {
        *self = Byte(self.0 + rhs)
    }
}
impl SubAssign<u8> for Byte {
    fn sub_assign(&mut self, rhs: u8) {
        *self = Byte(self.0 - rhs)
    }
}

impl Shr<u8> for Byte {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shl<u8> for Byte {
    type Output = Byte;
    fn shl(self, rhs: u8) -> Self::Output {
        Byte(self.0 << rhs)
    }
}

impl ShrAssign<u8> for Byte {
    fn shr_assign(&mut self, rhs: u8) {
        *self = Byte(self.0 >> rhs);
    }
}

impl ShlAssign<u8> for Byte {
    fn shl_assign(&mut self, rhs: u8) {
        *self = Byte(self.0 << rhs);
    }
}

impl Sub<u8> for Byte {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        Byte(self.0 - rhs)
    }
}

impl Add<u8> for Byte {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Byte(self.0 + rhs)
    }
}

impl Not for Byte {
    type Output = Self;
    fn not(self) -> Self::Output {
        Byte(!self.0)
    }
}

impl BitOr<u8> for Byte {
    type Output = u8;
    fn bitor(self, rhs: u8) -> Self::Output {
        self.0 | rhs
    }
}

impl BitAnd<u8> for Byte {
    type Output = u8;
    fn bitand(self, rhs: u8) -> Self::Output {
        self.0 & rhs
    }
}
impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<Addr> for Byte {
    fn from(value: Addr) -> Self {
        Self((value.0 & 0xff) as u8)
    }
}

impl Deref for Byte {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
