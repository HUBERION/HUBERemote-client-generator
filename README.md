# Environment Variables

| Variable | Description | Default |
| -------- | ----------- | ------- |
| PORT | The Port the Server is running on | 1234 |

# Usage

```shell
git clone https://github.com/HUBERION/HUBERemote-client-generator 
```

```shell
cd HUBERemote-client-generator
```

## With Docker

```shell
# This will take some time
docker build -t huberemote/download -f Docker/Dockerfile .
```

```shell
docker run -p 1234:1234 -e PORT=1234 huberemote/download
```