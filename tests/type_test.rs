use serde_hyprlang::Vec2;

#[test]
fn vec2_serialize() {
    let t = Vec2(5.0, 8.3);
    let res = serde_hyprlang::to_string(&t);

    
    let expect = 
"5 8.3".to_string();

    assert_eq!(res, Ok(expect), "Failed to encode 2D vector correctly");
}

#[test]
fn vec2_deserialize() {
    let t = "-6.45 2";
    let res = serde_hyprlang::from_str(&t);

    
    let expect = Vec2(-6.45, 2.0);

    assert_eq!(res, Ok(expect), "Failed to decode 2D vector correctly");
}
