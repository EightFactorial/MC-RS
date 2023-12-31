use std::{collections::HashMap, hash::Hash};

use azalea_chat::FormattedText;
use azalea_nbt::Nbt;
use byteorder::{ReadBytesExt, BE};
use compact_str::CompactString;
use smallvec::SmallVec;
use uuid::Uuid;

use super::{Decode, DecodeError, VarDecode};

impl Decode for bool {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        match u8::decode(buf)? {
            0 => Ok(false),
            1 => Ok(true),
            n => Err(DecodeError::Boolean(n)),
        }
    }
}

impl Decode for i8 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i8().map_err(DecodeError::from)
    }
}

impl Decode for u8 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u8().map_err(DecodeError::from)
    }
}

impl Decode for i16 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i16::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for u16 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u16::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for i32 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i32::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for u32 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u32::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for i64 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i64::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for u64 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u64::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for i128 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_i128::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for u128 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_u128::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for usize {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        u64::decode(buf)?.try_into().map_err(DecodeError::from)
    }
}

impl Decode for isize {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        i64::decode(buf)?.try_into().map_err(DecodeError::from)
    }
}

impl Decode for f32 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_f32::<BE>().map_err(DecodeError::from)
    }
}

impl Decode for f64 {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        buf.read_f64::<BE>().map_err(DecodeError::from)
    }
}

const MAX_STRING_LENGTH: u32 = 131068;

impl Decode for String {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        if len > MAX_STRING_LENGTH {
            Err(DecodeError::StringTooLong(len))
        } else {
            let mut vec = vec![0; len as usize];
            buf.read_exact(&mut vec)?;

            Ok(String::from_utf8(vec)?)
        }
    }
}

impl Decode for CompactString {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        if len > MAX_STRING_LENGTH {
            Err(DecodeError::StringTooLong(len))
        } else {
            let mut vec = vec![0; len as usize];
            buf.read_exact(&mut vec)?;

            Ok(CompactString::from_utf8(vec)?)
        }
    }
}

impl Decode for Uuid {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(Uuid::from_u128(u128::decode(buf)?))
    }
}

impl<T: Decode> Decode for Vec<T> {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::decode(buf)?);
        }

        Ok(vec)
    }
}

impl<T: Decode, const N: usize> Decode for SmallVec<[T; N]> {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        let mut vec = SmallVec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::decode(buf)?);
        }

        Ok(vec)
    }
}

impl<T: Decode, const N: usize> Decode for [T; N] {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let mut arr = Vec::with_capacity(N);
        for _ in 0..N {
            arr.push(T::decode(buf)?);
        }

        arr.try_into()
            .map_err(|_| unreachable!("Length is constant"))
    }
}

impl<T: Decode> Decode for Option<T> {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        match bool::decode(buf)? {
            true => Ok(Some(T::decode(buf)?)),
            false => Ok(None),
        }
    }
}

impl<K: Decode + Eq + Hash, V: Decode> Decode for HashMap<K, V> {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        let mut map = HashMap::with_capacity(len as usize);
        for _ in 0..len {
            map.insert(K::decode(buf)?, V::decode(buf)?);
        }

        Ok(map)
    }
}

impl<K: Decode + Eq + Hash, V: Decode> Decode for hashbrown::HashMap<K, V> {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let len = u32::var_decode(buf)?;

        let mut map = hashbrown::HashMap::with_capacity(len as usize);
        for _ in 0..len {
            map.insert(K::decode(buf)?, V::decode(buf)?);
        }

        Ok(map)
    }
}

impl Decode for Nbt {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Nbt::read(buf).map_err(|_| DecodeError::NbtError)
    }
}

impl Decode for FormattedText {
    #[inline]
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(serde_json::from_str(&String::decode(buf)?)?)
    }
}
