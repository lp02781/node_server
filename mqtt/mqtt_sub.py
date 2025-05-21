import paho.mqtt.client as mqtt
import time
import json
import requests

broker = "localhost"
port = 1883
subscribe_topic = "/mqtt/data"
http_endpoint = "http://127.0.0.1:5000/node/mqtt/data" 

client = mqtt.Client(protocol=mqtt.MQTTv5)

def on_connect(client, userdata, flags, rc, properties):
    print(f"[mqtt_sub] Connected with result code {rc}")
    if rc == 0:
        print("[mqtt_sub] Successfully connected to the broker.")
        client.subscribe(subscribe_topic)
    else:
        print(f"[mqtt_sub] Connection failed with code: {rc}")

def on_message(client, userdata, msg):
    try:
        message = json.loads(msg.payload)
        print(f"[mqtt_sub] Received message on topic {msg.topic}: {message}")

        response = requests.post(http_endpoint, json=message)
        print(f"[mqtt_sub] HTTP POST to {http_endpoint} responded with {response.status_code}")

    except json.JSONDecodeError:
        print(f"[mqtt_sub] Error decoding message: {msg.payload}")
    except requests.RequestException as e:
        print(f"[mqtt_sub] HTTP request failed: {e}")

client.on_connect = on_connect
client.on_message = on_message

print("[mqtt_sub] Connecting to broker...")
client.connect(broker, port, 60)
print("[mqtt_sub] Connection initiated.")

client.loop_forever()