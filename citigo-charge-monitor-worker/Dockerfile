FROM python:3.10-slim

WORKDIR /app
COPY ./requirements.txt ./src/*.py ./
RUN pip install --upgrade --force-reinstall -r requirements.txt
 
ENTRYPOINT [ "python", "main.py" ]