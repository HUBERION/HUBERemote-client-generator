# Usage

```shell
git clone https://github.com/HUBERION/HUBERemote-client-generator 
```

```shell
cd HUBERemote-client-generator
```

## Docker

```shell
# This will take some time
docker build -t huberemote/download -f Docker/Dockerfile .
```

```shell
docker run -p 1234:1234 huberemote/download
```

## Cargo

```shell
# This will take some time
cargo run --release
```