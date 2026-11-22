Host-side test drivers
======================

Scripts that run on the host against the bench programs in
`examples/pic32ck-curiosity-ultra` (and, where they say so, the SG00 and
PL10 boards' examples). Python 3; the serial ones need pyserial
(`uvx --with pyserial python vcom.py ...`, or `pip install pyserial`).

Over the network (HOST is a board's address or mDNS name, CERT the bench
certificate in `examples/pic32ck-curiosity-ultra/certs/cert.pem`; the TLS
scripts verify the board against it, so for them HOST is the mDNS name,
`pic32ck-` and the MAC's last three bytes in hex, `.local`):

- `eth_http_test.py HOST`: `eth_http`'s TCP echo and HTTP routes, rates.
- `eth_https_test.py HOST CERT`: `eth_https`'s TLS version, cipher and
  group, handshake times, routes, keep-alive and parallel requests.
- `eth_https_abuse.py HOST CERT`: awkward TLS clients against `eth_https`.
- `tls_records.sh HOST CERT PATH`: TLS records the server sends for one GET.
- `clock_check.py CERT HOST...`: each board's `/status` time against the
  host's clock (the SNTP check).
- `mdns_check.sh NAME...`: what macOS's mDNS sees for each board (`dns-sd`).
- `tcp_bench.py HOST [MEGABYTES]`: TCP echo/discard/chargen rates
  (`usbhs_ncm`).
- `udp_flood.py HOST SECONDS [SIZE]`: a UDP flood, to load a board's
  receive path.

Over serial (PORT is the device, e.g. `/dev/ttyACM0` or `/dev/cu.usbmodem…`):

- `uart_echo_test.py PORT BAUD`: `uart_echo` on the debugger's virtual COM
  port.
- `uart_dma_test.py PORT [BLOCKS] [BAUD]`: `uart_dma` on the SG00 or PL10
  board's virtual COM port, a 256-byte block at a time.
- `uart_ring_test.py PORT [BAUD] [BYTES] [SHARE]`: `uart_ring` on the SG00
  or PL10 board's virtual COM port: the overrun, then a stream echoed,
  paced to SHARE of the line rate if given (reset the board before each
  run).
- `can_echo.py INTERFACE SECONDS`: Linux, a raw CAN socket: the far end of
  a board's `can_ping`, sending each frame back with its ID's top bit set,
  classic and FD.
- `cdc_echo_test.py PORT`: a CDC-ACM echo (`usb_echo`).
- `cdc_hs_test.py PORT [MEGABYTES]`: every length to 1,100 bytes, then a
  stream each way (`usbhs_echo`).
- `vcom.py PORT SECONDS [BAUD]`: a timestamped capture of a board's log.
- `soak.py [--fs PORT] [--hs PORT] [--host HOST] [--hours H] [--log FILE]`:
  keeps `soak`'s USB ports and network busy, every byte checked, until
  stopped, with a summary a minute (needs pyserial).

Other:

- `usbmon_capture.sh [SECONDS] [OUT]`: Linux, as root: USB bus 1 by usbmon
  and tcpdump.
- `usbmon_pcap.py PCAP [DEVNUM]`: summarises such a capture by endpoint.
- `size_by_crate.py SYMS`: flash use by crate, from
  `llvm-nm -C --print-size` output.
