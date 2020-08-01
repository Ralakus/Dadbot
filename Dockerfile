FROM rust:stretch

WORKDIR /lab/

COPY . .

RUN cargo install --path .

CMD [ "dadbot" ]