# The Stack and Heap Memory

Whether a value is on the **stack** or **heap** affects how the language behaves and why we need to make decisions on where to assign what.

**Stack and Heap** are parts of memory available to your code to use at runtime.

---

## Stack Memory

### Characteristics

- Stores values **in order** and removes them in the **opposite order**
- Follows **LIFO** (Last In First Out) principle
- **Real-life analogy:** Stack of plates 🍽️

### Operations

- **Adding data** = Pushing onto the stack
- **Removing data** = Popping off the stack

### Requirements

All data stored in the stack must:

- Be **known at compile time**
- Have a **fixed size**

---

## Heap Memory

### When to Use Heap

If the data size is **not known at compile time**, store that value in the **heap**.

### Characteristics

- **Less organized** than the stack
- Requires allocation process

### How Heap Allocation Works

1. **Request space** for data
2. **Memory allocator** finds an empty spot in the heap that is big enough
3. Returns a **pointer** (address of the location)
4. This pointer can be stored in the **stack**

This process is called **"allocating on the heap"** or simply **"allocating"**.

⚡ **Note:** Pushing values to the stack is NOT called allocating — that's different!

### Real-life Analogy: Restaurant 🍽️

- You enter and mention how many people are in your group
- The host finds an empty table that fits everyone
- If someone comes late, they can ask where you're seated and find you

---

## Performance Comparison

### Adding New Elements

**Stack is faster** ✅

- No need to search for a place to store new data
- New data always goes to the **top of the stack**

**Heap is slower** ⚠️

- Memory allocator must find an empty spot
- More complex allocation process

---

### Accessing Data

**Stack is faster** ✅

- Direct access to data

**Heap is slower** ⚠️

- Need to **follow a pointer** to get the data
- Indirect access adds overhead

---

## Function Calls and Memory

When we call a function:

1. Values passed into the function are **pushed onto the stack**
2. When the function is over, those values **get popped off the stack**