# nostrust

This repo is an experimental code to play around with the nostr ecosystem.

Ideas I'm playing with include:

- nostr libraries
- relays
- aggregators
- micro apps

Library:

- [x] Parse and verify an event 
- [x] Generate and sign an event
- [x] Construct messages requests
- [x] Parse message responses
- [x] Direct message support 
- [x] Seed phrases

CLI: 

- [x] Read an event as json from stdin and verify
- [x] Generate an event from cli arguments and write to stdout as json.
- [x] Generate message requests
- [x] Generate a new key and print to stdout
- [x] Read the private key from an environment variable
- [x] Derive the private key from a mnemonic, read from an environment variable

NIPS:

- [NIP-01: Basic protocol flow description](https://github.com/nostr-protocol/nips/blob/master/01.md)
- [NIP-02: Contact List and Petnames](https://github.com/nostr-protocol/nips/blob/master/02.md)
- [NIP-04: Encrypted Direct Message](https://github.com/nostr-protocol/nips/blob/master/04.md)
- [NIP-06: Basic key derivation from mnemonic seed phrase](https://github.com/nostr-protocol/nips/blob/master/06.md)
- [NIP-19: bech32-encoded entities](https://github.com/nostr-protocol/nips/blob/master/19.md)
