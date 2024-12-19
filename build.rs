const PROTO_FILES: &[&str] = &[
    // -- App --
    "vendor/github.com/XTLS/Xray-core/app/commander/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/dispatcher/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/dns/fakedns/fakedns.proto",
    "vendor/github.com/XTLS/Xray-core/app/dns/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/log/command/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/log/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/metrics/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/observatory/burst/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/observatory/command/command.proto",
    "vendor/github.com/XTLS/Xray-core/app/observatory/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/policy/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/proxyman/command/command.proto",
    "vendor/github.com/XTLS/Xray-core/app/proxyman/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/reverse/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/router/command/command.proto",
    "vendor/github.com/XTLS/Xray-core/app/router/config.proto",
    "vendor/github.com/XTLS/Xray-core/app/stats/command/command.proto",
    "vendor/github.com/XTLS/Xray-core/app/stats/config.proto",

    // -- Common --
    "vendor/github.com/XTLS/Xray-core/common/log/log.proto",
    "vendor/github.com/XTLS/Xray-core/common/net/address.proto",
    "vendor/github.com/XTLS/Xray-core/common/net/destination.proto",
    "vendor/github.com/XTLS/Xray-core/common/net/network.proto",
    "vendor/github.com/XTLS/Xray-core/common/net/port.proto",
    "vendor/github.com/XTLS/Xray-core/common/protocol/headers.proto",
    "vendor/github.com/XTLS/Xray-core/common/protocol/server_spec.proto",
    "vendor/github.com/XTLS/Xray-core/common/protocol/user.proto",
    "vendor/github.com/XTLS/Xray-core/common/serial/typed_message.proto",

    // -- Core --
    "vendor/github.com/XTLS/Xray-core/core/config.proto",

    // -- Proxy --
    "vendor/github.com/XTLS/Xray-core/proxy/blackhole/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/dns/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/dokodemo/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/freedom/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/http/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/loopback/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/shadowsocks/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/shadowsocks_2022/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/socks/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/trojan/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/vless/encoding/addons.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/vless/inbound/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/vless/outbound/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/vless/account.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/vmess/inbound/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/vmess/outbound/config.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/vmess/account.proto",
    "vendor/github.com/XTLS/Xray-core/proxy/wireguard/config.proto",

    // -- Transport --
    "vendor/github.com/XTLS/Xray-core/transport/internet/grpc/encoding/stream.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/grpc/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/headers/dns/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/headers/http/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/headers/noop/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/headers/srtp/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/headers/tls/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/headers/utp/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/headers/wechat/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/headers/wireguard/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/httpupgrade/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/kcp/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/reality/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/splithttp/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/tcp/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/tls/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/udp/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/websocket/config.proto",
    "vendor/github.com/XTLS/Xray-core/transport/internet/config.proto",
];

pub fn main() {
    let mut config = prost_build::Config::new();
    config.enable_type_names();

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos_with_config(config, PROTO_FILES, &["vendor/github.com/XTLS/Xray-core"])
        .unwrap_or_else(|e| panic!("Failed to compile proto with: {e}"))
}
