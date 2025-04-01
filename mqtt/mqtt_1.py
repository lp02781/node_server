import paho.mqtt.client as mqtt
import time
import json
import random

broker = "localhost"
port = 1883
publish_topic = "/mqtt_1/data"
subscribe_topic = "/mqtt_actix/data"

client = mqtt.Client(protocol=mqtt.MQTTv5)

def on_connect(client, userdata, flags, rc, properties):
    print(f"Connected with result code {rc}")
    if rc == 0:
        print("Successfully connected to the broker.")
        client.subscribe(subscribe_topic)
    else:
        print(f"Connection failed with code: {rc}")

def on_message(client, userdata, msg):
    try:
        message = json.loads(msg.payload)
        print(f"Received message on topic {msg.topic}: {message}")
    except json.JSONDecodeError:
        print(f"Error decoding message: {msg.payload}")

client.on_connect = on_connect
client.on_message = on_message

print("Connecting to broker...")
client.connect(broker, port, 60)
print("Connection initiated.")

client.loop_start()

try:
    while True:
        temperature = random.uniform(15, 40)
        humidity = random.randint(30, 80)
        current = random.uniform(1, 10)
        sensor_data = {
            "id": "mqtt_1",
            "timestamp": time.time(),
            "data": {  
                "temperature": temperature,
                "humidity": humidity,
                "current": current 
            }
        }

        message = json.dumps(sensor_data)
        client.publish(publish_topic, message)
        print(f"Published to {publish_topic}: {message}")

        time.sleep(10)
        
except KeyboardInterrupt:
    print("Stopping MQTT client...")
    client.loop_stop()
    client.disconnect()
