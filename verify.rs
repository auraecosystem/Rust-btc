use my_crate::{function_under_test, meets_specification};

#[kani::proof]
fn check_my_property() {
   // Create a nondeterministic input
   let input: u8 = kani::any();

   // Call the function under verification
   let output = function_under_test(input);

   // Check that it meets the specification
   assert!(meets_specification(input, output));
}
