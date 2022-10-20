+++
title = "MIT SECURE on iwd"
+++

I use `iwd` on certain machines if I am using `systemd-networkd`. However,
there are some troubles when I am trying to connect to MIT SECURE. Before
`iwd`, I could connect to MIT SECURE by just providing my kerberos and
password. However, using `iwd` I had to learn more about WPA Enterprise
connections.

### EAP-PEAP

MIT SECURE uses EAP-PEAP which means you get a password protected access point
like EAP-PWD, but you also use a public key server certificate to establish a
secure connection when authenticating. The servers that are authenticating
users are called RADIUS (Remote Authentication Dial-In User Service) servers.
When using EAP-PEAP, you can provide a CA certificate. My previous network
managers did not require them, so I just left them blank. When using `iwd`, I
have to provide a CA Certificate (I think?). The CA certificate is useful
because before the authentication finishes, we don't know anything about the
network. The access point is just proxying requests to the RADIUS server and
the CA certificate helps validate the identity of the RADIUS server.

### So what's the config?

MIT provides the RADIUS servers and how you can verify them
[here](https://kb.mit.edu/confluence/display/istcontrib/MIT+SECURE+Wireless+Certificate+Fingerprints).
I had to look up the CA Certificate [AAA Certificate
Services](https://www.tbs-certificates.co.uk/FAQ/en/Comodo_AAA_Certificate_Services.html)
using the powers of the internet. The wiki provided fingerprints to verify if I
am using the correct certificate. My iwd configuration file looks like this:

```
# in "/var/lib/iwd/MIT SECURE.8021x"

[Security]
EAP-Method=PEAP
EAP-PEAP-CACert=embed:cert
EAP-PEAP-ServerDomainMask=oc11-radius-wireless-1.mit.edu;w92-radius-wireless-1.mit.edu
EAP-PEAP-Phase2-Method=MSCHAPV2
EAP-PEAP-Phase2-Identity=REDACTED
EAP-PEAP-Phase2-Password=REDACTED

[@pem@cert]
-----BEGIN CERTIFICATE-----
MIIEMjCCAxqgAwIBAgIBATANBgkqhkiG9w0BAQUFADB7MQswCQYDVQQGEwJHQjEb
MBkGA1UECAwSR3JlYXRlciBNYW5jaGVzdGVyMRAwDgYDVQQHDAdTYWxmb3JkMRow
GAYDVQQKDBFDb21vZG8gQ0EgTGltaXRlZDEhMB8GA1UEAwwYQUFBIENlcnRpZmlj
YXRlIFNlcnZpY2VzMB4XDTA0MDEwMTAwMDAwMFoXDTI4MTIzMTIzNTk1OVowezEL
MAkGA1UEBhMCR0IxGzAZBgNVBAgMEkdyZWF0ZXIgTWFuY2hlc3RlcjEQMA4GA1UE
BwwHU2FsZm9yZDEaMBgGA1UECgwRQ29tb2RvIENBIExpbWl0ZWQxITAfBgNVBAMM
GEFBQSBDZXJ0aWZpY2F0ZSBTZXJ2aWNlczCCASIwDQYJKoZIhvcNAQEBBQADggEP
ADCCAQoCggEBAL5AnfRu4ep2hxxNRUSOvkbIgwadwSr+GB+O5AL686tdUIoWMQua
BtDFcCLNSS1UY8y2bmhGC1Pqy0wkwLxyTurxFa70VJoSCsN6sjNg4tqJVfMiWPPe
3M/vg4aijJRPn2jymJBGhCfHdr/jzDUsi14HZGWCwEiwqJH5YZ92IFCokcdmtet4
YgNW8IoaE+oxox6gmf049vYnMlhvB/VruPsUK6+3qszWY19zjNoFmag4qMsXeDZR
rOme9Hg6jc8P2ULimAyrL58OAd7vn5lJ8S3frHRNG5i1R8XlKdH5kBjHYpy+g8cm
ez6KJcfA3Z3mNWgQIJ2P2N7Sw4ScDV7oL8kCAwEAAaOBwDCBvTAdBgNVHQ4EFgQU
oBEKIz6W8Qfs4q8p74Klf9AwpLQwDgYDVR0PAQH/BAQDAgEGMA8GA1UdEwEB/wQF
MAMBAf8wewYDVR0fBHQwcjA4oDagNIYyaHR0cDovL2NybC5jb21vZG9jYS5jb20v
QUFBQ2VydGlmaWNhdGVTZXJ2aWNlcy5jcmwwNqA0oDKGMGh0dHA6Ly9jcmwuY29t
b2RvLm5ldC9BQUFDZXJ0aWZpY2F0ZVNlcnZpY2VzLmNybDANBgkqhkiG9w0BAQUF
AAOCAQEACFb8AvCb6P+k+tZ7xkSAzk/ExfYAWMymtrwUSWgEdujm7l3sAg9g1o1Q
GE8mTgHj5rCl7r+8dFRBv/38ErjHT1r0iWAFf2C3BUrz9vHCv8S5dIa2LX1rzNLz
Rt0vxuBqw8M0Ayx9lt1awg6nCpnBBYurDC/zXDrPbDdVCYfeU0BsWO/8tqtlbgT2
G9w84FoVxp7Z8VlIMCFlA2zs6SFz7JsDoeA3raAVGI/6ugLOpyypEBMs1OUIJqsi
l2D4kF501KKaU73yqWjgom7C12yxow+ev+to51byrvLjKzg6CYG1a4XXvi3tPxq3
smPi9WIsgtRqAEFQ8TmDn5XpNpaYbg==
-----END CERTIFICATE-----
```
