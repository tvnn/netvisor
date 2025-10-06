// #[tokio::test]
// #[ignore] // Run with --ignored flag: cargo test --ignored
// async fn test_container_daemon_server_integration() {
//     use bollard::Docker;
//     use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
//     use bollard::network::CreateNetworkOptions;
//     use bollard::image::BuildImageOptions;
//     use bollard::models::HostConfig;

//     let docker = Docker::connect_with_local_defaults()
//         .expect("Failed to connect to Docker - is Docker running?");

//     let network_name = "netvisor-test-network";
//     docker.create_network(CreateNetworkOptions {
//         name: network_name,
//         driver: "bridge",
//         ..Default::default()
//     }).await.ok();

//     println!("Building server image from local files...");
//     let server_context = create_build_context("./backend").await;

//     let build_server_options = BuildImageOptions {
//         dockerfile: "Dockerfile",
//         t: "netvisor-server-test",
//         rm: true,
//         ..Default::default()
//     };

//     let mut build_stream = docker.build_image(build_server_options, None, Some(server_context.into()));

//     while let Some(build_info) = build_stream.next().await {
//         match build_info {
//             Ok(info) => {
//                 if let Some(stream) = info.stream {
//                     print!("{}", stream);
//                 }
//                 if let Some(error) = info.error {
//                     panic!("Server build failed: {}", error);
//                 }
//             }
//             Err(e) => panic!("Server build error: {}", e),
//         }
//     }

//     println!("Building daemon image...");
//     let daemon_context = create_build_context("./backend").await;

//     let build_daemon_options = BuildImageOptions {
//         dockerfile: "Dockerfile",
//         t: "netvisor-daemon-test",
//         rm: true,
//         ..Default::default()
//     };

//     let mut build_stream = docker.build_image(build_daemon_options, None, Some(daemon_context.into()));

//     while let Some(build_info) = build_stream.next().await {
//         match build_info {
//             Ok(info) => {
//                 if let Some(stream) = info.stream {
//                     print!("{}", stream);
//                 }
//                 if let Some(error) = info.error {
//                     panic!("Daemon build failed: {}", error);
//                 }
//             }
//             Err(e) => panic!("Daemon build error: {}", e),
//         }
//     }

//     println!("Starting server container...");
//     let server_config = Config {
//         image: Some("netvisor-server-test:latest"),
//         env: Some(vec![
//             "NETVISOR_PORT=60072",
//             "NETVISOR_LOG_LEVEL=debug",
//             "NETVISOR_DATABASE_PATH=/data/netvisor.db",
//         ]),
//         host_config: Some(HostConfig {
//             network_mode: Some(network_name.to_string()),
//             binds: Some(vec!["netvisor-test-data:/data".to_string()]),
//             ..Default::default()
//         }),
//         ..Default::default()
//     };

//     let server_container = docker
//         .create_container(
//             Some(CreateContainerOptions {
//                 name: "netvisor-test-server",
//                 ..Default::default()
//             }),
//             server_config,
//         )
//         .await
//         .expect("Failed to create server container");

//     docker
//         .start_container(&server_container.id, None::<StartContainerOptions<String>>)
//         .await
//         .expect("Failed to start server container");

//     let server_inspect = docker
//         .inspect_container(&server_container.id, None)
//         .await
//         .expect("Failed to inspect server");

//     let server_ip = server_inspect
//         .network_settings
//         .and_then(|ns| ns.networks)
//         .and_then(|networks| networks.get(network_name).cloned())
//         .and_then(|network| network.ip_address)
//         .expect("Failed to get server IP");

//     println!("Server IP: {}", server_ip);
//     println!("Waiting for server to start...");
//     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

//     println!("Starting daemon container...");
//     let daemon_config = Config {
//         image: Some("netvisor-daemon-test:latest"),
//         cmd: Some(vec![
//             "/app/daemon",
//             "--server-target", &server_ip,
//             "--server-port", "60072",
//         ]),
//         env: Some(vec!["RUST_LOG=debug"]),
//         host_config: Some(HostConfig {
//             network_mode: Some(network_name.to_string()),
//             binds: Some(vec!["netvisor-test-daemon-config:/root/.config/netvisor/daemon".to_string()]),
//             ..Default::default()
//         }),
//         ..Default::default()
//     };

//     let daemon_container = docker
//         .create_container(
//             Some(CreateContainerOptions {
//                 name: "netvisor-test-daemon",
//                 ..Default::default()
//             }),
//             daemon_config,
//         )
//         .await
//         .expect("Failed to create daemon container");

//     docker
//         .start_container(&daemon_container.id, None::<StartContainerOptions<String>>)
//         .await
//         .expect("Failed to start daemon container");

//     println!("Waiting for daemon to register...");
//     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

//     // Check logs
//     println!("Checking server logs...");
//     let mut log_stream = docker.logs::<String>(
//         &server_container.id,
//         Some(bollard::container::LogsOptions {
//             stdout: true,
//             stderr: true,
//             follow: false,
//             tail: "all",
//             ..Default::default()
//         }),
//     );

//     let mut server_logs = String::new();
//     while let Some(log) = log_stream.next().await {
//         if let Ok(log_output) = log {
//             let log_str = log_output.to_string();
//             server_logs.push_str(&log_str);
//             print!("{}", log_str);
//         }
//     }

//     println!("\nChecking daemon logs...");
//     let mut log_stream = docker.logs::<String>(
//         &daemon_container.id,
//         Some(bollard::container::LogsOptions {
//             stdout: true,
//             stderr: true,
//             follow: false,
//             tail: "all",
//             ..Default::default()
//         }),
//     );

//     let mut daemon_logs = String::new();
//     while let Some(log) = log_stream.next().await {
//         if let Ok(log_output) = log {
//             let log_str = log_output.to_string();
//             daemon_logs.push_str(&log_str);
//             print!("{}", log_str);
//         }
//     }

//     // Cleanup
//     println!("\nCleaning up...");
//     docker.stop_container(&daemon_container.id, None).await.ok();
//     docker.stop_container(&server_container.id, None).await.ok();
//     docker.remove_container(&daemon_container.id, None).await.ok();
//     docker.remove_container(&server_container.id, None).await.ok();
//     docker.remove_network(network_name).await.ok();

//     // Verify registration happened
//     let found_registration =
//         server_logs.contains("register") ||
//         server_logs.contains("Register") ||
//         server_logs.contains("daemon") ||
//         daemon_logs.contains("registered") ||
//         daemon_logs.contains("Registered") ||
//         daemon_logs.contains("Server at");

//     assert!(
//         found_registration,
//         "Daemon did not successfully register with server. Check logs above."
//     );

//     println!("✅ Container integration test passed!");
// }

// // Helper function to create tar archive for Docker build context
// async fn create_build_context(path: &str) -> Vec<u8> {
//     use std::fs;
//     use walkdir::WalkDir;
//     use std::io::Read;

//     let mut tar_data = Vec::new();
//     let mut tar_builder = Builder::new(&mut tar_data);

//     let base_path = PathBuf::from(path);

//     // Walk the directory and add files to tar
//     for entry in WalkDir::new(&base_path)
//         .into_iter()
//         .filter_map(|e| e.ok())
//         .filter(|e| e.file_type().is_file())
//     {
//         let file_path = entry.path();

//         // Skip target directory and other build artifacts
//         if file_path.to_str().unwrap().contains("/target/") ||
//            file_path.to_str().unwrap().contains("/.git/") ||
//            file_path.to_str().unwrap().contains("/node_modules/") {
//             continue;
//         }

//         let relative_path = file_path.strip_prefix(&base_path).unwrap();

//         let mut file = fs::File::open(file_path).unwrap();
//         let metadata = file.metadata().unwrap();

//         let mut header = Header::new_gnu();
//         header.set_path(relative_path).unwrap();
//         header.set_size(metadata.len());
//         header.set_mode(0o644);
//         header.set_cksum();

//         tar_builder.append(&header, &mut file).unwrap();
//     }

//     tar_builder.finish().unwrap();
//     drop(tar_builder);

//     tar_data
// }
