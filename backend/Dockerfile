FROM python:3-buster

WORKDIR /lab/
COPY . .
EXPOSE 80

RUN apt-get update
RUN apt-get install build-essential libopenblas-dev python3-dev -y

RUN pip install --no-cache-dir -r requirements.txt; exit 0

CMD [ "python", "./server.py" ]