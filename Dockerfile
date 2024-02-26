FROM rust:1.67
WORKDIR /app
COPY . .

CMD ["myapp"]

