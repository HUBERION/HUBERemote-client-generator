FROM rust:1.61

EXPOSE 1234

WORKDIR /usr/src/hr-gen
COPY . .

RUN cargo install --path .

COPY bin/supporter.exe bin/supporter.exe

CMD ["hr-gen"]