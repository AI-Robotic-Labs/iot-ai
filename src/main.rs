use std::error::Error;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use ros2_client::{Context, Node, Publisher};
use ros2_client::msg::String as ROSString;

#[derive(Serialize, Deserialize, Debug)]
struct SensorData {
    temperature: f32,
    vibration: f32,
    speed: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommandMessage {
    data: String,
}

async fn collect_sensor_data() -> Result<SensorData, Box<dyn Error>> {
    // Simulate reading from IoT sensors
    Ok(SensorData {
        temperature: 75.0,
        vibration: 0.5,
        speed: 1.2,
    })
}

async fn analyze_data(data: &SensorData) -> bool {
    // Simulate AI analysis (detects anomaly if temperature > 80)
    data.temperature > 80.0
}

async fn control_robot(publisher: &Publisher<ROSString>, command: &str) -> Result<(), Box<dyn Error>> {
    let msg = ROSString { data: command.to_string() };
    
    if let Err(e) = publisher.publish(&msg) {
        eprintln!("Failed to send command: {}", e);
        return Err(Box::new(e));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize ROS2 context
    let context = Context::new().map_err(|e| {
        eprintln!("Failed to create ROS2 context: {}", e);
        e
    })?;

    // Create ROS2 node
    let node = Node::create(context.clone(), "robot_control_node", Default::default()).map_err(|e| {
        eprintln!("Failed to create ROS2 node: {}", e);
        e
    })?;

    // Create ROS2 topic
    let topic = node.create_topic(
        "/robot/commands",
        &Default::default(),
    ).map_err(|e| {
        eprintln!("Failed to create ROS2 topic: {}", e);
        e
    })?;

    // Create ROS2 publisher
    let publisher = node.create_publisher(&topic, None).map_err(|e| {
        eprintln!("Failed to create ROS2 publisher: {}", e);
        e
    })?;

    // Main loop
    loop {
        let data = collect_sensor_data().await?;
        println!("Collected sensor data: {:?}", data);

        // Analyze data for any anomalies
        if analyze_data(&data).await {
            println!("Anomaly detected! Sending stop command to robot.");
            if let Err(e) = control_robot(&publisher, "STOP").await {
                eprintln!("Error sending stop command: {}", e);
            }
        } else {
            println!("All systems normal.");
        }

        // Wait for 5 seconds before the next loop iteration
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}