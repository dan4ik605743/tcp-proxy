use super::*;
use std::net::Ipv4Addr;

#[test]
fn test() {
    let obj = Proxy::new(2, String::new(), 0);
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    obj.add_user(localhost);
    assert_eq!(obj.users.lock().unwrap().get(&localhost), Some(&1));

    assert!(obj.check_user(localhost));
    obj.add_user(localhost);
    assert_eq!(obj.users.lock().unwrap().get(&localhost), Some(&2));
    assert!(!obj.check_user(localhost));

    obj.del_user(localhost);
    assert_eq!(obj.users.lock().unwrap().get(&localhost), Some(&1));
    obj.del_user(localhost);
    assert_eq!(obj.users.lock().unwrap().get(&localhost), None);
}
