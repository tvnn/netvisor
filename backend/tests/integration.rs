use std::process::Command;

use netvisor::server::daemons::types::base::Daemon;
use netvisor::server::shared::types::api::ApiResponse;

#[tokio::test]
async fn test_container_daemon_server_integration() {
    // Start all services (server, daemon, ui) with docker-compose in background
    println!("Starting containers with make dev-container...");
    let mut container_process = Command::new("make")
        .arg("dev-container")
        .current_dir("..")
        .spawn()
        .expect("Failed to start containers");

    println!("Waiting for services to fully initialize...");
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

    // Verify daemon registration via API with retries
    let client = reqwest::Client::new();
    let max_retries = 15;
    let mut daemons = Vec::new();
    let mut last_error = String::new();

    for attempt in 1..=max_retries {
        println!(
            "Attempt {}/{} to check daemon registration...",
            attempt, max_retries
        );

        match client
            .get("http://localhost:60072/api/daemons")
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    match response.json::<ApiResponse<Vec<Daemon>>>().await {
                        Ok(api_response) => {
                            if api_response.success {
                                if let Some(daemon_list) = api_response.data {
                                    daemons = daemon_list;
                                    if !daemons.is_empty() {
                                        println!("✅ Found {} daemon(s) registered", daemons.len());
                                        break;
                                    } else {
                                        println!("⏳ No daemons registered yet, waiting...");
                                    }
                                }
                            } else {
                                let error = api_response
                                    .error
                                    .unwrap_or_else(|| "Unknown error".to_string());
                                println!("API returned success=false: {}", error);
                                last_error = error;
                            }
                        }
                        Err(e) => {
                            println!("Failed to parse response: {}", e);
                            last_error = e.to_string();
                        }
                    }
                } else {
                    let body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not read body".to_string());
                    println!("Status {}: {}", status, body);
                    last_error = body;
                }
            }
            Err(e) => {
                println!("Request failed: {}", e);
                last_error = e.to_string();
            }
        }

        if attempt < max_retries {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    assert!(
        !daemons.is_empty(),
        "Expected 1 daemon to be registered, found 0. Last error: {}",
        last_error
    );

    assert_eq!(
        daemons.len(),
        1,
        "Expected 1 daemon to be registered, found {}",
        daemons.len()
    );

    println!("✅ Container integration test passed! Daemon successfully registered.");

    // Cleanup - kill the make process and stop containers
    println!("\nCleaning up containers...");
    let _ = container_process.kill();
    let _ = container_process.wait();

    let cleanup_output = Command::new("make")
        .arg("dev-down")
        .current_dir("..")
        .output()
        .expect("Failed to stop containers");

    if !cleanup_output.status.success() {
        eprintln!(
            "make dev-down warning: {}",
            String::from_utf8_lossy(&cleanup_output.stderr)
        );
    }
}
