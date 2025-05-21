FROM python:3.10-slim

WORKDIR /app

COPY mqtt_pub.py .
COPY mqtt_sub.py .
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

RUN pip install --upgrade pip
RUN pip install --upgrade paho-mqtt supervisor requests

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]