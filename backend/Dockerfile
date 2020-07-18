FROM python:3-alpine

WORKDIR /usr/src/app
COPY . .
EXPOSE 80

RUN apt-get install build-essential -y
RUN apt-get install python3-dev -y

RUN pip install --no-cache-dir -r requirements.txt

CMD [ "python", "./server.py" ]