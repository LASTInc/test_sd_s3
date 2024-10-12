use std::os::raw::c_char;
use std::ffi::{CStr, CString};


use std::{backtrace, path::Path};

use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_s3::operation::put_object::PutObjectOutput;
//use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_s3::{config::Region, meta::PKG_VERSION, primitives::ByteStream, types::Bucket, Client, Error};
use aws_credential_types;
use aws_config;

/// Lists your DynamoDB tables in the default Region or us-east-1 if a default Region isn't set.

pub async fn test_s3() -> Client {
    println!("Start");
    let my_credential = aws_credential_types::Credentials::new("", "", None, None, "ru-central1");
    

    let config = aws_sdk_s3::config::Builder::new()
    .credentials_provider(my_credential)
    .region(Region::new("ru-central1"))
    .endpoint_url("https://storage.yandexcloud.net")
    .build();
    let client = Client::from_conf(config);
    println!("Start");
    client
}


pub async fn get_bucket(client: &Client) -> Bucket
{
    let resp = client.list_buckets().send().await.unwrap();
    let buckets = resp.buckets();
    let my_bucket = buckets[0].clone();
    my_bucket
}

pub async fn upload_image(client: &Client, my_path: &String, key: &String) -> PutObjectOutput
{
    let file = ByteStream::from_path(Path::new(my_path)).await.unwrap();
    let resp = client
    .put_object()
    .bucket("ispu-test-project-1")
    .key(key)
    .body(file)
    .send()
    .await.unwrap();
    resp
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn uploadFile(path: *mut c_char, key: * mut c_char)
{
    unsafe 
    {
        if path.is_null() 
        {
            return ;
        }
        let my_path = CString::from_raw(path);
        let my_path = my_path.to_str().unwrap().to_string();
        let my_key = CString::from_raw(key);
        let my_key = my_key.to_str().unwrap().to_string();

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        //let mut my_ref: String = String::new();
        rt.block_on(async {
            let my_client = test_s3().await;
            let my_bucket = get_bucket(&my_client).await;
            let put_object_output = upload_image(&my_client, &my_path, &my_key).await;
            
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
