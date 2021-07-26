use std::env;


#[cxx::bridge]
mod ffi {
    // Any shared structs, whose fields will be visible to both languages.
    // struct BlobMetadata {
    //     size: usize,
    //     tags: Vec<String>,
    // }

    extern "Rust" {
        // Zero or more opaque types which both languages can pass around but
        // only Rust can see the fields.
        // type MultiBuf;
        //
        // // Functions implemented in Rust.
        // fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    extern "C++" {
    // include!("ray/api.h");

    // Zero or more opaque types which both languages can pass around but
    // only C++ can see the fields.
    // type BlobstoreClient;
    //
    // // Functions implemented in C++.
    // fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    // fn put(&self, parts: &mut MultiBuf) -> u64;
    // fn tag(&self, blobid: u64, tag: &str);
    // fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
}