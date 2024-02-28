#[macro_export]
macro_rules! auth_layer {
    () => {
        $crate::auth_layer!(
            $crate::session_store::memory::MemoryStore::default(),
            $crate::authentication::auth_basic::AuthBasic {}
        )
    };

    (store: $store:expr) => {
        $crate::auth_layer!($store, $crate::authentication::auth_basic::AuthBasic {})
    };

    (auth: $auth:expr) => {
        $crate::auth_layer!($crate::session_store::memory::MemoryStore::default(), $auth)
    };

    ($store:expr, $auth:expr) => {
        $crate::layer::AuthLayer {
            store: $store,
            auth: $auth,
        }
    };
}

#[macro_export]
macro_rules! auth_service {
    ($inner:expr, $auth:expr, $store:expr) => {
        $crate::session::Session {
            inner: $inner,
            auth: $auth,
            store: $store,
        }
    };
}

/// This is a private macro not to be used outside this crate
#[macro_export]
macro_rules! auth_service_clone {
    ($inner:expr, $auth:expr, $store:expr) => {
        $crate::auth_service!($inner, $auth.clone(), $store.clone())
    };
}
