use std::net::{SocketAddrV4, SocketAddrV6};

pub enum WirelessMode {
    Auto,       /* Let the driver decide */
    AdHoc,      /* Single cell network */
    Infra,      /* Multi cell network, roaming, ... */
    Master,     /* Synchronisation master or Access Point */
    Repeat,     /* Wireless Repeater (forwarder) */
    Second,     /* Secondary master/repeater (backup) */
    Monitor     /* Passive monitor (listen only) */
}

pub struct IwQuality {
    quality: u8,
    level: u8,
    noise: u8
}

pub struct IwStats {
    status: uint16,
    quality: IwQuality,
}

pub struct WirelessKey {
    key: &[u8],
    size: u32,
    flags: i32
}


pub struct WirelessNetwork {
    ap_addr4: Option<SocketAddrV4>,
    ap_addr6: Option<SocketAddrV6>,
    stats: Option<IwStats>,
    maxbitrate: Option<i32>,
    name: String,
    freq: Option<f64>,
    key: Option<WirelessKey>,
    essid: Option<String>,
    mode: Option<WirelessMode>
}

struct priv_iw_quality {
    qual: uint8_t,
    level: uint8_t,
    noise: uint8_t,
    updated: uint8_t
}

struct priv_iw_freq {
    m: int32_t,
    e: int16_t,
    i: uint8_t,
    flags: uint8_t
}

#[repr(C)]
struct  iw_range {
    /* Informative stuff (to choose between different interface) */
    throughput: uint32_t,   /* To give an idea... */
    /* In theory this value should be the maximum benchmarked
     * TCP/IP throughput, because with most of these devices the
     * bit rate is meaningless (overhead an co) to estimate how
     * fast the connection will go and pick the fastest one.
     * I suggest people to play with Netperf or any benchmark...
     */

    /* NWID (or domain id) */
    min_nwid: uint32_t, /* Minimal NWID we are able to set */
    max_nwid: uint32_t, /* Maximal NWID we are able to set */

    /* Old Frequency (backward compat - moved lower ) */
    old_num_channels: uint16_t,
    old_num_frequency: uint8_t,

    /* Wireless event capability bitmasks */
    event_capa: [uint32_t, 6],

    /* signal level threshold range */
    sensitivity: int32_t,

    /* Quality of link & SNR stuff */
    /* Quality range (link, level, noise)
     * If the quality is absolute, it will be in the range [0 , max_qual],
     * if the quality is dBm, it will be in the range [max_qual , 0].
     * Don't forget that we use 8 bit arithmetics... */
    max_qual: priv_iw_quality,  /* Quality of the link */
    /* This should contain the average/typical values of the quality
     * indicator. This should be the threshold between a "good" and
     * a "bad" link (example : monitor going from green to orange).
     * Currently, user space apps like quality monitors don't have any
     * way to calibrate the measurement. With this, they can split
     * the range between 0 and max_qual in different quality level
     * (using a geometric subdivision centered on the average).
     * I expect that people doing the user space apps will feedback
     * us on which value we need to put in each driver... */
    avg_qual: priv_iw_quality,  /* Quality of the link */

    /* Rates */
    num_bitrates: uint8_t,  /* Number of entries in the list */
    bitrate: [int32_t; IW_MAX_BITRATES],    /* list, in bps */

    /* RTS threshold */
    min_rts: int32_t,   /* Minimal RTS threshold */
    max_rts: int32_t,   /* Maximal RTS threshold */

    /* Frag threshold */
    min_frag: int32_t,  /* Minimal frag threshold */
    max_frag: int32_t,  /* Maximal frag threshold */

    /* Power Management duration & timeout */
    min_pmp: int32_t,   /* Minimal PM period */
    max_pmp: int32_t,   /* Maximal PM period */
    min_pmt: int32_t,   /* Minimal PM timeout */
    max_pmt: int32_t,   /* Maximal PM timeout */
    pmp_flags: uint16_t,    /* How to decode max/min PM period */
    pmt_flags uint16_t, /* How to decode max/min PM timeout */
    pm_capa: uint16_t,  /* What PM options are supported */

    /* Encoder stuff */
    encoding_size: [uint16_t; IW_MAX_ENCODING_SIZES],   /* Different token sizes */
    num_encoding_sizes: uint8_t,    /* Number of entry in the list */
    max_encoding_tokens: uint8_t,   /* Max number of tokens */
    /* For drivers that need a "login/passwd" form */
    encoding_login_index: uint8_t,  /* token index for login token */

    /* Transmit power */
    txpower_capa: uint16_t, /* What options are supported */
    num_txpower: uint8_t,   /* Number of entries in the list */
    txpower: [int32_t; IW_MAX_TXPOWER], /* list, in bps */

    /* Wireless Extension version info */
    we_version_compiled: uint8_t,   /* Must be WIRELESS_EXT */
    we_version_source: uint8_t, /* Last update of source */

    /* Retry limits and lifetime */
    retry_capa: uint16_t,   /* What retry options are supported */
    retry_flags: uint16_t,  /* How to decode max/min retry limit */
    r_time_flags: uint16_t, /* How to decode max/min retry life */
    min_retry: int32_t, /* Minimal number of retries */
    max_retry: int32_t, /* Maximal number of retries */
    min_r_time: int32_t,    /* Minimal retry lifetime */
    max_r_time: int32_t,    /* Maximal retry lifetime */

    /* Frequency */
    num_channels: uint16_t, /* Number of channels [0, num - 1] */
    num_frequency: uint8_t, /* Number of entry in the list */
    freq: [priv_iw_freq; IW_MAX_FREQUENCIES],   /* list */
    /* Note : this frequency list doesn't need to fit channel numbers,
     * because each entry contain its channel index */

    enc_capa: uint32_t, /* IW_ENC_CAPA_* bit field */

    /* More power management stuff */
    min_pms: int32_t,   /* Minimal PM saving */
    max_pms: int32_t,   /* Maximal PM saving */
    pms_flags: int16_t, /* How to decode max/min PM saving */

    /* All available modulations for driver (hw may support less) */
    modul_capa: uint32_t,   /* IW_MODUL_* bit field */

    /* More bitrate stuff */
    bitrate_capa: uint32_t, /* Types of bitrates supported */
}

pub struct WifiScan {
    networks: Vec<WirelessNetwork>
}

impl WifiScan {
    pub fn new() {
        WifiScan{ networks: Vec::new() }
    }

    pub fn scan() {
        // Scan things here
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
