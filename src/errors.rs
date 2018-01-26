use std::io;
use log;
use tera;
use serde_json;

error_chain! {
    foreign_links {
        LogInit(log::SetLoggerError);
        Template(tera::Error);
        FileOpen(io::Error);
        Json(serde_json::Error);
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

