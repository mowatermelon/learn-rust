// if not use ,it will failed to resolve. and see message,use of undeclared type or module `fmt`
// note that there are no double quotes between use and std
// not use::std::fmt;
// Also, be sure to add a semicolon at the end of the statement
use std::fmt;

// Define a structure named `List` containing a `Vec`.
// Note that the data type of the child variable
// is declared using an Angle, not parentheses
// not struct Water(Vec(i64));
struct Water(Vec<i64>);

impl fmt::Display for Water {
    // All objects except the first default private object
    // need to be declared before being used
    // like : variable name: variable value
    // example of the error : fmt(&self,f: fn &mut fmt::Formatter)

    // declare the return result type to use ::
    // example of the error :  -> fmt Result

    // note that there is a space before the curly braces are written,
    // otherwise you will get a compile error
    // note that you need to use the `&mut` here or you won't get the value of `f` in the current range
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing
        // and create a reference to `vec`.
        let vec = &self.0;
        // Implementing fmt::Display for a structure 
        // where the elements must each be handled sequentially is tricky. 
        // The problem is that each write! generates a `fmt::Result`. 
        // Proper handling of this requires dealing with all the results. 
        // Rust provides the ? operator for exactly this purpose.
        
        // Note that the write is followed by an `!` mark,
        // which represents the macro command being executed
        write!(f, "[")?;

        // Iterate over `vec` in `v` while enumerating the iteration

        // the name of the method used by the iterator is iter
        // example of the error : vec.itev()

        // the name of the method calling the enumeration is enumerate
        // example of the error : vec.iter().enum()
        for (index, item) in vec.iter().enumerate() {
            if index != 0 {
                write!(f, ",")?;
            }
            // mainly if you want to use {as an auxiliary symbol in formatting,
            // note that you need to write it twice,like `{{` and `}}`
            write!(f, "{{ index : {} ,value : {} }}", index, item)?;
        }

        // Note that the last statement does not end with a semicolon,
        // or a compilation error will occur

        // Notice also that the last write method ends without adding, okay?
        write!(f, "]")
    }
}

fn main() {
    // the structure initialization of an array object must be consistent with the type in the structure
    // example of the error : Water([1,2,3])

    // the struct initialization of an array object is done inside ()
    // example of the error : Water[vec![1,2,3]]

    // Note that the `vec` is followed by an `!` mark,
    // which represents the macro command being executed
    let temp_arr = Water(vec![1, 2, 3]);
    println!("{}", temp_arr);//[{ index : 0 ,value : 1 },{ index : 1 ,value : 2 },{ index : 2 ,value : 3 }]
}
