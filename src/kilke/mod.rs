pub fn run() {
    // This macro removes this inline module when Rust 
    // is not in test mode.
    //#[cfg(test)]
    //mod tests {
        // Notice that we don't immediately get access to the 
        // parent module. We must be explicit.
        //use super::*;
        //... tests ...
    //}
    println!("This is Kilke");
}