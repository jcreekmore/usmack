error_chain! {

    errors {
        InvalidLabel(t: String) {
            description("invalid label")
            display("invalid label: '{}'", t)
        }
    }
}
