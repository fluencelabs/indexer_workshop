import { Multiaddr, protocols } from 'multiaddr';
import { create, globSource, CID } from 'ipfs-http-client';
import { existsSync, readFileSync } from 'fs';

async function client(multiaddr) {
    let rpcMaddr = new Multiaddr(multiaddr).decapsulateCode(protocols.names.p2p.code);
    const ipfs = create(rpcMaddr);
    return ipfs;
}

const index = new Map();
const alive = new Set();

export async function plugins() {
    return {
        ipfs_client: {
            upload: async (multiaddr, path) => {
                if (!existsSync(path)) { throw `file ${path} doesn't exist` };
                const data = readFileSync(path);
                const ipfs = await client(multiaddr);
                const result = await ipfs.add(data, { pin: true });
                const cid = result.path;
                await ipfs.pin.add(cid);
                console.log(`did pin ${cid} to ${multiaddr}`)

                try {
                    const pinned = await ipfs.pin.ls({ paths: cid, type: 'all' });
                    for await (let r of pinned) {
                        if (r.type === 'recursive') {
                            console.log(`file ${cid} pinned to ${multiaddr}`);
                        } else {
                            console.log("pin result is strange", r);
                        }
                    };
                } catch (err) {
                    console.log(`file ${cid} failed to pin to ${multiaddr}`, err);
                }

                return cid;
            },
            id: async (multiaddr) => {
                const ipfs = await client(multiaddr);
                const result = await ipfs.id();
                return result;
            },
            exists: async (multiaddr, cid) => {
                const ipfs = await client(multiaddr);
                let exists;
                try {
                    const result = await ipfs.pin.ls({ paths: cid, type: 'all' });
                    for await (let _ of result) { };
                    exists = true
                } catch (err) {
                    if (err.toString().includes(`is not pinned`)) {
                        exists = false
                    } else {
                        throw err;
                    }
                }
                // console.dir(result);
                // // await result.then(wtf => console.dir(wtf)).catch(err => console.dir(err));
                // let exists;
                // try {
                //     console.log("exists result", result);
                //     for await (let r of result) {
                //         console.log("exists", r);
                //     };
                //     exists = true
                // } catch (err) {
                //     if (err.name && err.name == 'TimeoutError') {
                //         // treat TimeoutError as "not exists"
                //         exists = false;
                //     } else {
                //         // throw all other errors
                //         throw err;
                //     }
                // }
                // return exists;
                return exists;
            },
            remove: async (multiaddr, cid) => {
                const ipfs = await client(multiaddr);
                try {
                    await ipfs.pin.rm(cid, { recursive: true });
                } catch (_) { }

                try {
                    const rm = await ipfs.block.rm(cid, { force: true });
                    for await (let r of rm) {
                        if (r.error) { console.log("block rm failed", r.error) }
                    }
                    return rm;

                } catch (err) {
                    console.log("remove failed", err)
                }
            }
        },
        index: {
            add: (cid, peer_id, multiaddr) => {
                const providers = index.get(cid) ?? new Set();
                providers.add({ peer_id, multiaddr });
                index.set(cid, providers);
            },
            get: (cid) => {
                const set = index.get(cid) ?? new Set();
                return Array.from(set);
            },
            remove: (cid, peer_id, multiaddr) => {
                const set = index.get(cid);
                if (set) {
                    let deleted;
                    for (let e of set) {
                        if (e.peer_id === peer_id && e.multiaddr === multiaddr) {
                            deleted = set.delete(e);
                            break;
                        }
                    }
                    if (deleted) {
                        console.log("removed from index", peer_id);
                        index.set(cid, set);
                    }
                }
            }
        },
        alive: {
            add: (peer_id, multiaddr) => {
                alive.add({ peer_id, multiaddr });
            },
            remove: (peer_id, multiaddr) => {
                alive.delete({ peer_id, multiaddr })
            },
            list: () => { return Array.from(alive) },
            get: (peer_id) => {
                for (const p of alive) {
                    if (p.peer_id === peer_id) {
                        return [p]
                    }
                }
                return []
            }
        },
        log: {
            removal: (before, after, peer, res) => {
                const bef = before ? "was pinned" : "wasn't pinned";
                const aft = after ? "is pinned still" : "isn't pinned anymore";
                console.log(bef, aft, peer, res);
            }
        }
    };
}
