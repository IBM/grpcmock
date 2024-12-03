mod pb {
    tonic::include_proto!("grpc.health.v1");
}

#[cfg(test)]
mod tests {
    use super::pb::{health_client::HealthClient, HealthCheckRequest, HealthCheckResponse};
    use grpcmock::prelude::*;
    use tonic::transport::Channel;

    grpcmock::generate_server!("grpc.health.v1.Health", MockHealthServer);

    #[tokio::test]
    async fn test_health() -> Result<(), anyhow::Error> {
        let mut mocks = MockSet::new();
        mocks.insert(
            GrpcMethod::new("grpc.health.v1.Health", "Check")?,
            Mock::new(
                HealthCheckRequest { service: "".into() },
                HealthCheckResponse { status: 1 },
            ),
        );
        let server = MockHealthServer::start(mocks).await?;

        // Create client
        let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.addr().port()))?
            .connect()
            .await?;
        let mut client = HealthClient::new(channel);

        let result = client
            .check(HealthCheckRequest { service: "".into() })
            .await;
        // dbg!(&result);
        assert!(result.is_ok());

        Ok(())
    }
}