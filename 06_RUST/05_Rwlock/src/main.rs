use std::sync::RwLock;

enum Request{
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32)
}

static GET_COUNT: RwLock<u32> = RwLock::new(0);
static POST_COUNT: RwLock<u32> = RwLock::new(0);
static DELETE_COUNT: RwLock<u32> = RwLock::new(0);
static TOTAL_COUNT: RwLock<u32> = RwLock::new(0);

fn handle_request(request :&Request){
    match request {
        Request::Get { endpoint } => {
            println!("Get Request");
            println!("Get request hit on {}.",endpoint);
            let mut get_counter = GET_COUNT.write().unwrap();
            *get_counter += 1;
        },
        Request::Post { endpoint,payload_size } => {
            println!("Post Request");
            println!("Post request hit on {} and this is the {} Payload of the request.",endpoint,payload_size);
            let mut post_counter = POST_COUNT.write().unwrap();
            *post_counter += 1;
        },
        Request::Delete (id) => {
            println!("Delete Request");
            println!("Id : {} is deleted",id);
            let mut delete_counter = DELETE_COUNT.write().unwrap();
            *delete_counter += 1;
        }
    }
    let mut total_counter = TOTAL_COUNT.write().unwrap();
    *total_counter += 1;
}


fn main(){

    println!("RW LOCK");

    let request_arr: [Request;3] = [
        Request::Get{endpoint: String::from("http://localhost:8000")},
        Request::Post { endpoint: String::from("http://localhost:8000"), payload_size: 30 },
        Request::Delete(3),
    ];

    for request in request_arr.iter(){
        handle_request(request);
    }

    let total_count = TOTAL_COUNT.read().unwrap();
    println!("Total Request Count : {}",total_count);
}