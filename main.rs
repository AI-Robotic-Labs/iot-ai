use serde::{Serialize, Deserialize};
use tokio::time::{sleep, Duration};
use r2r::RobotCommander;

#[derive(Serialize, Deserialize)]
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

async fn control_robot(command: &str) {
    let robot = RobotCommander::new();
    robot.send_command(command).await.unwrap();
}

#[tokio::main]
async fn main() {
    loop {
        let data = collect_sensor_data().await;
        println!("Collected sensor data: {:?}", data);

        if analyze_data(&data).await {
            println!("Anomaly detected! Sending stop command to robot.");
            control_robot("STOP").await;
        } else {
            println!("All systems normal.");
        }

        sleep(Duration::from_secs(5)).await;
    }
}
