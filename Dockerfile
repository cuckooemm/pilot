FROM rust:1.60 as builder

LABEL image.authors="cuckooemm@gmail.com"

WORKDIR /workspace

# 缓存更新依赖
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./config $CARGO_HOME/
RUN mkdir -p src/ && mkdir -p entity/src/
COPY entity/Cargo.toml entity/
RUN touch entity/src/lib.rs
RUN echo "fn main() {println!(\"hello world!\")}" > src/main.rs
RUN cargo build --release
# 以 lib 引入 entity 还需删除 libentity* 文件
RUN rm -rf target/release/deps/pilot* && rm -rf target/release/deps/entity* && rm -rf target/release/deps/libentity*
RUN rm -rf src/* && rm -rf entity/src/*

COPY ./src ./src
COPY ./entity/src ./entity/src

RUN cargo build --release
RUN mv target/release/pilot ./

FROM debian AS app

LABEL image.authors="cuckooemm@gmail.com"

WORKDIR /workspace
ENV PILOT_LISTEN_ADDR=0.0.0.0:80 PILOT_LOG_LEVEL=info PILOT_DB_MASTER_HOST=mysql://root:cuckooemm@localhost/cmm 
ENV PILOT_HASHER_SLAT=laksjdhfgzmxnxcbvpqowiweuyry1231 PILOT_JWT_SECRET=alskdjfhgmznxbcvpqiwueyrto

RUN mkdir bin/
COPY --from=builder /workspace/pilot ./bin/
# COPY --from=builder /workspace/target/x86_64-unknown-linux-musl/release/mchat ./bin/
EXPOSE 8000
CMD [ "./bin/pilot" ]