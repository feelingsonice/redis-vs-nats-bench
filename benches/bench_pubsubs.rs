use async_nats::Subject;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{distributions::Alphanumeric, Rng};
use redis::{aio::ConnectionManagerConfig, AsyncCommands};
use redis_vs_nats_bench::{Id, NatsConfig, PubSub, RedisConfig};
use testcontainers::{core::IntoContainerPort, runners::SyncRunner, GenericImage, ImageExt};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

async fn bench_redis_pubsub(msg_count: usize) {
    // random topic name
    let topic: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let cloned_topic = topic.clone();
    let token = CancellationToken::new();
    let token_clone = token.clone();

    let pull_task = tokio::spawn(async move {
        let mut rx = std::pin::pin!(redis::Client::managed_stream(&topic).await);
        loop {
            tokio::select! {
                _ = token_clone.cancelled() => break,
                msg = rx.next() => {
                    match msg {
                        Some(_) => {},
                        None => panic!("Stream ended unexpectedly"),
                    }
                }
            }
        }
    });

    let managed_redis_con = RedisConfig::client()
        .get_connection_manager_with_config(ConnectionManagerConfig::new())
        .await
        .expect("Failed getting connection manager");
    let mut handles = Vec::with_capacity(msg_count);
    let (b, _) = tokio::sync::broadcast::channel(2);
    for _ in 0..msg_count {
        let cloned_topic = cloned_topic.clone();
        let mut cloned = managed_redis_con.clone();
        let mut r = b.subscribe();
        handles.push(tokio::spawn(async move {
            let _ = r.recv().await;
            cloned
                .publish::<_, _, ()>(&cloned_topic, Id { x: 0, y: 0, z: 0 })
                .await
                .expect("Failed to publish");
        }));
    }
    b.send(()).unwrap();

    for handle in handles {
        handle.await.unwrap();
    }
    token.cancel();
    pull_task.await.unwrap();
}

async fn bench_nats_pubsub(msg_count: usize) {
    // random topic name
    let topic: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let cloned_topic = topic.clone();
    let token = CancellationToken::new();
    let token_clone = token.clone();

    let pull_task = tokio::spawn(async move {
        let mut rx = std::pin::pin!(async_nats::Client::managed_stream(&topic).await);
        loop {
            tokio::select! {
                _ = token_clone.cancelled() => break,
                msg = rx.next() => {
                    match msg {
                        Some(_) => {},
                        None => panic!("Stream ended unexpectedly"),
                    }
                }
            }
        }
    });

    let cloned_topic = Subject::from(cloned_topic);
    let client = NatsConfig::client().await;
    let mut handles = Vec::with_capacity(msg_count);
    let (b, _) = tokio::sync::broadcast::channel(2);
    for _ in 0..msg_count {
        let cloned_topic = cloned_topic.clone();
        let cloned = client.clone();
        let mut r = b.subscribe();
        handles.push(tokio::spawn(async move {
            let _ = r.recv().await;
            cloned
                .publish(cloned_topic.clone(), Id { x: 0, y: 0, z: 0 }.into())
                .await
                .expect("Failed to publish");
        }));
    }
    b.send(()).unwrap();
    client.flush().await.expect("Failed to flush");

    for handle in handles {
        handle.await.unwrap();
    }
    token.cancel();
    pull_task.await.unwrap();
}

fn benchmark_simplex_pubsub_group(c: &mut Criterion) {
    let nats_port = portpicker::pick_unused_port().expect("No ports free");
    std::env::set_var::<&str, &str>("NATS_PORT", &nats_port.to_string());
    let _nats_container = GenericImage::new("nats", "2.10")
        .with_exposed_port(4222.tcp())
        .with_mapped_port(nats_port, 4222.tcp())
        .start()
        .expect("NATS test container failed to start");

    let redis_port = portpicker::pick_unused_port().expect("No ports free");
    std::env::set_var::<&str, &str>("REDIS_PORT", &redis_port.to_string());
    let _redis_container = GenericImage::new("redis", "7.4")
        .with_exposed_port(6379.tcp())
        .with_mapped_port(redis_port, 6379.tcp())
        .start()
        .expect("Redis test container failed to start");

    let mut group = c.benchmark_group("PubSub");
    for i in [10000, 20000, 40000, 80000, 160000, 320000, 640000, 1280000] {
        group.bench_with_input(BenchmarkId::new("NATS", i), &i, |b, i| {
            b.to_async(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_time()
                    .enable_io()
                    .build()
                    .unwrap(),
            )
            .iter(|| bench_nats_pubsub(*i));

            // b.iter(|| fibonacci_slow(*i));
        });

        group.bench_with_input(BenchmarkId::new("Redis", i), &i, |b, i| {
            b.to_async(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_time()
                    .enable_io()
                    .build()
                    .unwrap(),
            )
            .iter(|| bench_redis_pubsub(*i));
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_simplex_pubsub_group);
criterion_main!(benches);
