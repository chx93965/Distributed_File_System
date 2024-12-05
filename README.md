## Build cluster
```bash
docker compose build
```

## Start cluster
```bash
docker compose up -d
```
```bash
docker compose scale chunkserver=3
```
```bash
docker compose up --scale chunkserver=3
```