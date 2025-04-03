use bytes::Bytes;

pub(crate) trait RequestBody {
    fn apply_to(self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}

pub(crate) trait ResponseBody {
    type Data;

    fn from_body(bs: Bytes) -> crate::Result<Self::Data>;
}

pub(crate) struct JsonBody<T>(pub T);
pub(crate) struct NoBody;
pub(crate) struct BytesBody;

impl<T> RequestBody for JsonBody<T>
where
    T: serde::Serialize,
{
    fn apply_to(self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder.json(&self.0)
    }
}

impl RequestBody for NoBody {
    fn apply_to(self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder
    }
}

impl ResponseBody for BytesBody {
    type Data = Bytes;

    fn from_body(bs: Bytes) -> crate::Result<Self::Data> {
        Ok(bs)
    }
}

impl<T> ResponseBody for JsonBody<T>
where
    T: serde::de::DeserializeOwned,
{
    type Data = T;

    fn from_body(bs: Bytes) -> crate::Result<T> {
        match serde_json::from_slice::<T>(&bs) {
            Ok(res) => Ok(res),

            Err(err) => {
                let body = String::from_utf8_lossy(&bs).into_owned();
                Err(crate::Error::DeserializingJson { err, body })
            }
        }
    }
}
