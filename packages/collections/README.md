# Collections (SA-3-05 deferred)

Rename map targets `import Collections` merging OrderedSet + Deque.

**Deferred:** concatenated module trips `S023` class-shadows-builtin for `OrderedSet`/`Deque`
homonyms. Keep `import OrderedSet` / `import Deque` until SA-4/SA-5 clears shadow rules.
