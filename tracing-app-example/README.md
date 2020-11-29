
Simple application intended for generating tracing traffic w/i a kubernetes cluster.

For local development
```
cargo build # build grpc client and server
RUST_LOG=info cargo run --bin tracingapp-server
./target/debug/tracingapp-client
```

Building locally docker
```
docker build -t unboundev/tracing-app .
docker run --net=host -p 8080 -it --rm --name tracing-app unboundev/tracing-app
docker push docker.io/unboundev/tracing-app
```

Installing in kubernetes
```
minikube start --driver=virtualbox --kubernetes-version=v1.19.4
kubectl apply -f ../hack/kubernetes/tracingapp.yaml
kubectl port-forward service/tracingapp 8080:80
./target/debug/tracingapp-client
```

App references:
- grpc rust client: https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
- tracing examples: https://github.com/tokio-rs/tracing/tree/master/examples/examples
- tracing subscriber: https://docs.rs/tracing-subscriber/0.2.15/tracing_subscriber/index.html