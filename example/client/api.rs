// trait CRUD<T> {
//     fn create(t: T) -> ();
//     fn read() -> ();
//     fn update(t: T) -> ();
//     fn delete(t: T) -> ();
// }
//

// What I need:
// * API
// * Version

use hyper::Client;

trait API {
    fn path();
    fn version();
}

fn create<T>(t : T) {
}

