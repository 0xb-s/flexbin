# FlexBin

FlexBin is an experimental binary serialization protocol focusing on schema evolution, self-describing data, and optional compression. Encoding involves schema embedding and data serialization; decoding reverses this. Typical byte costs: 4 bytes for integers, 1 byte + string length for strings. Use for research only; not secure for production.
