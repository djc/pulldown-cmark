// This file is auto-generated by the build script
// Please, do not modify it manually

extern crate pulldown_cmark;

include!("normalize_html.rs.inc");


    #[test]
    fn regression_test_1() {
        let original = r##"see the [many] [articles] [on] [QuickCheck].

[many]: https://medium.com/@jlouis666/quickcheck-advice-c357efb4e7e6
[articles]: http://www.quviq.com/products/erlang-quickcheck/
[on]: https://wiki.haskell.org/Introduction_to_QuickCheck1
[QuickCheck]: https://hackage.haskell.org/package/QuickCheck
"##;
        let expected = r##"<p>see the 
  <a href="https://medium.com/@jlouis666/quickcheck-advice-c357efb4e7e6">many</a> 
  <a href="http://www.quviq.com/products/erlang-quickcheck/">articles</a> 
  <a href="https://wiki.haskell.org/Introduction_to_QuickCheck1">on</a> 
  <a href="https://hackage.haskell.org/package/QuickCheck">QuickCheck</a>.
</p>
"##;

        use pulldown_cmark::{Parser, html, Options};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_TABLES);
        opts.insert(Options::ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p);

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn regression_test_2() {
        let original = r##"[![debug-stub-derive on crates.io][cratesio-image]][cratesio]
[![debug-stub-derive on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/debug_stub_derive.svg
[cratesio]: https://crates.io/crates/debug_stub_derive
[docsrs-image]: https://docs.rs/debug_stub_derive/badge.svg?version=0.3.0
[docsrs]: https://docs.rs/debug_stub_derive/0.3.0/
"##;
        let expected = r##"<p><a href="https://crates.io/crates/debug_stub_derive"><img src="https://img.shields.io/crates/v/debug_stub_derive.svg" alt="debug-stub-derive on crates.io" /></a>
<a href="https://docs.rs/debug_stub_derive/0.3.0/"><img src="https://docs.rs/debug_stub_derive/badge.svg?version=0.3.0" alt="debug-stub-derive on docs.rs" /></a></p>
"##;

        use pulldown_cmark::{Parser, html, Options};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_TABLES);
        opts.insert(Options::ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p);

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn regression_test_3() {
        let original = r##"|  Title A  |  Title B  |
| --------- | --------- |
| Content   | Content   |

|  Title A  |  Title B  |  Title C  |  Title D  |
| --------- | --------- | --------- | ---------:|
| Content   | Content   | Conent    | Content   |
"##;
        let expected = r##"<table><thead><tr><th>Title A  </th><th>Title B  </th></tr></thead><tbody>
<tr><td>Content   </td><td>Content   </td></tr>
</tbody></table>
<table><thead><tr><th>Title A  </th><th>Title B  </th><th>Title C  </th><th align="right">Title D  </th></tr></thead><tbody>
<tr><td>Content   </td><td>Content   </td><td>Conent    </td><td align="right">Content   </td></tr>
</tbody></table>
"##;

        use pulldown_cmark::{Parser, html, Options};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_TABLES);
        opts.insert(Options::ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p);

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }