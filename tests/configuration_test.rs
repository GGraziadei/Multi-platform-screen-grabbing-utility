use std::time::Duration;
use mpsg::configuration::{AcquireMode, Configuration};
use mpsg::configuration::ImageFmt::PNG;

#[test]
fn configuration_test(){
    let configuration = Configuration::default();

    assert_eq!(configuration.get_app_name(), Some("MPSGU".to_string()));
}

#[test]
fn configuration_test_write(){
    let mut configuration = Configuration::default();
    configuration.set_app_name("TEST".to_string());
    assert_eq!(configuration.get_app_name(), Some("TEST".to_string()));
}

#[test]
fn configuration_test_persistence(){
    let mut configuration = Configuration::default();
    configuration.set_app_name("TEST".to_string());


    configuration = Configuration::new();
    assert_eq!(configuration.get_app_name(), Some("TEST".to_string()));
}

#[test]
fn configuration_shortcut(){
    let configuration = Configuration::new();
    let m = configuration.get_hot_key_map().unwrap();

    assert!(m.contains_key(&AcquireMode::AllScreens));
    assert!(m.contains_key(&AcquireMode::Portion));
    assert!(m.contains_key(&AcquireMode::CurrentScreen));
    assert!(m.contains_key(&AcquireMode::SelectScreen));

    for kc in m.values(){
        //test Display
        println!("{}", kc);
    }
}

#[test]
fn configuration_test_write_bulk(){
    let mut configuration = Configuration::new();
    let name = configuration.get_app_name();

    configuration.bulk(None, None, None, Some(PNG),
                       None, None,None, None,None);

    assert_eq!(Some(PNG), configuration.get_image_fmt());
    assert_eq!(name, configuration.get_app_name());
}