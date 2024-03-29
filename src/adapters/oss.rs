use std::{
	collections::HashMap,
	path::{Path, PathBuf},
	str::FromStr,
	time::SystemTime,
};

use mime::Mime;
use oss_sdk_rs::{object::ObjectAPI, oss::{ObjectMeta, OSS}};

use crate::{contents::Contents, Visibility};

use super::Adapter;

#[derive(Debug, Clone)]
pub struct Config {
	pub bucket: String,
	pub endpoint: String,
	pub access_key: String,
	pub secret_key: String,
}

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct OssAdapter<'a> {
	client: OSS<'a>,
}

impl<'a> OssAdapter<'a> {
    async fn head_object(&self, path: &Path) -> Result<ObjectMeta,Error> {
        let meta = self.client.head_object(path.to_str().ok_or(Error::InvalidPath)?).await?;
        Ok(meta)
    }
}



impl<'a> Adapter for OssAdapter<'a> {
	type Error = Error;
	type Config = Config;

	async fn new(config: Self::Config) -> Result<Self, Self::Error> {
		let Config {
			bucket,
			endpoint,
			access_key,
			secret_key,
		} = config;

		let oss = OSS::new(access_key, secret_key, endpoint, bucket);
		Ok(Self { client: oss })
	}
	async fn file_exists(&self, path: &Path) -> Result<bool, Self::Error> {
		let res = self
			.client
			.get_object(
				path.to_str().ok_or(Error::InvalidPath)?,
				None::<HashMap<&str, &str>>,
				None,
			)
			.await;
		if res.is_err() {
			Ok(false)
		} else {
			Ok(true)
		}
	}
	async fn directory_exists(&self, path: &Path) -> Result<bool, Self::Error> {
		todo!()
	}
	async fn write<C: AsRef<[u8]> + Send>(
		&mut self,
		path: &Path,
		content: C,
	) -> Result<(), Self::Error> {
		let mut headers = HashMap::new();
		headers.insert("content-type", "text/plain");

		let mut resources: HashMap<&str, Option<&str>> = HashMap::new();
		resources.insert("acl", None);
		resources.insert("response-content-type", Some("ContentType"));

		self.client
			.put_object(
				content.as_ref(),
				path.to_str().ok_or(Error::InvalidPath)?,
				headers,
				resources,
			)
			.await?;
		Ok(())
	}
	async fn read<T: TryFrom<Contents>>(&self, path: &Path) -> Result<T, Self::Error> {
		let res = self
			.client
			.get_object(
				path.to_str().ok_or(Error::InvalidPath)?,
				None::<HashMap<&str, &str>>,
				None,
			)
			.await?;
		Contents::from(res)
			.try_into()
			.map_err(|_| Error::DecodeError)
	}
	async fn delete_directory(&mut self, path: &Path) -> Result<(), Self::Error> {
		todo!()

	}
	async fn create_directory(&mut self, path: &Path) -> Result<(), Self::Error> {
		todo!()
	}
	async fn set_visibility(
		&mut self,
		path: &Path,
		visibility: Visibility,
	) -> Result<(), Self::Error> {
		todo!()
	}
	async fn visibility(&self, path: &Path) -> Result<Visibility, Self::Error> {
		todo!()
	}
	async fn mime_type(&self, path: &Path) -> Result<Mime, Self::Error> {
        let meta = self.head_object(path).await?;
        
		todo!()
	}
	async fn last_modified(&self, path: &Path) -> Result<SystemTime, Self::Error> {
		let meta = self.head_object(path).await?;
        Ok(meta.last_modified)
	}
	async fn file_size(&self, path: &Path) -> Result<u64, Self::Error> {
		todo!()
	}
	async fn delete(&mut self, path: &Path) -> Result<(), Self::Error> {
		self.client
			.delete_object(path.to_str().ok_or(Error::InvalidPath)?)
			.await?;
		Ok(())
	}
	async fn list_contents(
		&self,
		path: &Path,
		deep: bool,
	) -> Result<Vec<std::path::PathBuf>, Self::Error> {
		let res = self
			.client
			.list_object_v2(Some(path.to_str().ok_or(Error::InvalidPath)?), None)
			.await?;
		let p = res
			.contents
            .unwrap()
			// .unwrap_or(|| vec![])
			.iter()
			.map(|x| PathBuf::from_str(x.key.as_str()).unwrap())
			.collect();

		Ok(p)
	}
	async fn r#move(&mut self, source: &Path, destination: &Path) -> Result<(), Self::Error> {
		todo!()
	}
	async fn copy(&mut self, source: &Path, destination: &Path) -> Result<(), Self::Error> {
		todo!()
	}
	async fn checksum(&self, path: &Path) -> Result<String, Self::Error> {
		todo!()
	}
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("The provided path contains invalid characters")]
	InvalidPath,

	#[error("OSS error")]
	OssError(#[from] oss_sdk_rs::errors::OSSError),

	#[error("Failed to decode response contents")]
	DecodeError,

	#[error("Failed to convert path")]
	PathError,
}
