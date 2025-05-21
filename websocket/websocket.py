import asyncio
import websockets
import json
import random
import time

ws_uri = f"ws://127.0.0.1:5000/ws/websocket"

async def send_sensor_data(websocket):
    while True:
        temperature = random.uniform(15, 40)
        humidity = random.randint(30, 80)
        current = random.uniform(1, 10)
        sensor_data = {
            "id": "websocket",
            "timestamp": time.time(),
            "data": {
                "temperature": temperature,
                "humidity": humidity,
                "current": current
            }
        }

        message = json.dumps(sensor_data)
        await websocket.send(message)
        print(f"Sent: {message}")
        await asyncio.sleep(8)

async def receive_data(websocket):
    async for message in websocket:
        try:
            data = json.loads(message)
            print(f"Received: {data}")
        except json.JSONDecodeError:
            print(f"Invalid JSON received: {message}")

async def main():
    async with websockets.connect(ws_uri) as websocket:
        print(f"Connected to {ws_uri}")
        await asyncio.gather(
            send_sensor_data(websocket),
            receive_data(websocket)
        )

try:
    asyncio.run(main())
except KeyboardInterrupt:
    print("WebSocket client stopped.")
