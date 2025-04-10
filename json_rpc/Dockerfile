FROM python:3.10-slim

WORKDIR /app

COPY json_rpc_1.py .
COPY json_rpc_2.py .
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

RUN pip install --upgrade pip
RUN pip install json-rpc

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]