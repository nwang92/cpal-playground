use clap::Parser;

pub struct Header {
    // timestamp
    timestamp: u8,
    // sequence number
    seq_number: u8,
    // buffer size in samples
    buffer_size: u8,
    // sampling rate in JackAudioInterface::samplingRateT
    sampling_rate: u8,
    // audio bit resolution
    bit_resolution: u8,
    // number of incoming channels from the network
    num_incoming_channels_from_net: u8,
    // number of outgoing channels to the network
    num_outcoming_channels_to_net: u8,
}

#[derive(Parser, Debug)]
#[command(version, about = "jacktrip-rs options", long_about = None)]
pub struct Opts {
    // Remote client name
    #[arg(long, value_name = "REMOTE_NAME", default_value_t = String::from("default"))]
    remote_name: String,

    // The input audio device to use
    #[arg(long, value_name = "INPUT_DEVICE", default_value_t = String::from("default"))]
    input_device: String,

    // The output audio device to use
    #[arg(long, value_name = "OUTPUT_DEVICE", default_value_t = String::from("default"))]
    output_device: String,

    // Number of send channels to the network (# greater than 0)
    #[arg(long, value_name = "SEND_CHANNELS", default_value_t = 2)]
    send_channels: u64,

    // Number of receive channels from the network (# greater than 0)
    #[arg(long, value_name = "RECEIVE_CHANNELS", default_value_t = 2)]
    receive_channels: u64,

    // Sample rate
    #[arg(long, value_name = "SAMPLE_RATE", default_value_t = 48000)]
    sample_rate: u64,

    // Buffer size
    #[arg(long, value_name = "BUFFER_SIZE", default_value_t = 128.0)]
    buffer_size: f32,

    // Queue buffer length, in packet size
    #[arg(long, value_name = "RECEIVE_CHANNELS", default_value_t = 4)]
    queue: u64,

    // Enable broadcast - duplicate receive ports with the specified broadcast_queue length. Broadcast outputs have higher latency but less packet loss.
    #[arg(long, value_name = "BROADCAST_QUEUE", default_value_t = 0)]
    broadcast: u64,

    // Bind port
    #[arg(long, value_name = "BIND_PORT", default_value_t = 4464)]
    bind_port: u16,

    // Peer port
    #[arg(long, value_name = "PEER_PORT", default_value_t = 4464)]
    peer_port: u16,

    // Enable JACK
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    #[arg(short, long)]
    #[allow(dead_code)]
    jack: bool,
}
