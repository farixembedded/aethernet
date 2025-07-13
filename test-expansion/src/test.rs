#![allow(dead_code, clippy::module_inception)]

pub mod calculator {
    mod calculator {
        const INTERFANCE_NAME: &str = "Calculator";
        const DEFAULT_SERVICE_NAME: &str = "Calculator";
        mod rpc {

            pub struct AddReq {
                pub a: i32,
                pub b: i32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for AddReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "AddReq",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "a",
                            &self.a,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "b",
                            &self.b,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for AddReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "a" => _serde::__private::Ok(__Field::__field0),
                                    "b" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"a" => _serde::__private::Ok(__Field::__field0),
                                    b"b" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<AddReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = AddReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct AddReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct AddReq with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct AddReq with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(AddReq {
                                    a: __field0,
                                    b: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<i32> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<i32> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("a")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("b")?
                                    }
                                };
                                _serde::__private::Ok(AddReq {
                                    a: __field0,
                                    b: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["a", "b"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "AddReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<AddReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct AddReqRef {
                pub a: i32,
                pub b: i32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for AddReqRef {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "AddReqRef",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "a",
                            &self.a,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "b",
                            &self.b,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            pub struct Add {}
            impl<'a> ::aethernet::AethernetRpc<'a> for Add {
                const METHOD_NAME: &'static str = "add";
                type ReqType = AddReq;
                type ReqRefType = AddReqRef;
                type RepType = i32;
            }

            pub struct SubReq {
                pub a: i32,
                pub b: i32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for SubReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "SubReq",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "a",
                            &self.a,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "b",
                            &self.b,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for SubReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "a" => _serde::__private::Ok(__Field::__field0),
                                    "b" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"a" => _serde::__private::Ok(__Field::__field0),
                                    b"b" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<SubReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SubReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct SubReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct SubReq with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct SubReq with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(SubReq {
                                    a: __field0,
                                    b: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<i32> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<i32> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("a")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("b")?
                                    }
                                };
                                _serde::__private::Ok(SubReq {
                                    a: __field0,
                                    b: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["a", "b"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SubReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<SubReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct SubReqRef {
                pub a: i32,
                pub b: i32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for SubReqRef {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "SubReqRef",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "a",
                            &self.a,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "b",
                            &self.b,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            pub struct Sub {}
            impl<'a> ::aethernet::AethernetRpc<'a> for Sub {
                const METHOD_NAME: &'static str = "sub";
                type ReqType = SubReq;
                type ReqRefType = SubReqRef;
                type RepType = i32;
            }

            pub struct TestReq {
                pub test_intput: i32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for TestReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "TestReq",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "test_intput",
                            &self.test_intput,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for TestReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "test_intput" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"test_intput" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<TestReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = TestReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct TestReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct TestReq with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(TestReq {
                                    test_intput: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<i32> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "test_intput",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("test_intput")?
                                    }
                                };
                                _serde::__private::Ok(TestReq {
                                    test_intput: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["test_intput"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "TestReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<TestReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct TestReqRef {
                pub test_intput: i32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for TestReqRef {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "TestReqRef",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "test_intput",
                            &self.test_intput,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            pub struct Test {}
            impl<'a> ::aethernet::AethernetRpc<'a> for Test {
                const METHOD_NAME: &'static str = "test";
                type ReqType = TestReq;
                type ReqRefType = TestReqRef;
                type RepType = ();
            }

            pub struct StringTestReq {
                pub count: u32,
                pub message: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for StringTestReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "StringTestReq",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "count",
                            &self.count,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "message",
                            &self.message,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for StringTestReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "count" => _serde::__private::Ok(__Field::__field0),
                                    "message" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"count" => _serde::__private::Ok(__Field::__field0),
                                    b"message" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<StringTestReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = StringTestReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct StringTestReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct StringTestReq with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 = match _serde::de::SeqAccess::next_element::<String>(
                                    &mut __seq,
                                )? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct StringTestReq with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(StringTestReq {
                                    count: __field0,
                                    message: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<u32> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("count"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<u32>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "message",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("count")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("message")?
                                    }
                                };
                                _serde::__private::Ok(StringTestReq {
                                    count: __field0,
                                    message: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["count", "message"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "StringTestReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<StringTestReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct StringTestReqRef<'a> {
                pub count: u32,
                pub message: &'a str,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'a> ::aethernet::_deps::serde::Serialize for StringTestReqRef<'a> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "StringTestReqRef",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "count",
                            &self.count,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "message",
                            &self.message,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            pub struct StringTest {}
            impl<'a> ::aethernet::AethernetRpc<'a> for StringTest {
                const METHOD_NAME: &'static str = "string_test";
                type ReqType = StringTestReq;
                type ReqRefType = StringTestReqRef<'a>;
                type RepType = ();
            }

            pub struct VecFnReq {
                pub input: Vec<i32>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for VecFnReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "VecFnReq",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "input",
                            &self.input,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for VecFnReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "input" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"input" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<VecFnReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = VecFnReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct VecFnReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<Vec<i32>>(
                                    &mut __seq,
                                )? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct VecFnReq with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(VecFnReq { input: __field0 })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Vec<i32>> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("input"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<Vec<i32>>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("input")?
                                    }
                                };
                                _serde::__private::Ok(VecFnReq { input: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["input"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "VecFnReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<VecFnReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct VecFnReqRef<'a> {
                pub input: &'a [i32],
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'a> ::aethernet::_deps::serde::Serialize for VecFnReqRef<'a> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "VecFnReqRef",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "input",
                            &self.input,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            pub struct VecFn {}
            impl<'a> ::aethernet::AethernetRpc<'a> for VecFn {
                const METHOD_NAME: &'static str = "vec_fn";
                type ReqType = VecFnReq;
                type ReqRefType = VecFnReqRef<'a>;
                type RepType = Vec<i32>;
            }
        }
        mod pubsub {

            pub struct HeartbeatReq {
                pub tick: u32,
                pub has_error: bool,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for HeartbeatReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "HeartbeatReq",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "tick",
                            &self.tick,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "has_error",
                            &self.has_error,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for HeartbeatReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "tick" => _serde::__private::Ok(__Field::__field0),
                                    "has_error" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"tick" => _serde::__private::Ok(__Field::__field0),
                                    b"has_error" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<HeartbeatReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = HeartbeatReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct HeartbeatReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct HeartbeatReq with 2 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 = match _serde::de::SeqAccess::next_element::<bool>(
                                    &mut __seq,
                                )? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct HeartbeatReq with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(HeartbeatReq {
                                    tick: __field0,
                                    has_error: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<u32> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<bool> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("tick"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<u32>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "has_error",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<bool>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("tick")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("has_error")?
                                    }
                                };
                                _serde::__private::Ok(HeartbeatReq {
                                    tick: __field0,
                                    has_error: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["tick", "has_error"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "HeartbeatReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<HeartbeatReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct HeartbeatReqRef {
                pub tick: u32,
                pub has_error: bool,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for HeartbeatReqRef {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "HeartbeatReqRef",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "tick",
                            &self.tick,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "has_error",
                            &self.has_error,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };

            pub struct Heartbeat {}
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for Heartbeat {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Heartbeat",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for Heartbeat {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Heartbeat>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Heartbeat;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Heartbeat",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(Heartbeat {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(Heartbeat {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Heartbeat",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Heartbeat>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl<'a> ::aethernet::AethernetPubsub<'a> for Heartbeat {
                const MESSAGE_NAME: &'static str = "heartbeat";
                type MsgType = HeartbeatReq;
                type MsgRefType = HeartbeatReqRef;
            }

            pub struct CounterReq {
                pub arg: u32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for CounterReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CounterReq",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "arg",
                            &self.arg,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for CounterReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "arg" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"arg" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<CounterReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CounterReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CounterReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct CounterReq with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(CounterReq { arg: __field0 })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<u32> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("arg"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<u32>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("arg")?
                                    }
                                };
                                _serde::__private::Ok(CounterReq { arg: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["arg"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CounterReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<CounterReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct CounterReqRef {
                pub arg: u32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for CounterReqRef {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CounterReqRef",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "arg",
                            &self.arg,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };

            pub struct Counter {}
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for Counter {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Counter",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for Counter {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Counter>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Counter;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Counter",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(Counter {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(Counter {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Counter",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Counter>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl<'a> ::aethernet::AethernetPubsub<'a> for Counter {
                const MESSAGE_NAME: &'static str = "counter";
                type MsgType = CounterReq;
                type MsgRefType = CounterReqRef;
            }

            pub struct SetOnceReq {
                pub arg: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for SetOnceReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "SetOnceReq",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "arg",
                            &self.arg,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for SetOnceReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "arg" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"arg" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<SetOnceReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SetOnceReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct SetOnceReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<String>(
                                    &mut __seq,
                                )? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct SetOnceReq with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(SetOnceReq { arg: __field0 })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("arg"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("arg")?
                                    }
                                };
                                _serde::__private::Ok(SetOnceReq { arg: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["arg"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SetOnceReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<SetOnceReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct SetOnceReqRef<'a> {
                pub arg: &'a str,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'a> ::aethernet::_deps::serde::Serialize for SetOnceReqRef<'a> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "SetOnceReqRef",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "arg",
                            &self.arg,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };

            pub struct SetOnce {}
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for SetOnce {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "SetOnce",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for SetOnce {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<SetOnce>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SetOnce;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct SetOnce",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(SetOnce {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(SetOnce {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SetOnce",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<SetOnce>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl<'a> ::aethernet::AethernetPubsub<'a> for SetOnce {
                const MESSAGE_NAME: &'static str = "set_once";
                type MsgType = SetOnceReq;
                type MsgRefType = SetOnceReqRef<'a>;
            }

            pub struct VecPubReq {
                pub nums: Vec<i32>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for VecPubReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "VecPubReq",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "nums",
                            &self.nums,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for VecPubReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "nums" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"nums" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<VecPubReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = VecPubReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct VecPubReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<Vec<i32>>(
                                    &mut __seq,
                                )? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct VecPubReq with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(VecPubReq { nums: __field0 })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Vec<i32>> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("nums"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<Vec<i32>>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("nums")?
                                    }
                                };
                                _serde::__private::Ok(VecPubReq { nums: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["nums"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "VecPubReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<VecPubReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct VecPubReqRef<'a> {
                pub nums: &'a [i32],
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'a> ::aethernet::_deps::serde::Serialize for VecPubReqRef<'a> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "VecPubReqRef",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "nums",
                            &self.nums,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };

            pub struct VecPub {}
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for VecPub {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "VecPub",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for VecPub {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<VecPub>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = VecPub;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct VecPub",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(VecPub {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(VecPub {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "VecPub",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<VecPub>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl<'a> ::aethernet::AethernetPubsub<'a> for VecPub {
                const MESSAGE_NAME: &'static str = "vec_pub";
                type MsgType = VecPubReq;
                type MsgRefType = VecPubReqRef<'a>;
            }

            pub struct UnimplementedPublishReq {
                pub dummy: i32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for UnimplementedPublishReq {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "UnimplementedPublishReq",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "dummy",
                            &self.dummy,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for UnimplementedPublishReq {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "dummy" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"dummy" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<UnimplementedPublishReq>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = UnimplementedPublishReq;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct UnimplementedPublishReq",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<i32>(
                                    &mut __seq,
                                )? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct UnimplementedPublishReq with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(UnimplementedPublishReq { dummy: __field0 })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<i32> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("dummy"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("dummy")?
                                    }
                                };
                                _serde::__private::Ok(UnimplementedPublishReq { dummy: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["dummy"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "UnimplementedPublishReq",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<UnimplementedPublishReq>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };

            pub struct UnimplementedPublishReqRef {
                pub dummy: i32,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for UnimplementedPublishReqRef {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "UnimplementedPublishReqRef",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "dummy",
                            &self.dummy,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };

            pub struct UnimplementedPublish {}
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl ::aethernet::_deps::serde::Serialize for UnimplementedPublish {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> ::aethernet::_deps::serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: ::aethernet::_deps::serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "UnimplementedPublish",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths
            )]
            const _: () = {
                use ::aethernet::_deps::serde as _serde;
                #[automatically_derived]
                impl<'de> ::aethernet::_deps::serde::Deserialize<'de> for UnimplementedPublish {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> ::aethernet::_deps::serde::__private::Result<Self, __D::Error>
                    where
                        __D: ::aethernet::_deps::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<UnimplementedPublish>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = UnimplementedPublish;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct UnimplementedPublish",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(UnimplementedPublish {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(UnimplementedPublish {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "UnimplementedPublish",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<UnimplementedPublish>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl<'a> ::aethernet::AethernetPubsub<'a> for UnimplementedPublish {
                const MESSAGE_NAME: &'static str = "unimplemented_publish";
                type MsgType = UnimplementedPublishReq;
                type MsgRefType = UnimplementedPublishReqRef;
            }
        }
        pub trait CalculatorRpcHandlers: Send {
            fn add(&self, a: i32, b: i32) -> impl ::core::future::Future<Output = i32> + Send;
            fn sub(&self, a: i32, b: i32) -> impl ::core::future::Future<Output = i32> + Send;
            fn test(&self, test_intput: i32) -> impl ::core::future::Future<Output = ()> + Send;
            fn string_test(
                &self,
                count: u32,
                message: String,
            ) -> impl ::core::future::Future<Output = ()> + Send;
            fn vec_fn(
                &self,
                input: Vec<i32>,
            ) -> impl ::core::future::Future<Output = Vec<i32>> + Send;
        }
        pub struct CalculatorRpcServer<T: CalculatorRpcHandlers> {
            handlers: Box<T>,
            agent: ::aethernet::AethernetServer,
        }
        impl<T: CalculatorRpcHandlers + 'static> CalculatorRpcServer<T> {
            pub async fn new(
                connection_string: &str,
                handlers: Box<T>,
            ) -> Result<Self, ::aethernet::AethernetError> {
                let agent = ::aethernet::AethernetServer::new(
                    connection_string,
                    DEFAULT_SERVICE_NAME,
                    INTERFANCE_NAME,
                )
                .await?;
                Ok(Self { handlers, agent })
            }
            #[must_use = "The task guard must be assigned to prevent the task from being aborted immediately"]
            pub async fn spawn_handler(
                connection_string: &str,
                handlers: Box<T>,
            ) -> Result<::aethernet::AethernetHandlerGuard, ::aethernet::AethernetError>
            {
                let connection_string = connection_string.to_string();
                let handler_instance = Self::new(&connection_string, handlers).await?;
                let join_handle = ::tokio::spawn(async move {
                    let mut handler_instance = handler_instance;
                    loop {
                        handler_instance.handle_one_incoming().await;
                    }
                });
                Ok(::aethernet::AethernetHandlerGuard { join_handle })
            }
            async fn handle_one_incoming(&mut self) {
                let maybe_req_envelope = self.agent.wait_for_and_deserialize_next_request().await;
                let req_envelope = match maybe_req_envelope {
                    Ok(envelope) => envelope,
                    Err(_) => return,
                };
                let send_result = match req_envelope.req_type.as_str() {
                    "add" => {
                        match ::aethernet::_deps::serde_json::from_value::<
                            <rpc::Add as ::aethernet::AethernetRpc>::ReqType,
                        >(req_envelope.req)
                        {
                            Ok(req) => {
                                let rep = self.handlers.add(req.a, req.b).await;
                                self.agent
                                    .send_rpc_reply::<rpc::Add>(&req_envelope.req_id, &Ok(rep))
                                    .await;
                                Ok(())
                            }
                            Err(err) => {
                                Err(::aethernet::AethernetError::SerdeError(err.to_string()))
                            }
                        }
                    }
                    "sub" => {
                        match ::aethernet::_deps::serde_json::from_value::<
                            <rpc::Sub as ::aethernet::AethernetRpc>::ReqType,
                        >(req_envelope.req)
                        {
                            Ok(req) => {
                                let rep = self.handlers.sub(req.a, req.b).await;
                                self.agent
                                    .send_rpc_reply::<rpc::Sub>(&req_envelope.req_id, &Ok(rep))
                                    .await;
                                Ok(())
                            }
                            Err(err) => {
                                Err(::aethernet::AethernetError::SerdeError(err.to_string()))
                            }
                        }
                    }
                    "test" => {
                        match ::aethernet::_deps::serde_json::from_value::<
                            <rpc::Test as ::aethernet::AethernetRpc>::ReqType,
                        >(req_envelope.req)
                        {
                            Ok(req) => {
                                let rep = self.handlers.test(req.test_intput).await;
                                self.agent
                                    .send_rpc_reply::<rpc::Test>(&req_envelope.req_id, &Ok(rep))
                                    .await;
                                Ok(())
                            }
                            Err(err) => {
                                Err(::aethernet::AethernetError::SerdeError(err.to_string()))
                            }
                        }
                    }
                    "string_test" => {
                        match ::aethernet::_deps::serde_json::from_value::<
                            <rpc::StringTest as ::aethernet::AethernetRpc>::ReqType,
                        >(req_envelope.req)
                        {
                            Ok(req) => {
                                let rep = self.handlers.string_test(req.count, req.message).await;
                                self.agent
                                    .send_rpc_reply::<rpc::StringTest>(
                                        &req_envelope.req_id,
                                        &Ok(rep),
                                    )
                                    .await;
                                Ok(())
                            }
                            Err(err) => {
                                Err(::aethernet::AethernetError::SerdeError(err.to_string()))
                            }
                        }
                    }
                    "vec_fn" => {
                        match ::aethernet::_deps::serde_json::from_value::<
                            <rpc::VecFn as ::aethernet::AethernetRpc>::ReqType,
                        >(req_envelope.req)
                        {
                            Ok(req) => {
                                let rep = self.handlers.vec_fn(req.input).await;
                                self.agent
                                    .send_rpc_reply::<rpc::VecFn>(&req_envelope.req_id, &Ok(rep))
                                    .await;
                                Ok(())
                            }
                            Err(err) => {
                                Err(::aethernet::AethernetError::SerdeError(err.to_string()))
                            }
                        }
                    }
                    _ => Err(::aethernet::AethernetError::MethodNotImplemented),
                };
                if send_result.is_err() {
                    self.agent
                        .send_rpc_reply::<::aethernet::AethernetError>(
                            &req_envelope.req_id,
                            &send_result,
                        )
                        .await;
                }
            }
        }
        pub trait CalculatorPubsubHandlers: Send {
            fn heartbeat(
                &self,
                tick: u32,
                has_error: bool,
            ) -> impl ::core::future::Future<Output = ()> + Send {
                async {
                    let _ = tick;
                    let _ = has_error;
                }
            }
            fn counter(&self, arg: u32) -> impl ::core::future::Future<Output = ()> + Send {
                async {
                    let _ = arg;
                }
            }
            fn set_once(&self, arg: String) -> impl ::core::future::Future<Output = ()> + Send {
                async {
                    let _ = arg;
                }
            }
            fn vec_pub(&self, nums: Vec<i32>) -> impl ::core::future::Future<Output = ()> + Send {
                async {
                    let _ = nums;
                }
            }
            fn unimplemented_publish(
                &self,
                dummy: i32,
            ) -> impl ::core::future::Future<Output = ()> + Send {
                async {
                    let _ = dummy;
                }
            }
        }
        pub struct CalculatorSubscriber<T: CalculatorPubsubHandlers> {
            handlers: Box<T>,
            agent: ::aethernet::AethernetSubscriber,
        }
        impl<T: CalculatorPubsubHandlers + 'static> CalculatorSubscriber<T> {
            pub async fn new(
                connection_string: &str,
                handlers: Box<T>,
            ) -> Result<Self, ::aethernet::AethernetError> {
                let agent = ::aethernet::AethernetSubscriber::new(
                    connection_string,
                    DEFAULT_SERVICE_NAME,
                    INTERFANCE_NAME,
                )
                .await;
                agent.subscribe::<pubsub::Heartbeat>().await.unwrap();
                agent.subscribe::<pubsub::Counter>().await.unwrap();
                agent.subscribe::<pubsub::SetOnce>().await.unwrap();
                agent.subscribe::<pubsub::VecPub>().await.unwrap();
                agent
                    .subscribe::<pubsub::UnimplementedPublish>()
                    .await
                    .unwrap();
                Ok(Self { handlers, agent })
            }
            #[must_use = "The task guard must be assigned to prevent the task from being aborted immediately"]
            pub async fn spawn_handler(
                connection_string: &str,
                handlers: Box<T>,
            ) -> Result<::aethernet::AethernetHandlerGuard, ::aethernet::AethernetError>
            {
                let connection_string = connection_string.to_string();
                let handler_instance = Self::new(&connection_string, handlers).await?;
                let join_handle = ::tokio::spawn(async move {
                    let mut handler_instance = handler_instance;
                    loop {
                        handler_instance.handle_one_incoming().await;
                    }
                });
                Ok(::aethernet::AethernetHandlerGuard { join_handle })
            }
            async fn handle_one_incoming(&mut self) {
                let (msg_type, msg_json) = match self.agent.get_one_sub_message().await {
                    Ok(value) => value,
                    Err(_) => return,
                };
                match msg_type.as_str() {
                    "heartbeat" => {
                        type Msg<'a> =
                            <pubsub::Heartbeat as ::aethernet::AethernetPubsub<'a>>::MsgType;
                        match ::aethernet::_deps::serde_json::from_str::<Msg>(&msg_json) {
                            Ok(req) => self.handlers.heartbeat(req.tick, req.has_error).await,
                            Err(_) => {}
                        }
                    }
                    "counter" => {
                        type Msg<'a> =
                            <pubsub::Counter as ::aethernet::AethernetPubsub<'a>>::MsgType;
                        match ::aethernet::_deps::serde_json::from_str::<Msg>(&msg_json) {
                            Ok(req) => self.handlers.counter(req.arg).await,
                            Err(_) => {}
                        }
                    }
                    "set_once" => {
                        type Msg<'a> =
                            <pubsub::SetOnce as ::aethernet::AethernetPubsub<'a>>::MsgType;
                        match ::aethernet::_deps::serde_json::from_str::<Msg>(&msg_json) {
                            Ok(req) => self.handlers.set_once(req.arg).await,
                            Err(_) => {}
                        }
                    }
                    "vec_pub" => {
                        type Msg<'a> =
                            <pubsub::VecPub as ::aethernet::AethernetPubsub<'a>>::MsgType;
                        match ::aethernet::_deps::serde_json::from_str::<Msg>(&msg_json) {
                            Ok(req) => self.handlers.vec_pub(req.nums).await,
                            Err(_) => {}
                        }
                    }
                    "unimplemented_publish" => {
                        type Msg<'a> = <pubsub::UnimplementedPublish as ::aethernet::AethernetPubsub<
                            'a,
                        >>::MsgType;
                        match ::aethernet::_deps::serde_json::from_str::<Msg>(&msg_json) {
                            Ok(req) => self.handlers.unimplemented_publish(req.dummy).await,
                            Err(_) => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        pub struct CalculatorPublisher {
            server: ::aethernet::AethernetServer,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CalculatorPublisher {
            #[inline]
            fn clone(&self) -> CalculatorPublisher {
                CalculatorPublisher {
                    server: ::core::clone::Clone::clone(&self.server),
                }
            }
        }
        impl CalculatorPublisher {
            pub async fn new(connection_string: &str) -> Result<Self, ::aethernet::AethernetError> {
                Ok(Self {
                    server: ::aethernet::AethernetServer::new(
                        connection_string,
                        DEFAULT_SERVICE_NAME,
                        INTERFANCE_NAME,
                    )
                    .await?,
                })
            }
            pub async fn heartbeat(
                &self,
                tick: u32,
                has_error: bool,
            ) -> Result<(), ::aethernet::AethernetError> {
                type MsgRef<'a> =
                    <pubsub::Heartbeat as ::aethernet::AethernetPubsub<'a>>::MsgRefType;
                let msg = MsgRef { tick, has_error };
                self.server.publish::<pubsub::Heartbeat>(&msg).await
            }
            pub async fn counter(&self, arg: u32) -> Result<(), ::aethernet::AethernetError> {
                type MsgRef<'a> = <pubsub::Counter as ::aethernet::AethernetPubsub<'a>>::MsgRefType;
                let msg = MsgRef { arg };
                self.server.publish::<pubsub::Counter>(&msg).await
            }
            pub async fn set_once(&self, arg: &str) -> Result<(), ::aethernet::AethernetError> {
                type MsgRef<'a> = <pubsub::SetOnce as ::aethernet::AethernetPubsub<'a>>::MsgRefType;
                let msg = MsgRef { arg };
                self.server.publish::<pubsub::SetOnce>(&msg).await
            }
            pub async fn vec_pub(&self, nums: &[i32]) -> Result<(), ::aethernet::AethernetError> {
                type MsgRef<'a> = <pubsub::VecPub as ::aethernet::AethernetPubsub<'a>>::MsgRefType;
                let msg = MsgRef { nums };
                self.server.publish::<pubsub::VecPub>(&msg).await
            }
            pub async fn unimplemented_publish(
                &self,
                dummy: i32,
            ) -> Result<(), ::aethernet::AethernetError> {
                type MsgRef<'a> =
                    <pubsub::UnimplementedPublish as ::aethernet::AethernetPubsub<'a>>::MsgRefType;
                let msg = MsgRef { dummy };
                self.server
                    .publish::<pubsub::UnimplementedPublish>(&msg)
                    .await
            }
        }
        pub struct CalculatorClient {
            client: ::aethernet::AethernetRpcClient,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CalculatorClient {
            #[inline]
            fn clone(&self) -> CalculatorClient {
                CalculatorClient {
                    client: ::core::clone::Clone::clone(&self.client),
                }
            }
        }
        impl CalculatorClient {
            pub async fn new(connection_string: &str) -> Result<Self, ::aethernet::AethernetError> {
                Ok(Self {
                    client: ::aethernet::AethernetRpcClient::new(
                        connection_string,
                        DEFAULT_SERVICE_NAME,
                        INTERFANCE_NAME,
                    )
                    .await?,
                })
            }
            ///client call
            pub async fn add(&self, a: i32, b: i32) -> Result<i32, ::aethernet::AethernetError> {
                type ReqRef<'a> = <rpc::Add as ::aethernet::AethernetRpc<'a>>::ReqRefType;
                let req = ReqRef { a, b };
                self.client.call::<rpc::Add>(req).await
            }
            ///client call
            pub async fn sub(&self, a: i32, b: i32) -> Result<i32, ::aethernet::AethernetError> {
                type ReqRef<'a> = <rpc::Sub as ::aethernet::AethernetRpc<'a>>::ReqRefType;
                let req = ReqRef { a, b };
                self.client.call::<rpc::Sub>(req).await
            }
            ///client call
            pub async fn test(&self, test_intput: i32) -> Result<(), ::aethernet::AethernetError> {
                type ReqRef<'a> = <rpc::Test as ::aethernet::AethernetRpc<'a>>::ReqRefType;
                let req = ReqRef { test_intput };
                self.client.call::<rpc::Test>(req).await
            }
            ///client call
            pub async fn string_test(
                &self,
                count: u32,
                message: &str,
            ) -> Result<(), ::aethernet::AethernetError> {
                type ReqRef<'a> = <rpc::StringTest as ::aethernet::AethernetRpc<'a>>::ReqRefType;
                let req = ReqRef { count, message };
                self.client.call::<rpc::StringTest>(req).await
            }
            ///client call
            pub async fn vec_fn(
                &self,
                input: &[i32],
            ) -> Result<Vec<i32>, ::aethernet::AethernetError> {
                type ReqRef<'a> = <rpc::VecFn as ::aethernet::AethernetRpc<'a>>::ReqRefType;
                let req = ReqRef { input };
                self.client.call::<rpc::VecFn>(req).await
            }
            pub async fn get_heartbeat(
                &self,
            ) -> Result<
                <pubsub::Heartbeat as ::aethernet::AethernetPubsub>::MsgType,
                ::aethernet::AethernetError,
            > {
                self.client.get::<pubsub::Heartbeat>().await
            }
            pub async fn get_counter(&self) -> Result<u32, ::aethernet::AethernetError> {
                let msg = self.client.get::<pubsub::Counter>().await?;
                Ok(msg.arg)
            }
            pub async fn get_set_once(&self) -> Result<String, ::aethernet::AethernetError> {
                let msg = self.client.get::<pubsub::SetOnce>().await?;
                Ok(msg.arg)
            }
            pub async fn get_vec_pub(&self) -> Result<Vec<i32>, ::aethernet::AethernetError> {
                let msg = self.client.get::<pubsub::VecPub>().await?;
                Ok(msg.nums)
            }
            pub async fn get_unimplemented_publish(
                &self,
            ) -> Result<i32, ::aethernet::AethernetError> {
                let msg = self.client.get::<pubsub::UnimplementedPublish>().await?;
                Ok(msg.dummy)
            }
        }
    }
    #[allow(unused_imports)]
    pub use calculator::{
        CalculatorClient, CalculatorPublisher, CalculatorPubsubHandlers, CalculatorRpcHandlers,
        CalculatorRpcServer, CalculatorSubscriber,
    };
}
