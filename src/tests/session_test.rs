
use crate::menu::{cache::{self, del_key, key_exists, get_process, key_ex}, models};



#[test]
fn test_set_data() {
    let  key = models::Process::new();
    let res = cache::set_process(key.to_owned());
    assert_eq!(res.is_ok(), true);
    let del = del_key(&key.proc_id);
    assert_eq!(del.is_ok(), true)
}

#[test]
fn test_exists_key() {
    let key = models::Process::new();
    let res = cache::set_process(key.to_owned());
    assert_eq!(res.is_ok(), true);
    let res = key_exists(&key.proc_id);
    assert_eq!(res.is_ok(), true);
    let del = del_key(&key.proc_id);
    assert_eq!(del.is_ok(), true)
}

#[test]
fn test_get_data() {
    
    let key = models::Process::new();
    let res = cache::set_process(key.to_owned());
    assert_eq!(res.is_ok(), true);
    let res = get_process(&key.proc_id);
    assert_eq!(res.is_ok(), true);
    let del = del_key(&key.proc_id);
    assert_eq!(del.is_ok(), true)
}

#[test]
fn test_del_data() {
    
    let key = models::Process::new();
    let res = cache::set_process(key.to_owned());
    assert_eq!(res.is_ok(), true);
    let del = del_key(&key.proc_id);
    assert_eq!(del.is_ok(), true)
}

#[test]
fn test_expres_data() {
    
    let key = models::Process::new();
    let res = cache::set_process(key.to_owned());
    assert_eq!(res.is_ok(), true);
    let res = key_ex(&key.proc_id, 10);
    assert!(res.is_ok());
    let del = del_key(&key.proc_id);
    assert_eq!(del.is_ok(), true)
}

#[test]
fn edue() {
    for x in 1..1+1 {
        println!("{x}");
    }
}