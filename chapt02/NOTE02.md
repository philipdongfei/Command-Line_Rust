# Test for Echo

## How echo Works

## Getting Started

The **unit type** is like an empty value and is signified with a set of empty parentheses: (). The documentation says this "is used when there is no other meaningful value that could be returned." It's not quite like a null pointer or undefined value in other languages, a concept first introduced by Tony Hoare (no relation to Rust creator Graydon Hoare), who called the null reference his "billion-dollar mistake." Since Rust does not (normally) allow you to dereference a null
pointer, it must logically be worth at least a billion dollars.


### Accessing the Command-Line Arguments

### Adding clap as a Dependency

### Parsing Command-Line Arguments Using clap

### Creating the Program Output

## Writing Integration Tests



### Creating the Test Output Files


### Comparing Program Output

### Using the Result Type

I should be more cautious, so I'm going to create a *type alias* called **TestResult**. This will be a specific type of **Result** that is either an **Ok** that always contains the unit type or some value that implements the **std::error::Error trait**:
    
    type TestResult = Result<(), Box<dyn std::error::Error>>;

In the preceding code, **Box** indicates that the error will live inside a kind of pointer where the memory is dynamically allocated on the heap rather than the stack, and **dyn** indicates that the method calls on the **std::error::Error** trait are dynamically dispatched. That's really a lot of information, and I don't blame you if your eyes glazed over. In short, I'm saying that the **Ok** part of **TestResult** will only ever hold the unit type, and the **Err** part can hold
anything that implements the **std::error::Error** trait.

>#### Stack and Heap Memory
>First there is the *stack*, where items of known sizes are accessed in a particular order. The calssic analogy is to a stack of cafeteria trays where new items go on top and are taken back off the top in *last-in, first-out* (LIFO) order. Items on the stack have fixed, known size, making it possible for Rust to set aside a particular chunk of memory and find it quickly.
>
>The other type of memory is the *heap*, where the sizes of the values may change over time. For instance, the documentation for the **Vec\(vector\) type** describes this structure as a "contiguous growable array type." *Growable* is the key word here, as the number and sizes of the elements in a vector can change during the lifetime of the program. Rust makes an initial estimation of the amount of memory it needs for the vector. If the vector grows beyond the original allocation, Rust
>will find another chunk of memory to hold the data. To find the memory where the data lives, Rust stores the memory address on the stack. This called a *pointer** because it points to the actual data, and so is also said to be a *reference* to the data. Rust knows how to *deference* a **Box** to find the data.
>



## Summary
