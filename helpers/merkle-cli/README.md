# Merkle CLI

To generate merkle root:
```bash
merkle-cli generate-root testdata/uosmo_only.csv
```

To generate merkle proof:
```bash
merkle-cli generate-proof testdata/uosmo_only.csv osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo --print
```

or

```bash
merkle-cli generate-proof testdata/uosmo_only.csv osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo testdata/proof_data.json
```

To verify merkle proof:
```bash
merkle-cli verify-proof Nz54SQtyBVHwsmEqNI//mxFgiq8MRD7sS92IGkhgMvo= osmo1003cay8wpc456n3adq785xn0r0pqvmfxlakpxh9442uosmo testdata/proof_data.json
```
