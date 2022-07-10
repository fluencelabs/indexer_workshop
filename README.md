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
