# learn-polkadot-sdk


O minimal template foi insalado seguindo o repositório https://github.com/paritytech/polkadot-sdk-minimal-template

Para extrair os metadados do nodo local e da rede de polkadot:
 subxt metadata --url="wss:///127.0.0.1:9933" -f bytes > polkadot_metadata.scale
 subxt metadata --url="wss://rpc.polkadot.io:443" -f bytes > polkadot_metadata.scale



Uma serie de exemplos de como utilizar subxt:
 https://github.com/paritytech/subxt/tree/ddb5d4c9d7de07f9d02a832c4264b1a32e39eaf4/subxt/examples

Tutorial basico do Karin:
 https://forum.polkadot.network/t/getting-started-using-rust-and-subxt-for-polkadot-data-extraction/7652

Detalhes sobre extrinsics e eventos:
 https://wiki.polkadot.network/docs/build-protocol-info#extrinsics-and-events

Especificação das mensagens da cadeia:
 https://spec.polkadot.network/chap-networking#sect-msg-block-announce


