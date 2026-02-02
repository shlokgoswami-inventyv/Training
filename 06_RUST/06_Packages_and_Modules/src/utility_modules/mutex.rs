use std::sync::Mutex;

enum Request{
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32)
}

static GET_COUNT: Mutex<u32> = Mutex::new(0);
static POST_COUNT: Mutex<u32> = Mutex::new(0);
static DELETE_COUNT: Mutex<u32> = Mutex::new(0);
static TOTAL_COUNT: Mutex<u32> = Mutex::new(0);

fn handle_request(request :&Request){
    match request {
        Request::Get { endpoint } => {
            println!("Get Request");
            println!("Get request hit on {}.",endpoint);
            let mut get_counter = GET_COUNT.lock().unwrap();
            *get_counter += 1;
        },
        Request::Post { endpoint,payload_size } => {
            println!("Post Request");
            println!("Post request hit on {} and this is the {} Payload of the request.",endpoint,payload_size);
            let mut post_counter = POST_COUNT.lock().unwrap();
            *post_counter += 1;
        },
        Request::Delete (id) => {
            println!("Delete Request");
            println!("Id : {} is deleted",id);
            let mut delete_counter = DELETE_COUNT.lock().unwrap();
            *delete_counter += 1;
        }
    }
    let mut total_counter = TOTAL_COUNT.lock().unwrap();
    *total_counter += 1;
}

pub fn mutex(){

    let request_arr: [Request;3] = [
        Request::Get{endpoint: String::from("http://localhost:8000")},
        Request::Post { endpoint: String::from("http://localhost:8000"), payload_size: 30 },
        Request::Delete(3),
    ];
    // let delete_request = Request::Delete(3);
    // let get_request = Request::Get{endpoint: String::from("http://localhost:8000")};
    // let post_request = Request::Post { endpoint: String::from("http://localhost:8000"), payload_size: 30 };

    for request in request_arr.iter(){
        handle_request(request);
    }

    let total_count = TOTAL_COUNT.lock().unwrap();
    println!("Total Request Count : {}",total_count);
}