FROM rust:1.67.0 as builder

WORKDIR /hello-rocket

COPY . .

RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
  & apt-get install -y extra-runtime-dependencies \
  & rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder \
  /hello-rocket/target/release/hello-rocket \
  ${APP}/hello-rocket

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./hello-rocket"]
