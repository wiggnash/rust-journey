**REPL** stands for **Read-Eval-Print Loop**. 

It is an interactive programming environment that continuously processes user inputs, evaluates them, and displays results.

### **How REPL Works**
A REPL follows this cycle:

1. **Read** – Takes user input (command or code).
2. **Eval** – Evaluates (executes) the input.
3. **Print** – Displays the output/result.
4. **Loop** – Repeats the process until the user exits

### **How This Relates to Your Shell in Rust**

Since you're building a shell, you are essentially creating a **REPL-based system** where:

- **Read:** Accept user commands from standard input (`stdin`).
- **Eval:** Parse and execute the command.
- **Print:** Display the command’s output or error message.
- **Loop:** Repeat the process until the user exits (`exit` or `Ctrl+D`).