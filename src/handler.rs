
#[async_trait::async_trait]
pub trait Handler<D: Send + Sync + 'static> {
    async fn call(&self, req: super::request::Request<D>) -> super::response::Response;
    fn name(&self) -> String;
}
