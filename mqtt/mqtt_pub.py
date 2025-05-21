import paho.mqtt.client as mqtt
import time
import json
import random

broker = "localhost"
port = 1883
publish_topic = "/mqtt/data"
#subscribe_topic = "/mqtt/data"

client = mqtt.Client(protocol=mqtt.MQTTv5)

def on_connect(client, userdata, flags, rc, properties):
    print(f"[mqtt_pub] Connected with result code {rc}")
    if rc == 0:
        print("[mqtt_pub] Successfully connected to the broker.")
        #client.subscribe(subscribe_topic)
    else:
        print(f"[mqtt_pub] Connection failed with code: {rc}")

#def on_message(client, userdata, msg):
#    try:
#        message = json.loads(msg.payload)
#        print(f"Received message on topic {msg.topic}: {message}")
#    except json.JSONDecodeError:
#        print(f"Error decoding message: {msg.payload}")

client.on_connect = on_connect
#client.on_message = on_message

print("[mqtt_pub] Connecting to broker...")
client.connect(broker, port, 60)
print("[mqtt_pub] Connection initiated.")

client.loop_start()

try:
    while True:
        temperature = random.uniform(15, 40)
        humidity = random.randint(30, 80)
        current = random.uniform(1, 10)
        sensor_data = {
            "id": "mqtt",
            "timestamp": time.time(),
            "data": {  
                "temperature": temperature,
                "humidity": humidity,
                "current": current 
            }
        }

        message = json.dumps(sensor_data)
        client.publish(publish_topic, message)
        print(f"[mqtt_pub] Published to {publish_topic}: {message}")

        time.sleep(10)
        
except KeyboardInterrupt:
    print("[mqtt_pub] Stopping MQTT client...")
    client.loop_stop()
    client.disconnect()
