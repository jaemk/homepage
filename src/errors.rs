use std::io;
use log;
use tera;

error_chain! {
    foreign_links {
        LogInit(log::SetLoggerError);
        Template(tera::Error);
        FileOpen(io::Error);
    }
    errors {
        DoesNotExist(s: String) {
            description("Query result does not exist")
            display("DoesNotExist Error: {}", s)
        }
        BadRequest(s: String) {
            description("Bad request")
            display("BadRequest: {}", s)
        }
    }
}

