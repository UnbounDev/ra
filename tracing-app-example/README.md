
Simple application intended for generating tracing traffic w/i a kubernetes cluster.

For local development
```
cargo build # build grpc client and server
RUST_LOG=info cargo run --bin tracingapp-server
./target/debug/tracingapp-client
```

Building docker (TODO: move to github actions after push to main)
```
docker build -t unboundev/tracing-app .
docker run --net=host -p 8080 -it --rm --name tracing-app unboundev/tracing-app
docker push docker.io/unboundev/tracing-app
```