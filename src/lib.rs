use {
    app::{
        log::command::logger_service_client::LoggerServiceClient,
        proxyman::command::handler_service_client::HandlerServiceClient,
        router::command::routing_service_client::RoutingServiceClient,
        stats::command::stats_service_client::StatsServiceClient,
    },
    core::observatory::command::observatory_service_client::ObservatoryServiceClient,
    tonic::transport::{Channel, Error},
    transport::internet::grpc::grpc_service_client::GrpcServiceClient,
};

pub use prost_types;
pub use tonic;

pub mod app {
    pub mod commander {
        tonic::include_proto!("xray.app.commander");
    }

    pub mod dispatcher {
        tonic::include_proto!("xray.app.dispatcher");
    }

    /// <https://xtls.github.io/en/config/dns.html>
    pub mod dns {
        tonic::include_proto!("xray.app.dns");

        /// <https://xtls.github.io/en/config/fakedns.html>
        pub mod fakedns {
            tonic::include_proto!("xray.app.dns.fakedns");
        }
    }

    /// <https://xtls.github.io/en/config/log.html>
    pub mod log {
        tonic::include_proto!("xray.app.log");

        #[cfg(feature = "client")]
        pub mod command {
            tonic::include_proto!("xray.app.log.command");
        }
    }

    /// <https://xtls.github.io/en/config/metrics.html>
    pub mod metrics {
        tonic::include_proto!("xray.app.metrics");
    }

    /// <https://xtls.github.io/en/config/policy.html>
    pub mod policy {
        tonic::include_proto!("xray.app.policy");
    }

    pub mod proxyman {
        tonic::include_proto!("xray.app.proxyman");

        #[cfg(feature = "client")]
        pub mod command {
            tonic::include_proto!("xray.app.proxyman.command");
        }
    }

    /// <https://xtls.github.io/en/config/reverse.html>
    pub mod reverse {
        tonic::include_proto!("xray.app.reverse");
    }

    /// <https://xtls.github.io/en/config/routing.html>
    pub mod router {
        tonic::include_proto!("xray.app.router");

        #[cfg(feature = "client")]
        pub mod command {
            tonic::include_proto!("xray.app.router.command");
        }
    }

    /// <https://xtls.github.io/en/config/stats.html>
    pub mod stats {
        tonic::include_proto!("xray.app.stats");

        #[cfg(feature = "client")]
        pub mod command {
            tonic::include_proto!("xray.app.stats.command");
        }
    }
}

pub mod common {
    pub mod log {
        tonic::include_proto!("xray.common.log");
    }

    pub mod net {
        tonic::include_proto!("xray.common.net");
    }

    pub mod protocol {
        tonic::include_proto!("xray.common.protocol");
    }

    pub mod serial {
        tonic::include_proto!("xray.common.serial");
    }
}

/// <https://xtls.github.io/en/config/>
pub mod core {
    tonic::include_proto!("xray.core");

    /// <https://xtls.github.io/en/config/observatory.html>
    pub mod observatory {
        tonic::include_proto!("xray.core.app.observatory");

        #[cfg(feature = "client")]
        pub mod command {
            tonic::include_proto!("xray.core.app.observatory.command");
        }
    }
}

pub mod proxy {
    /// <https://xtls.github.io/en/config/outbounds/blackhole.html>
    pub mod blackhole {
        tonic::include_proto!("xray.proxy.blackhole");
    }

    /// <https://xtls.github.io/en/config/outbounds/dns.html>
    pub mod dns {
        tonic::include_proto!("xray.proxy.dns");
    }

    /// <https://xtls.github.io/en/config/inbounds/dokodemo.html>
    pub mod dokodemo {
        tonic::include_proto!("xray.proxy.dokodemo");
    }

    /// <https://xtls.github.io/en/config/outbounds/freedom.html>
    pub mod freedom {
        tonic::include_proto!("xray.proxy.freedom");
    }

    /// <https://xtls.github.io/en/config/inbounds/http.html><br>
    /// <https://xtls.github.io/en/config/outbounds/http.html>
    pub mod http {
        tonic::include_proto!("xray.proxy.http");
    }

    /// <https://xtls.github.io/en/config/outbounds/loopback.html>
    pub mod loopback {
        tonic::include_proto!("xray.proxy.loopback");
    }

    /// <https://xtls.github.io/en/config/inbounds/shadowsocks.html><br>
    /// <https://xtls.github.io/en/config/outbounds/shadowsocks.html>
    pub mod shadowsocks {
        tonic::include_proto!("xray.proxy.shadowsocks");
    }

    /// <https://xtls.github.io/en/config/inbounds/shadowsocks.html><br>
    /// <https://xtls.github.io/en/config/outbounds/shadowsocks.html>
    pub mod shadowsocks_2022 {
        tonic::include_proto!("xray.proxy.shadowsocks_2022");
    }

    /// <https://xtls.github.io/en/config/inbounds/socks.html><br>
    /// <https://xtls.github.io/en/config/outbounds/socks.html>
    pub mod socks {
        tonic::include_proto!("xray.proxy.socks");
    }

    /// <https://xtls.github.io/en/config/inbounds/trojan.html><br>
    /// <https://xtls.github.io/en/config/outbounds/trojan.html>
    pub mod trojan {
        tonic::include_proto!("xray.proxy.trojan");
    }

    /// <https://xtls.github.io/en/development/protocols/vless.html>
    pub mod vless {
        tonic::include_proto!("xray.proxy.vless");

        /// <https://xtls.github.io/en/config/inbounds/vless.html>
        pub mod inbound {
            tonic::include_proto!("xray.proxy.vless.inbound");
        }

        /// <https://xtls.github.io/en/config/outbounds/vless.html>
        pub mod outbound {
            tonic::include_proto!("xray.proxy.vless.outbound");
        }
    }

    /// <https://xtls.github.io/en/development/protocols/vmess.html>
    pub mod vmess {
        tonic::include_proto!("xray.proxy.vmess");

        /// <https://xtls.github.io/en/config/inbounds/vmess.html>
        pub mod inbound {
            tonic::include_proto!("xray.proxy.vmess.inbound");
        }

        /// <https://xtls.github.io/en/config/outbounds/vmess.html>
        pub mod outbound {
            tonic::include_proto!("xray.proxy.vmess.outbound");
        }
    }

    /// <https://xtls.github.io/en/config/inbounds/wireguard.html><br>
    /// <https://xtls.github.io/en/config/outbounds/wireguard.html>
    pub mod wireguard {
        tonic::include_proto!("xray.proxy.wireguard");
    }
}


pub mod transport {
    tonic::include_proto!("xray.transport");

    pub mod internet {
        tonic::include_proto!("xray.transport.internet");

        pub mod domainsocket {
            tonic::include_proto!("xray.transport.internet.domainsocket");
        }

        /// <https://xtls.github.io/en/config/transports/grpc.html>
        pub mod grpc {
            tonic::include_proto!("xray.transport.internet.grpc.encoding");
        }

        /// <https://xtls.github.io/en/config/transports/h2.html>
        pub mod http {
            tonic::include_proto!("xray.transport.internet.http");
        }

        /// <https://xtls.github.io/en/config/transports/httpupgrade.html>
        pub mod httpupgrade {
            tonic::include_proto!("xray.transport.internet.httpupgrade");
        }

        /// <https://xtls.github.io/en/config/transports/mkcp.html>
        pub mod kcp {
            tonic::include_proto!("xray.transport.internet.kcp");
        }

        pub mod quic {
            tonic::include_proto!("xray.transport.internet.quic");
        }

        pub mod reality {
            tonic::include_proto!("xray.transport.internet.reality");
        }

        /// <https://xtls.github.io/en/config/transports/splithttp.html>
        pub mod splithttp {
            tonic::include_proto!("xray.transport.internet.splithttp");
        }

        /// <https://xtls.github.io/en/config/transports/tcp.html>
        pub mod tcp {
            tonic::include_proto!("xray.transport.internet.tcp");
        }

        /// <https://xtls.github.io/en/config/transport.html#tlsobject>
        pub mod tls {
            tonic::include_proto!("xray.transport.internet.tls");
        }

        pub mod udp {
            tonic::include_proto!("xray.transport.internet.udp");
        }

        /// <https://xtls.github.io/en/config/transports/websocket.html>
        pub mod websocket {
            tonic::include_proto!("xray.transport.internet.websocket");
        }

        pub mod headers {
            pub mod dns {
                tonic::include_proto!("xray.transport.internet.headers.dns");
            }

            pub mod http {
                tonic::include_proto!("xray.transport.internet.headers.http");
            }

            pub mod noop {
                tonic::include_proto!("xray.transport.internet.headers.noop");
            }

            pub mod srtp {
                tonic::include_proto!("xray.transport.internet.headers.srtp");
            }

            pub mod tls {
                tonic::include_proto!("xray.transport.internet.headers.tls");
            }

            pub mod utp {
                tonic::include_proto!("xray.transport.internet.headers.utp");
            }

            pub mod wechat {
                tonic::include_proto!("xray.transport.internet.headers.wechat");
            }

            pub mod wireguard {
                tonic::include_proto!("xray.transport.internet.headers.wireguard");
            }
        }
    }
}

/// Connect creates a channel to xray GRPC socket.
///
/// This helper intended to be used in conjunction with [Tokio](https://tokio.rs) runtime.
#[cfg(feature = "connect")]
pub async fn connect<S: AsRef<str>>(path: S) -> Result<Channel, Error> {
    use tonic::transport::Endpoint;

    let path = path.as_ref();
    
    let channel = Endpoint::try_from(format!("https://{path}"))?
        .connect()
        .await?;

    Ok(channel)
}

/// Wrapper around tonic-generated client impl.
#[cfg(feature = "client")]
pub struct Client {
    channel: Channel,
}

#[cfg(feature = "client")]
impl Client {
    #[cfg(feature = "connect")]
    pub async fn from_url<S: AsRef<str>>(url: S) -> Result<Self, Error> {
        let channel = connect(url).await?;
        Ok(Self { channel })
    }

    #[inline]
    pub fn logger(&self) -> LoggerServiceClient<Channel> {
        LoggerServiceClient::new(self.channel.clone())
    }

    #[inline]
    pub fn handler(&self) -> HandlerServiceClient<Channel> {
        HandlerServiceClient::new(self.channel.clone())
    }

    #[inline]
    pub fn routing(&self) -> RoutingServiceClient<Channel> {
        RoutingServiceClient::new(self.channel.clone())
    }

    #[inline]
    pub fn stats(&self) -> StatsServiceClient<Channel> {
        StatsServiceClient::new(self.channel.clone())
    }

    #[inline]
    pub fn observatory(&self) -> ObservatoryServiceClient<Channel> {
        ObservatoryServiceClient::new(self.channel.clone())
    }

    #[inline]
    pub fn grpc(&self) -> GrpcServiceClient<Channel> {
        GrpcServiceClient::new(self.channel.clone())
    }
}

#[cfg(test)]
mod tests {
    use prost_types::Any;
    use crate::app::proxyman::InboundConfig;

    #[test]
    fn any_roundtrip() {
        let original = InboundConfig {};

        let any = Any::from_msg(&original).expect("should not fail to encode");
        let decoded: InboundConfig = any.to_msg().expect("should not fail to decode");

        assert_eq!(original, decoded)
    }
}
