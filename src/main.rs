use serde::{Serialize, Deserialize};
use tokio::time::{sleep, Duration};
use ros2_client::{Context, Node, Publisher};
use ros2_client::messages::StringMessage;
#[derive(Serialize, Deserialize, Debug)]
struct SensorData {
    temperature: f32,
    vibration: f32,
    speed: f32,
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

async fn control_robot(publisher: &Publisher<StringMessage>, command: &str) {
    let msg = StringMessage { data: command.into() };
    publisher.publish(msg).expect("Failed to send command");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;
    let node = Node::new(context.clone(), "robot_control_node", "")?;
    let publisher = node.create_publisher::<StringMessage>("/robot/commands", 10)?;

    loop {
        let data = collect_sensor_data().await;
        println!("Collected sensor data: {:?}", data);

        if analyze_data(&data).await {
            println!("Anomaly detected! Sending stop command to robot.");
            control_robot(&publisher, "STOP").await;
        } else {
            println!("All systems normal.");
        }

        sleep(Duration::from_secs(5)).await;
    }
}