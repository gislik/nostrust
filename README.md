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

- [ ] Read an event as JSON from STDIN and verify
- [ ] Generate an event from CLI arguments and write to STDOUT as JSON.



