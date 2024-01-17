use heed::{BoxedError, BytesDecode, BytesEncode};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct SerdePostcard<T>(PhantomData<T>);

impl<'a, T: 'a> BytesEncode<'a> for SerdePostcard<T>
where
    T: Serialize,
{
    type EItem = T;

    fn bytes_encode(item: &Self::EItem) -> Result<Cow<[u8]>, BoxedError> {
        postcard::to_allocvec(item)
            .map(Cow::Owned)
            .map_err(Into::into)
    }
}

impl<'a, T: 'a> BytesDecode<'a> for SerdePostcard<T>
where
    T: Deserialize<'a>,
{
    type DItem = T;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, BoxedError> {
        postcard::from_bytes(bytes).map_err(Into::into)
    }
}

unsafe impl<T> Send for SerdePostcard<T> {}

unsafe impl<T> Sync for SerdePostcard<T> {}
