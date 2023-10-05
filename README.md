# rust-websocket-server
一个练手rust项目\
使用websocket进行通信

### 配置项
| 配置项名称     | 值的类型        | 说明           |
|-----------|-------------|--------------|
| host      | String      | 服务器地址        |
| port      | u16         | 服务器端口        |
| whitelist | Vec[String] | 连接白名单，留空为不限制 |
