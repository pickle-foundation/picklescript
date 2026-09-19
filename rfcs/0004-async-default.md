# 0004 - Async Default

- Status: draft (Milestone 7)
- Related: `docs/07-runtime.md`

`async fn` lowers to state machines; `task { }` spawns OS threads (or pool
coroutines); channels are rendezvous buffers; `parallel { }` uses a
work-stealing pool; the runtime drains the task pool on exit.