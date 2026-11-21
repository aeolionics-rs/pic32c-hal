# Bench certificate (TEST ONLY)

A self-signed ECDSA P-256 certificate for `eth_https` on the bench, with
both boards' mDNS names (`pic32ck-` and the MAC's last three bytes) in its
SAN, so clients reach the boards by name. The private key is committed on
purpose: it protects nothing. Never use it outside the bench. For other
boards, reissue it with their names and the same key, as below.

Made with:

    openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes -days 3650 \
      -subj "/CN=curiosity-pic32ck bench (test only)" \
      -addext "subjectAltName=DNS:pic32ck-624743.local,DNS:pic32ck-879f1c.local" \
      -addext "basicConstraints=critical,CA:FALSE" -addext "extendedKeyUsage=serverAuth" \
      -keyout key.pem -out cert.pem
    openssl x509 -in cert.pem -outform der -out cert.der
    openssl pkcs8 -topk8 -nocrypt -in key.pem -outform der -out key.der

Reissued with the same key (`openssl pkey -inform der -in key.der -out
key.pem`, then the first command with `-key key.pem` in place of `-newkey
...` and `-keyout`).
