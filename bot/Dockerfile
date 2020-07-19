FROM rust:stretch

WORKDIR /lab/

COPY . .

RUN apt-get update
RUN apt-get install openssl -y

RUN cargo install --path .

CMD [ "dadbot" ]