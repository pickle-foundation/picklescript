# 0003 - Memory Model

- Status: accepted
- Source of truth: `docs/05-memory.md`

GC by default (non-moving, stop-the-world, mark-and-sweep; shadow stack;
size classes), value vs reference semantics, `unsafe` pointers/references,
custom allocators and arenas, and the collector's C ABI.