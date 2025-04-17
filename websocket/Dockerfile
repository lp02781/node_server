FROM python:3.10-slim

WORKDIR /app

COPY websocket_1.py .
COPY websocket_2.py .
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

RUN pip install --upgrade pip
RUN pip install websockets
RUN apt-get update && apt-get install -y supervisor

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]