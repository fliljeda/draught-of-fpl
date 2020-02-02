pub mod client;
pub mod server;
pub mod storage;
pub mod structs;


pub fn start() {
    test_local_fetch();
    server::start();
    //test_fpl_fetch(12);
}


#[allow(dead_code)]
fn test_local_fetch(){
    let mut fpl_client = client::new().unwrap();
    fpl_client.set_local(true);
    let gw = 22;
    
    let json = match fpl_client.get_gw_points_live(gw){
        Ok(res) => res,
        Err(e) => {
            println!("{}",e);
            String::from("")
        },
    };
    match structs::live::from_str(&json) {
        Ok(des) => println!("{:?}", des),
        Err(e) => println!("{}", e),
    };
    //println!("{}", &league_details);

}

#[allow(dead_code)]
fn test_fpl_fetch(gw: u32){
    let fpl_client = client::new().unwrap();
    let json = match fpl_client.get_gw_points_live(gw){
        Ok(res) => res,
        Err(e) => {
            println!("{}",e);
            String::from("")
        },
    };
    match structs::live::from_str(&json) {
        Ok(des) => println!("{:?}", des.elements["10"]),
        Err(e) => println!("{}", e),
    };
    //println!("{}", &league_details);

}