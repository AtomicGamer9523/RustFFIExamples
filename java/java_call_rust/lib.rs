// Importing the jni crate
use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv
};

#[utils::ffi(type = "system")]
pub fn Java_Main_greet<'a>(
    mut env: JNIEnv<'a>, _class: JClass<'a>, input: JString<'a>
) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env.get_string(&input).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(
        format!("Hello, {}!", input)
    ).expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_raw()
}
