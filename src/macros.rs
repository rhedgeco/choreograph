#[macro_export]
macro_rules! node_alias {
    ($pub:vis $alias:ident = async impl $concrete:path) => {
        $pub trait $alias: $crate::Node<Output = Self::Future> {
            type Future: ::core::future::Future<Output = Self::NodeOutput>;
            type NodeOutput: $concrete;
        }

        impl<O, F, T> $alias for T
        where
            O: $concrete,
            F: ::core::future::Future<Output = O>,
            T: $crate::Node<Output = F>,
        {
            type Future = F;
            type NodeOutput = O;
        }
    };
    ($pub:vis $alias:ident = async $concrete:path) => {
        $pub trait $alias: $crate::Node<Output = Self::Future> {
            type Future: ::core::future::Future<Output = $concrete>;
        }

        impl<F, T> $alias for T
        where
            F: ::core::future::Future<Output = $concrete>,
            T: $crate::Node<Output = F>,
        {
            type Future = F;
        }
    };
    ($pub:vis $alias:ident = impl $concrete:path) => {
        $pub trait $alias: $crate::Node<Output = Self::NodeOutput> {
            type NodeOutput: $concrete;
        }

        impl<NodeOutput: $concrete, T: $crate::Node<Output = NodeOutput>> $alias for T {
            type NodeOutput = NodeOutput;
        }
    };
    ($pub:vis $alias:ident = $concrete:path) => {
        $pub trait $alias: $crate::Node<Output = $concrete> {}
        impl<T: $crate::Node<Output = $concrete>> $alias for T {}
    };
}
