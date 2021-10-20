/// EchoRequest is the request for echo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMove {
    #[prost(enumeration = "player_move::Direction", tag = "1")]
    pub direction: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PlayerMove`.
pub mod player_move {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        Top = 0,
        Left = 1,
        Bottom = 2,
        Right = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(int32, tag = "1")]
    pub x: i32,
    #[prost(int32, tag = "2")]
    pub y: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerState {
    #[prost(message, repeated, tag = "1")]
    pub line_stripe: ::prost::alloc::vec::Vec<Point>,
}
/// EchoResponse is the response for echo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Login {
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendResult {}
#[doc = r" Generated client implementations."]
pub mod snake_server_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Echo is the echo service."]
    #[derive(Debug, Clone)]
    pub struct SnakeServerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SnakeServerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SnakeServerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SnakeServerClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SnakeServerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn make_move(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayerMove>,
        ) -> Result<tonic::Response<super::SendResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/snake.SnakeServer/MakeMove");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn observe_game_state(
            &mut self,
            request: impl tonic::IntoRequest<super::Login>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::PlayerState>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/snake.SnakeServer/ObserveGameState");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " UnaryEcho is unary echo."]
        pub async fn receive_message(
            &mut self,
            request: impl tonic::IntoRequest<super::Login>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ChatMessage>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/snake.SnakeServer/ReceiveMessage");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn send_message(
            &mut self,
            request: impl tonic::IntoRequest<super::ChatMessage>,
        ) -> Result<tonic::Response<super::SendResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/snake.SnakeServer/SendMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod snake_server_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SnakeServerServer."]
    #[async_trait]
    pub trait SnakeServer: Send + Sync + 'static {
        async fn make_move(
            &self,
            request: tonic::Request<super::PlayerMove>,
        ) -> Result<tonic::Response<super::SendResult>, tonic::Status>;
        #[doc = "Server streaming response type for the ObserveGameState method."]
        type ObserveGameStateStream: futures_core::Stream<Item = Result<super::PlayerState, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn observe_game_state(
            &self,
            request: tonic::Request<super::Login>,
        ) -> Result<tonic::Response<Self::ObserveGameStateStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ReceiveMessage method."]
        type ReceiveMessageStream: futures_core::Stream<Item = Result<super::ChatMessage, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " UnaryEcho is unary echo."]
        async fn receive_message(
            &self,
            request: tonic::Request<super::Login>,
        ) -> Result<tonic::Response<Self::ReceiveMessageStream>, tonic::Status>;
        async fn send_message(
            &self,
            request: tonic::Request<super::ChatMessage>,
        ) -> Result<tonic::Response<super::SendResult>, tonic::Status>;
    }
    #[doc = " Echo is the echo service."]
    #[derive(Debug)]
    pub struct SnakeServerServer<T: SnakeServer> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SnakeServer> SnakeServerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SnakeServerServer<T>
    where
        T: SnakeServer,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/snake.SnakeServer/MakeMove" => {
                    #[allow(non_camel_case_types)]
                    struct MakeMoveSvc<T: SnakeServer>(pub Arc<T>);
                    impl<T: SnakeServer> tonic::server::UnaryService<super::PlayerMove> for MakeMoveSvc<T> {
                        type Response = super::SendResult;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlayerMove>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).make_move(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MakeMoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/snake.SnakeServer/ObserveGameState" => {
                    #[allow(non_camel_case_types)]
                    struct ObserveGameStateSvc<T: SnakeServer>(pub Arc<T>);
                    impl<T: SnakeServer> tonic::server::ServerStreamingService<super::Login>
                        for ObserveGameStateSvc<T>
                    {
                        type Response = super::PlayerState;
                        type ResponseStream = T::ObserveGameStateStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Login>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).observe_game_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ObserveGameStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/snake.SnakeServer/ReceiveMessage" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveMessageSvc<T: SnakeServer>(pub Arc<T>);
                    impl<T: SnakeServer> tonic::server::ServerStreamingService<super::Login> for ReceiveMessageSvc<T> {
                        type Response = super::ChatMessage;
                        type ResponseStream = T::ReceiveMessageStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Login>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).receive_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReceiveMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/snake.SnakeServer/SendMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageSvc<T: SnakeServer>(pub Arc<T>);
                    impl<T: SnakeServer> tonic::server::UnaryService<super::ChatMessage> for SendMessageSvc<T> {
                        type Response = super::SendResult;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChatMessage>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: SnakeServer> Clone for SnakeServerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SnakeServer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SnakeServer> tonic::transport::NamedService for SnakeServerServer<T> {
        const NAME: &'static str = "snake.SnakeServer";
    }
}
