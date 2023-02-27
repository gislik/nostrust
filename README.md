# nostrust

This repo is an experimental code to play around with the nostr ecosystem.

Ideas I'm playing with include:

- nostr libraries
- relays
- micro apps

The initial goal is to be able to:

- [x] Parse and verify an event
- [x] Generate and sign an event
- [x] Construct messages

Now that the initial goal has been accomplished, the next task is to create a 
command line utility which leverages the parts that have been implemented:

- [x] Read an event as json from stdin and verify
- [x] Generate an event from cli arguments and write to stdout as json.


Next I think it would be cool to be able to manage nostr keys:

- [ ] Generate a new key and print to stdout
- [ ] Read the private key from an environment variable
