/// Basic text matching capabilities, enough to write a top-level lexer.
///
/// ```rust
/// use df_utils::text::matcher::{Keyword, Matcher, TracePos};
/// use df_utils::variant_matcher;
///
/// variant_matcher! {
///     ThingMatcher for Thing {
///         A(keyword_a: Keyword),
///         B(keyword_b: Keyword),
///     }
/// }
///
/// let mut cursor = TracePos::new();
/// let result = ThingMatcher::new(Keyword::new("a"), Keyword::new("b"))
///     .match_string(&"b".to_string(), &mut cursor)?;
/// assert!(matches!(result, Thing::B(())));
/// assert_eq!(cursor.char, 1);
/// # Ok::<(), String>(())
/// ```
pub mod matcher;

/// Generates an enum and a matcher that tries each supplied matcher in order.
///
/// Each matcher type determines the payload type of its corresponding enum variant.
/// A failed alternative leaves the cursor unchanged; if all alternatives fail, their
/// errors are combined with [`matcher::TracePos::variant_err`].
///
/// The syntax is `MatcherName for EnumName { Variant(matcher_variable: MatcherType), ... }`.
#[macro_export]
macro_rules! variant_matcher {
    (
        $(#[$enum_meta:meta])*
        $vis:vis $matcher_name:ident for $enum_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident ($matcher_variable:ident : $matcher_type:ty)
            ),* $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        $vis enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant(
                    <$matcher_type as $crate::text::matcher::Matcher>::Output
                )
            ),*
        }

        $vis struct $matcher_name {
            $($matcher_variable: $matcher_type),*
        }

        impl $matcher_name {
            $vis fn new($($matcher_variable: $matcher_type),*) -> Self {
                Self { $($matcher_variable),* }
            }
        }

        impl $crate::text::matcher::Matcher for $matcher_name {
            type Output = $enum_name;

            fn match_string(
                self,
                data: &String,
                cursor: &mut $crate::text::matcher::TracePos,
            ) -> Result<Self::Output, String> {
                let Self { $($matcher_variable),* } = self;
                let mut errors = Vec::new();

                $(
                    let mut trial_cursor = cursor.clone();
                    match $matcher_variable.match_string(data, &mut trial_cursor) {
                        Ok(value) => {
                            *cursor = trial_cursor;
                            return Ok($enum_name::$variant(value));
                        }
                        Err(error) => errors.push(error),
                    }
                )*

                Err(cursor.variant_err(errors))
            }
        }
    };
}
