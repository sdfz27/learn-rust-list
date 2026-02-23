# Why We Need to Manually Implement `Drop` for `List`

## 1. What the default `Drop` does

If you don't implement `Drop`, Rust generates one that simply drops each field in order.

For your types that means:

- **`Drop` for `List`** → drop the field `head` (the `Link`).
- **`Drop` for `Link`** →
  - `Empty`: nothing to do.
  - `More(boxed_node)`: drop the `Box<Node>`.
- **`Drop` for `Box<Node>`** → drop the `Node` inside.
- **`Drop` for `Node`** → drop `elem` (trivial), then drop the field `next` (another `Link`).

So dropping a `List` becomes: drop `head` → drop first `Box<Node>` → drop first `Node` → drop its `next` (second `Link`) → drop second `Box<Node>` → drop second `Node` → drop its `next` → … one full chain of drops per node.

---

## 2. Why that causes a stack overflow

Each of those drops is a separate “step” (conceptually another layer of drop logic). So you get:

- `drop(list)`  
  - `drop(head)`  
    - `drop(Box<Node>)`  
      - `drop(Node)`  
        - `drop(next)`  
          - `drop(Box<Node>)`  
            - `drop(Node)`  
              - `drop(next)`  
                - …

So the **call stack** grows with the length of the list: one “frame” per node. For a long list (e.g. 100,000 nodes), that’s 100,000 frames and you get a **stack overflow**. The default `Drop` is effectively **recursive** over the list structure.

---

## 3. What the manual `Drop` does differently

The idea is: **don’t drop the list recursively; walk it in a loop and drop one node at a time**, so the stack depth stays constant.

### Step 1: Steal the list so we can iterate, not recurse

```rust
let mut cur_link = mem::replace(&mut self.head, Link::Empty);
```

- `self.head` is replaced with `Empty` (so when `List` is finally dropped, there’s nothing left to drop).
- The previous `head` (the whole chain) is moved into `cur_link`.
- From now on we only drop what we put in `cur_link`; we never let Rust “drop the list from the head” and recurse into `next`.

### Step 2: Loop — one node per iteration

```rust
while let Link::More(mut boxed_node) = cur_link {
    cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
}
```

- **Match:** We only care about `Link::More(boxed_node)`. For `Empty` we exit the loop (and `cur_link` is dropped normally; for `Empty` that’s trivial).
- **Steal `next`:**  
  `mem::replace(&mut boxed_node.next, Link::Empty)` moves the *rest of the list* out of the node into `cur_link`, and sets `boxed_node.next` to `Empty`.
- **Drop one node:** At the end of the loop iteration, `boxed_node` goes out of scope. Dropping it only drops:
  - `elem` (an `i32`),
  - `next` (now `Link::Empty`).
  So dropping this node does **not** drop another `Box<Node>`; no recursion.
- **Repeat:** On the next iteration, `cur_link` is the next `Link::More(...)`, and we do the same thing. So we only ever have one `Box<Node>` being dropped per iteration, and the “rest of list” lives in `cur_link` in the loop variable, not in the call stack.

### Step 3: Result

- Stack depth is **constant** (one loop, a few locals).
- No recursive `Drop` calls, so no stack overflow no matter how long the list is.

---

## 4. Summary

| Aspect | Default `Drop` | Manual `Drop` |
|--------|----------------|----------------|
| How it runs | Recursively: drop head → drop node → drop `next` (same again) | Loop: steal list into a variable, then repeatedly steal `next` and drop one node |
| Stack usage | One “frame” per node → stack overflow for long lists | Constant (one loop) → safe for any length |
| Why | Rust’s auto drop is “drop each field”; `Node`’s field is another `Link`/`Box<Node>`, so it’s recursive | You explicitly turn the recursive structure into a linear loop over `cur_link` and only drop one node at a time |

So you need to implement `Drop` yourself for this `List` so that dropping is done in a loop (constant stack) instead of recursively (stack proportional to list length). The comments in `first.rs` around the `Drop` impl (“no unbounded recursion”) are exactly about this: we’ve made the drop **iterative** instead of recursive.
