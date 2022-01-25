FROM rust:1.57 as builder
RUN USER=root cargo new --bin server_runner
WORKDIR ./server_runner
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
ADD . ./
RUN rm ./target/release/deps/server_runner*
RUN cargo build --release
FROM debian:latest
ARG APP=/usr/src/app
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libc6 lsof net-tools curl\
    && rm -rf /var/lib/apt/lists/*
ENV TZ=Etc/UTC \
    APP_USER=ue4user
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}
COPY --from=builder /server_runner/target/release/server_runner ${APP}/server_runner
COPY ./server ${APP}/server
RUN chown ${APP_USER}:${APP_USER} -R ${APP}/server
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}
EXPOSE 3030
EXPOSE 7777-7787/udp
CMD ["./server_runner"]