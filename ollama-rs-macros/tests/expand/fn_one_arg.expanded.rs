#[macro_use]
extern crate ollama_rs_macros;
#[doc(hidden)]
mod __hello_world_data {
    #[allow(unused_imports)]
    use super::*;
    #[doc(hidden)]
    #[allow(non_camel_case_types, missing_docs)]
    pub struct __hello_world__Params {
        ///Whom to say hello to
        pub name: String,
    }
}
#[allow(non_camel_case_types)]
struct hello_world;
impl ::ollama_rs::generation::tools::Tool for hello_world {
    type Params = __hello_world_data::__hello_world__Params;
    #[inline]
    fn name() -> &'static str {
        "hello_world"
    }
    #[inline]
    fn description() -> &'static str {
        "Say hello"
    }
    async fn call(
        &mut self,
        Self::Params { name }: Self::Params,
    ) -> ::std::result::Result<
        ::std::string::String,
        ::std::boxed::Box<dyn ::std::error::Error>,
    > {
        {
            Ok(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("Hello {0}", name));
                    res
                }),
            )
        }
    }
}
