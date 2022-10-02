# Merkle CLI

## Addresses and Coins

### Generate Root
```bash
merkle-cli generate-root testdata/uosmo_only.csv
```

### Generate Proof
```bash
merkle-cli generate-proof testdata/uosmo_only.csv osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo --print
```

or

```bash
merkle-cli generate-proof testdata/uosmo_only.csv osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo testdata/proof_data.json
```

### Verify Proof
```bash
merkle-cli verify-proof Nz54SQtyBVHwsmEqNI//mxFgiq8MRD7sS92IGkhgMvo= osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo testdata/proof_data.json
```

## Addresses Only

### Generate Root

```bash
merkle-cli generate-root testdata/addresses_only.csv
```

Expected result:
```
b/srjqiN+80ZroBeeCcSooksHNzxYJ/h4tMrAF9zZg8=
```

### Generate Proof

```bash
merkle-cli generate-proof testdata/addresses_only.csv osmo1hqslwuc8ukaaaxfmahgnquyqx3w0tmrluwxmxj testdata/proof_data_addresses_only.json
```

### Verify Proof

```bash
merkle-cli verify-proof b/srjqiN+80ZroBeeCcSooksHNzxYJ/h4tMrAF9zZg8= osmo1hqslwuc8ukaaaxfmahgnquyqx3w0tmrluwxmxj testdata/proof_data_addresses_only.json
```
