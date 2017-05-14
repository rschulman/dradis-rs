extern crate libc;

use std::net::{SocketAddrV4, SocketAddrV6};
use std::ffi::CString;
use std::ptr;
use std::io::{Error, ErrorKind};
use libc::*;

const IW_AUTH_WPA_VERSION_DISABLED: u8 = 0x00000001;
const IW_AUTH_WPA_VERSION_WPA: u8 = 0x00000002;
const IW_AUTH_WPA_VERSION_WPA2: u8 = 0x00000004;
const IW_MAX_BITRATES: usize = 32;
const IW_MAX_ENCODING_SIZES: usize = 8;
const IW_MAX_FREQUENCIES: usize = 32;
const IW_MAX_TXPOWER: usize = 8;

pub enum WirelessMode {
    Auto, /* Let the driver decide */
    AdHoc, /* Single cell network */
    Infra, /* Multi cell network, roaming, ... */
    Master, /* Synchronisation master or Access Point */
    Repeat, /* Wireless Repeater (forwarder) */
    Second, /* Secondary master/repeater (backup) */
    Monitor, /* Passive monitor (listen only) */
}

pub struct IwQuality {
    quality: u8,
    level: u8,
    noise: u8,
}

pub struct IwStats {
    status: uint16_t,
    quality: IwQuality,
}

pub struct WirelessKey<'a> {
    key: &'a [u8],
    size: u32,
    flags: i32,
}

/// The WirelessNetwork struct holds details about a single network,
/// including ssid, encryption type, bitrate, and signal strength.
pub struct WirelessNetwork<'a> {
    ap_addr4: Option<SocketAddrV4>,
    ap_addr6: Option<SocketAddrV6>,
    stats: Option<IwStats>,
    maxbitrate: Option<i32>,
    name: String,
    freq: Option<f64>,
    key: Option<WirelessKey<'a>>,
    essid: Option<String>,
    mode: Option<WirelessMode>,
    encryption: String
}

#[derive(Copy, Clone)]
#[repr(C)]
struct priv_iw_quality {
    qual: uint8_t,
    level: uint8_t,
    noise: uint8_t,
    updated: uint8_t,
}

impl Default for priv_iw_quality {
    fn default() -> priv_iw_quality {
        priv_iw_quality {
            qual: 0,
            level: 0,
            noise: 0,
            updated: 0
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct priv_iw_freq {
    m: int32_t,
    e: int16_t,
    i: uint8_t,
    flags: uint8_t,
}

impl Default for priv_iw_freq {
    fn default() -> priv_iw_freq {
        priv_iw_freq {
            m: 0,
            e: 0,
            i: 0,
            flags: 0
        }
    }
}

#[repr(C)]
struct iw_range {
    /* Informative stuff (to choose between different interface) */
    throughput: uint32_t, /* To give an idea... */
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
    event_capa: [uint32_t; 6],

    /* signal level threshold range */
    sensitivity: int32_t,

    /* Quality of link & SNR stuff */
    /* Quality range (link, level, noise)
     * If the quality is absolute, it will be in the range [0 , max_qual],
     * if the quality is dBm, it will be in the range [max_qual , 0].
     * Don't forget that we use 8 bit arithmetics... */
    max_qual: priv_iw_quality, /* Quality of the link */
    /* This should contain the average/typical values of the quality
     * indicator. This should be the threshold between a "good" and
     * a "bad" link (example : monitor going from green to orange).
     * Currently, user space apps like quality monitors don't have any
     * way to calibrate the measurement. With this, they can split
     * the range between 0 and max_qual in different quality level
     * (using a geometric subdivision centered on the average).
     * I expect that people doing the user space apps will feedback
     * us on which value we need to put in each driver... */
    avg_qual: priv_iw_quality, /* Quality of the link */

    /* Rates */
    num_bitrates: uint8_t, /* Number of entries in the list */
    bitrate: [int32_t; IW_MAX_BITRATES], /* list, in bps */

    /* RTS threshold */
    min_rts: int32_t, /* Minimal RTS threshold */
    max_rts: int32_t, /* Maximal RTS threshold */

    /* Frag threshold */
    min_frag: int32_t, /* Minimal frag threshold */
    max_frag: int32_t, /* Maximal frag threshold */

    /* Power Management duration & timeout */
    min_pmp: int32_t, /* Minimal PM period */
    max_pmp: int32_t, /* Maximal PM period */
    min_pmt: int32_t, /* Minimal PM timeout */
    max_pmt: int32_t, /* Maximal PM timeout */
    pmp_flags: uint16_t, /* How to decode max/min PM period */
    pmt_flags: uint16_t, /* How to decode max/min PM timeout */
    pm_capa: uint16_t, /* What PM options are supported */

    /* Encoder stuff */
    encoding_size: [uint16_t; IW_MAX_ENCODING_SIZES], /* Different token sizes */
    num_encoding_sizes: uint8_t, /* Number of entry in the list */
    max_encoding_tokens: uint8_t, /* Max number of tokens */
    /* For drivers that need a "login/passwd" form */
    encoding_login_index: uint8_t, /* token index for login token */

    /* Transmit power */
    txpower_capa: uint16_t, /* What options are supported */
    num_txpower: uint8_t, /* Number of entries in the list */
    txpower: [int32_t; IW_MAX_TXPOWER], /* list, in bps */

    /* Wireless Extension version info */
    we_version_compiled: uint8_t, /* Must be WIRELESS_EXT */
    we_version_source: uint8_t, /* Last update of source */

    /* Retry limits and lifetime */
    retry_capa: uint16_t, /* What retry options are supported */
    retry_flags: uint16_t, /* How to decode max/min retry limit */
    r_time_flags: uint16_t, /* How to decode max/min retry life */
    min_retry: int32_t, /* Minimal number of retries */
    max_retry: int32_t, /* Maximal number of retries */
    min_r_time: int32_t, /* Minimal retry lifetime */
    max_r_time: int32_t, /* Maximal retry lifetime */

    /* Frequency */
    num_channels: uint16_t, /* Number of channels [0, num - 1] */
    num_frequency: uint8_t, /* Number of entry in the list */
    freq: [priv_iw_freq; IW_MAX_FREQUENCIES], /* list */
    /* Note : this frequency list doesn't need to fit channel numbers,
     * because each entry contain its channel index */
    enc_capa: uint32_t, /* IW_ENC_CAPA_* bit field */

    /* More power management stuff */
    min_pms: int32_t, /* Minimal PM saving */
    max_pms: int32_t, /* Maximal PM saving */
    pms_flags: int16_t, /* How to decode max/min PM saving */

    /* All available modulations for driver (hw may support less) */
    modul_capa: uint32_t, /* IW_MODUL_* bit field */

    /* More bitrate stuff */
    bitrate_capa: uint32_t, /* Types of bitrates supported */
}

impl Default for iw_range {
    fn default() -> iw_range {
        iw_range {
            throughput: 0,
            min_nwid: 0,
            max_nwid: 0,
            old_num_channels: 0,
            old_num_frequency: 0,
            event_capa: [0; 6u],
            sensitivity: 0,
            max_qual: Default::default(), /* Quality of the link */
            avg_qual: Default::default(), /* Quality of the link */
            num_bitrates: 0, /* Number of entries in the list */
            bitrate: [0; IW_MAX_BITRATES], /* list, in bps */
            min_rts: 0, /* Minimal RTS threshold */
            max_rts: 0, /* Maximal RTS threshold */
            min_frag: 0, /* Minimal frag threshold */
            max_frag: 0, /* Maximal frag threshold */
            min_pmp: 0, /* Minimal PM period */
            max_pmp: 0, /* Maximal PM period */
            min_pmt: 0, /* Minimal PM timeout */
            max_pmt: 0, /* Maximal PM timeout */
            pmp_flags: 0, /* How to decode max/min PM period */
            pmt_flags: 0, /* How to decode max/min PM timeout */
            pm_capa: 0, /* What PM options are supported */
            encoding_size: [0; IW_MAX_ENCODING_SIZES], /* Different token sizes */
            num_encoding_sizes: 0, /* Number of entry in the list */
            max_encoding_tokens: 0, /* Max number of tokens */
            encoding_login_index: 0, /* token index for login token */
            txpower_capa: 0, /* What options are supported */
            num_txpower: 0, /* Number of entries in the list */
            txpower: [0; IW_MAX_TXPOWER], /* list, in bps */
            we_version_compiled: 0, /* Must be WIRELESS_EXT */
            we_version_source: 0, /* Last update of source */
            retry_capa: 0, /* What retry options are supported */
            retry_flags: 0, /* How to decode max/min retry limit */
            r_time_flags: 0, /* How to decode max/min retry life */
            min_retry: 0, /* Minimal number of retries */
            max_retry: 0, /* Maximal number of retries */
            min_r_time: 0, /* Minimal retry lifetime */
            max_r_time: 0, /* Maximal retry lifetime */
            num_channels: 0, /* Number of channels [0, num - 1] */
            num_frequency: 0, /* Number of entry in the list */
            freq: [Default::default(); IW_MAX_FREQUENCIES], /* list */
            enc_capa: 0, /* IW_ENC_CAPA_* bit field */
            min_pms: 0, /* Minimal PM saving */
            max_pms: 0, /* Maximal PM saving */
            pms_flags: 0, /* How to decode max/min PM saving */
            modul_capa: 0, /* IW_MODUL_* bit field */
            bitrate_capa: 0, /* Types of bitrates supported */
        }
    }
}

#[link(name="iwlib")]
extern {
    fn iw_socket_open() -> c_int;
    fn iw_get_range_info(socket: c_int,
                         interface: CString,
                         range: &iw_range) -> c_int;
    fn iw_scan(socket: c_int,
               interface: CString,
               version: c_int,
               head: &wireless_scan_head) -> c_int;
}

/// The WifiScan struct is the base object for the dradis library.
/// This struct runs the scan when created and consists of an array of available networks.
pub struct WifiScan<'a> {
    networks: Vec<WirelessNetwork<'a>>,
}

impl<'a> WifiScan<'a> {
    /// Run a scan of the local wifi networks and return a Result with either
    /// a WifiScan instance that contains a `Vec<WirelessNetwork>` called `networks`.
    /// `interface` is a `String` containing the name of the wireless interface to be scanned.
    ///
    /// ```
    /// use dradis::WifiScan;
    ///
    /// let local_networks = match WifiScan::scan("wlan0".to_string()) {
    ///     Ok(scan) => scan,
    ///     Err(err) => panic!("Failed to scan for wireless networks: {}", err)
    /// }
    ///
    /// local_networks.networks.map(|network| {
    ///                             println!("{}: Encryption: {}", network.essid,
    ///                             network.encryption); });
    /// ```
    ///
    pub fn scan(interface: String) -> Result<WifiScan<'a>, Error> {
        // Scan things here
        let mut list = Vec::new();
        // First get an iw socket.
        let sock = iw_socket_open();
        let interface_name = CString::new(interface).unwrap(); // TODO: Make the interface name configurable.
        let range: iw_range;
        let head: wireless_scan_head;
        if unsafe {iw_get_range_info(sock, interface_name, &range) < 0 } {
            // We have to make this call in order to get the version of the library on the computer
            Err(Error::new(ErrorKind::InvalidData, "Got an error from the iw library"))
        }
        if unsafe {iw_scan(sock, interface_name, range.we_version_compiled as c_int, &head) <0 } {
            // This is the actual scan call that fills in the `head` struct with information about the visible networks.
            Error::new(ErrorKind::InvalidData, "Got an error from the iw library")
        }
        let result = head.result;
        while result != ptr::null {
            // The scan results are a linked list of structs with a bunch of information about each network
            // The type of encryption is encoded in a bitflag called `key_flags` which we check by doing
            // a bitwise and against the known bitflags.
            let answer = if result.b.key_flags & IW_AUTH_WPA_VERSION_DISABLED > 0 {
                "None".to_string()
            } else if result.b.key_flags & IW_AUTH_WPA_VERSION_WPA > 0 {
                "WPA".to_string()
            } else if result.b.key_flags & IW_AUTH_WPA_VERSION_WPA2 > 0 {
                "WPA2".to_string()
            } else {
                "Error".to_string()
            };
            list.push(WirelessNetwork {
                ap_addr4: None,
                ap_addr6: None,
                maxbitrate: None,
                name: result.b.essid.to_string(),
                freq: None,
                key: None,
                mode: None,
                essid: result.b.essid.to_string(),
                encryption: answer, // TODO Figure out how to get encryption type from `result`
                stats: result.stats
            });
            result = result.next;
        }
        Ok( WifiScan { networks: list })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
