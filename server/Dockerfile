FROM rust:1.67

WORKDIR /usr/src/server
COPY . .
EXPOSE 8080
RUN cargo install --path .
CMD ["server"]