#[macro_use] extern crate rocket;
use flatbuffers;
use detective::data::suspect::{Mood, Location, LocationArgs, Position,
                                Suspect, SuspectArgs};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/info")]
fn info() -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new();

    // basic information
    let name = builder.create_string("Moriarty");
    let age: u32 = 59;
    let face_img = builder
        .create_string(
            "https://bkimg.cdn.bcebos.com/pic/0ff41bd5ad6eddc451dafa147a91a1fd5266d1169580"
        );

    // previous location
    let pos = Position::new(51.50787055157849, -0.16329938822309534);
    let street = builder.create_string("Hyde Park");
    let location_prev1 = Location::create(&mut builder, &LocationArgs{
        pos: Some(&pos),
        street: Some(street),
    });
    let pos = Position::new(51.51647664020246, -0.234050548394412);
    let street = builder.create_string("Hammersmith Hospital");
    let location_prev2 = Location::create(&mut builder, &LocationArgs{
        pos: Some(&pos),
        street: Some(street),
    });

    // current location
    let pos = Position::new(51.521010377638554, -0.15669042517487825);
    let street = builder.create_string("Baker Street");
    let location = Location::create(&mut builder, &LocationArgs{
        pos: Some(&pos),
        street: Some(street),
    });

    // build the path
    let path = builder.create_vector(&[location_prev1, location_prev2, location]);

    let suspect = Suspect::create(&mut builder, &SuspectArgs{
        name: Some(name),
        face_img: Some(face_img),
        mood: Mood::Mad,
        location: Some(location),
        path: Some(path),
        age
    });

    builder.finish(suspect, None);

    builder.finished_data().to_vec()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, info])
}
