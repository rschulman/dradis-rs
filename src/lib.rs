extern crate libc;

use std::net::{SocketAddrV4, SocketAddrV6};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::mem;
use std::io::{Error, ErrorKind};
use std::str;
use std::fmt;
use libc::*;

const IW_AUTH_WPA_VERSION_DISABLED: u8 = 0x00000001;
const IW_AUTH_WPA_VERSION_WPA: u8 = 0x00000002;
const IW_AUTH_WPA_VERSION_WPA2: u8 = 0x00000004;
const IW_MAX_BITRATES: usize = 32;
const IW_MAX_ENCODING_SIZES: usize = 8;
const IW_MAX_FREQUENCIES: usize = 32;
const IW_MAX_TXPOWER: usize = 8;
const IW_ESSID_MAX_SIZE: usize = 32;
const IW_ENCODING_TOKEN_MAX: usize = 64;
const IFNAMSIZ: usize = 16; // Defined in /include/uapi/linux/if.h but easier to just redefine here

pub enum WirelessMode {
    Auto, /* Let the driver decide */
    AdHoc, /* Single cell network */
    Infra, /* Multi cell network, roaming, ... */
    Master, /* Synchronisation master or Access Point */
    Repeat, /* Wireless Repeater (forwarder) */
    Second, /* Secondary master/repeater (backup) */
    Monitor, /* Passive monitor (listen only) */
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IwQuality {
    quality: u8,
    level: u8,
    noise: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IwStats {
    status: uint16_t,
    quality: IwQuality,
}

#[derive(Debug)]
#[repr(C)]
pub struct IwParam {
    value: int32_t, /* The value of the parameter itself */
    fixed: uint8_t, /* Hardware should not use auto select */
    disabled: uint8_t, /* Disable the feature */
    flags: uint16_t, /* Various specifc flags (if any) */
}

#[repr(C)]
pub struct WirelessKey<'a> {
    key: &'a [u8],
    size: u32,
    flags: i32,
}

/// The WirelessNetwork struct holds details about a single network,
/// including ssid, encryption type, bitrate, and signal strength.
#[repr(C)]
pub struct WirelessNetwork<'a> {
    pub ap_addr4: Option<SocketAddrV4>,
    pub ap_addr6: Option<SocketAddrV6>,
    pub stats: Option<IwStats>,
    pub maxbitrate: Option<i32>,
    pub freq: Option<f64>,
    pub key: Option<WirelessKey<'a>>,
    pub essid: Option<String>,
    pub mode: Option<WirelessMode>,
    pub encryption: String,
}

#[repr(C)]
struct WirelessScanHead {
    result: *const WirelessScan,
    retry: c_int,
}

#[repr(C)]
struct WirelessScan {
    next: *const WirelessScan,
    has_ap_addr: c_int,
    ap_addr: sockaddr,
    b: WirelessConfig,
    stats: IwStats,
    has_stats: c_int,
    maxbitrate: IwParam,
    has_maxbitrate: c_int,
}

impl fmt::Debug for WirelessScan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "WirelessScan {{next: {:?},
    has_ap_addr: {:?},
    ap_addr: sockaddr,
    b: \
                {:?},
    stats: {:?},
    has_stats: {:?},
    maxbitrate: {:?},
    \
                has_maxbitrate: {:?}
 }}",
               self.next,
               self.has_ap_addr,
               self.b,
               self.stats,
               self.has_stats,
               self.maxbitrate,
               self.has_maxbitrate)
    }
}

#[repr(C)]
struct WirelessConfig {
    name: [c_char; IFNAMSIZ + 1], /* Wireless/protocol name */
    has_nwid: c_int,
    nwid: IwParam, /* Network ID */
    has_freq: c_int,
    freq: f64, /* Frequency/channel */
    freq_flags: c_int,
    has_key: c_int,
    key: [c_uchar; IW_ENCODING_TOKEN_MAX], /* Encoding key used */
    key_size: c_int, /* Number of bytes */
    key_flags: c_int, /* Various flags */
    has_essid: c_int,
    essid_on: c_int,
    essid: [c_char; IW_ESSID_MAX_SIZE + 2], // ESSID (extended network)
    essid_len: c_int,
    has_mode: c_int,
    mode: c_int, /* Operation mode */
}

impl fmt::Debug for WirelessConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut better_essid = "[".to_string();
        for &byte in self.essid.iter() {
            better_essid.push_str(&format!("{:02X}, ", byte))
        }
        better_essid.push_str("]");
        let mut better_key = "[".to_string();
        for &byte in self.key.iter() {
            better_key.push_str(&format!("{:02X}, ", byte))
        }
        better_key.push_str("]");

        write!(f,
               "WirelessConfig {{name: {:?},
                has_nwid: {:?},
                \
                nwid: {:?},
                has_freq: {:?},
                freq: {:?},
                \
                freq_flags: {:?},
                has_key: {:?},
                key: {:?},
                \
                key_size: {:?},
                key_flags: {:?},
                has_essid: {:?},
                \
                essid_on: {:?},
                essid: {:?},
                essid_len: {:?},
                has_mode: {:?},
                \
                mode: {:?},
                }}",
               self.name,
               self.has_nwid,
               self.nwid,
               self.has_freq,
               self.freq,
               self.freq_flags,
               self.has_key,
               better_key,
               self.key_size,
               self.key_flags,
               self.has_essid,
               self.essid_on,
               better_essid,
               self.essid_len,
               self.has_mode,
               self.mode)
    }
}

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
            updated: 0,
        }
    }
}

#[derive(Clone, Copy)]
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
            flags: 0,
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
            event_capa: [0; 6],
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

#[link(name="iw")]
extern "C" {
    fn iw_sockets_open() -> c_int;
    fn iw_get_range_info(socket: c_int, interface: *mut c_char, range: &iw_range) -> c_int;
    fn iw_scan(socket: c_int,
               interface: *mut c_char,
               version: c_int,
               head: *mut WirelessScanHead)
               -> c_int;
}

/// The WifiScan struct is the base object for the dradis library.
/// This struct runs the scan when created and consists of an array of available networks.
pub struct WifiScan<'a> {
    pub networks: Vec<WirelessNetwork<'a>>,
}

impl<'a> WifiScan<'a> {
    /// Run a scan of the local wifi networks and return a Result with an error or
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
        let sock = unsafe { iw_sockets_open() };
        let interface_name = CString::new(interface).unwrap();
        let range: iw_range = Default::default();
        let head: *mut WirelessScanHead;
        unsafe {
            head = mem::uninitialized();
        }
        if unsafe { iw_get_range_info(sock, interface_name.as_ptr() as *mut c_char, &range) < 0 } {
            // We have to make this call in order to get the version of the library on the computer
            return Err(Error::new(ErrorKind::InvalidData, "Got an error from the iw library"));
        }
        if unsafe {
            iw_scan(sock,
                    interface_name.as_ptr() as *mut c_char,
                    range.we_version_compiled as c_int,
                    head) < 0
        } {
            // This is the actual scan call that fills in the `head` struct with information about the visible networks.
            return Err(Error::new(ErrorKind::InvalidData, "Got an error from the iw library"));
        }

        let mut result = unsafe { (*head).result };
        while !result.is_null() {
            // The scan results are a linked list of structs with a bunch of information about each network
            // The type of encryption is encoded in a bitflag called `key_flags` which we check by doing
            // a bitwise and against the known bitflags.
            unsafe {
                let answer =
                    if (*result).b.key_flags & IW_AUTH_WPA_VERSION_DISABLED as c_int > 0 {
                        "None".to_string()
                    } else if (*result).b.key_flags & IW_AUTH_WPA_VERSION_WPA as c_int > 0 {
                        "WPA".to_string()
                    } else if (*result).b.key_flags & IW_AUTH_WPA_VERSION_WPA2 as c_int > 0 {
                        "WPA2".to_string()
                    } else {
                        "Error".to_string()
                    };
                let network_name;
                if (*result).b.has_essid == 1 {
                    let u8slice: [u8; 34] = mem::transmute((*result).b.essid);
                    //let mut ssid_string = CStr::new("data");
                    let ssid_string =
                        match CStr::from_bytes_with_nul(u8slice.split_at(u8slice.into_iter()
                                .position(|&byte| byte == 0x0)
                                .unwrap() + 1)
                            .0) { 
                            Ok(good_string) => good_string,
                            Err(err) => panic!("Could not parse essid string: {}", err),
                        };
                    network_name = Some(ssid_string.to_string_lossy().into_owned());
                } else {
                    network_name = None;
                }
                list.push(WirelessNetwork {
                    ap_addr4: None,
                    ap_addr6: None,
                    maxbitrate: None,
                    freq: None,
                    key: None,
                    mode: None,
                    essid: network_name,
                    encryption: answer,
                    stats: Some((*result).stats.clone()),
                });
                result = (*result).next;
            }
        }
        Ok(WifiScan { networks: list })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
