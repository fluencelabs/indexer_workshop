# Workshop
## Experimenting on IPFS with Aqua
### What's the value?
- Write, run and evolve distributed algorithms
- Quick experimentation
- Declarative-ish language, focus on algorithms, not nitty-gritty
- Iterative development: start REALLY simple, go sophisticated gradually
- Subnet-oriented thinking

### What's this workshop isn't about
- How to build production-ready decentralized IPFS indexer

### What's the goal?
- Seed ideas about a quite different approach to distributed systems
- Request for comments, problems, ideas and approaches: what would YOU do with Aqua?
- Have fun, melt brains a little bit

### Scenario
Imagine we're building an app, and want to use IPFS as a data plane.

- Take some peers from the p2p network, and command them to obey (they can't resist. yet.)
- Use these peers as: IPFS-powered data plane, distributed index, orchestrators
    - These could be all separate peers, but won't bother for the workshop
- Upload files, store location info to Distributed Index (tm), make nodes lose files, repair from index

### Tooling, moving parts
- `aqua run` - takes Aqua script, compiles it to AIR, sends to network. Acts as a short-lived JS peer.
- rust peers - running on cloud, each has go-ipfs as a sidecar
- `aqua-ipfs` - WASM service wraps IPFS cli and provides it as an API to Fluence network, hosted on all Fluence Rust Peers
- `ipfs_client.mjs` - JS module that provides ipfs-js peer as an API to Fluence network, works on my laptop
- indexer service written in rust, compiled to wasm, upload to ipfs and deployed to Fluence

SHOW: picture of topology layout

### Let's run it: make subnet, upload, get index, loose files, repair
SHOW: pictures of what happened on each step

### Let's review code: upload

### Let's review code: repair

### Let's review code: whos_absent

# On your own
## Where to start
Read the [main.aqua](src/aqua/main.aqua)

## How to run
### Create subnet
```
npm run subnet
```

### Upload to subnet
```
FILE="$(pwd)/package-lock.json" npm run upload
```

### Get index
Copy the CID from logs of `upload`, and paste it as `CID=`

```
CID=QmdQepGdyPro8GQsEUs4Ep8fZ1G1HsPNHAuFNRCEiC3ahY npm run get_index
```

### Unpin file from some nodes
```
ipfs --api "/ip4/134.209.186.43/tcp/5002/p2p/12D3KooWNWW5rX9QWSfsHJZVhuKyUb7rPKDvBNqyCuK21JmjyGPt" pin rm QmdQepGdyPro8GQsEUs4Ep8fZ1G1HsPNHAuFNRCEiC3ahY

ipfs --api "/ip4/134.209.186.43/tcp/5005/p2p/12D3KooWHtjVS2LSKpQsFFbNxPaLTAV8Si2LiohkK2CrQsyJymMX" pin rm QmdQepGdyPro8GQsEUs4Ep8fZ1G1HsPNHAuFNRCEiC3ahY
```

### Check absent
```
CID=QmdQepGdyPro8GQsEUs4Ep8fZ1G1HsPNHAuFNRCEiC3ahY npm run absent
```

### Run repair
```
CID=QmdQepGdyPro8GQsEUs4Ep8fZ1G1HsPNHAuFNRCEiC3ahY npm run repair
```

### Check absent again
```
CID=QmdQepGdyPro8GQsEUs4Ep8fZ1G1HsPNHAuFNRCEiC3ahY npm run absent
```
