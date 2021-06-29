use struct_field_names::{EnumVariantNames, StructFieldNames};

#[test]
fn test_field_names() {
    #[allow(dead_code)]
    #[derive(StructFieldNames)]
    struct Struct {
        field_one: i32,
        field_two: Vec<bool>,
        hello: (String,),
        world: [u64; 3],
    }
    assert_eq!(Struct::FIELD_NAMES.field_one, "field_one");
    assert_eq!(Struct::FIELD_NAMES.field_two, "field_two");
    assert_eq!(Struct::FIELD_NAMES.hello, "hello");
    assert_eq!(Struct::FIELD_NAMES.world, "world");
}

#[test]
fn test_with_generic_struct() {
    #[allow(dead_code)]
    #[derive(StructFieldNames)]
    struct Struct<'a, T: PartialEq, U> {
        field_one: &'a T,
        field_two: Vec<U>,
    }
    assert_eq!(Struct::<'_, (), ()>::FIELD_NAMES.field_one, "field_one");
    assert_eq!(Struct::<'_, (), ()>::FIELD_NAMES.field_two, "field_two");
}

#[test]
fn not_a_test_skip_attribute() {
    #[allow(dead_code)]
    #[derive(StructFieldNames)]
    struct Struct {
        field_one: bool,
        #[struct_field_names(skip)]
        field_two: usize,
    }
    assert_eq!(Struct::FIELD_NAMES.field_one, "field_one");
    // Uncommenting the line below should produce an error.
    // assert_eq!(Struct::FIELD_NAMES.field_two, "field_two");
}

#[test]
fn not_a_test_field_visibility() {
    mod module {
        use struct_field_names::StructFieldNames;
        #[allow(dead_code)]
        #[derive(StructFieldNames)]
        pub struct PublicStruct {
            pub public_field: i32,
            private_field: i32,
        }
        #[allow(dead_code)]
        #[derive(StructFieldNames)]
        struct PrivateStruct {
            pub public_field: i32,
            private_field: i32,
        }
    }
    assert_eq!(
        module::PublicStruct::FIELD_NAMES.public_field,
        "public_field"
    );
    // Uncommenting any of the lines below should produce an error.
    /*
    let _ = module::PublicStruct::FIELD_NAMES.private_field;
    let _ = module::PublicStructFieldStaticStr {
        public_field: "asdf",
        private_field: "asdf",
    };
    let _ = module::PrivateStructFieldStaticStr {
        public_field: "asdf",
        private_field: "asdf"
    };
    */
}

#[test]
fn test_variant_names() {
    #[allow(dead_code)]
    #[derive(EnumVariantNames)]
    enum Enum {
        VarOne(i32),
        VarTwo,
        VarThree { inner: String },
    }
    assert_eq!(Enum::VARIANT_NAMES.VarOne, "VarOne");
    assert_eq!(Enum::VARIANT_NAMES.VarTwo, "VarTwo");
    assert_eq!(Enum::VARIANT_NAMES.VarThree, "VarThree");
}
