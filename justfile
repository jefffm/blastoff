set shell := ["powershell", "-c"]

serve package: (build package)
    miniserve --index index.html ./target/{{package}}/

build package: (clean package)
    mkdir -p ./target/{{package}}/
    cp ./index.html ./target/{{package}}/
    cargo build --release --package {{package}} --target wasm32-unknown-unknown
    wasm-bindgen ./target/wasm32-unknown-unknown/release/{{package}}.wasm --out-dir ./target/{{package}} --no-modules --no-typescript

clean package:
    Remove-Item -Recurse -Force ./target/{{package}}/