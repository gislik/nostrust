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
- [ ] Read the private key from an environment variable
- [ ] Derive the private key from a mnemonic, read from an environment variable
