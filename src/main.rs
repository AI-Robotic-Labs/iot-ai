use serde::{Serialize, Deserialize};
use tokio::time::{sleep, Duration};
use ros2_client::{Context, Node, Publisher};
use std::error::Error;

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

async fn collect_sensor_data() -> SensorData {
    // Simulate reading from IoT sensors
    SensorData {
        temperature: 75.0,
        vibration: 0.5,
        speed: 1.2,
    }
}

async fn analyze_data(data: &SensorData) -> bool {
    // Simulate AI analysis (detects anomaly if temperature > 80)
    data.temperature > 80.0
}

async fn control_robot(publisher: &Publisher<CommandMessage>, command: &str) -> Result<(), Box<dyn Error>> {
    let msg = CommandMessage { data: command.into() };
    
    if let Err(e) = publisher.publish(msg) {
        eprintln!("Failed to send command: {}", e);
        return Err(Box::new(e));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::new().map_err(|e| {
        eprintln!("Failed to create ROS2 context: {}", e);
        e
    })?;
    

    let node = Node::create(context.clone(), "robot_control_node").map_err(|e| {
        eprintln!("Failed to create ROS2 node: {}", e);
        e
    })?;

    let topic = node.create_topic("/robot/commands", "std_msgs/msg/String", &Default::default()).map_err(|e| {
        eprintln!("Failed to create ROS2 topic: {}", e);
        e
    })?;

    let publisher = node.create_publisher(&topic, None).map_err(|e| {
        eprintln!("Failed to create ROS2 publisher: {}", e);
        e
    })?;

    loop {
        let data = collect_sensor_data().await;
        println!("Collected sensor data: {:?}", data);

        if analyze_data(&data).await {
            println!("Anomaly detected! Sending stop command to robot.");
            if let Err(e) = control_robot(&publisher, "STOP").await {
                eprintln!("Error sending stop command: {}", e);
            }
        } else {
            println!("All systems normal.");
        }

        sleep(Duration::from_secs(5)).await;
    }
}