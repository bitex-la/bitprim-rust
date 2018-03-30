use exit_code::ExitCode;

error_chain!{
    errors {
        ErrorExitCode(t: ExitCode) {
            description("Exit code was non zero")
            display("Exit code '{:?}'", t)
        }
    }
}
