use std::io::Read;

#[cfg(feature = "oss")]
use bytes::Bytes;

pub struct Contents {
	data: Vec<u8>,
}

impl Contents {
	#[cfg(feature = "s3")]
	pub(crate) async fn from_bytestream(
		bytes: aws_smithy_types::byte_stream::ByteStream,
	) -> Result<Self, aws_smithy_types::byte_stream::error::Error> {
		Ok(Self {
			data: bytes.collect().await?.to_vec(),
		})
	}
}

// #[cfg(feature = "oss")]
// impl From<Contents> for Bytes {
// 	fn from(value: Contents) -> Self {
// 		Bytes::from_static(value.data.bytes())
// 	}
// }
#[cfg(feature = "oss")]
impl From<Bytes> for Contents {
	fn from(value: Bytes) -> Self {
		Self {
			data: value.to_vec(),
		}
	}
}


impl From<Contents> for Vec<u8> {
	fn from(contents: Contents) -> Self {
		contents.data
	}
}

impl From<Vec<u8>> for Contents {
	fn from(data: Vec<u8>) -> Self {
		Self { data }
	}
}

impl TryFrom<Contents> for String {
	type Error = std::string::FromUtf8Error;

	fn try_from(contents: Contents) -> Result<Self, Self::Error> {
		Self::from_utf8(contents.data)
	}
}
