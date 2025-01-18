use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct SimpleStruct {
    first: String,
    second: (bool, usize)
}

#[test]
fn simple_struct_serialize() {
    let h = SimpleStruct {
        first: "Vaxry".to_string(),
        second: (true, 1)
    };

    let res = serde_hyprlang::to_string(&h);
    
    let expect = "first = Vaxry\nsecond = true, 1\n".to_string();

    assert_eq!(res, Ok(expect), "Failed to encode a simple struct");
}

#[derive(Debug, Clone, Serialize)]
struct SimpleSequence {
    exec: Vec<(usize, String)>
}

#[test]
fn simple_sequence_serialize() {
    let t = SimpleSequence {
        exec: vec![(4, "Hewo".to_string()), (1, "everynya".to_string())]
    };
    let res = serde_hyprlang::to_string(&t);
    
    let expect = "exec = 4, Hewo\nexec = 1, everynya\n\n".to_string();

    assert_eq!(res, Ok(expect), "Failed to encode a simple sequence");
}

#[derive(Debug, Clone, Serialize)]
struct SimpleStructSequence {
    random_thing: i8,
    keep_it_simple_stupid: Vec<SimpleStruct>
}


#[test]
fn simple_sequence_struct_serialize() {
    let t = SimpleStructSequence {
        keep_it_simple_stupid: vec![
            SimpleStruct {
                first: "I wish".to_string(),
                second: (true, 3)
            },
            SimpleStruct {
                first: "I were".to_string(),
                second: (true, 1)
            },
            SimpleStruct {
                first: "a bird".to_string(),
                second: (false, 0)
            },
        ],
        random_thing: 120
    };
    let res = serde_hyprlang::to_string(&t);
    
    let expect = 
"random_thing = 120
keep_it_simple_stupid {
    first = I wish
    second = true, 3
}

keep_it_simple_stupid {
    first = I were
    second = true, 1
}

keep_it_simple_stupid {
    first = a bird
    second = false, 0
}


".to_string();

    assert_eq!(res, Ok(expect), "Failed to encode a simple sequence of structs");
}

#[derive(Debug, Clone, Serialize)]
struct SimpleNestedStruct {
    config: SimpleStruct,
    other: bool
}

#[derive(Debug, Clone, Serialize)]
struct SimpleNestedRootStruct {
    test: usize,
    nested: SimpleNestedStruct,
}

#[test]
fn simple_nested_struct_serializer() {
    let t = SimpleNestedRootStruct {
        nested: SimpleNestedStruct {
            other: true,
            config: SimpleStruct {
                first: "Oh my God!".to_string(),
                second: (false, 64)
            },
        },
        test: 6732
    };
    let res = serde_hyprlang::to_string(&t);
    
    let expect = 
"test = 6732
nested {
    config {
        first = Oh my God!
        second = false, 64
    }

    other = true
}

".to_string();

    assert_eq!(res, Ok(expect), "Failed to encode a simple nesting of structs");
}

#[derive(Debug, Clone, Serialize)]
struct SimpleOptionEncoding {
    here: Option<isize>,
    not_here: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    omit: Option<bool>,
    
    not_st: Option<SimpleStruct>,
    st: Option<SimpleStruct>,
    
    inside_tupple: (bool, Option<String>, bool),
    inside_tupple_2: (Option<bool>, Option<String>, bool),

}

#[test]
fn simple_option_serialize() {
    let t = SimpleOptionEncoding {
        here: Some(-23),
        not_here: None,
        
        not_st: None,
        st: Some(SimpleStruct {
            first: "Not another one...".to_string(),
            second: (true, 777)
        }),

        inside_tupple: (false, None, true),
        inside_tupple_2: (None, Some("Text".to_string()), false),

        omit: None
    };
    let res = serde_hyprlang::to_string(&t);
    
    let expect = 
"here = -23
not_here = 
not_st = 
st {
    first = Not another one...
    second = true, 777
}

inside_tupple = false, , true
inside_tupple_2 = , Text, false
".to_string();

    assert_eq!(res, Ok(expect), "Failed to encode a options and their nulls incorrectly");
}

#[derive(Debug, Clone, Serialize)]
enum SimpleEnumHandling {
    NewTypeVarient(SimpleStruct),
    TuppleVarient(usize, bool),
    StructVarient{ thing: f64 },
    Root{ new_type: Box<SimpleEnumHandling>, tupple: Box<SimpleEnumHandling>, st: Box<SimpleEnumHandling>}
}

#[test]
fn simple_enum_serialization() {
    let t = SimpleEnumHandling::Root { 
        new_type: Box::new(SimpleEnumHandling::NewTypeVarient(
            SimpleStruct {
                first: "REDNote 小红书国际版".to_string(),
                second: (false, 1989)
            }
        )), 
        tupple: Box::new(SimpleEnumHandling::TuppleVarient(75, false)),
        st: Box::new(SimpleEnumHandling::StructVarient { thing: 5.56 })
    };
    let res = serde_hyprlang::to_string(&t);
    
    let expect = 
"new_type {
    first = REDNote 小红书国际版
    second = false, 1989
}

tupple = 75, false
st {
    thing = 5.56
}

".to_string();

    assert_eq!(res, Ok(expect), "Failed to encode the different handled enums correctly");
}
