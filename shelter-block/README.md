# Shelter block

> Base unit of Shelter filesystem


## Objective

- Self descriptive block
- Future proof

We can change hash and cryptographic function used without making a major version.
If a vulnerability is discovered, it's easy to upgrade to new functions.


## Multicodec table

We use [standard multicodec code](https://github.com/multiformats/multicodec/blob/master/table.csv) when possible.

List of code convention used by this crate:

| name               | code | description   | status |
| ------------------ | ---- | ------------- | ------ |
| identity           | 0x00 | raw binary    | stable |
| blake3             | 0x1e | hash function | stable |
| ShelterSuperBlock  | 0x31 |               | custom |
| ShelterBlob        | 0x32 |               | custom |
| ShelterFile        | 0x33 |               | custom |
| ShelterTree        | 0x34 |               | custom |
| ShelterFileVersion | 0x35 |               | custom |
| XChaCha20Poly1305  | 0x37 | AEADs         | custom |
| AEZ                | 0x38 | AEADs         | custom |


## Status

Each multicodec code has a status:

* standard - these encodings should be implemented by all implementations and are widely used.
* custom -  these encodings are not standard and are only used by us


## License

...