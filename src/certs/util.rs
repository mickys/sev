// Copyright 2019 Red Hat
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use std::mem::{size_of, uninitialized};
use std::slice::{from_raw_parts, from_raw_parts_mut};

#[cfg(feature = "openssl")]
pub trait FromLe: Sized {
    fn from_le(value: &[u8]) -> Result<Self>;
}

#[cfg(feature = "openssl")]
pub trait IntoLe<T> {
    fn into_le(&self) -> T;
}

#[cfg(feature = "openssl")]
impl FromLe for openssl::bn::BigNum {
    #[inline]
    fn from_le(value: &[u8]) -> Result<Self> {
        Ok(Self::from_slice(
            &value.iter().rev().cloned().collect::<Vec<_>>(),
        )?)
    }
}

#[cfg(feature = "openssl")]
impl IntoLe<[u8; 72]> for openssl::bn::BigNumRef {
    fn into_le(&self) -> [u8; 72] {
        let mut buf = [0u8; 72];

        for (i, b) in self.to_vec().iter().rev().cloned().enumerate() {
            buf[i] = b;
        }

        buf
    }
}

#[cfg(feature = "openssl")]
impl IntoLe<[u8; 512]> for openssl::bn::BigNumRef {
    fn into_le(&self) -> [u8; 512] {
        let mut buf = [0u8; 512];

        for (i, b) in self.to_vec().iter().rev().cloned().enumerate() {
            buf[i] = b;
        }

        buf
    }
}

pub trait TypeLoad: Read {
    fn load<T: Sized + Copy>(&mut self) -> Result<T> {
        let mut t = unsafe { uninitialized() };
        let p = &mut t as *mut T as *mut u8;
        let s = unsafe { from_raw_parts_mut(p, size_of::<T>()) };
        self.read_exact(s)?;
        Ok(t)
    }
}

pub trait TypeSave: Write {
    fn save<T: Sized + Copy>(&mut self, value: &T) -> Result<()> {
        let p = value as *const T as *const u8;
        let s = unsafe { from_raw_parts(p, size_of::<T>()) };
        self.write_all(s)
    }
}

impl<T: Read> TypeLoad for T {}
impl<T: Write> TypeSave for T {}
