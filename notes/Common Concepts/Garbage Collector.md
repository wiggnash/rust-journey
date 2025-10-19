## Traditional Approaches to Memory Management

### C/C++ Approach: Manual Memory Management

- We **allocate memory** explicitly
- We must **manually free it** when done using it
- If we forget to free memory → it remains occupied forever
- This causes a **memory leak** ⚠️

**Real-life analogy:** Hosting a party with **no house help** — you need to clean everything after using it (like washing dishes immediately after each use).

---

### Python/JS Approach: Garbage Collection

- We **create objects** but never manually delete them
- Memory is cleaned automatically using a **Garbage Collector**

**Real-life analogy:** Hosting a party with **one house help** — this person constantly cleans whatever you're using in the background.

#### How Garbage Collectors Work

The garbage collector:

1. **Tracks** all objects present in memory
2. **Figures out** if someone is using them
3. **Frees memory** when nobody is using it

But _when_ does this happen? There are **two main approaches**:

---

### 1. Reference Counting (Python)

**Basic idea:** Count the references to each object

- If there are **no references** → garbage collector can delete it
- Simple and deterministic

---

### 2. Tracing / Mark and Sweep (JS, Java, C#)

**Process:**

1. **Program pauses** when the garbage collector detects memory is about to be full
2. **Marking phase:** Mark all variables that have references
3. **Sweeping phase:** Delete all variables that don't have references

⚡ **Note:** This approach can cause pause times during program execution.