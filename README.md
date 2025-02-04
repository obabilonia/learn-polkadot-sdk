# learn-polkadot-sdk


**O minimal template foi instalado seguindo o repositório**:
<br> https://github.com/paritytech/polkadot-sdk-minimal-template

**Pasos para compilar e executar o minimal-node**:
<br> git clone https://github.com/paritytech/polkadot-sdk-minimal-template.git
<br> cd polkadot-sdk-minimal-template/
<br> sudo apt update
<br> sudo apt upgrade -y
<br> cargo update
<br> rustp update
<br> sudo apt-get install protobuf-compiler
<br> sudo apt install -y clang cmake make gcc g++ libstdc++-dev librocksdb-dev pkg-config
<br> gcc --version
<br> sudo apt install libstdc++-11-dev
<br> sudo apt autoremove
<br> sudo apt install libstdc++-11-dev
<br> sudo apt install -y clang cmake make gcc g++ pkg-config libssl-dev zlib1g-dev libbz2-dev liblz4-dev libsnappy-dev librocksdb-dev
<br> cargo build --release
<br> ./target/release/minimal-template-node --dev --rpc-port 9933

**Repositorio de instalação do parachai-template:**:
<br>https://github.com/paritytech/polkadot-sdk-parachain-template

**Para extrair os metadados do nodo local e da rede de polkadot**:
<br> subxt metadata --url="wss:///127.0.0.1:9933" -f bytes > polkadot_metadata.scale
<br> subxt metadata --url="wss://rpc.polkadot.io:443" -f bytes > polkadot_metadata.scale


**Uma serie de exemplos de como utilizar subxt**:
<br> https://github.com/paritytech/subxt/tree/ddb5d4c9d7de07f9d02a832c4264b1a32e39eaf4/subxt/examples


**Tutorial basico do Karin**:
<br> https://forum.polkadot.network/t/getting-started-using-rust-and-subxt-for-polkadot-data-extraction/7652


**Detalhes sobre extrinsics e eventos**:
<br> https://wiki.polkadot.network/docs/build-protocol-info#extrinsics-and-events


**Especificação das mensagens da cadeia**:
<br> https://spec.polkadot.network/chap-networking#sect-msg-block-announce


**Link para entender sobre Zombienet**:
https://github.com/paritytech/zombienet
