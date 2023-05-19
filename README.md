# c_rust_go_lab_5

## Program 1 
Use threads to perform parallel computation.   Create a function that takes as
input a reference to a large array or vector.   Use threads to sum all the
values in this collection in parallel.  Make the number of threads spawned a
parameter.

Create a main function that tests your parallel sum function.  It might also be
helpful to create a function that returns a large array with random values for
testing.

## Program 2 
Create a version of bank.c from lab 3, but this time in Rust.  Make sure it is
thread safe.

## Program 3
For program 1, implement a generic binary search tree, similar to lab 3 (c).

It is not required that this tree be balanced.   That would make the code much more complicated. Note, only implement In Order Traversal, omit the post and pre order traversals.

Thus the tree would have a new, insert, in order traversal and search
methods.  If you want a challenge, implement the delete method.

Create a main function that tests our implementation of this binary search tree.
