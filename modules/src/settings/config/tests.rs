use super::*;
use anyhow::Result;

#[test]
fn test() -> Result<()> {
    let obj = ClientOrServer::new("127.0.0.1".to_string(), 7878);
    let config_path = "./src/settings/config/.config.json";

    write_config(&obj, config_path)?;
    let readed_obj = read_config::<ClientOrServer>(config_path)?;

    assert_eq!(readed_obj.ip, obj.ip);
    assert_eq!(readed_obj.port, obj.port);

    Ok(())
}
