use tower_layer::Layer;
#[allow(dead_code)]
#[derive(Clone)]
pub struct AuthLayer<A, S> {
    pub auth: A,
    pub store: S,
}

impl<L, A: Clone, S: Clone> Layer<L> for AuthLayer<A, S> {
    type Service = crate::session::Session<L, A, S>;

    fn layer(&self, inner: L) -> Self::Service {
        crate::auth_service_clone!(inner, self.auth, self.store)
    }
}
