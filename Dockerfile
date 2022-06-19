FROM rust:1.61.0-alpine AS lyrebird-build

WORKDIR /lyrebird

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src

RUN apk update \
   && apk add libc-dev \
   && cargo build -r

FROM alpine

COPY --from=lyrebird-build /lyrebird/target/release/lyrebird /usr/local/bin/lyrebird

RUN chmod a+rx /usr/local/bin/lyrebird

CMD /usr/local/bin/lyrebird
