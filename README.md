[中文文档](./README.zh-CN.md)

#### Introduction
It designed to be an api server that proxys and aggregates information.

#### Support API
- coinmarketcap: `https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest` => `/cryptocurrency/latest`
- awtmt: `https://api-ddc-wscn.awtmt.com/market/real` => `/market/latest`;

#### How to build?
- Install `Rust` and `Cargo`
- Run `make`
- See [Makefile](./Makefile) for more information

#### Reference
- [Rocket](https://rocket.rs/v0.5-rc/guide/introduction/)
