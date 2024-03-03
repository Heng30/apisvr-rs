[English Document](./README.md)

#### 介绍
API服务器。代理和聚合数据。

#### 支持的API
- coinmarketcap: `https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest` => `/cryptocurrency/latest`
- awtmt: `https://api-ddc-wscn.awtmt.com/market/real` => `/market/latest`;

#### 如何构建？
- 安装`Rust`和`Cargo`
- 执行`make`
- [Makefile](./Makefile)了解更多

#### 参考
- [Rocket](https://rocket.rs/v0.5-rc/guide/introduction/)
