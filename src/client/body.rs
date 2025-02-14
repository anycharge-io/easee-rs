pub(crate) trait Body {
    fn apply_to(self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}

pub(crate) struct JsonBody<'a, T>(pub &'a T);
pub(crate) struct NoBody;

impl<T> Body for JsonBody<'_, T>
where
    T: serde::Serialize,
{
    fn apply_to(self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder.json(self.0)
    }
}

impl Body for NoBody {
    fn apply_to(self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder
    }
}
