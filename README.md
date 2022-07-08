## Where to start
Read the [main.aqua](src/aqua/main.aqua)

## How to run
### Create subnet
```
 aqua run --verbose --timeout 60000 --log-level "aquavm=off" -i src/aqua/ -f 'make_subnet_scenario()' --addr stage-02 --sk "I8Sc2Nrd3qk0+l9yeuVPGv54cfOBFCKx9Ch0dOZ1MQ4=" --plugin src/plugins
``

### Upload to subnet and retrieve index
```
aqua run --timeout 30000 --log-level "aquavm=off"  -i src/aqua -f 'upload_to_subnet("/Users/folex/package-lock.json")' --addr stage-02 --sk "I8Sc2Nrd3qk0+l9yeuVPGv54cfOBFCKx9Ch0dOZ1MQ4=" --plugin src/plugins
```