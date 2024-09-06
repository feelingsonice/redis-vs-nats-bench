use bytes::{BufMut, Bytes, BytesMut};
use envconfig::Envconfig;
use futures::Stream;
use redis::{
    aio::{ConnectionManager, ConnectionManagerConfig},
    ConnectionAddr, ConnectionInfo, IntoConnectionInfo, ProtocolVersion, RedisConnectionInfo,
    RedisResult,
};
use redis::{FromRedisValue, ToRedisArgs, Value};
use std::{future::Future, time::Duration};
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;
use tokio_stream::StreamExt;

pub trait PubSub: Send + Sync {
    fn managed_stream(topic: &str) -> impl Future<Output = impl Stream<Item = Id> + Send> + Send;
}

impl PubSub for redis::Client {
    async fn managed_stream(topic: &str) -> impl Stream<Item = Id> + Send {
        let client = RedisConfig::client();
        let backoff = ExponentialBackoff::from_millis(500)
            .max_delay(Duration::from_secs(2))
            .map(jitter)
            .take(5);

        async_stream::stream! {
            loop {
                let pubsub = Retry::spawn(backoff.clone(), || async {
                    let mut pubsub = client.get_async_pubsub().await?;
                    pubsub.subscribe(topic).await?;
                    Ok::<_, redis::RedisError>(pubsub)
                }).await.expect("Failed getting pubsub after retries"); // Should shutdown the system

                // Could just use this stream here but the pubsub connection needs to be managed
                let mut stream = pubsub.into_on_message();
                while let Some(msg) = stream.next().await {
                    match msg.get_payload::<Id>() {
                        Ok(id) => yield id,
                        Err(_) => tracing::warn!("Failed to deserialize message: {:?}", msg),
                    }
                }

                tracing::error!("PubSub stream ended unexpectedly, retrying...")
            }
        }
    }
}

impl PubSub for async_nats::Client {
    async fn managed_stream(topic: &str) -> impl Stream<Item = Id> + Send {
        let backoff = ExponentialBackoff::from_millis(500)
            .max_delay(Duration::from_secs(2))
            .map(jitter)
            .take(5);

        let topic = topic.to_owned();
        async_stream::stream! {
            loop {
                let mut subscriber = Retry::spawn(backoff.clone(), || {
                    let topic = topic.clone();
                    async move {
                        let client = NatsConfig::client().await;
                        client.subscribe(topic).await
                    }
                }).await.expect("Failed getting pubsub after retries");

                while let Some(msg) = subscriber.next().await {
                    match msg.payload.try_into() {
                        Ok(id) => yield id,
                        Err(_) => tracing::warn!("Failed to deserialize message"), // TODO: print it out for debugging
                    }
                }

                tracing::error!("PubSub stream ended unexpectedly, retrying...")
            }
        }
    }
}

#[derive(Envconfig, Debug)]
pub struct RedisConfig {
    #[envconfig(from = "REDIS_HOST", default = "localhost")]
    pub host: String,
    #[envconfig(from = "REDIS_PORT", default = "6379")]
    pub port: u16,
    #[envconfig(from = "REDIS_DB", default = "0")]
    pub db: u8,
}

impl IntoConnectionInfo for RedisConfig {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        Ok(ConnectionInfo {
            addr: ConnectionAddr::Tcp(self.host, self.port),
            redis: RedisConnectionInfo {
                db: self.db as i64,
                username: None,
                password: None,
                protocol: ProtocolVersion::RESP3,
            },
        })
    }
}

impl RedisConfig {
    pub fn client() -> redis::Client {
        let config = RedisConfig::init_from_env().expect("Failed initializing Redis config");
        redis::Client::open(config).expect("Failed opening Redis client")
    }

    pub async fn managed() -> ConnectionManager {
        let client = RedisConfig::client();
        client
            .get_connection_manager_with_config(ConnectionManagerConfig::new())
            .await
            .expect("Failed getting connection manager")
    }
}

#[derive(Envconfig, Debug)]
pub struct NatsConfig {
    #[envconfig(from = "NATS_HOST", default = "localhost")]
    pub host: String,
    #[envconfig(from = "NATS_PORT", default = "4222")]
    pub port: u16,
}

impl NatsConfig {
    pub fn addr(&self) -> String {
        format!("nats://{}:{}", self.host, self.port)
            .parse()
            .expect("Failed to parse address")
    }
}

impl NatsConfig {
    pub async fn client() -> async_nats::Client {
        let central_config = NatsConfig::init_from_env().expect("Failed initializing NATS config");
        async_nats::connect(central_config.addr().to_string())
            .await
            .expect("Failed connecting to NATS")
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Id {
    pub z: u32,
    pub x: u32,
    pub y: u32,
}

impl ToRedisArgs for Id {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        let mut buf = [0u8; 12]; // 3 u32 values, each 4 bytes
        buf[0..4].copy_from_slice(&self.z.to_be_bytes());
        buf[4..8].copy_from_slice(&self.x.to_be_bytes());
        buf[8..12].copy_from_slice(&self.y.to_be_bytes());
        out.write_arg(&buf);
    }
}

impl FromRedisValue for Id {
    fn from_redis_value(value: &Value) -> redis::RedisResult<Self> {
        match value {
            Value::BulkString(v) => {
                if v.len() != 12 {
                    return Err(redis::RedisError::from((
                        redis::ErrorKind::ParseError,
                        "Invalid byte length",
                        format!("Expected 12 but got: {}", v.len()),
                    )));
                }

                Ok(Id {
                    z: u32::from_be_bytes(v[0..4].try_into().unwrap()),
                    x: u32::from_be_bytes(v[4..8].try_into().unwrap()),
                    y: u32::from_be_bytes(v[8..12].try_into().unwrap()),
                })
            }
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Invalid Value type",
                format!("Expected Value::BulkString type but got: {:?}", value),
            ))),
        }
    }
}

impl TryFrom<Bytes> for Id {
    type Error = std::io::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        if value.len() != 12 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected 12 bytes but got: {}", value.len()),
            ));
        }

        Ok(Id {
            z: u32::from_be_bytes(value[0..4].try_into().unwrap()),
            x: u32::from_be_bytes(value[4..8].try_into().unwrap()),
            y: u32::from_be_bytes(value[8..12].try_into().unwrap()),
        })
    }
}

impl From<Id> for Bytes {
    fn from(value: Id) -> Self {
        let mut bytes = BytesMut::with_capacity(12);
        bytes.put_u32(value.z);
        bytes.put_u32(value.x);
        bytes.put_u32(value.y);
        bytes.freeze()
    }
}
