use aws_sdk_dynamodb::config;
use aws_sdk_s3::{config::Region, meta::PKG_VERSION, primitives::ByteStream, types::Bucket, Client, Error};
use bytes::Bytes;

pub struct S3Client{
    client: Client,
    bucket_name: String,
}


impl S3Client {
    pub async fn new(credintials_s3: aws_credential_types::Credentials, end_point:String, backet_name: String) -> Self{
        let config_s3 = aws_sdk_s3::config::Builder::new()
        .credentials_provider(credintials_s3)
        .region(Region::new("ru-central1"))
        .endpoint_url(end_point)
        .build();
        let client =  Client::from_conf(config_s3);
        Self {
            client: client,
            bucket_name: backet_name,
        }
    }


    pub async fn upload_file(&self, file_to_upload: Bytes, key: String) -> Result<String, i32> {
        let bytes_stream = ByteStream::from(file_to_upload);
        let put_object = self.client
        .put_object()
        .bucket(& self.bucket_name)
        .key(key)
        .body(bytes_stream)
        .send()
        .await;
        match put_object {
            Ok(_) => {
                Ok(String::from("sf"))
            },
            Err(err) => {
                Err(-1)
            }
        }
    }
}



