use std::{collections::HashMap, sync::Arc};

static mut INSTANCE: Option<Arc<CharacterSet>> = None;

pub struct CharacterSet {
    pub map: HashMap<u8, String>,
}

impl CharacterSet {
    pub fn get_instance() -> Arc<Self> {
        unsafe {
            if INSTANCE.is_none() {
                INSTANCE.replace(Arc::new(Self::new()));
            }

            INSTANCE.as_ref().unwrap().clone()
        }
    }

    pub fn get(&self, i: &u8) -> Option<&String> {
        self.map.get(&i)
    }

    fn new() -> Self {
        let mut map: HashMap<u8, String> = HashMap::new();
        map.insert(0xA1, String::from("0"));
        map.insert(0xA2, String::from("1"));
        map.insert(0xA3, String::from("2"));
        map.insert(0xA4, String::from("3"));
        map.insert(0xA5, String::from("4"));
        map.insert(0xA6, String::from("5"));
        map.insert(0xA7, String::from("6"));
        map.insert(0xA8, String::from("7"));
        map.insert(0xA9, String::from("8"));
        map.insert(0xAA, String::from("9"));
        map.insert(0xAB, String::from("!"));
        map.insert(0xAC, String::from("?"));
        map.insert(0xAD, String::from("."));
        map.insert(0xAE, String::from("-"));
        map.insert(0xAF, String::from("・"));
        map.insert(0xB0, String::from("..."));
        map.insert(0xB1, String::from("“"));
        map.insert(0xB2, String::from("”"));
        map.insert(0xB3, String::from("‘"));
        map.insert(0xB4, String::from("’"));
        map.insert(0xB5, String::from("♂"));
        map.insert(0xB6, String::from("♀"));
        map.insert(0xB7, String::from("$"));
        map.insert(0xB8, String::from(","));
        map.insert(0xB9, String::from("×"));
        map.insert(0xBA, String::from("/"));
        map.insert(0xBB, String::from("A"));
        map.insert(0xBC, String::from("B"));
        map.insert(0xBD, String::from("C"));
        map.insert(0xBE, String::from("D"));
        map.insert(0xBF, String::from("E"));
        map.insert(0xC0, String::from("F"));
        map.insert(0xC1, String::from("G"));
        map.insert(0xC2, String::from("H"));
        map.insert(0xC3, String::from("I"));
        map.insert(0xC4, String::from("J"));
        map.insert(0xC5, String::from("K"));
        map.insert(0xC6, String::from("L"));
        map.insert(0xC7, String::from("M"));
        map.insert(0xC8, String::from("N"));
        map.insert(0xC9, String::from("O"));
        map.insert(0xCA, String::from("P"));
        map.insert(0xCB, String::from("Q"));
        map.insert(0xCC, String::from("R"));
        map.insert(0xCD, String::from("S"));
        map.insert(0xCE, String::from("T"));
        map.insert(0xCF, String::from("U"));
        map.insert(0xD0, String::from("V"));
        map.insert(0xD1, String::from("W"));
        map.insert(0xD2, String::from("X"));
        map.insert(0xD3, String::from("Y"));
        map.insert(0xD4, String::from("Z"));
        map.insert(0xD5, String::from("a"));
        map.insert(0xD6, String::from("b"));
        map.insert(0xD7, String::from("c"));
        map.insert(0xD8, String::from("d"));
        map.insert(0xD9, String::from("e"));
        map.insert(0xDA, String::from("f"));
        map.insert(0xDB, String::from("g"));
        map.insert(0xDC, String::from("h"));
        map.insert(0xDD, String::from("i"));
        map.insert(0xDE, String::from("j"));
        map.insert(0xDF, String::from("k"));
        map.insert(0xE0, String::from("l"));
        map.insert(0xE1, String::from("m"));
        map.insert(0xE2, String::from("n"));
        map.insert(0xE3, String::from("o"));
        map.insert(0xE4, String::from("p"));
        map.insert(0xE5, String::from("q"));
        map.insert(0xE6, String::from("r"));
        map.insert(0xE7, String::from("s"));
        map.insert(0xE8, String::from("t"));
        map.insert(0xE9, String::from("u"));
        map.insert(0xEA, String::from("v"));
        map.insert(0xEB, String::from("w"));
        map.insert(0xEC, String::from("x"));
        map.insert(0xED, String::from("y"));
        map.insert(0xEE, String::from("z"));

        Self { map }
    }
}
